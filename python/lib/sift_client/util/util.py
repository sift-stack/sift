from __future__ import annotations

from typing import TYPE_CHECKING, Any, NamedTuple

if TYPE_CHECKING:
    from sift_client.resources import (
        AssetsAPIAsync,
        CalculatedChannelsAPIAsync,
        ChannelsAPIAsync,
        DataExportAPIAsync,
        DataImportAPIAsync,
        FileAttachmentsAPIAsync,
        IngestionAPIAsync,
        JobsAPIAsync,
        PingAPIAsync,
        PrincipalAttributesAPI,
        PrincipalAttributesAPIAsync,
        ReportsAPIAsync,
        ResourceAttributesAPI,
        ResourceAttributesAPIAsync,
        RulesAPIAsync,
        RunsAPIAsync,
        TagsAPIAsync,
        TestResultsAPIAsync,
    )


class AccessControlAPI:
    """Access-control namespace. Groups the ABAC APIs; roles, policies, and user groups
    will live here as they are added.
    """

    resource_attributes: ResourceAttributesAPI
    """Attribute keys assigned to entities (assets, channels, runs)."""

    principal_attributes: PrincipalAttributesAPI
    """Attribute keys assigned to principals (users, user groups)."""

    def __init__(
        self,
        *,
        resource_attributes: ResourceAttributesAPI,
        principal_attributes: PrincipalAttributesAPI,
    ):
        """Initialize the access-control namespace with its sub-APIs."""
        self.resource_attributes = resource_attributes
        self.principal_attributes = principal_attributes


class AccessControlAPIAsync:
    """Asynchronous counterpart to `AccessControlAPI`."""

    resource_attributes: ResourceAttributesAPIAsync
    """Attribute keys assigned to entities (assets, channels, runs)."""

    principal_attributes: PrincipalAttributesAPIAsync
    """Attribute keys assigned to principals (users, user groups)."""

    def __init__(
        self,
        *,
        resource_attributes: ResourceAttributesAPIAsync,
        principal_attributes: PrincipalAttributesAPIAsync,
    ):
        """Initialize the access-control namespace with its sub-APIs."""
        self.resource_attributes = resource_attributes
        self.principal_attributes = principal_attributes


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

    access_control: AccessControlAPIAsync
    """Namespace for the access-control APIs (resource and principal attributes)."""

    tags: TagsAPIAsync
    """Instance of the Tags API for making asynchronous requests."""

    test_results: TestResultsAPIAsync
    """Instance of the Test Results API for making asynchronous requests."""

    data_export: DataExportAPIAsync
    """Instance of the Data Export API for making asynchronous requests."""

    data_import: DataImportAPIAsync
    """Instance of the Data Import API for making asynchronous requests."""


def count_non_none(*args: Any) -> int:
    """Count the number of non-none arguments."""
    return sum(1 for arg in args if arg is not None)
