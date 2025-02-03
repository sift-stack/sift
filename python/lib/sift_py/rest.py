from abc import ABC
from typing import TypedDict

import requests
from requests.adapters import HTTPAdapter
from typing_extensions import NotRequired
from urllib3.util import Retry

from sift_py.grpc.transport import _clean_uri

_DEFAULT_REST_RETRY = Retry(total=3, status_forcelist=[500, 502, 503, 504], backoff_factor=1)


class SiftRestConfig(TypedDict):
    """
    Config class used to to interact with services that use Sift's REST API.`.
    - `uri`: The URI of Sift's REST API. The scheme portion of the URI i.e. `https://` should be ommitted.
    - `apikey`: User-generated API key generated via the Sift application.
    - `retry`: Urllib3 Retry configuration. If not provided, a default of 3 retries is used.
    - `use_ssl`: INTERNAL USE. Meant to be used for local development.
    """

    uri: str
    apikey: str
    retry: NotRequired[Retry]
    use_ssl: NotRequired[bool]


def compute_uri(restconf: SiftRestConfig) -> str:
    uri = restconf["uri"]
    use_ssl = restconf.get("use_ssl", True)
    clean_uri = _clean_uri(uri, use_ssl)

    if use_ssl:
        return f"https://{clean_uri}"

    return f"http://{clean_uri}"


class _RestService(ABC):
    """
    Abstract service that implements a REST session.
    """

    def __init__(self, rest_conf: SiftRestConfig):
        self._rest_conf = rest_conf
        self._base_uri = compute_uri(rest_conf)
        self._apikey = rest_conf["apikey"]

        self._session = requests.Session()
        self._session.headers = {"Authorization": f"Bearer {self._apikey}"}
        adapter = HTTPAdapter(max_retries=rest_conf.get("retry", _DEFAULT_REST_RETRY))
        self._session.mount("https://", adapter)
        self._session.mount("http://", adapter)
