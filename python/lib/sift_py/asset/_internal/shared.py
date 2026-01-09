from typing import List, Optional, Tuple, Union, cast

from sift.assets.v1.assets_pb2 import Asset, ListAssetsRequest, ListAssetsResponse
from sift.assets.v1.assets_pb2_grpc import AssetServiceStub
from sift_py._internal.cel import cel_in


def list_assets_impl(
    _asset_service_stub: AssetServiceStub,
    names: Optional[Union[Tuple[str], List[str]]] = None,
    ids: Optional[Union[Tuple[str], List[str]]] = None,
) -> List[Asset]:
    """
    Lists assets in an organization.

    Args:
        _asset_service_stub: The asset service stub to use.
        names: Optional collection of names to filter by.
        ids: Optional collection of IDs to filter by.

    Returns:
        A list of assets matching the criteria.
    """

    def get_assets_with_filter(
        _asset_service_stub: AssetServiceStub, cel_filter: str
    ) -> List[Asset]:
        assets: List[Asset] = []
        next_page_token = ""
        while True:
            req = ListAssetsRequest(
                filter=cel_filter,
                page_size=1_000,
                page_token=next_page_token,
            )
            res = cast(ListAssetsResponse, _asset_service_stub.ListAssets(req))
            assets.extend(res.assets)

            if not res.next_page_token:
                break
            next_page_token = res.next_page_token

        return assets

    if names is None:
        names = []
    if ids is None:
        ids = []

    if names:
        names_cel = cel_in("name", names)
        return get_assets_with_filter(_asset_service_stub, names_cel)
    elif ids:
        ids_cel = cel_in("asset_id", ids)
        return get_assets_with_filter(_asset_service_stub, ids_cel)
    else:
        return []
