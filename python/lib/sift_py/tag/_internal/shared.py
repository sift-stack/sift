from typing import List, Optional, Tuple, Union, cast

from sift.tags.v2.tags_pb2 import ListTagsRequest, ListTagsResponse, Tag, TagType
from sift.tags.v2.tags_pb2_grpc import TagServiceStub
from sift_py._internal.cel import cel_in


def list_tags_impl(
    tag_service_stub: TagServiceStub,
    names: Optional[Union[Tuple[str], List[str]]] = None,
    ids: Optional[Union[Tuple[str], List[str]]] = None,
    tag_type: Optional[TagType] = None,
) -> List[Tag]:
    """
    Lists tags in an organization.

    Args:
        tag_service_stub: The tag service stub to use.
        names: Optional collection of names to filter by.
        ids: Optional collection of IDs to filter by.
        tag_type: Optional tag type to filter by.

    Returns:
        A list of tags matching the criteria.
    """

    def get_tags_with_filter(
        tag_service_stub: TagServiceStub, cel_filter: str, tag_type: Optional[TagType]
    ) -> List[Tag]:
        tags: List[Tag] = []
        next_page_token = ""
        while True:
            req_kwargs = {
                "filter": cel_filter,
                "page_size": 1_000,
                "page_token": next_page_token,
            }
            if tag_type is not None:
                req_kwargs["tag_type"] = tag_type
            req = ListTagsRequest(**req_kwargs)
            res = cast(ListTagsResponse, tag_service_stub.ListTags(req))
            tags.extend(res.tags)

            if not res.next_page_token:
                break
            next_page_token = res.next_page_token

        return tags

    if names is None:
        names = []
    if ids is None:
        ids = []

    results: List[Tag] = []
    if names:
        names_cel = cel_in("name", names)
        results.append(get_tags_with_filter(tag_service_stub, names_cel, tag_type))
    if ids:
        ids_cel = cel_in("tag_id", ids)
        results.append(get_tags_with_filter(tag_service_stub, ids_cel, tag_type))
    if not names and not ids:
        # If no filter, but tag_type is specified, fetch all tags of that type
        if tag_type is not None:
            results.append(get_tags_with_filter(tag_service_stub, "", tag_type))

    return results
