"""Synchronous API wrappers generated from async classes.
This package contains synchronous versions of all async API classes.
"""

from sift_client._internal.sync_wrapper import generate_sync_api
from sift_client.resources import (
    AssetsAPIAsync,
    CalculatedChannelsAPIAsync,
    ChannelsAPIAsync,
    FileAttachmentsAPIAsync,
    JobsAPIAsync,
    PingAPIAsync,
    ReportsAPIAsync,
    RulesAPIAsync,
    RunsAPIAsync,
    TagsAPIAsync,
    TestResultsAPIAsync,
)

PingAPI = generate_sync_api(PingAPIAsync, "PingAPI")
AssetsAPI = generate_sync_api(AssetsAPIAsync, "AssetsAPI")
CalculatedChannelsAPI = generate_sync_api(CalculatedChannelsAPIAsync, "CalculatedChannelsAPI")
ChannelsAPI = generate_sync_api(ChannelsAPIAsync, "ChannelsAPI")
FileAttachmentsAPI = generate_sync_api(FileAttachmentsAPIAsync, "FileAttachmentsAPI")
JobsAPI = generate_sync_api(JobsAPIAsync, "JobsAPI")
RulesAPI = generate_sync_api(RulesAPIAsync, "RulesAPI")
RunsAPI = generate_sync_api(RunsAPIAsync, "RunsAPI")
ReportsAPI = generate_sync_api(ReportsAPIAsync, "ReportsAPI")
TagsAPI = generate_sync_api(TagsAPIAsync, "TagsAPI")
TestResultsAPI = generate_sync_api(TestResultsAPIAsync, "TestResultsAPI")

__all__ = [
    "AssetsAPI",
    "CalculatedChannelsAPI",
    "ChannelsAPI",
    "FileAttachmentsAPI",
    "JobsAPI",
    "PingAPI",
    "ReportsAPI",
    "RulesAPI",
    "RunsAPI",
    "TagsAPI",
    "TestResultsAPI",
]
