from __future__ import annotations

import asyncio
import atexit
import inspect
import threading
from typing import Any

import pytest

from sift_client._internal.sync_wrapper import generate_sync_api
from sift_client.resources._base import ResourceBase


class MockClient:
    """Mock client that simulates the SiftClient with an event loop."""

    def __init__(self):
        """Initialize the mock client."""
        self._default_loop = asyncio.new_event_loop()
        atexit.register(self.close_sync)

        def _run_default_loop():
            asyncio.set_event_loop(self._default_loop)
            self._default_loop.run_forever()

        self._default_loop_thread = threading.Thread(
            target=_run_default_loop,
            daemon=True,
        )
        self._default_loop_thread.start()

    def get_asyncio_loop(self):
        """Return the event loop used for async operations."""
        return self._default_loop

    def close_sync(self):
        try:
            self._default_loop.call_soon_threadsafe(self._default_loop.stop)
            self._default_loop_thread.join(timeout=1.0)
        except ValueError:
            ...


class MockResourceAsync(ResourceBase):
    """Mock async resource class for testing the sync wrapper."""

    def __init__(self, client=None, value: str = "default"):
        super().__init__(client)
        self._value = value
        self._calls: dict[str, int] = {}

    @property
    def value(self) -> str:
        """Test property that returns a value."""
        print("sync called")
        self._record_call("value")
        return self._value

    @property
    async def async_value(self) -> str:
        """Test async property that returns a value."""
        print("async called")
        self._record_call("async_value")
        await asyncio.sleep(0.01)  # Small delay to simulate async operation
        return self._value

    def sync_method(self, arg1: str, arg2: int = 0) -> str:
        """Test synchronous method."""
        self._record_call("sync_method")
        return f"{arg1}:{arg2}"

    async def async_method(self, arg1: str, arg2: int = 0) -> str:
        """Test asynchronous method."""
        self._record_call("async_method")
        await asyncio.sleep(0.01)  # Small delay to simulate async operation
        return f"{arg1}:{arg2}"

    async def async_method_with_exception(self) -> None:
        """Test asynchronous method that raises an exception."""
        self._record_call("async_method_with_exception")
        await asyncio.sleep(0.01)
        raise ValueError("Test exception")

    async def async_method_with_complex_args(
        self, arg1: str, arg2: dict[str, Any] | None = None, *args, **kwargs
    ) -> dict[str, Any]:
        """Test asynchronous method with complex arguments."""
        self._record_call("async_method_with_complex_args")
        await asyncio.sleep(0.01)
        result = {
            "arg1": arg1,
            "arg2": arg2 or {},
            "args": args,
            "kwargs": kwargs,
        }
        return result

    def _record_call(self, method_name: str) -> None:
        """Record that a method was called."""
        if method_name not in self._calls:
            self._calls[method_name] = 0
        self._calls[method_name] += 1

    def get_call_count(self, method_name: str) -> int:
        """Get the number of times a method was called."""
        return self._calls.get(method_name, 0)


class TestSyncWrapper:
    """Tests for the sync_wrapper module."""

    @pytest.fixture
    def mock_resource_async(self):
        """Create a mock async resource."""
        mock_client = MockClient()
        return MockResourceAsync(mock_client, value="testVal")

    @pytest.fixture
    def mock_resource_sync(self):
        """Create a mock sync resource."""
        mock_client = MockClient()
        MockResource = generate_sync_api(MockResourceAsync, "MockResource")  # noqa: N806
        return MockResource(mock_client, value="testVal")

    def test_sync_class_creation(self):
        """Test that a sync class is created correctly."""
        MockResource = generate_sync_api(MockResourceAsync, "MockResource")  # noqa: N806

        # Check class properties
        assert MockResource.__name__ == "MockResource"
        assert MockResource.__module__ == MockResourceAsync.__module__
        assert "Sync counterpart to" in MockResource.__doc__ if MockResource.__doc__ else False

        # Check that methods were properly wrapped
        assert hasattr(MockResource, "sync_method")
        assert hasattr(MockResource, "async_method")
        assert hasattr(MockResource, "async_method_with_exception")
        assert hasattr(MockResource, "async_method_with_complex_args")

        # Check that properties were properly wrapped
        assert isinstance(inspect.getattr_static(MockResource, "value"), property)
        assert isinstance(inspect.getattr_static(MockResource, "async_value"), property)

    def test_sync_method_call(self, mock_resource_sync):
        """Test calling a synchronous method on the sync wrapper."""
        result = mock_resource_sync.sync_method("arg", 42)
        assert result == "arg:42"
        assert mock_resource_sync._async_impl.get_call_count("sync_method") == 1

    def test_async_method_call(self, mock_resource_sync):
        """Test calling an asynchronous method on the sync wrapper."""
        result = mock_resource_sync.async_method("arg", 42)
        assert result == "arg:42"
        assert mock_resource_sync._async_impl.get_call_count("async_method") == 1

    def test_property_access(self, mock_resource_sync):
        """Test accessing a property on the sync wrapper."""
        result = mock_resource_sync.value
        assert result == "testVal"
        assert mock_resource_sync._async_impl.get_call_count("value") == 1

    def test_async_property_access(self, mock_resource_sync):
        """Test accessing an async property on the sync wrapper."""
        result = mock_resource_sync.async_value
        assert result == "testVal"
        assert mock_resource_sync._async_impl.get_call_count("async_value") == 1

    def test_exception_propagation(self, mock_resource_sync):
        """Test that exceptions from async methods are propagated correctly."""
        with pytest.raises(ValueError, match="Test exception"):
            mock_resource_sync.async_method_with_exception()
        assert mock_resource_sync._async_impl.get_call_count("async_method_with_exception") == 1

    def test_complex_arguments(self, mock_resource_sync):
        """Test calling a method with complex arguments."""
        result = mock_resource_sync.async_method_with_complex_args(
            "test_arg", {"key": "value"}, "extra_arg", keyword="keyword_value"
        )

        assert result["arg1"] == "test_arg"
        assert result["arg2"] == {"key": "value"}
        assert result["args"] == ("extra_arg",)
        assert result["kwargs"] == {"keyword": "keyword_value"}
        assert mock_resource_sync._async_impl.get_call_count("async_method_with_complex_args") == 1
