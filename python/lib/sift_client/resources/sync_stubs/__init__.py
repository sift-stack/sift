"""Synchronous API wrappers generated from async classes.
This package contains synchronous versions of all async API classes.
"""

from sift_client._internal.sync_wrapper import generate_sync_api
from sift_client.resources import (
    AssetsAPIAsync,
    CalculatedChannelsAPIAsync,
    ChannelsAPIAsync,
    PingAPIAsync,
    RulesAPIAsync,
    RunsAPIAsync,
    TestResultsAPIAsync,
)

PingAPI = generate_sync_api(PingAPIAsync, "PingAPI")
AssetsAPI = generate_sync_api(AssetsAPIAsync, "AssetsAPI")
CalculatedChannelsAPI = generate_sync_api(CalculatedChannelsAPIAsync, "CalculatedChannelsAPI")
ChannelsAPI = generate_sync_api(ChannelsAPIAsync, "ChannelsAPI")
RulesAPI = generate_sync_api(RulesAPIAsync, "RulesAPI")
RunsAPI = generate_sync_api(RunsAPIAsync, "RunsAPI")
TestResultsAPI = generate_sync_api(TestResultsAPIAsync, "TestResultsAPI")

__all__ = ["AssetsAPI", "CalculatedChannelsAPI", "PingAPI", "RunsAPI", "TestResultsAPI"]
