"""Regression tests for the crate-owned tokio runtime and its shutdown.

Runtime threads that finish work while the interpreter is finalizing used to
re-enter Python and crash the process (SIGSEGV/SIGABRT on Linux). The module
now registers ``shutdown()`` with ``atexit`` so the runtime is stopped and its
threads joined before finalization starts.

Every scenario runs in a subprocess: ``shutdown()`` is process-wide and cannot
be undone, and the crash under test happens at interpreter exit.
"""

import os
import socket
import struct
import subprocess
import sys
import textwrap
import threading

import pytest
from sift_stream_bindings import is_shut_down, shutdown

CHILD_TIMEOUT_S = 60


class ResettingListener:
    """Accepts and holds connections; resets them all when told to over a control socket.

    Each ``build()`` against ``uri`` connects and then waits for a gRPC reply that never
    comes. Resetting the connection fails the build a few milliseconds later, which makes a
    runtime thread re-enter Python to reject the awaitable. The control socket lets the child
    choose the exact moment, including from inside interpreter finalization.
    """

    def __init__(self) -> None:
        self._data = socket.create_server(("127.0.0.1", 0), backlog=128)
        self._ctrl = socket.create_server(("127.0.0.1", 0), backlog=16)
        self.uri = f"http://127.0.0.1:{self._data.getsockname()[1]}"
        self.ctrl_port = self._ctrl.getsockname()[1]
        self._lock = threading.Lock()
        self._conns: list[socket.socket] = []
        threading.Thread(target=self._accept_data, daemon=True).start()
        threading.Thread(target=self._accept_ctrl, daemon=True).start()

    def _accept_data(self) -> None:
        while True:
            conn, _ = self._data.accept()
            with self._lock:
                self._conns.append(conn)

    def _accept_ctrl(self) -> None:
        while True:
            conn, _ = self._ctrl.accept()
            threading.Thread(target=self._reset_on_signal, args=(conn,), daemon=True).start()

    def _reset_on_signal(self, ctrl: socket.socket) -> None:
        try:
            ctrl.recv(2)
        finally:
            with self._lock:
                conns, self._conns = self._conns, []
            for conn in conns:
                # SO_LINGER 0 turns close() into a TCP RST.
                conn.setsockopt(socket.SOL_SOCKET, socket.SO_LINGER, struct.pack("ii", 1, 0))
                conn.close()


def run_child(source: str, *args: str) -> subprocess.CompletedProcess:
    return subprocess.run(
        [sys.executable, "-c", textwrap.dedent(source), *args],
        capture_output=True,
        text=True,
        timeout=CHILD_TIMEOUT_S,
        env={**os.environ, "RUST_BACKTRACE": "0"},
    )


def assert_exited_cleanly(child: subprocess.CompletedProcess) -> None:
    assert child.returncode == 0, f"child exited {child.returncode}\n{child.stderr}"
    assert "panicked" not in child.stderr, child.stderr
    assert "Fatal Python error" not in child.stderr, child.stderr


# Starts a few builds that stay pending, then arranges for their connections to be reset
# from a __del__ that runs during module teardown, after CPython has marked itself
# finalizing. Without the atexit shutdown the runtime threads re-enter Python at that
# point: pyo3 panics on every worker, and on Linux the process dies by signal.
PENDING_BUILDS_THEN_EXIT = """
    import asyncio, os, socket, sys, time
    from sift_stream_bindings import IngestionConfigFormPy, SiftStreamBuilderPy

    uri, ctrl_port, mode = sys.argv[1], int(sys.argv[2]), sys.argv[3]
    ctrl = socket.create_connection(("127.0.0.1", ctrl_port))
    ctrl_fd = os.dup(ctrl.fileno())  # a raw fd survives module teardown; the socket object may not

    async def start_builds():
        for _ in range(5):
            builder = SiftStreamBuilderPy(uri=uri, apikey="not-a-real-key")
            builder.enable_tls = False
            builder.ingestion_config_form = IngestionConfigFormPy(
                asset_name="repro", client_key="repro", flows=[]
            )
            builder.build()
        await asyncio.sleep(0.6)  # let every build connect and block on its first request

    asyncio.run(start_builds())

    class ResetDuringFinalization:
        def __init__(self, fd, write, sleep):
            self.fd, self.write, self.sleep = fd, write, sleep

        def __del__(self):
            self.write(self.fd, b"go")
            self.sleep(0.5)  # releases the GIL so runtime threads can try to re-enter

    _keep = ResetDuringFinalization(ctrl_fd, os.write, time.sleep)
    if mode == "explicit-shutdown":
        import sift_stream_bindings
        sift_stream_bindings.shutdown()
        assert sift_stream_bindings.is_shut_down()
"""


