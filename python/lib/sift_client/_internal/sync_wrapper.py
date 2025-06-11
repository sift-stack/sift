"""
Utility for generating synchronous API wrappers from asynchronous API classes.
"""
from __future__ import annotations

import asyncio
import inspect
import pathlib
import os
from functools import wraps
from typing import Type, Any, Dict, Set, TypeVar, Generic, Callable

# registry of all classes decorated with @generate_sync_api
_registered: list[tuple[Type[Any],Type[Any]]] = []

T = TypeVar('T')
S = TypeVar('S')

def generate_sync_api(cls: Type[T]) -> Type[S]:
    """
    Generate a synchronous wrapper class for the given async API class.
    
    It creates a new class whose name is derived from the async class by
    stripping a trailing 'Async' (e.g. PingAPIAsync -> PingAPI). For each
    public coroutine method on the async class, it defines a sync method that
    invokes the async one on the default loop using run_coroutine_threadsafe.

    Usage:
        from sift_client._internal.sync_wrapper import generate_sync_api
        PingAPI = generate_sync_api(PingAPIAsync)
    
    Returns:
        A new class that wraps the async class with synchronous methods.
    """
    # derive sync class name
    name = cls.__name__
    assert name.endswith('Async'), f"Expected {name} to end with 'Async'"
    sync_name = name[:-5]
    module = cls.__module__

    # Build an __init__ that stores the async implementation:
    def __init__(self, client, *args, **kwargs):
        self._client = client
        self._async_impl = cls(client, *args, **kwargs)

    def _run(self, coro):
        return asyncio.run_coroutine_threadsafe(coro, self._client._default_loop).result()

    namespace = {
        "__module__": module,
        "__doc__": f"Sync counterpart to `{name}`.\n\n{cls.__doc__ or ''}",
        "__init__": __init__,
        "_run": _run,
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

    SyncClass = type(sync_name, (object,), namespace)
    _registered.append((cls, SyncClass))

    return SyncClass