## rust(fix): stop the sift-stream-bindings tokio runtime before interpreter exit

### Problem

A customer on `sift-stream-bindings` 0.3.0 reported Python processes dying with SIGSEGV, and sometimes SIGABRT, after their program had finished. The faulting thread was always a `tokio-rt-worker` inside `libpython`. Their production hit rate was 2 of 34 processes. Every stream was awaited and `finish()`ed before exit, so the crash turned successful runs into non-zero exit codes for their supervisor.

The current release (0.5.1) has the same bug. Both versions use pyo3-async-runtimes 0.28, which builds its tokio runtime in a process-global `OnceLock` with no shutdown path. Every awaitable the bindings hand to Python is resolved by a runtime thread that re-enters the interpreter to call `set_result` or `set_exception`. CPython runs `atexit`, then marks itself finalizing and tears down its thread-state machinery. A runtime thread that finishes after that point either dereferences freed interpreter state in `PyGILState_Ensure` (SIGSEGV) or gets `pthread_exit` called on it with Rust frames on the stack, which glibc unwinds and Rust aborts (SIGABRT). pyo3's own finalizing guard only applies on Python 3.13 and up and is racy.

### Fix

The bindings now own their tokio runtime (`src/runtime.rs`) and plug it into `pyo3_async_runtimes::generic`, so task locals, cancellation, and future resolution are unchanged. All sixteen `future_into_py` call sites route through it.

- New `shutdown(timeout=5.0)` stops the runtime and joins its threads with the GIL released. The module registers it with `atexit` at import, so a normal interpreter exit always runs it while the interpreter is still intact. It can also be called directly, for example before `os._exit()`.
- New `is_shut_down()` reports whether it has run. Calls that need the runtime afterwards raise `RuntimeError` instead of crashing.
- Runtime threads are named `sift-stream-worker` so future crash logs identify them.

### Verification

Linux container (Debian 13, aarch64, CPython 3.11.16, glibc 2.41), running the customer's repro script unchanged:

| Build | Customer repro, 150 trials | Deterministic repro, 20 trials |
|---|---|---|
| main (0.5.1) | 134 clean, 10 SIGABRT, 6 SIGSEGV | 20 clean, pyo3 panics on every worker |
| this branch | 150 clean | 20 clean, no stderr |

macOS does not unwind Rust frames on `pthread_exit`, so the customer's repro passes there even on main. A deterministic variant that fires connection resets from a `__del__` during module teardown shows the same re-entry on macOS on Python 3.11 and 3.13, and is clean with the fix.

- Bindings test suite: 63 passed on Linux 3.11, macOS 3.11, and macOS 3.13. Four new subprocess tests in `tests/test_runtime_shutdown.py` cover the finalization scenario, explicit `shutdown()`, idempotence, shutdown before first use, and bad timeouts.
- `cargo clippy --all-targets -D warnings` clean. Generated `.pyi` stub unchanged.

### Releases

- `sift-stream-bindings` 0.5.1 to 0.5.2, changelog entry under Unreleased in `rust/CHANGELOG.md`.
- The Python package is not touched here. Once the 0.5.2 wheel is on PyPI, a separate PR bumps `sift-stack-py`, moves its pins to `sift-stream-bindings==0.5.2`, and regenerates `uv.lock`.

### Customer follow-up

Their diagnosis was correct on all three points: a `shutdown()` API now exists, the `atexit` hook lives inside the extension, and runtime ownership did not change between 0.3.0 and 0.5.1. They need the 0.5.2 release, not an upgrade to any existing version.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