@pytest.mark.parametrize("mode", ["atexit", "explicit-shutdown"])
def test_runtime_threads_do_not_reenter_python_during_finalization(mode: str) -> None:
    listener = ResettingListener()
    child = run_child(PENDING_BUILDS_THEN_EXIT, listener.uri, str(listener.ctrl_port), mode)
    assert_exited_cleanly(child)


def test_shutdown_is_idempotent_and_blocks_new_work() -> None:
    child = run_child(
        """
        import asyncio
        import sift_stream_bindings
        from sift_stream_bindings import IngestionConfigFormPy, SiftStreamBuilderPy

        assert not sift_stream_bindings.is_shut_down()
        sift_stream_bindings.shutdown(timeout=1.0)
        sift_stream_bindings.shutdown()  # second call is a no-op
        assert sift_stream_bindings.is_shut_down()

        async def main():
            builder = SiftStreamBuilderPy(uri="http://127.0.0.1:1", apikey="k")
            builder.ingestion_config_form = IngestionConfigFormPy(
                asset_name="a", client_key="a", flows=[]
            )
            try:
                builder.build()
            except RuntimeError as e:
                assert "shut down" in str(e), e
            else:
                raise AssertionError("build() should fail after shutdown()")

        asyncio.run(main())
        """
    )
    assert_exited_cleanly(child)


def test_shutdown_before_any_use_is_safe() -> None:
    child = run_child(
        """
        import sift_stream_bindings
        sift_stream_bindings.shutdown()
        assert sift_stream_bindings.is_shut_down()
        """
    )
    assert_exited_cleanly(child)


def test_shutdown_warns_when_threads_outlive_timeout() -> None:
    # Builds that are blocked on a server that never replies keep runtime threads busy. A zero
    # timeout cannot wait for them, so shutdown() must say that threads are still alive.
    listener = ResettingListener()
    child = run_child(
        """
        import asyncio, sys, warnings
        import sift_stream_bindings
        from sift_stream_bindings import IngestionConfigFormPy, SiftStreamBuilderPy

        async def main():
            for _ in range(3):
                builder = SiftStreamBuilderPy(uri=sys.argv[1], apikey="not-a-real-key")
                builder.enable_tls = False
                builder.ingestion_config_form = IngestionConfigFormPy(
                    asset_name="repro", client_key="repro", flows=[]
                )
                builder.build()
            await asyncio.sleep(0.3)
            with warnings.catch_warnings(record=True) as caught:
                warnings.simplefilter("always")
                sift_stream_bindings.shutdown(timeout=0.0)
            messages = [str(w.message) for w in caught if w.category is RuntimeWarning]
            assert any("timed out" in m and "still running" in m for m in messages), caught

        asyncio.run(main())
        """,
        listener.uri,
    )
    assert_exited_cleanly(child)


def test_shutdown_rejects_bad_timeout() -> None:
    # Runs in-process: an invalid timeout is rejected before the runtime is touched.
    assert not is_shut_down()
    with pytest.raises(ValueError):
        shutdown(timeout=-1.0)
    with pytest.raises(ValueError):
        shutdown(timeout=float("nan"))
    assert not is_shut_down()
