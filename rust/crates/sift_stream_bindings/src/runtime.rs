//! Converts Rust futures into asyncio awaitables without entering Python during finalization.
//!
//! `pyo3_async_runtimes::tokio::future_into_py` resolves each asyncio future by attaching a tokio
//! blocking thread to Python. Its runtime is a process-global that is never shut down, so a call
//! that completes near interpreter exit can still be attached when `Py_FinalizeEx` begins, and
//! the process crashes (SIGSEGV or SIGABRT) after the Python program has already finished. The
//! crash can land after the result is handed over, because `call_soon_threadsafe` runs Python
//! code that may give up the GIL to the exiting main thread before the tokio thread detaches.
//!
//! This module keeps the same runtime but resolves futures itself, so it can count every thread
//! that is attached for delivery until it has fully detached. Python runs `atexit` handlers
//! before it starts finalizing; the handler registered here stops new deliveries and waits,
//! with the GIL released, for in-flight ones to detach. Every async method in this crate must go
//! through [`future_into_py`].

use pyo3::{IntoPyObjectExt, prelude::*, types::PyDict};
use pyo3_async_runtimes::err::RustPanic;
use std::{
    future::Future,
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
    time::{Duration, Instant},
};
use tokio::task::AbortHandle;

static SHUTTING_DOWN: AtomicBool = AtomicBool::new(false);

/// Deliveries that may be attached to Python. Released only after the thread detaches.
static IN_DELIVERY: AtomicUsize = AtomicUsize::new(0);

/// Upper bound on how long interpreter exit waits for in-flight deliveries.
const DELIVERY_TIMEOUT: Duration = Duration::from_secs(2);

/// Replacement for `pyo3_async_runtimes::tokio::future_into_py` with the same semantics:
/// the returned asyncio future resolves with the Rust result, a Rust panic surfaces as
/// `RustPanic`, and cancelling the asyncio future drops the Rust future.
pub fn future_into_py<F, T>(py: Python<'_>, fut: F) -> PyResult<Bound<'_, PyAny>>
where
    F: Future<Output = PyResult<T>> + Send + 'static,
    T: for<'py> IntoPyObject<'py> + Send + 'static,
{
    let locals = pyo3_async_runtimes::tokio::get_current_locals(py)?;
    let event_loop = locals.event_loop(py);
    let py_fut = event_loop.call_method0(pyo3::intern!(py, "create_future"))?;

    let runtime = pyo3_async_runtimes::tokio::get_runtime();
    let work = runtime.spawn(fut);
    py_fut.call_method1(
        pyo3::intern!(py, "add_done_callback"),
        (CancelOnDone(work.abort_handle()),),
    )?;

    let target = py_fut.clone().unbind();
    let event_loop = event_loop.unbind();
    let context = locals.context(py).unbind();
    runtime.spawn(async move {
        let result = match work.await {
            Ok(result) => result,
            Err(e) if e.is_panic() => Err(RustPanic::new_err(format!(
                "rust future panicked: {}",
                panic_message(&e.into_panic())
            ))),
            // Cancelled from Python; nothing to deliver.
            Err(_) => return,
        };

        // Pairs with the SeqCst store and load in `on_exit`: either the handler sees this
        // increment and waits for it, or this task sees the flag and never attaches.
        IN_DELIVERY.fetch_add(1, Ordering::SeqCst);
        if SHUTTING_DOWN.load(Ordering::SeqCst) {
            IN_DELIVERY.fetch_sub(1, Ordering::SeqCst);
            // Park instead of dropping so Python references are never released off the GIL
            // during exit.
            return std::future::pending().await;
        }

        // Holding the GIL inside a runtime worker would block other tasks.
        // Detached: the delivery is tracked by `IN_DELIVERY`, not by this handle.
        drop(tokio::task::spawn_blocking(move || {
            Python::attach(|py| deliver(py, &target, &event_loop, &context, result));
            // Only now is this thread detached from the interpreter.
            IN_DELIVERY.fetch_sub(1, Ordering::SeqCst);
        }));
    });

    Ok(py_fut)
}

/// Schedules the result onto the event loop thread, which owns the asyncio future.
fn deliver<T: for<'py> IntoPyObject<'py>>(
    py: Python<'_>,
    target: &Py<PyAny>,
    event_loop: &Py<PyAny>,
    context: &Py<PyAny>,
    result: PyResult<T>,
) {
    let scheduled = (|| {
        let (value, is_err) = match result.and_then(|v| v.into_py_any(py)) {
            Ok(value) => (value, false),
            Err(err) => (err.into_value(py).into_any(), true),
        };
        let kwargs = PyDict::new(py);
        kwargs.set_item(pyo3::intern!(py, "context"), context)?;
        event_loop.bind(py).call_method(
            pyo3::intern!(py, "call_soon_threadsafe"),
            (wrap_pyfunction!(complete, py)?, target, value, is_err),
            Some(&kwargs),
        )?;
        PyResult::Ok(())
    })();

    // The loop may already be closed if its owner exited without awaiting this call.
    if let Err(err) = scheduled {
        err.print(py);
    }
}

/// Runs on the event loop thread. Skips futures the caller has already cancelled.
#[pyfunction]
fn complete(target: &Bound<'_, PyAny>, value: &Bound<'_, PyAny>, is_err: bool) -> PyResult<()> {
    let py = target.py();
    if target
        .call_method0(pyo3::intern!(py, "cancelled"))?
        .is_truthy()?
    {
        return Ok(());
    }
    let method = if is_err {
        pyo3::intern!(py, "set_exception")
    } else {
        pyo3::intern!(py, "set_result")
    };
    target.call_method1(method, (value,))?;
    Ok(())
}

/// asyncio done callback that drops the Rust future when the caller cancels.
#[pyclass]
struct CancelOnDone(AbortHandle);

#[pymethods]
impl CancelOnDone {
    fn __call__(&self, fut: &Bound<'_, PyAny>) -> PyResult<()> {
        if fut
            .call_method0(pyo3::intern!(fut.py(), "cancelled"))?
            .is_truthy()?
        {
            self.0.abort();
        }
        Ok(())
    }
}

fn panic_message(payload: &(dyn std::any::Any + Send)) -> &str {
    if let Some(s) = payload.downcast_ref::<&str>() {
        s
    } else if let Some(s) = payload.downcast_ref::<String>() {
        s
    } else {
        "unknown error"
    }
}

#[pyfunction]
fn on_exit(py: Python<'_>) {
    SHUTTING_DOWN.store(true, Ordering::SeqCst);

    // Delivering threads need the GIL to finish, so release it while waiting.
    py.detach(|| {
        let deadline = Instant::now() + DELIVERY_TIMEOUT;
        while IN_DELIVERY.load(Ordering::SeqCst) > 0 && Instant::now() < deadline {
            std::thread::sleep(Duration::from_millis(1));
        }
    });
}

/// Registers the interpreter exit hook. Called once from module init.
pub fn register_exit_hook(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let hook = wrap_pyfunction!(on_exit, m)?;
    m.py().import("atexit")?.call_method1("register", (hook,))?;
    Ok(())
}
