from __future__ import annotations

from unittest.mock import MagicMock

from sift_client._internal.util.file import download_file


class _FakeResponse:
    """Minimal ``requests.Response`` stand-in for the streamed-download tests."""

    def __init__(self, chunks: list[bytes]):
        self._chunks = chunks
        self.headers: dict[str, str] = {}

    def raise_for_status(self) -> None: ...

    def iter_content(self, chunk_size: int):
        return iter(self._chunks)

    def __enter__(self):
        return self

    def __exit__(self, *_):
        return False


def _install_rest_client_stub(response: _FakeResponse) -> tuple[MagicMock, dict]:
    """Return a rest_client mock whose ``.get(...)`` records call kwargs."""
    captured: dict = {}
    rest_client = MagicMock()

    def _fake_get(url, **kwargs):
        captured["url"] = url
        captured.update(kwargs)
        return response

    rest_client.get.side_effect = _fake_get
    return rest_client, captured


class TestDownloadFileExtraHeaders:
    """``extra_headers`` merges on top of the internal ``Authorization: None``
    strip so callers can override the Host header (or add a range, etc.) for
    presigned URLs whose signature covers a specific ``Host`` distinct from the
    URL's authority. Without this the caller has to bypass ``download_file``
    entirely for signed-URL edge cases (e.g. local minio via a container-
    internal alias) and reimplement streaming + auth stripping from scratch.
    """

    def test_no_extra_headers_strips_only_authorization(self, tmp_path):
        rest_client, captured = _install_rest_client_stub(
            _FakeResponse([b"hello ", b"world"])
        )
        target = tmp_path / "out.bin"

        download_file(
            "https://example.com/file?sig=1",
            target,
            rest_client=rest_client,
        )

        assert captured["headers"] == {"Authorization": None}
        assert target.read_bytes() == b"hello world"

    def test_extra_headers_are_forwarded(self, tmp_path):
        rest_client, captured = _install_rest_client_stub(
            _FakeResponse([b"payload"])
        )
        target = tmp_path / "out.bin"

        download_file(
            "https://example.com/file?sig=1",
            target,
            rest_client=rest_client,
            extra_headers={"Host": "signed.example.com", "X-Trace": "abc"},
        )

        assert captured["headers"] == {
            "Authorization": None,
            "Host": "signed.example.com",
            "X-Trace": "abc",
        }
        assert target.read_bytes() == b"payload"

    def test_extra_headers_can_override_authorization_strip(self, tmp_path):
        # The strip is a default, not a hard invariant -- if a caller has a
        # concrete reason to pass an Authorization on the download request
        # (e.g. a proxy-scoped bearer), let their value win.
        rest_client, captured = _install_rest_client_stub(
            _FakeResponse([b"payload"])
        )
        target = tmp_path / "out.bin"

        download_file(
            "https://example.com/file?sig=1",
            target,
            rest_client=rest_client,
            extra_headers={"Authorization": "Bearer proxy-token"},
        )

        assert captured["headers"]["Authorization"] == "Bearer proxy-token"
