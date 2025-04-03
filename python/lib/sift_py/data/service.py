import asyncio
from collections import defaultdict
from typing import Dict, Iterable, List, Optional, Set, Tuple, Union, cast

from google.protobuf.any_pb2 import Any
from sift.assets.v1.assets_pb2 import Asset, ListAssetsRequest, ListAssetsResponse
from sift.assets.v1.assets_pb2_grpc import AssetServiceStub
from sift.calculated_channels.v1.calculated_channels_pb2 import (
    ExpressionChannelReference,
    ExpressionRequest,
)
from sift.calculated_channels.v1.calculated_channels_pb2_grpc import CalculatedChannelsServiceStub
from sift.channels.v3.channels_pb2 import Channel, ListChannelsRequest, ListChannelsResponse
from sift.channels.v3.channels_pb2_grpc import ChannelServiceStub
from sift.data.v2.data_pb2 import CalculatedChannelQuery as CalculatedChannelQueryPb
from sift.data.v2.data_pb2 import ChannelQuery as ChannelQueryPb
from sift.data.v2.data_pb2 import GetDataRequest, GetDataResponse, Query
from sift.data.v2.data_pb2_grpc import DataServiceStub
from sift.runs.v2.runs_pb2 import ListRunsRequest, ListRunsResponse, Run
from sift.runs.v2.runs_pb2_grpc import RunServiceStub
from typing_extensions import TypeAlias

from sift_py._internal.cel import cel_in
from sift_py._internal.channel import channel_fqn
from sift_py._internal.convert.timestamp import to_pb_timestamp
from sift_py.data._channel import ChannelTimeSeries
from sift_py.data._deserialize import try_deserialize_channel_data
from sift_py.data._validate import validate_channel_reference
from sift_py.data.error import DataError
from sift_py.data.query import CalculatedChannelQuery, ChannelQuery, DataQuery, DataQueryResult
from sift_py.error import SiftError, _component_deprecation_warning
from sift_py.grpc.transport import SiftAsyncChannel
from sift_py.ingestion.channel import ChannelDataType


