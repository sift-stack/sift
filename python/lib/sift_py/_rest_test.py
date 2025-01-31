import io

from urllib3.exceptions import ProtocolError
from urllib3.response import HTTPResponse

from sift_py.rest import RestService, SiftRestConfig


class TestRetryService(RestService):
    def get_dummy(self):
        return self._session.get(f"{self._base_uri}/dummy")


def test_http_adapter_retries(mocker):
    n_tries = 3  # Default configuration is 3 retries

    call_count = 0

    def fake_make_request(self, conn, method, url, timeout=None, chunked=False, **kwargs):
        nonlocal call_count
        call_count += 1
        if call_count == 1:
            raise ProtocolError("Simulated connection error")
        elif call_count < n_tries:
            return HTTPResponse(
                body=io.BytesIO(b"Gateway Timeout"), status=504, headers={}, preload_content=True
            )
        else:
            return HTTPResponse(
                body=io.BytesIO(b"Success"), status=200, headers={}, preload_content=True
            )

    mocker.patch(
        "urllib3.connectionpool.HTTPConnectionPool._make_request",
        side_effect=fake_make_request,
        autospec=True,
    )

    test_config: SiftRestConfig = {
        "uri": "dummy.com",
        "apikey": "dummy",
        # If retry not provided, default one used
    }

    service = TestRetryService(test_config)

    response = service.get_dummy()

    assert call_count == n_tries
    assert response.status_code == 200
