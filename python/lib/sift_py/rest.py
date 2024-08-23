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
