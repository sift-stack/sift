"""Utility for generating synchronous API wrappers from asynchronous API classes."""

from __future__ import annotations

import asyncio
import inspect
import sys
from functools import wraps
from typing import TYPE_CHECKING, Any, TypeVar

from typing_extensions import TypedDict

if TYPE_CHECKING:
    from sift_client.resources._base import ResourceBase


# registry of all classes decorated with @generate_sync_api
class SyncAPIRegistration(TypedDict):
    async_cls: type[Any]
    sync_cls: type[Any]


_registered: list[SyncAPIRegistration] = []

S = TypeVar("S")


def generate_sync_api(
    cls: type[ResourceBase],
    sync_name: str,
    nested_resources: dict[str, type] | None = None,
) -> type:
    """Generate a synchronous wrapper class for the given async API class.

    It creates a new class whose name is derived from the async class by
    stripping a trailing 'Async' (e.g. PingAPIAsync -> PingAPI). For each
    public coroutine method on the async class, it defines a sync method that
    invokes the async one on the default loop using run_coroutine_threadsafe.

    Usage:
        from sift_client._internal.sync_wrapper import generate_sync_api
        PingAPI = generate_sync_api(PingAPIAsync)

    Args:
        cls: The async API class to wrap.
        sync_name: The name of the generated sync class.
        nested_resources: Maps attribute names of nested resource APIs on the async
            class to their already generated sync classes. Each entry becomes a
            property on the sync class that returns a cached sync wrapper around
            the async instance held by the parent (e.g. `client.reports.templates`).

    Returns:
        A new class that wraps the async class with synchronous methods.
    """
    # derive sync class name
    name = cls.__name__
    module = cls.__module__

    orig_init = cls.__init__

    # Build an __init__ that stores the async implementation:
    @wraps(orig_init)
    def __init__(self, *args, **kwargs):  # noqa: N807
        self._async_impl = cls(*args, **kwargs)
        self._async_impl._is_sync = True

    def _run(self, coro):
        client = self._async_impl.client
        loop = client.get_asyncio_loop()

        # Fail fast if the loop has stopped (e.g. the client was closed during
        # teardown). Scheduling onto a stopped loop would block forever because
        # the coroutine can never run.
        loop_running = getattr(client, "is_loop_running", None)
        if loop_running is None:
            loop_running = loop.is_running()
        if not loop_running:
            coro.close()
            raise RuntimeError("Sift client is closed; cannot make synchronous API calls.")

        # No wall-clock cap here: stalled calls are bounded at the transport layer
        # (GrpcConfig/RestConfig request_timeout), and waiting on the whole coroutine
        # lets methods like wait_until_complete honor their own timeout_secs.
        return asyncio.run_coroutine_threadsafe(coro, loop).result()

    namespace = {
        "__module__": module,
        "__doc__": f"Sync counterpart to `{name}`.\n\n{(cls.__doc__ or '').strip()}",
        "__init__": __init__,
        "_run": _run,
        "__qualname__": sync_name,  # Add __qualname__ to help static analyzers
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
            is_async_prop = inspect.iscoroutinefunction(func)

            # Capture the current name in the closure
            prop_name = name

            if is_async_prop:
                # wrap the async property getter _prop_name passed to ensure name is correct when called
                @property  # type: ignore[misc]
                @wraps(func)
                def sync_prop_wrapper(self, _prop_name=prop_name):
                    # Directly call the original function with the async implementation as self
                    coro = getattr(self._async_impl, _prop_name)
                    return self._run(coro)

                namespace[name] = sync_prop_wrapper

            else:
                # wrap the sync property getter _prop_name passed to ensure name is correct when called
                @property  # type: ignore[misc]
                @wraps(func)
                def sync_prop(self, _prop_name=prop_name):
                    # Access the property directly using getattr with the captured name
                    return getattr(self._async_impl, _prop_name)

                namespace[name] = sync_prop

            continue

        # ───────── staticmethod ─────────
        if isinstance(attr, staticmethod):
            # Currently assumes that we have the _async_impl which is from class instantiation.
            raise NotImplementedError("staticmethod is not supported sync_wrapper")

        # ───────── classmethod ─────────
        if isinstance(attr, classmethod):
            # Currently assumes that we have the _async_impl which is from class instantiation.
            raise NotImplementedError("classmethod is not supported for sync_wrapper")

        # ───────── plain method ─────────
        if inspect.iscoroutinefunction(attr):
            namespace[name] = _make_sync(name)
            continue

        namespace[name] = _wrap_sync(name)

    # ───────── nested resource APIs ─────────
    def _make_nested_resource_property(attr_name: str, nested_sync_cls: type) -> property:
        cache_attr = f"_{attr_name}_sync"

        def fget(self):
            cached = self.__dict__.get(cache_attr)
            if cached is None:
                # Wrap the async instance the parent already holds so patches or
                # state on it are visible through both the sync and async surfaces.
                nested_async_impl = getattr(self._async_impl, attr_name)
                wrapper = object.__new__(nested_sync_cls)
                wrapper._async_impl = nested_async_impl
                nested_async_impl._is_sync = True
                # setdefault keeps a single winner if two threads race the first access.
                cached = self.__dict__.setdefault(cache_attr, wrapper)
            return cached

        fget.__name__ = attr_name
        fget.__qualname__ = f"{sync_name}.{attr_name}"
        fget.__annotations__ = {"return": nested_sync_cls.__name__}
        fget.__doc__ = f"Nested {nested_sync_cls.__name__} for making synchronous requests."
        return property(fget)

    if nested_resources:
        for attr_name, nested_sync_cls in nested_resources.items():
            namespace[attr_name] = _make_nested_resource_property(attr_name, nested_sync_cls)

    # Create the sync class
    sync_class = type(sync_name, (object,), namespace)

    # Register the class in the module's globals
    # This helps static analysis tools recognize it as a proper class
    if module in sys.modules:
        module_globals = sys.modules[module].__dict__
        module_globals[sync_name] = sync_class

    _registered.append(SyncAPIRegistration(async_cls=cls, sync_cls=sync_class))

    return sync_class
