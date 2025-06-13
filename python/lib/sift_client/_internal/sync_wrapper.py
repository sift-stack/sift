"""
Utility for generating synchronous API wrappers from asynchronous API classes.
"""

from __future__ import annotations

import asyncio
import inspect
from functools import wraps
from typing import Any, Type, TypeVar

from typing_extensions import TypedDict

from sift_client.transport.base_connection import WithGrpcClient


# registry of all classes decorated with @generate_sync_api
class SyncAPIRegistration(TypedDict):
    async_cls: Type[Any]
    sync_cls: Type[Any]


_registered: list[SyncAPIRegistration] = []

T = TypeVar("T")
S = TypeVar("S")


def generate_sync_api(cls: Type[WithGrpcClient], sync_name: str) -> Type[S]:
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
    module = cls.__module__

    orig_init = cls.__init__

    # Build an __init__ that stores the async implementation:
    @wraps(orig_init)
    def __init__(self, *args, **kwargs):
        self._async_impl = cls(*args, **kwargs)

    def _run(self, coro):
        return asyncio.run_coroutine_threadsafe(coro, self._async_impl._get_loop()).result()

    namespace = {
        "__module__": module,
        "__doc__": f"Sync counterpart to `{name}`.\n\n{cls.__doc__ or ''}",
        "__init__": __init__,
        "_run": _run,
    }

    # helper to wrap an async method and make into a sync method
    def _make_sync(func_name: str):
        async_func = getattr(cls, func_name)

        @wraps(async_func)
        def sync_func(self, *a, **kw):
            return self._run(getattr(self._async_impl, func_name)(*a, **kw))

        return sync_func

    def _wrap_sync(func_name: str):
        func = getattr(cls, func_name)

        @wraps(func)
        def wrapped_func(self, *a, **kw):
            return getattr(self._async_impl, func_name)(*a, **kw)

        return wrapped_func

    for name, attr in cls.__dict__.items():
        if name.startswith("_"):
            continue

        # ───────── property ─────────
        if isinstance(attr, property) and attr.fget:
            func = attr.fget
            # async
            if inspect.iscoroutinefunction(func):

                @property  # type: ignore[misc]
                @wraps(func)
                def sync_prop(self):
                    return self._run(func.__get__(self._async_impl)())
            # sync
            else:

                @property  # type: ignore[misc]
                @wraps(func)
                def sync_prop(self):
                    return func.__get__(self._async_impl)()

            namespace[name] = sync_prop
            continue

        # ───────── staticmethod ─────────
        if isinstance(attr, staticmethod):
            raise NotImplementedError("staticmethod is not supported sync_wrapper")

        # ───────── classmethod ─────────
        if isinstance(attr, classmethod):
            raise NotImplementedError("classmethod is not supported for sync_wrapper")

        # ───────── plain method ─────────
        if inspect.iscoroutinefunction(attr):
            namespace[name] = _make_sync(name)
            continue

        namespace[name] = _wrap_sync(name)

    SyncClass = type(sync_name, (object,), namespace)
    _registered.append(SyncAPIRegistration(async_cls=cls, sync_cls=SyncClass))

    return SyncClass
