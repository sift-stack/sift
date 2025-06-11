"""
Utility for generating synchronous API wrappers from asynchronous API classes.
"""
from __future__ import annotations

from functools import wraps
import asyncio
import inspect
from typing import Type, Any


def generate_sync_api(cls: Type) -> Type:
    """
    Generate a synchronous wrapper class for the given async API class.

    It creates a new class whose name is derived from the async class by
    stripping a trailing 'Async' (e.g. PingAPIAsync -> PingAPI). For each
    public coroutine method on the async class, it defines a sync method that
    invokes the async one on the default loop using run_coroutine_threadsafe.

    Usage:
        from sift_client._internal.utils import generate_sync_api
        PingAPI = generate_sync_api(PingAPIAsync)
    """
    # derive sync class name
    name = cls.__name__
    module = cls.__module__
    assert name.endswith('Async'), f"Expected {name} to end with 'Async'"
    sync_name = name[:-5]

    # Build an __init__ that stores the async implementation:
    def __init__(self, client, *args, **kwargs):
        self._client = client
        self._async_impl = cls(client, *args, **kwargs)

    # method runner
    def _run(self, coro) -> Any:  # noqa: F811
        future = asyncio.run_coroutine_threadsafe(coro, self._client._default_loop)
        return future.result()


    namespace = {
        # Copy over the class docstring
        "__doc__": f"Sync counterpart to `{cls.__name__}`.\n\nOriginal doc:\n{cls.__doc__ or ''}",
        "__module__": module,
        '__init__': __init__,
        '_run': _run,
    }
    # generate sync methods
    for name, func in inspect.getmembers(cls, predicate=inspect.iscoroutinefunction):
        if name.startswith('_'):
            continue

        @wraps(func)
        def make_sync(func_name):  # noqa: F811
            async_method = getattr(cls, func_name)
            @wraps(async_method)
            def sync_method(self, *args, **kwargs):  # noqa: F811
                return self._run(getattr(self._async_impl, func_name)(*args, **kwargs))
            return sync_method
        namespace[name] = make_sync(name)

    return type(sync_name, (object,), namespace)
