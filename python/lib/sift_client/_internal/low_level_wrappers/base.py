from __future__ import annotations

from abc import ABC
from typing import Any, Callable


class LowLevelClientBase(ABC):
    @staticmethod
    async def _handle_pagination(
        func: Callable,
        page_size: int | None = None,
        page_token: str | None = None,
        query_filter: str | None = None,
        order_by: str | None = None,
        max_results: int | None = None,
    ) -> list[Any]:
        results: list[Any] = []
        if page_token is None:
            page_token = ""
        while True:
            if max_results is not None and len(results) >= max_results:
                break
            response, page_token = await func(
                page_size=page_size,
                page_token=page_token,
                query_filter=query_filter,
                order_by=order_by,
            )
            results.extend(response)
            if page_token == "":
                break
        return results
