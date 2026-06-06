from __future__ import annotations

from contextlib import contextmanager

from sift_client._internal.util import progress


class _FakeStream:
    """Minimal stream exposing the attributes alive_progress probes for."""

    def write(self, *_): ...

    def flush(self): ...

    def fileno(self):
        return 1


class _WriteOnlyStream:
    """Stream missing fileno, which alive_progress rejects."""

    def write(self, *_): ...

    def flush(self): ...


def test_stdout_supports_progress_true_for_streamlike(monkeypatch):
    monkeypatch.setattr("sys.stdout", _FakeStream())
    assert progress._stdout_supports_progress() is True


def test_stdout_supports_progress_false_when_none(monkeypatch):
    monkeypatch.setattr("sys.stdout", None)
    assert progress._stdout_supports_progress() is False


def test_stdout_supports_progress_false_when_missing_fileno(monkeypatch):
    monkeypatch.setattr("sys.stdout", _WriteOnlyStream())
    assert progress._stdout_supports_progress() is False


def test_alive_bar_noop_does_not_raise_when_stdout_none(monkeypatch):
    # Without the guard this raises ValueError: Invalid config value: file=None.
    monkeypatch.setattr("sys.stdout", None)
    with progress.alive_bar(100, disable=True) as bar:
        bar(10)
        bar.title("polling")  # arbitrary bar attribute resolves to a no-op callable


def test_alive_bar_does_not_touch_real_bar_when_stdout_none(monkeypatch):
    def _boom(*_args, **_kwargs):
        raise AssertionError("real alive_bar must not be called when stdout is None")

    monkeypatch.setattr("sys.stdout", None)
    monkeypatch.setattr(progress, "_alive_bar", _boom)
    with progress.alive_bar(5) as bar:
        bar(1)


def test_alive_bar_delegates_to_real_bar_when_supported(monkeypatch):
    sentinel = object()

    @contextmanager
    def _fake_alive_bar(*_args, **_kwargs):
        yield sentinel

    monkeypatch.setattr("sys.stdout", _FakeStream())
    monkeypatch.setattr(progress, "_alive_bar", _fake_alive_bar)
    with progress.alive_bar(5) as bar:
        assert bar is sentinel
