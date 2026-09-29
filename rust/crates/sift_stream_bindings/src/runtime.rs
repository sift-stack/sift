//! The tokio runtime that backs every awaitable this crate hands to Python.
//!
//! `pyo3_async_runtimes::tokio` keeps its runtime in a `OnceLock` with no way to
//! shut it down, so its worker threads stay alive through interpreter
//! finalization. A worker that completes a future after CPython has torn down
//! its thread-state bookkeeping re-enters Python to resolve the asyncio future
//! and crashes the process: SIGSEGV once `PyGILState_Ensure` dereferences the
//! freed interpreter state, or SIGABRT when CPython calls `pthread_exit` on a
//! thread that has Rust frames on its stack.
//!
//! This module owns the runtime instead. [`shutdown`] stops it and joins its
//! threads while the interpreter is still intact, and the module init registers
//! it with `atexit` so normal interpreter shutdown always runs it. Everything
//! else (task locals, resolving the asyncio future, cancellation) still comes
//! from `pyo3_async_runtimes::generic`, driven through [`SiftRuntime`].

use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use pyo3_async_runtimes::TaskLocals;
use pyo3_async_runtimes::generic::{self, ContextExt, Runtime as GenericRuntime};
use std::cell::OnceCell;
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Mutex, OnceLock, PoisonError};
use std::time::Duration;
use tokio::runtime::{Builder, Handle, Runtime};

/// How long [`shutdown`] waits for in-flight work when the caller gives no timeout.
pub const DEFAULT_SHUTDOWN_TIMEOUT_SECS: f64 = 5.0;

/// The runtime itself. `None` before first use and again after [`shutdown`].
static RUNTIME: Mutex<Option<Runtime>> = Mutex::new(None);
/// Handle used to spawn work. Set once, when the runtime is first built.
static HANDLE: OnceLock<Handle> = OnceLock::new();
/// Set by [`shutdown`] before it drains `RUNTIME`; never cleared.
static SHUT_DOWN: AtomicBool = AtomicBool::new(false);
/// Runtime threads (workers and blocking pool) currently alive, kept by tokio's thread hooks.
static LIVE_THREADS: AtomicUsize = AtomicUsize::new(0);

tokio::task_local! {
    static TASK_LOCALS: OnceCell<TaskLocals>;
}

/// Marker type that plugs this crate's runtime into `pyo3_async_runtimes::generic`.
pub(crate) struct SiftRuntime;

impl GenericRuntime for SiftRuntime {
    type JoinError = tokio::task::JoinError;
    type JoinHandle = tokio::task::JoinHandle<()>;

    fn spawn<F>(fut: F) -> Self::JoinHandle
    where
        F: Future<Output = ()> + Send + 'static,
    {
        handle().spawn(fut)
    }

    fn spawn_blocking<F>(f: F) -> Self::JoinHandle
    where
        F: FnOnce() + Send + 'static,
    {
        handle().spawn_blocking(f)
    }
}

impl ContextExt for SiftRuntime {
    fn scope<F, R>(locals: TaskLocals, fut: F) -> Pin<Box<dyn Future<Output = R> + Send>>
    where
        F: Future<Output = R> + Send + 'static,
    {
        let cell = OnceCell::new();
        cell.set(locals)
            .unwrap_or_else(|_| unreachable!("fresh OnceCell is empty"));
        Box::pin(TASK_LOCALS.scope(cell, fut))
    }

    fn get_task_locals() -> Option<TaskLocals> {
        TASK_LOCALS
            .try_with(|c| c.get().cloned())
            .unwrap_or_default()
    }
}

/// Converts a Rust future into a Python awaitable that runs on this crate's runtime.
///
/// Drop-in replacement for `pyo3_async_runtimes::tokio::future_into_py`. Raises
/// `RuntimeError` once [`shutdown`] has run, since there is no runtime left to
/// drive the future.
pub(crate) fn future_into_py<'py, F, T>(py: Python<'py>, fut: F) -> PyResult<Bound<'py, PyAny>>
where
    F: Future<Output = PyResult<T>> + Send + 'static,
    T: for<'a> IntoPyObject<'a> + Send + 'static,
{
    ensure_started()?;
    let locals = generic::get_current_locals::<SiftRuntime>(py)?;
    generic::future_into_py_with_locals::<SiftRuntime, _, _>(py, locals, fut)
}

/// Whether [`shutdown`] has run.
pub(crate) fn is_shut_down() -> bool {
    SHUT_DOWN.load(Ordering::SeqCst)
}

/// Stops the runtime and waits up to `timeout` for its threads to finish.
///
/// Work still in flight is cancelled at its next await point, awaitables that
/// have not completed never resolve, and every later call that needs the
/// runtime raises `RuntimeError`. The GIL is released while waiting so that a
/// task which is already resolving a Python future can finish. Calling this more
/// than once is harmless.
///
/// Returns the number of runtime threads still alive when the wait ended. Tokio
/// gives no signal when `shutdown_timeout` gives up, so this is how a caller
/// learns that threads outlived the timeout and could still re-enter Python.
pub(crate) fn shutdown(py: Python<'_>, timeout: Duration) -> usize {
    SHUT_DOWN.store(true, Ordering::SeqCst);
    let runtime = RUNTIME
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .take();
    match runtime {
        Some(runtime) => py.detach(|| {
            runtime.shutdown_timeout(timeout);
            LIVE_THREADS.load(Ordering::SeqCst)
        }),
        None => 0,
    }
}

fn shut_down_err() -> PyErr {
    PyRuntimeError::new_err(
        "the sift_stream_bindings runtime has been shut down; it stops at interpreter exit \
         or when sift_stream_bindings.shutdown() is called",
    )
}

/// Builds the runtime on first use. Fails with `RuntimeError` after [`shutdown`].
fn ensure_started() -> PyResult<()> {
    if is_shut_down() {
        return Err(shut_down_err());
    }
    if HANDLE.get().is_some() {
        return Ok(());
    }

    let mut slot = RUNTIME.lock().unwrap_or_else(PoisonError::into_inner);
    if HANDLE.get().is_some() {
        // Another thread built it while we waited for the lock.
        return Ok(());
    }
    let runtime = Builder::new_multi_thread()
        .enable_all()
        .thread_name("sift-stream-worker")
        .on_thread_start(|| {
            LIVE_THREADS.fetch_add(1, Ordering::SeqCst);
        })
        .on_thread_stop(|| {
            LIVE_THREADS.fetch_sub(1, Ordering::SeqCst);
        })
        .build()
        .map_err(|e| {
            PyRuntimeError::new_err(format!(
                "failed to start the sift_stream_bindings runtime: {e}"
            ))
        })?;
    let _ = HANDLE.set(runtime.handle().clone());
    // `shutdown` sets the flag before draining the slot, so if it ran while we
    // were building, drop this runtime now rather than leave its threads alive.
    if is_shut_down() {
        drop(slot);
        drop(runtime);
        return Err(shut_down_err());
    }
    *slot = Some(runtime);
    Ok(())
}

/// The spawn handle. Every caller goes through [`future_into_py`], which runs
/// [`ensure_started`] first, so the handle exists by the time this is reached.
fn handle() -> &'static Handle {
    HANDLE
        .get()
        .expect("sift_stream_bindings runtime is started before work is spawned on it")
}
