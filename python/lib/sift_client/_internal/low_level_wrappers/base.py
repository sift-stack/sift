from __future__ import annotations

from abc import ABC
from typing import Any, Callable


class LowLevelClientBase(ABC):
    @staticmethod
    async def _handle_pagination(
        func: Callable,
        kwargs: dict[str, Any] | None = None,
        page_size: int | None = None,
        page_token: str | None = None,
        order_by: str | None = None,
        max_results: int | None = None,
    ) -> list[Any]:
        """Handle pagination for a given function by calling the function until all results are retrieved or the max_results is reached.

        Args:
            func: The function to call.
            kwargs: Keyword arguments to pass to the function.
            page_size: The number of results to return per page.
            page_token: The token to use for the next page.
            order_by: How to order the retrieved results.
            max_results: Maximum number of results to return.

        Returns:
            A list of all matching results.
        """
        if kwargs is None:
            kwargs = {}

        results: list[Any] = []
        if max_results == 0:
            return results
        if page_token is None:
            page_token = ""
        while True:
            if max_results is not None and len(results) >= max_results:
                break
            response, page_token = await func(
                page_size=page_size,
                page_token=page_token,
                order_by=order_by,
                **kwargs,
            )
            results.extend(response)
            if page_token == "":
                break
        if max_results and len(results) > max_results:
            results = results[:max_results]
        return results
