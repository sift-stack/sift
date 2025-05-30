from typing import List, cast

from sift.assets.v1.assets_pb2 import Asset, ListAssetsRequest, ListAssetsResponse
from sift.assets.v1.assets_pb2_grpc import AssetServiceStub

from sift_py._internal.cel import cel_in
from sift_py.grpc.transport import SiftChannel


def get_assets(channel: SiftChannel, names: List[str] = [], ids: List[str] = []) -> List[Asset]:
    """Returns the list of Asset information given a list of names and IDs.

    Args:
        channel: Sift gRPC Channel.
        names: List of asset names.
        ids: List of asset IDs.

    Returns:
        List of Assets.
    """
    asset_service_stub = AssetServiceStub(channel)

    def get_assets_with_filter(cel_filter: str):
        assets: List[Asset] = []
        next_page_token = ""
        while True:
            req = ListAssetsRequest(
                filter=cel_filter,
                page_size=1_000,
                page_token=next_page_token,
            )
            res = cast(ListAssetsResponse, asset_service_stub.ListAssets(req))
            assets.extend(res.assets)

            if not res.next_page_token:
                break
            next_page_token = res.next_page_token

        return assets

    results = []
    if names:
        names_cel = cel_in("name", names)
        results.extend(get_assets_with_filter(names_cel))

    if ids:
        ids_cel = cel_in("asset_id", ids)
        results.extend(get_assets_with_filter(ids_cel))

    return results
