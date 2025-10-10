"""Tests for LowLevelClientBase.

These tests validate the functionality of the LowLevelClientBase class including:
- Pagination handling with various scenarios
- Edge cases and error handling
- Parameter validation and behavior
"""

from unittest.mock import AsyncMock

import pytest

from sift_client._internal.low_level_wrappers.base import LowLevelClientBase


class TestLowLevelClientBase:
    """Test suite for LowLevelClientBase functionality."""

    class TestHandlePagination:
        """Tests for the _handle_pagination static method."""

        @pytest.mark.asyncio
        async def test_basic_pagination_single_page(self):
            """Test pagination with a single page of results."""
            # Mock function that returns results and empty page token (indicating no more pages)
            mock_func = AsyncMock(return_value=([1, 2, 3], ""))

            results = await LowLevelClientBase._handle_pagination(mock_func)

            assert results == [1, 2, 3]
            mock_func.assert_called_once_with(page_size=None, page_token="", order_by=None)

        @pytest.mark.asyncio
        async def test_pagination_multiple_pages(self):
            """Test pagination across multiple pages."""
            # Mock function that returns different results for different page tokens
            mock_func = AsyncMock()
            mock_func.side_effect = [
                ([1, 2, 3], "token1"),  # First page
                ([4, 5, 6], "token2"),  # Second page
                ([7, 8, 9], ""),  # Last page (empty token)
            ]

            results = await LowLevelClientBase._handle_pagination(mock_func)

            assert results == [1, 2, 3, 4, 5, 6, 7, 8, 9]
            assert mock_func.call_count == 3

            # Verify the calls were made with correct page tokens
            calls = mock_func.call_args_list
            assert calls[0][1]["page_token"] == ""
            assert calls[1][1]["page_token"] == "token1"
            assert calls[2][1]["page_token"] == "token2"

        @pytest.mark.asyncio
        async def test_pagination_with_page_size(self):
            """Test pagination with specified page size."""
            mock_func = AsyncMock(return_value=([1, 2], ""))

            results = await LowLevelClientBase._handle_pagination(mock_func, page_size=2)

            assert results == [1, 2]
            mock_func.assert_called_once_with(page_size=2, page_token="", order_by=None)

        @pytest.mark.asyncio
        async def test_pagination_with_order_by(self):
            """Test pagination with order_by parameter."""
            mock_func = AsyncMock(return_value=([1, 2, 3], ""))

            results = await LowLevelClientBase._handle_pagination(mock_func, order_by="name asc")

            assert results == [1, 2, 3]
            mock_func.assert_called_once_with(page_size=None, page_token="", order_by="name asc")

        @pytest.mark.asyncio
        async def test_pagination_with_initial_page_token(self):
            """Test pagination starting with a specific page token."""
            mock_func = AsyncMock(return_value=([4, 5, 6], ""))

            results = await LowLevelClientBase._handle_pagination(
                mock_func, page_token="start_token"
            )

            assert results == [4, 5, 6]
            mock_func.assert_called_once_with(
                page_size=None, page_token="start_token", order_by=None
            )

        @pytest.mark.asyncio
        async def test_pagination_with_kwargs(self):
            """Test pagination with additional keyword arguments."""
            mock_func = AsyncMock(return_value=([1, 2, 3], ""))
            kwargs = {"filter": "active", "include_archived": False}

            results = await LowLevelClientBase._handle_pagination(mock_func, kwargs=kwargs)

            assert results == [1, 2, 3]
            mock_func.assert_called_once_with(
                page_size=None,
                page_token="",
                order_by=None,
                filter="active",
                include_archived=False,
            )

        @pytest.mark.asyncio
        async def test_pagination_with_max_results_single_page(self):
            """Test pagination with max_results that fits in a single page."""
            mock_func = AsyncMock(return_value=([1, 2, 3, 4, 5], ""))

            results = await LowLevelClientBase._handle_pagination(mock_func, max_results=3)

            # Should return only the max results
            assert results == [1, 2, 3]
            mock_func.assert_called_once()

        @pytest.mark.asyncio
        async def test_pagination_with_max_results_multiple_pages(self):
            """Test pagination with max_results across multiple pages."""
            mock_func = AsyncMock()
            mock_func.side_effect = [
                ([1, 2, 3], "token1"),  # First page (3 items)
                ([4, 5, 6], "token2"),  # Second page (6 total items, exceeds max_results=5)
            ]

            results = await LowLevelClientBase._handle_pagination(mock_func, max_results=5)

            # Should include 2 pages and return the full first page but limited 2nd page
            assert results == [1, 2, 3, 4, 5]
            assert mock_func.call_count == 2

        @pytest.mark.asyncio
        async def test_pagination_with_max_results_exact_match(self):
            """Test pagination when results exactly match max_results."""
            mock_func = AsyncMock()
            mock_func.side_effect = [
                ([1, 2, 3], "token1"),  # First page
                ([4, 5], ""),  # Second page, total = 5
            ]

            results = await LowLevelClientBase._handle_pagination(mock_func, max_results=5)

            assert results == [1, 2, 3, 4, 5]
            assert mock_func.call_count == 2

        @pytest.mark.asyncio
        async def test_pagination_empty_results(self):
            """Test pagination when function returns empty results."""
            mock_func = AsyncMock(return_value=([], ""))

            results = await LowLevelClientBase._handle_pagination(mock_func)

            assert results == []
            mock_func.assert_called_once()

        @pytest.mark.asyncio
        async def test_pagination_max_results_zero(self):
            """Test pagination with max_results=0."""
            mock_func = AsyncMock(return_value=([1, 2, 3], ""))

            results = await LowLevelClientBase._handle_pagination(mock_func, max_results=0)

            # Should return empty list without calling the function
            assert results == []
            mock_func.assert_not_called()
