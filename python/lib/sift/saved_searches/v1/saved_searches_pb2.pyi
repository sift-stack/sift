"""
@generated by mypy-protobuf.  Do not edit manually!
isort:skip_file
"""

import builtins
import collections.abc
import google.protobuf.descriptor
import google.protobuf.field_mask_pb2
import google.protobuf.internal.containers
import google.protobuf.message
import google.protobuf.timestamp_pb2
import typing

DESCRIPTOR: google.protobuf.descriptor.FileDescriptor

@typing.final
class SavedSearch(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    SAVED_SEARCH_ID_FIELD_NUMBER: builtins.int
    ORGANIZATION_ID_FIELD_NUMBER: builtins.int
    NAME_FIELD_NUMBER: builtins.int
    PROPERTIES_FIELD_NUMBER: builtins.int
    CREATED_BY_USER_ID_FIELD_NUMBER: builtins.int
    MODIFIED_BY_USER_ID_FIELD_NUMBER: builtins.int
    CREATED_DATE_FIELD_NUMBER: builtins.int
    MODIFIED_DATE_FIELD_NUMBER: builtins.int
    saved_search_id: builtins.str
    organization_id: builtins.str
    name: builtins.str
    created_by_user_id: builtins.str
    modified_by_user_id: builtins.str
    @property
    def properties(self) -> global___SavedSearchProperties: ...
    @property
    def created_date(self) -> google.protobuf.timestamp_pb2.Timestamp: ...
    @property
    def modified_date(self) -> google.protobuf.timestamp_pb2.Timestamp: ...
    def __init__(
        self,
        *,
        saved_search_id: builtins.str = ...,
        organization_id: builtins.str = ...,
        name: builtins.str = ...,
        properties: global___SavedSearchProperties | None = ...,
        created_by_user_id: builtins.str = ...,
        modified_by_user_id: builtins.str = ...,
        created_date: google.protobuf.timestamp_pb2.Timestamp | None = ...,
        modified_date: google.protobuf.timestamp_pb2.Timestamp | None = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["created_date", b"created_date", "modified_date", b"modified_date", "properties", b"properties"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["created_by_user_id", b"created_by_user_id", "created_date", b"created_date", "modified_by_user_id", b"modified_by_user_id", "modified_date", b"modified_date", "name", b"name", "organization_id", b"organization_id", "properties", b"properties", "saved_search_id", b"saved_search_id"]) -> None: ...

global___SavedSearch = SavedSearch

@typing.final
class SavedSearchProperties(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    OVERVIEW_MODE_FIELD_NUMBER: builtins.int
    SEARCH_TERM_FIELD_NUMBER: builtins.int
    FROM_DATE_TIME_FIELD_NUMBER: builtins.int
    TO_DATE_TIME_FIELD_NUMBER: builtins.int
    ASSET_ITEMS_FIELD_NUMBER: builtins.int
    USER_ITEMS_FIELD_NUMBER: builtins.int
    TAG_ITEMS_FIELD_NUMBER: builtins.int
    ANNOTATION_ITEMS_FIELD_NUMBER: builtins.int
    RUN_ITEMS_FIELD_NUMBER: builtins.int
    overview_mode: builtins.str
    search_term: builtins.str
    @property
    def from_date_time(self) -> google.protobuf.timestamp_pb2.Timestamp: ...
    @property
    def to_date_time(self) -> google.protobuf.timestamp_pb2.Timestamp: ...
    @property
    def asset_items(self) -> google.protobuf.internal.containers.RepeatedCompositeFieldContainer[global___SavedSearchFilterItem]: ...
    @property
    def user_items(self) -> google.protobuf.internal.containers.RepeatedCompositeFieldContainer[global___SavedSearchFilterItem]: ...
    @property
    def tag_items(self) -> google.protobuf.internal.containers.RepeatedCompositeFieldContainer[global___SavedSearchFilterItem]: ...
    @property
    def annotation_items(self) -> google.protobuf.internal.containers.RepeatedCompositeFieldContainer[global___SavedSearchFilterItem]: ...
    @property
    def run_items(self) -> google.protobuf.internal.containers.RepeatedCompositeFieldContainer[global___SavedSearchFilterItem]: ...
    def __init__(
        self,
        *,
        overview_mode: builtins.str = ...,
        search_term: builtins.str | None = ...,
        from_date_time: google.protobuf.timestamp_pb2.Timestamp | None = ...,
        to_date_time: google.protobuf.timestamp_pb2.Timestamp | None = ...,
        asset_items: collections.abc.Iterable[global___SavedSearchFilterItem] | None = ...,
        user_items: collections.abc.Iterable[global___SavedSearchFilterItem] | None = ...,
        tag_items: collections.abc.Iterable[global___SavedSearchFilterItem] | None = ...,
        annotation_items: collections.abc.Iterable[global___SavedSearchFilterItem] | None = ...,
        run_items: collections.abc.Iterable[global___SavedSearchFilterItem] | None = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["_from_date_time", b"_from_date_time", "_search_term", b"_search_term", "_to_date_time", b"_to_date_time", "from_date_time", b"from_date_time", "search_term", b"search_term", "to_date_time", b"to_date_time"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["_from_date_time", b"_from_date_time", "_search_term", b"_search_term", "_to_date_time", b"_to_date_time", "annotation_items", b"annotation_items", "asset_items", b"asset_items", "from_date_time", b"from_date_time", "overview_mode", b"overview_mode", "run_items", b"run_items", "search_term", b"search_term", "tag_items", b"tag_items", "to_date_time", b"to_date_time", "user_items", b"user_items"]) -> None: ...
    @typing.overload
    def WhichOneof(self, oneof_group: typing.Literal["_from_date_time", b"_from_date_time"]) -> typing.Literal["from_date_time"] | None: ...
    @typing.overload
    def WhichOneof(self, oneof_group: typing.Literal["_search_term", b"_search_term"]) -> typing.Literal["search_term"] | None: ...
    @typing.overload
    def WhichOneof(self, oneof_group: typing.Literal["_to_date_time", b"_to_date_time"]) -> typing.Literal["to_date_time"] | None: ...

global___SavedSearchProperties = SavedSearchProperties

@typing.final
class SavedSearchFilterItem(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    ID_FIELD_NUMBER: builtins.int
    NAME_FIELD_NUMBER: builtins.int
    id: builtins.str
    name: builtins.str
    def __init__(
        self,
        *,
        id: builtins.str = ...,
        name: builtins.str = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["id", b"id", "name", b"name"]) -> None: ...

global___SavedSearchFilterItem = SavedSearchFilterItem

@typing.final
class GetSavedSearchRequest(google.protobuf.message.Message):
    """The request for a call to `SavedSearchService_GetSavedSearch` to retrieve a saved search;"""

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    SAVED_SEARCH_ID_FIELD_NUMBER: builtins.int
    saved_search_id: builtins.str
    def __init__(
        self,
        *,
        saved_search_id: builtins.str = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["saved_search_id", b"saved_search_id"]) -> None: ...

global___GetSavedSearchRequest = GetSavedSearchRequest

@typing.final
class GetSavedSearchResponse(google.protobuf.message.Message):
    """The response of a call to `SavedSearchService_GetSavedSearch`."""

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    SAVED_SEARCH_FIELD_NUMBER: builtins.int
    @property
    def saved_search(self) -> global___SavedSearch: ...
    def __init__(
        self,
        *,
        saved_search: global___SavedSearch | None = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["saved_search", b"saved_search"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["saved_search", b"saved_search"]) -> None: ...

global___GetSavedSearchResponse = GetSavedSearchResponse

@typing.final
class ListSavedSearchesRequest(google.protobuf.message.Message):
    """The request for a call to `SavedSearchService_ListSavedSearches` to retrieve saved searches."""

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    PAGE_SIZE_FIELD_NUMBER: builtins.int
    PAGE_TOKEN_FIELD_NUMBER: builtins.int
    FILTER_FIELD_NUMBER: builtins.int
    ORGANIZATION_ID_FIELD_NUMBER: builtins.int
    page_size: builtins.int
    """The maximum number of saved searches to return. The service may return fewer than this value.
    If unspecified, at most 50 saved searches will be returned. The maximum value is 1000; values above
    1000 will be coerced to 1000. Optional.
    """
    page_token: builtins.str
    """A page token, received from a previous `ListSavedSearches` call.
    Provide this to retrieve the subsequent page.
    When paginating, all other parameters provided to `ListSavedSearches` must match
    the call that provided the page token. Optional.
    """
    filter: builtins.str
    """A [Common Expression Language (CEL)](https://github.com/google/cel-spec) filter string.
    Available fields to filter by are 'name' and 'saved_search_id'.
    For further information about how to use CELs, please refer to [this guide](https://github.com/google/cel-spec/blob/master/doc/langdef.md#standard-definitions).
    For more information about the fields used for filtering, please refer to [this definition](/protocol-buffers/documentation#saved_searches). Optional.
    """
    organization_id: builtins.str
    """This field is only required if your user belongs to multiple organizations."""
    def __init__(
        self,
        *,
        page_size: builtins.int = ...,
        page_token: builtins.str = ...,
        filter: builtins.str = ...,
        organization_id: builtins.str = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["filter", b"filter", "organization_id", b"organization_id", "page_size", b"page_size", "page_token", b"page_token"]) -> None: ...

global___ListSavedSearchesRequest = ListSavedSearchesRequest

@typing.final
class ListSavedSearchesResponse(google.protobuf.message.Message):
    """The response of a call to `SavedSearchService_ListSavedSearchesResponse`."""

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    SAVED_SEARCHES_FIELD_NUMBER: builtins.int
    NEXT_PAGE_TOKEN_FIELD_NUMBER: builtins.int
    next_page_token: builtins.str
    @property
    def saved_searches(self) -> google.protobuf.internal.containers.RepeatedCompositeFieldContainer[global___SavedSearch]: ...
    def __init__(
        self,
        *,
        saved_searches: collections.abc.Iterable[global___SavedSearch] | None = ...,
        next_page_token: builtins.str = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["next_page_token", b"next_page_token", "saved_searches", b"saved_searches"]) -> None: ...

global___ListSavedSearchesResponse = ListSavedSearchesResponse

@typing.final
class CreateSavedSearchRequest(google.protobuf.message.Message):
    """The request for a call to `SavedSearchService_CreateSavedSearch` to create a saved search."""

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    NAME_FIELD_NUMBER: builtins.int
    PROPERTIES_FIELD_NUMBER: builtins.int
    ORGANIZATION_ID_FIELD_NUMBER: builtins.int
    name: builtins.str
    organization_id: builtins.str
    """This field is only required if your user belongs to multiple organizations."""
    @property
    def properties(self) -> global___SavedSearchProperties: ...
    def __init__(
        self,
        *,
        name: builtins.str = ...,
        properties: global___SavedSearchProperties | None = ...,
        organization_id: builtins.str = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["properties", b"properties"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["name", b"name", "organization_id", b"organization_id", "properties", b"properties"]) -> None: ...

global___CreateSavedSearchRequest = CreateSavedSearchRequest

@typing.final
class CreateSavedSearchResponse(google.protobuf.message.Message):
    """The response for a call to `SavedSearchService_CreateSavedResponse`."""

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    SAVED_SEARCH_FIELD_NUMBER: builtins.int
    @property
    def saved_search(self) -> global___SavedSearch: ...
    def __init__(
        self,
        *,
        saved_search: global___SavedSearch | None = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["saved_search", b"saved_search"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["saved_search", b"saved_search"]) -> None: ...

global___CreateSavedSearchResponse = CreateSavedSearchResponse

@typing.final
class DeleteSavedSearchRequest(google.protobuf.message.Message):
    """The request for a call to `SavedSearchService_DeleteSavedSearch` to delete a saved search."""

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    SAVED_SEARCH_ID_FIELD_NUMBER: builtins.int
    saved_search_id: builtins.str
    def __init__(
        self,
        *,
        saved_search_id: builtins.str = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["saved_search_id", b"saved_search_id"]) -> None: ...

global___DeleteSavedSearchRequest = DeleteSavedSearchRequest

@typing.final
class DeleteSavedSearchResponse(google.protobuf.message.Message):
    """The response of a call to `SavedSearchService_DeleteSavedSearch`."""

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    def __init__(
        self,
    ) -> None: ...

global___DeleteSavedSearchResponse = DeleteSavedSearchResponse

@typing.final
class BatchDeleteSavedSearchesRequest(google.protobuf.message.Message):
    """The request for a call to `SavedSearchService_BatchDeleteSavedSearches` to delete saved searches."""

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    SAVED_SEARCH_IDS_FIELD_NUMBER: builtins.int
    @property
    def saved_search_ids(self) -> google.protobuf.internal.containers.RepeatedScalarFieldContainer[builtins.str]: ...
    def __init__(
        self,
        *,
        saved_search_ids: collections.abc.Iterable[builtins.str] | None = ...,
    ) -> None: ...
    def ClearField(self, field_name: typing.Literal["saved_search_ids", b"saved_search_ids"]) -> None: ...

global___BatchDeleteSavedSearchesRequest = BatchDeleteSavedSearchesRequest

@typing.final
class BatchDeleteSavedSearchesResponse(google.protobuf.message.Message):
    """The response of a call to `SavedSearchService_BatchDeleteSavedSearches`."""

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    def __init__(
        self,
    ) -> None: ...

global___BatchDeleteSavedSearchesResponse = BatchDeleteSavedSearchesResponse

@typing.final
class UpdateSavedSearchRequest(google.protobuf.message.Message):
    """The request for a call to `SavedSearchService_UpdateSavedSearch` to update a saved search."""

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    SAVED_SEARCH_FIELD_NUMBER: builtins.int
    UPDATE_MASK_FIELD_NUMBER: builtins.int
    @property
    def saved_search(self) -> global___SavedSearch:
        """The saved search to update."""

    @property
    def update_mask(self) -> google.protobuf.field_mask_pb2.FieldMask:
        """The list of fields to be updated. The fields available to be updated are `name` and `properties`."""

    def __init__(
        self,
        *,
        saved_search: global___SavedSearch | None = ...,
        update_mask: google.protobuf.field_mask_pb2.FieldMask | None = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["saved_search", b"saved_search", "update_mask", b"update_mask"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["saved_search", b"saved_search", "update_mask", b"update_mask"]) -> None: ...

global___UpdateSavedSearchRequest = UpdateSavedSearchRequest

@typing.final
class UpdateSavedSearchResponse(google.protobuf.message.Message):
    """The response of a call to `SavedSearchService_UpdateSavedSearch`."""

    DESCRIPTOR: google.protobuf.descriptor.Descriptor

    SAVED_SEARCH_FIELD_NUMBER: builtins.int
    @property
    def saved_search(self) -> global___SavedSearch: ...
    def __init__(
        self,
        *,
        saved_search: global___SavedSearch | None = ...,
    ) -> None: ...
    def HasField(self, field_name: typing.Literal["saved_search", b"saved_search"]) -> builtins.bool: ...
    def ClearField(self, field_name: typing.Literal["saved_search", b"saved_search"]) -> None: ...

global___UpdateSavedSearchResponse = UpdateSavedSearchResponse
