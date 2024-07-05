from typing import Dict, List, Set, Union, cast

from sift.assets.v1.assets_pb2 import Asset, ListAssetsRequest, ListAssetsResponse
from sift.assets.v1.assets_pb2_grpc import AssetServiceStub
from sift.channels.v2.channels_pb2 import Channel, ListChannelsRequest, ListChannelsResponse
from sift.channels.v2.channels_pb2_grpc import ChannelServiceStub
from sift.data.v1.data_pb2 import ChannelQuery as ChannelQueryPb
from sift.data.v1.data_pb2 import GetDataRequest
from sift.data.v1.data_pb2_grpc import DataServiceStub
from sift.runs.v2.runs_pb2 import ListRunsRequest, ListRunsResponse, Run
from sift.runs.v2.runs_pb2_grpc import RunServiceStub
from typing_extensions import TypeAlias

from sift_py._internal.convert.timestamp import to_pb_timestamp
from sift_py.data.error import DataError
from sift_py.data.query import CalculatedChannelQuery, ChannelQuery, DataQuery
from sift_py.error import SiftError
from sift_py.grpc.transport import SiftChannel


class DataService:
    AssetName: TypeAlias = str
    ChannelFqn: TypeAlias = str
    RunName: TypeAlias = str

    _asset_service_stub: AssetServiceStub
    _channel_service_stub: ChannelServiceStub
    _data_service_stub: DataServiceStub
    _run_service_stub: RunServiceStub

    _cached_assets: Dict[AssetName, Asset]
    _cached_channels: Dict[AssetName, Dict[ChannelFqn, Channel]]
    _cached_runs: Dict[RunName, Run]

    def __init__(self, channel: SiftChannel):
        self._asset_service_stub = AssetServiceStub(channel)
        self._channel_service_stub = ChannelServiceStub(channel)
        self._data_service_stub = DataServiceStub(channel)
        self._run_service_stub = RunServiceStub(channel)

    def execute(self, query: DataQuery, bust_cache: bool = False):
        if bust_cache:
            self._bust_cache()

        asset = self._load_asset(query.asset_name)
        channels = self._load_channels(asset)
        runs = self._load_runs(query.channels)

        queries = []

        for channel_query in query.channels:
            if isinstance(channel_query, ChannelQuery):
                channel_fqn = channel_query.fqn()
                run_name = channel_query.run_name
                channel = channels.get(channel_fqn)

                if channel is None:
                    raise SiftError(
                        f"An unexpected error occurred. Expected channel '{channel_fqn}' to have been loaded."
                    )

                query = ChannelQueryPb(channel_id=channel.channel_id)

                if run_name is not None:
                    run = runs.get(run_name)

                    if run is None:
                        raise SiftError(
                            f"An unexpected error occurred. Expected run '{run_name}' to have been loaded."
                        )

                    query.run_id = run.run_id

                queries.append(query)

            elif isinstance(channel_query, CalculatedChannelQuery):
                raise NotImplementedError("Calculated channel downloading is not yet implemented.")
            else:
                raise DataError("Unknown channel query type.")

        request = GetDataRequest(
            start_time=to_pb_timestamp(query.start_time),
            end_time=to_pb_timestamp(query.end_time),
            sample_ms=query.sample_ms,
            page_size=query.page_size,
            queries=queries,
        )

        # Serialize and pass off to Rust
        wire_format = request.SerializeToString()

    def _bust_cache(self):
        self._cached_assets.clear()
        self._cached_channels.clear()
        self._cached_runs.clear()

    def _load_asset(self, asset_name: str) -> Asset:
        asset = self._cached_assets.get(asset_name)

        if asset is None:
            asset = self._get_asset_by_name(asset_name)
            self._cached_assets[asset.name] = asset

        return asset

    def _load_channels(self, asset: Asset) -> Dict[ChannelFqn, Channel]:
        channels = self._cached_channels.get(asset.name)

        if channels is None:
            sift_channels = self._get_channels_by_asset_id(asset.asset_id)
            channels = {c.name: c for c in sift_channels}
            self._cached_channels[asset.name] = channels

        return channels

    def _load_runs(
        self, channel_queries: List[Union[ChannelQuery, CalculatedChannelQuery]]
    ) -> Dict[RunName, Run]:
        run_names: Set[str] = set()

        for channel_query in channel_queries:
            run_name = channel_query.run_name

            if run_name is not None and len(run_name) > 0:
                run_names.add(run_name)

        runs = {}
        run_names_to_fetch = set()

        for run_name in run_names:
            run = self._cached_runs.get(run_name)

            if run is not None:
                runs[run.name] = run
            else:
                run_names_to_fetch.add(run_name)

        for run in self._get_runs_by_names(run_names_to_fetch):
            self._cached_runs[run.name] = run
            runs[run.name] = run

        return runs

    def _get_asset_by_name(self, asset_name: str) -> Asset:
        req = ListAssetsRequest(
            filter=f'name=="{asset_name}"',
            page_size=1,
        )
        res = cast(ListAssetsResponse, self._asset_service_stub.ListAssets(req))
        assets = res.assets

        if len(assets) == 0:
            raise DataError(f"Asset of name '{asset_name}' does not exist.")

        return res.assets[0]

    def _get_runs_by_names(self, run_names: Set[str]) -> List[Run]:
        if len(run_names) == 0:
            return []

        names = ",".join([f'"{name}"' for name in run_names])

        req = ListRunsRequest(
            filter=f"name in [{names}]",
            page_size=1,
        )

        res = cast(ListRunsResponse, self._run_service_stub.ListRuns(req))
        runs = res.runs

        seen_sift_runs = set()

        for sift_run in runs:
            seen_sift_runs.add(sift_run.name)

        for run_name in run_names:
            if run_name not in seen_sift_runs:
                raise DataError(f"Run of name '{run_name}' does not exist.")

        return runs

    def _get_channels_by_asset_id(self, asset_id: str) -> List[Channel]:
        channels: List[Channel] = []

        req = ListChannelsRequest(
            filter=f'asset_id=="{asset_id}"',
            page_size=1_000,
            page_token="",
        )
        res = cast(ListChannelsResponse, self._channel_service_stub.ListChannels(req))
        channels.extend(res.channels)
        next_page_token = res.next_page_token

        while len(next_page_token) > 0:
            req = ListChannelsRequest(
                filter=f'asset_id=="{asset_id}"',
                page_size=1_000,
                page_token=next_page_token,
            )
            res = cast(ListChannelsResponse, self._channel_service_stub.ListChannels(req))
            channels.extend(res.channels)
            next_page_token = res.next_page_token

        return channels