class DataService:
    """
    A service that asynchronously executes a `sift_py.data.query.DataQuery` to retrieve telemetry
    for an arbitrary amount of channels (or calculated channels) within a user-specified time-range
    and sampling rate.
    """

    # TODO: There is a pagination issue API side when requesting multiple channels in single request.
    # If all data points for all channels in a single request don't fit into a single page, then
    # paging seems to omit all but a single channel. We can increase this batch size once that issue
    # has been resolved. In the mean time each channel gets its own request.
    REQUEST_BATCH_SIZE = 1

    AssetName: TypeAlias = str
    ChannelFqn: TypeAlias = str
    RunName: TypeAlias = str

    _asset_service_stub: AssetServiceStub
    _channel_service_stub: ChannelServiceStub
    _calculated_channel_service_stub: CalculatedChannelsServiceStub
    _data_service_stub: DataServiceStub
    _run_service_stub: RunServiceStub

    _cached_assets: Dict[AssetName, Asset]
    _cached_channels: Dict[AssetName, Dict[ChannelFqn, List[Channel]]]
    _cached_runs: Dict[RunName, Run]

    def __init__(self, channel: SiftAsyncChannel):
        self._asset_service_stub = AssetServiceStub(channel)
        self._channel_service_stub = ChannelServiceStub(channel)
        self._calculated_channel_service_stub = CalculatedChannelsServiceStub(channel)
        self._data_service_stub = DataServiceStub(channel)
        self._run_service_stub = RunServiceStub(channel)

        self._cached_assets = {}
        self._cached_channels = {}
        self._cached_runs = {}

    async def execute(self, query: DataQuery, bust_cache: bool = False) -> DataQueryResult:
        """
        Performs the actual query to retrieve telemetry.
        """

        if bust_cache:
            self._bust_cache()

        asset = await self._load_asset(query.asset_name)

        channel_queries: List[ChannelQuery] = []
        for c in query.channels:
            if isinstance(c, ChannelQuery):
                channel_queries.append(c)
            elif isinstance(c, CalculatedChannelQuery):
                for ref in c.expression_channel_references:
                    channel_name = ref["channel_name"]

                    # Deprecated component field
                    component = ref.get("component")
                    if component is not None:
                        _component_deprecation_warning()
                        channel_name = channel_fqn(name=channel_name, component=component)

                    channel_queries.append(ChannelQuery(channel_name=channel_name))

        channels = await self._load_channels(asset, channel_queries)
        runs = await self._load_runs(query.channels)

        queries: List[Query] = []

        for channel_query in query.channels:
            if isinstance(channel_query, ChannelQuery):
                fqn = channel_query.fqn()
                run_name = channel_query.run_name
                targets = channels.get(fqn)

                if not targets:
                    raise SiftError(
                        f"An unexpected error occurred. Expected channel '{fqn}' to have been loaded. Are you sure the asset '{asset.name}' has the channel '{fqn}'?"
                    )
                cqueries = [ChannelQueryPb(channel_id=channel.channel_id) for channel in targets]

                if run_name is not None:
                    run = runs.get(run_name)

                    if run is None:
                        raise SiftError(
                            f"An unexpected error occurred. Expected run '{run_name}' to have been loaded."
                        )

                    for cquery in cqueries:
                        cquery.run_id = run.run_id

                for cquery in cqueries:
                    queries.append(Query(channel=cquery))

            elif isinstance(channel_query, CalculatedChannelQuery):
                expression_channel_references = []

                for expr_ref in channel_query.expression_channel_references:
                    validate_channel_reference(expr_ref["reference"])

                    channel_name = expr_ref["channel_name"]
                    component = expr_ref.get("component")
                    if component is not None:
                        _component_deprecation_warning()
                        channel_name = channel_fqn(name=channel_name, component=component)

                    targets = channels.get(channel_name)

                    if not targets:
                        raise SiftError(
                            f"An unexpected error occurred. Expected channel '{channel_name}' to have been loaded."
                        )

                    channel_id = targets[0].channel_id

                    if len(targets) > 1:
                        target_data_type = expr_ref.get("data_type")

                        if target_data_type is None:
                            raise ValueError(
                                f"Found multiple channels with the fully qualified name '{channel_name}'. A 'data_type' must be provided in `ExpressionChannelReference`."
                            )

                        for target in targets:
                            if ChannelDataType.from_pb(target.data_type) == target_data_type:
                                channel_id = target.channel_id
                                break

                    expression_channel_references.append(
                        ExpressionChannelReference(
                            channel_reference=expr_ref["reference"], channel_id=channel_id
                        )
                    )

                expression_request = ExpressionRequest(
                    expression=channel_query.expression,
                    expression_channel_references=expression_channel_references,
                )

                calculated_cquery = CalculatedChannelQueryPb(
                    channel_key=channel_query.channel_key,
                    expression=expression_request,
                )

                run_name = channel_query.run_name

                if run_name is not None:
                    run = runs.get(run_name)

                    if run is None:
                        raise SiftError(
                            f"An unexpected error occurred. Expected run '{run_name}' to have been loaded."
                        )

                    calculated_cquery.run_id = run.run_id

                queries.append(Query(calculated_channel=calculated_cquery))

            else:
                raise DataError("Unknown channel query type.")

        await self._validate_queries(queries)

        start_time = to_pb_timestamp(query.start_time)
        end_time = to_pb_timestamp(query.end_time)
        sample_ms = query.sample_ms
        page_size = query.page_size

        tasks = []

        for batch in self._batch_queries(queries):
            req = GetDataRequest(
                start_time=start_time,
                end_time=end_time,
                sample_ms=sample_ms,
                page_size=page_size,
                queries=batch,
            )
            task = asyncio.create_task(self._get_data(req))
            tasks.append(task)

        data_pages: List[Iterable[Any]] = []

        for pages in await asyncio.gather(*tasks):
            # Empty pages will have no effect
            data_pages.extend(pages)

        return DataQueryResult(self._merge_and_sort_pages(data_pages))

    async def _get_data(self, req: GetDataRequest) -> List[Iterable[Any]]:
        pages: List[Iterable[Any]] = []

        start_time = req.start_time
        end_time = req.end_time
        sample_ms = req.sample_ms
        page_size = req.page_size
        queries = req.queries
        next_page_token = ""

        while True:
            next_page_req = GetDataRequest(
                start_time=start_time,
                end_time=end_time,
                sample_ms=sample_ms,
                page_size=page_size,
                queries=queries,
                page_token=next_page_token,
            )
            response = cast(GetDataResponse, await self._data_service_stub.GetData(next_page_req))

            pages.append(response.data)
            next_page_token = response.next_page_token

            if len(next_page_token) == 0:
                break

        return pages

    def _merge_and_sort_pages(
        self, pages: List[Iterable[Any]]
    ) -> Dict[str, List[ChannelTimeSeries]]:
        if len(pages) == 0:
            return {}

        merged_values_by_channel: Dict[str, List[ChannelTimeSeries]] = {}

        for page in pages:
            for raw_channel_values in page:
                parsed_channel_data = try_deserialize_channel_data(cast(Any, raw_channel_values))

                for metadata, cvalues in parsed_channel_data:
                    channel = metadata.channel

                    channel_name = channel.name or channel.channel_id

                    time_series = merged_values_by_channel.get(channel_name)

                    if time_series is None:
                        merged_values_by_channel[channel_name] = [
                            ChannelTimeSeries(
                                data_type=cvalues.data_type,
                                time_column=cvalues.time_column,
                                value_column=cvalues.value_column,
                            ),
                        ]
                    else:
                        for series in time_series:
                            if series.data_type == cvalues.data_type:
                                series.time_column.extend(cvalues.time_column)
                                series.value_column.extend(cvalues.value_column)
                                break
                        else:  # for-else
                            # Situation in which multiple channels with identical fully-qualified names but different types.
                            time_series.append(
                                ChannelTimeSeries(
                                    data_type=cvalues.data_type,
                                    time_column=cvalues.time_column,
                                    value_column=cvalues.value_column,
                                )
                            )

        for data in merged_values_by_channel.values():
            for channel_data in data:
                channel_data.sort_time_series()

        return merged_values_by_channel

    def _bust_cache(self):
        self._cached_assets.clear()
        self._cached_channels.clear()
        self._cached_runs.clear()

    async def _load_asset(self, asset_name: str) -> Asset:
        asset = self._cached_assets.get(asset_name)

        if asset is None:
            asset = await self._get_asset_by_name(asset_name)
            self._cached_assets[asset.name] = asset

        return asset

    async def _load_channels(
        self,
        asset: Asset,
        channel_queries: List[ChannelQuery],
    ) -> Dict[ChannelFqn, List[Channel]]:
        if self._cached_channels.get(asset.name) is None:
            sift_channels = await self._get_channels_by_asset_id(asset.asset_id, channel_queries)

            channels = defaultdict(list)

            for c in sift_channels:
                channels[c.name].append(c)

            self._cached_channels[asset.name] = channels
            return self._cached_channels[asset.name]

        cached_channels = self._cached_channels[asset.name]
        channels_to_retrieve: List[ChannelQuery] = []
        for query in channel_queries:
            if cached_channels.get(query.channel_name) is None:
                channels_to_retrieve.append(query)

        sift_channels = []
        if len(channels_to_retrieve) > 0:
            sift_channels = await self._get_channels_by_asset_id(
                asset.asset_id, channels_to_retrieve
            )

        channels = defaultdict(list)

        for c in sift_channels:
            channels[c.name].append(c)

        if len(channels) > 0:
            self._cached_channels[asset.name].update(channels)

        return self._cached_channels[asset.name]

    async def _load_runs(
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

        for run in await self._get_runs_by_names(run_names_to_fetch):
            self._cached_runs[run.name] = run
            runs[run.name] = run

        return runs

    async def _get_asset_by_name(self, asset_name: str) -> Asset:
        req = ListAssetsRequest(
            filter=f'name=="{asset_name}"',
            page_size=1,
        )
        res = cast(ListAssetsResponse, await self._asset_service_stub.ListAssets(req))
        assets = res.assets

        if len(assets) == 0:
            raise DataError(f"Asset of name '{asset_name}' does not exist.")

        return res.assets[0]

    async def _get_runs_by_names(self, run_names: Set[str]) -> List[Run]:
        if len(run_names) == 0:
            return []

        runs: List[Run] = []

        filter = cel_in("name", run_names)
        page_size = 1_000
        next_page_token = ""

        while True:
            req = ListRunsRequest(
                filter=filter,
                page_size=page_size,
                page_token=next_page_token,
            )
            res = cast(ListRunsResponse, await self._run_service_stub.ListRuns(req))
            runs.extend(res.runs)

            next_page_token = res.next_page_token

            if len(next_page_token) == 0:
                break

        seen_sift_runs = set()

        for sift_run in runs:
            seen_sift_runs.add(sift_run.name)

        for run_name in run_names:
            if run_name not in seen_sift_runs:
                raise DataError(f"Run of name '{run_name}' does not exist.")

        return runs

    async def _get_channels_by_asset_id(
        self, asset_id: str, channel_queries: List[ChannelQuery]
    ) -> List[Channel]:
        if len(asset_id) == 0 or len(channel_queries) == 0:
            return []

        channels: List[Channel] = []

        channel_names = []

        for query in channel_queries:
            channel_names.append(query.channel_name)

        name_in = cel_in("name", channel_names)

        filter = f'asset_id=="{asset_id}" && {name_in}'
        page_size = 1_000
        next_page_token = ""

        while True:
            req = ListChannelsRequest(
                filter=filter,
                page_size=page_size,
                page_token=next_page_token,
            )
            res = cast(ListChannelsResponse, await self._channel_service_stub.ListChannels(req))
            channels.extend(res.channels)
            next_page_token = res.next_page_token

            if len(next_page_token) == 0:
                break

        return channels

    def _batch_queries(self, queries: List[Query]) -> List[List[Query]]:
        if len(queries) == 0:
            return []

        batches: List[List[Query]] = []
        batch_size = self.__class__.REQUEST_BATCH_SIZE

        for i in range(0, len(queries), batch_size):
            batches.append(queries[i : i + batch_size])

        return batches

    async def _validate_queries(self, queries: List[Query]):
        queries_to_validate: List[ExpressionRequest] = []

        for query in queries:
            if query.HasField("calculated_channel"):
                queries_to_validate.append(query.calculated_channel.expression)

        if len(queries_to_validate) > 0:
            tasks = []

            for to_validate in queries_to_validate:
                task = asyncio.create_task(self._validate_expression(to_validate))
                tasks.append(task)

            for result in await asyncio.gather(*tasks):
                if result is not None:
                    expr, err = result
                    raise ValueError(f"Encountered an invalid expression '{expr}': {err}")

    async def _validate_expression(self, req: ExpressionRequest) -> Optional[Tuple[str, Exception]]:
        try:
            self._calculated_channel_service_stub.ValidateExpression(req)
            return None
        except Exception as err:
            return (req.expression, err)
