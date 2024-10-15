from typing import TypedDict

from typing_extensions import NotRequired

from sift_py.grpc.transport import _clean_uri


class SiftRestConfig(TypedDict):
    """
    Config class used to to interact with services that use Sift's REST API.`.
    - `uri`: The URI of Sift's REST API. The scheme portion of the URI i.e. `https://` should be ommitted.
    - `apikey`: User-generated API key generated via the Sift application.
    - `use_ssl`: INTERNAL USE. Meant to be used for local development.
    """

    uri: str
    apikey: str
    use_ssl: NotRequired[bool]


def compute_uri(restconf: SiftRestConfig) -> str:
    uri = restconf["uri"]
    use_ssl = restconf.get("use_ssl", True)
    clean_uri = _clean_uri(uri, use_ssl)

    if use_ssl:
        return f"https://{clean_uri}"

    return f"http://{clean_uri}"
