from __future__ import annotations

from typing import TYPE_CHECKING, Any, NamedTuple

if TYPE_CHECKING:
    from sift_client.resources import (
        AssetsAPIAsync,
        CalculatedChannelsAPIAsync,
        ChannelsAPIAsync,
        FileAttachmentsAPIAsync,
        IngestionAPIAsync,
        JobsAPIAsync,
        PingAPIAsync,
        ReportsAPIAsync,
        RulesAPIAsync,
        RunsAPIAsync,
        TagsAPIAsync,
        TestResultsAPIAsync,
    )


class AsyncAPIs(NamedTuple):
    """Simple accessor for the asynchronous APIs, still uses the SiftClient instance."""

    ping: PingAPIAsync
    """Instance of the Ping API for making asynchronous requests."""

    assets: AssetsAPIAsync
    """Instance of the Assets API for making asynchronous requests."""

    calculated_channels: CalculatedChannelsAPIAsync
    """Instance of the Calculated Channels API for making asynchronous requests."""

    channels: ChannelsAPIAsync
    """Instance of the Channels API for making asynchronous requests."""

    file_attachments: FileAttachmentsAPIAsync
    """Instance of the File Attachments API for making asynchronous requests."""

    ingestion: IngestionAPIAsync
    """Instance of the Ingestion API for making asynchronous requests."""

    jobs: JobsAPIAsync
    """Instance of the Jobs API for making asynchronous requests."""

    reports: ReportsAPIAsync
    """Instance of the Reports API for making asynchronous requests."""

    runs: RunsAPIAsync
    """Instance of the Runs API for making asynchronous requests."""

    rules: RulesAPIAsync
    """Instance of the Rules API for making asynchronous requests."""

    tags: TagsAPIAsync
    """Instance of the Tags API for making asynchronous requests."""

    test_results: TestResultsAPIAsync
    """Instance of the Test Results API for making asynchronous requests."""


def count_non_none(*args: Any) -> int:
    """Count the number of non-none arguments."""
    return sum(1 for arg in args if arg is not None)
