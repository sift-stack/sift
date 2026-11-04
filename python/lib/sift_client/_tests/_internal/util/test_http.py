from __future__ import annotations

import pytest
from requests import HTTPError, Request, Response

from sift_client._internal.util.http import raise_for_status

URL = "https://example.test/api/v2/data-imports:upload/import-id"


def _response(status_code: int, reason: str, body: bytes) -> Response:
    response = Response()
    response.status_code = status_code
    response.reason = reason
    response.url = URL
    response.request = Request("POST", URL).prepare()
    response.encoding = "utf-8"
    response._content = body
    return response


class TestRaiseForStatus:
    @pytest.mark.parametrize(
        ("status_code", "reason", "body"),
        [
            (
                400,
                "Bad Request",
                'failed to parse row 20: parsing time "10-29-2026 16:53:45" as '
                '"2006-01-02 15:04:05": cannot parse "10-29-2026 16:53:45" as "2006"',
            ),
            (500, "Internal Server Error", "upload failed\n"),
            (502, "Bad Gateway", "<html>Bad Gateway</html>"),
        ],
    )
    def test_error_includes_server_body(self, status_code, reason, body):
        response = _response(status_code, reason, body.encode("utf-8"))

        with pytest.raises(HTTPError) as exc_info:
            raise_for_status(response)

        message = str(exc_info.value)
        assert body.strip() in message
        assert str(status_code) in message
        assert reason in message
        assert URL in message
        assert exc_info.value.response is response
        assert exc_info.value.request is response.request

    @pytest.mark.parametrize("body", [b"", b" \n\t"])
    def test_empty_body_preserves_http_error(self, body):
        response = _response(400, "Bad Request", body)

        with pytest.raises(HTTPError) as exc_info:
            raise_for_status(response)

        assert str(exc_info.value) == f"400 Client Error: Bad Request for url: {URL}"
        assert exc_info.value.response is response

    def test_success_does_not_raise(self):
        raise_for_status(_response(200, "OK", b'{"jobId":"job-id"}'))
