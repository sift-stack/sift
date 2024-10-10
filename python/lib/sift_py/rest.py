import re
from typing import TypedDict

from typing_extensions import NotRequired


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

    scheme_match = re.match(r"^(\w+://).+", uri)
    if scheme_match:
        raise Exception(f"The URL scheme '{scheme_match.groups()[0]}' should not be included")

    if restconf.get("use_ssl", True):
        return f"https://{uri}"

    return f"http://{uri}"
