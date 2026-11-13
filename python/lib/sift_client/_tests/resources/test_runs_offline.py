"""Offline tests for archive compatibility in the Runs API."""

from __future__ import annotations

from unittest.mock import AsyncMock, MagicMock

import pytest

from sift_client.resources.runs import RunsAPIAsync
from sift_client.util import cel_utils as cel


def _api(mock_client) -> tuple[RunsAPIAsync, AsyncMock]:
    api = RunsAPIAsync(mock_client)
    api._low_level_client = MagicMock()
    list_all_runs = AsyncMock(return_value=[])
    api._low_level_client.list_all_runs = list_all_runs
    return api, list_all_runs


@pytest.mark.asyncio
async def test_list_include_archived_builds_both_state_filter(mock_client):
    api, list_all_runs = _api(mock_client)

    await api.list_(include_archived=True)

    call = list_all_runs.await_args
    assert call is not None
    query_filter = call.kwargs["query_filter"]
    assert query_filter == cel.parens(
        cel.or_(cel.equals("is_archived", True), cel.equals("is_archived", False))
    )


@pytest.mark.asyncio
async def test_list_excludes_archived_by_default(mock_client):
    api, list_all_runs = _api(mock_client)

    await api.list_()

    call = list_all_runs.await_args
    assert call is not None
    query_filter = call.kwargs["query_filter"]
    assert query_filter == cel.equals("is_archived", False)
