from __future__ import annotations

from typing import TYPE_CHECKING

from requests import HTTPError

if TYPE_CHECKING:
    from requests import Response


def raise_for_status(response: Response) -> None:
    """Like ``Response.raise_for_status``, with the server's response body in the message.

    Raises:
        requests.HTTPError: If the response has a 4xx or 5xx status.
    """
    try:
        response.raise_for_status()
    except HTTPError as error:
        body = response.text.strip()
        if body:
            raise HTTPError(f"{error}\nServer response: {body}", response=response) from error
        raise
