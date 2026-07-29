"""Synchronous API wrappers generated from async classes.
This package contains synchronous versions of all async API classes.
"""

from sift_client._internal.sync_wrapper import generate_sync_api
from sift_client.resources import (
    AssetsAPIAsync,
    CalculatedChannelsAPIAsync,
    ChannelsAPIAsync,
    DataExportAPIAsync,
    DataImportAPIAsync,
    FileAttachmentsAPIAsync,
    JobsAPIAsync,
    PingAPIAsync,
    PrincipalAttributeAssignmentsAPIAsync,
    PrincipalAttributeEnumValuesAPIAsync,
    PrincipalAttributeKeysAPIAsync,
    PrincipalAttributesAPIAsync,
    ReportsAPIAsync,
    ReportTemplatesAPIAsync,
    ResourceAttributeAssignmentsAPIAsync,
    ResourceAttributeEnumValuesAPIAsync,
    ResourceAttributeKeysAPIAsync,
    ResourceAttributesAPIAsync,
    RulesAPIAsync,
    RunsAPIAsync,
    TagsAPIAsync,
    TestResultsAPIAsync,
    UsersAPIAsync,
)

PingAPI = generate_sync_api(PingAPIAsync, "PingAPI")
AssetsAPI = generate_sync_api(AssetsAPIAsync, "AssetsAPI")
CalculatedChannelsAPI = generate_sync_api(CalculatedChannelsAPIAsync, "CalculatedChannelsAPI")
ChannelsAPI = generate_sync_api(ChannelsAPIAsync, "ChannelsAPI")
FileAttachmentsAPI = generate_sync_api(FileAttachmentsAPIAsync, "FileAttachmentsAPI")
JobsAPI = generate_sync_api(JobsAPIAsync, "JobsAPI")
RulesAPI = generate_sync_api(RulesAPIAsync, "RulesAPI")
RunsAPI = generate_sync_api(RunsAPIAsync, "RunsAPI")
# ReportTemplatesAPI must be generated before ReportsAPI so it can be nested under it.
ReportTemplatesAPI = generate_sync_api(ReportTemplatesAPIAsync, "ReportTemplatesAPI")
ReportsAPI = generate_sync_api(
    ReportsAPIAsync, "ReportsAPI", nested_resources={"templates": ReportTemplatesAPI}
)
# The attribute sub-resource APIs must be generated before their parents so they
# can be nested under them.
ResourceAttributeKeysAPI = generate_sync_api(
    ResourceAttributeKeysAPIAsync, "ResourceAttributeKeysAPI"
)
ResourceAttributeEnumValuesAPI = generate_sync_api(
    ResourceAttributeEnumValuesAPIAsync, "ResourceAttributeEnumValuesAPI"
)
ResourceAttributeAssignmentsAPI = generate_sync_api(
    ResourceAttributeAssignmentsAPIAsync, "ResourceAttributeAssignmentsAPI"
)
ResourceAttributesAPI = generate_sync_api(
    ResourceAttributesAPIAsync,
    "ResourceAttributesAPI",
    nested_resources={
        "keys": ResourceAttributeKeysAPI,
        "enum_values": ResourceAttributeEnumValuesAPI,
        "assignments": ResourceAttributeAssignmentsAPI,
    },
)
PrincipalAttributeKeysAPI = generate_sync_api(
    PrincipalAttributeKeysAPIAsync, "PrincipalAttributeKeysAPI"
)
PrincipalAttributeEnumValuesAPI = generate_sync_api(
    PrincipalAttributeEnumValuesAPIAsync, "PrincipalAttributeEnumValuesAPI"
)
PrincipalAttributeAssignmentsAPI = generate_sync_api(
    PrincipalAttributeAssignmentsAPIAsync, "PrincipalAttributeAssignmentsAPI"
)
PrincipalAttributesAPI = generate_sync_api(
    PrincipalAttributesAPIAsync,
    "PrincipalAttributesAPI",
    nested_resources={
        "keys": PrincipalAttributeKeysAPI,
        "enum_values": PrincipalAttributeEnumValuesAPI,
        "assignments": PrincipalAttributeAssignmentsAPI,
    },
)
TagsAPI = generate_sync_api(TagsAPIAsync, "TagsAPI")
TestResultsAPI = generate_sync_api(TestResultsAPIAsync, "TestResultsAPI")
UsersAPI = generate_sync_api(UsersAPIAsync, "UsersAPI")
DataExportAPI = generate_sync_api(DataExportAPIAsync, "DataExportAPI")
DataImportAPI = generate_sync_api(DataImportAPIAsync, "DataImportAPI")

__all__ = [
    "AssetsAPI",
    "CalculatedChannelsAPI",
    "ChannelsAPI",
    "DataExportAPI",
    "DataImportAPI",
    "FileAttachmentsAPI",
    "JobsAPI",
    "PingAPI",
    "PrincipalAttributeAssignmentsAPI",
    "PrincipalAttributeEnumValuesAPI",
    "PrincipalAttributeKeysAPI",
    "PrincipalAttributesAPI",
    "ReportTemplatesAPI",
    "ReportsAPI",
    "ResourceAttributeAssignmentsAPI",
    "ResourceAttributeEnumValuesAPI",
    "ResourceAttributeKeysAPI",
    "ResourceAttributesAPI",
    "RulesAPI",
    "RunsAPI",
    "TagsAPI",
    "TestResultsAPI",
    "UsersAPI",
]
