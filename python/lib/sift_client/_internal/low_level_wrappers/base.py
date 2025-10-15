from __future__ import annotations

from abc import ABC
from typing import Any, Callable, TypeVar

from sift_py.grpc.cache import ignore_cache, with_cache, with_force_refresh

T = TypeVar("T")


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

    @staticmethod
    async def _call_with_cache(
        stub_method: Callable[[Any, tuple[tuple[str, str], ...]], T],
        request: Any,
        *,
        use_cache: bool = True,
        force_refresh: bool = False,
        ttl: int | None = None,
    ) -> T:
        """Call a gRPC stub method with cache control.
        
        This is a convenience method for low-level wrappers to easily enable caching
        on their gRPC calls without manually constructing metadata.
        
        Args:
            stub_method: The gRPC stub method to call (e.g., stub.GetData).
            request: The protobuf request object.
            use_cache: Whether to enable caching for this request. Default: True.
            force_refresh: Whether to force refresh the cache. Default: False.
            ttl: Optional custom TTL in seconds. If not provided, uses the default TTL.
        
        Returns:
            The response from the gRPC call.
        
        Example:
            # Enable caching
            response = await self._call_with_cache(
                stub.GetData,
                request,
                use_cache=True,
            )
            
            # Force refresh
            response = await self._call_with_cache(
                stub.GetData,
                request,
                force_refresh=True,
            )
            
            # With custom TTL
            response = await self._call_with_cache(
                stub.GetData,
                request,
                use_cache=True,
                ttl=7200,  # 2 hours
            )
            
            # Ignore cache
            response = await self._call_with_cache(
                stub.GetData,
                request,
                use_cache=False,
            )
        """

        if force_refresh:
            metadata = with_force_refresh(ttl=ttl)
        elif use_cache:
            metadata = with_cache(ttl=ttl)
        else:
            metadata = ignore_cache()

        return await stub_method(request, metadata=metadata)
