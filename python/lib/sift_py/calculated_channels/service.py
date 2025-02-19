from __future__ import annotations

from pathlib import Path
from typing import Any, Dict, List, Optional, Tuple, Union, cast

from google.protobuf.field_mask_pb2 import FieldMask
from google.protobuf.timestamp_pb2 import Timestamp
from sift.assets.v1.assets_pb2 import Asset, ListAssetsRequest, ListAssetsResponse
from sift.assets.v1.assets_pb2_grpc import AssetServiceStub
from sift.calculated_channels.v2.calculated_channels_pb2 import (
    CalculatedChannel,
    CalculatedChannelAbstractChannelReference,
    CalculatedChannelAssetConfiguration,
    CalculatedChannelConfiguration,
    CalculatedChannelQueryConfiguration,
    CalculatedChannelValidationResult,
    CreateCalculatedChannelRequest,
    CreateCalculatedChannelResponse,
    GetCalculatedChannelRequest,
    GetCalculatedChannelResponse,
    ListCalculatedChannelsRequest,
    ListCalculatedChannelsResponse,
    ListCalculatedChannelVersionsRequest,
    UpdateCalculatedChannelRequest,
)
from sift.calculated_channels.v2.calculated_channels_pb2_grpc import CalculatedChannelServiceStub

from sift_py._internal.cel import cel_in
from sift_py.calculated_channels.config import CalculatedChannelConfig, CalculatedChannelUpdate
from sift_py.grpc.transport import SiftChannel
from sift_py.rule.config import (
    _channel_references_from_dicts,
)
from sift_py.yaml.calculated_channels import load_calculated_channels


class CalculatedChannelService:
    """
    A service for managing reusable Calculated Channels. Allows for creating, updating, and retrieving Calculated Channels.
    """

    _calculated_channel_service_stub: CalculatedChannelServiceStub
    _asset_service_stub: AssetServiceStub

    def __init__(self, channel: SiftChannel):
        self._calculated_channel_service_stub = CalculatedChannelServiceStub(channel)
        self._asset_service_stub = AssetServiceStub(channel)

    def get_calculated_channel(
        self, calculated_channel_id: Optional[str] = None, client_key: Optional[str] = None
    ) -> CalculatedChannelConfig:
        """
        Get a `CalculatedChannel`.  See `Sift docs`_
        for more information on available arguments.

        .. _Sift docs: https://docs.siftstack.com/docs/api/grpc/protocol-buffers/calculated_channels
        """
        return self._calculated_channel_to_config(
            self._get_calculated_channel(
                calculated_channel_id=calculated_channel_id, client_key=client_key
            )
        )

    def _get_calculated_channel(
        self, calculated_channel_id: Optional[str] = None, client_key: Optional[str] = None
    ) -> CalculatedChannel:
        if not calculated_channel_id and not client_key:
            raise ValueError("Must provide either `id` or `client_key`")

        if calculated_channel_id:
            req = GetCalculatedChannelRequest(
                calculated_channel_id=calculated_channel_id,
            )
        else:
            req = GetCalculatedChannelRequest(
                client_key=client_key,  # type: ignore
            )

        res = cast(
            GetCalculatedChannelResponse,
            self._calculated_channel_service_stub.GetCalculatedChannel(req),
        )
        return cast(CalculatedChannel, res.calculated_channel)

    def list_calculated_channels(
        self,
        page_size: Optional[int] = None,
        page_token: Optional[str] = None,
        filter: Optional[str] = None,
        order_by: Optional[str] = None,
    ) -> Tuple[List[CalculatedChannelConfig], str]:
        """
        List available Calculated Channels. See `Sift docs`_
        for more information on available arguments.

        Returns a tuple of a list of `CalculatedChannel` objects and a next page token.

        .. _Sift docs: https://docs.siftstack.com/docs/api/grpc/protocol-buffers/calculated_channels
        """
        request_kwargs: Dict[str, Any] = {}
        if page_size is not None:
            request_kwargs["page_size"] = page_size
        if page_token is not None:
            request_kwargs["page_token"] = page_token
        if filter is not None:
            request_kwargs["filter"] = filter
        if order_by is not None:
            request_kwargs["order_by"] = order_by

        req = ListCalculatedChannelsRequest(**request_kwargs)
        resp = cast(
            ListCalculatedChannelsResponse,
            self._calculated_channel_service_stub.ListCalculatedChannels(req),
        )
        return (
            [
                self._calculated_channel_to_config(cast(CalculatedChannel, chan))
                for chan in resp.calculated_channels
            ],
            resp.next_page_token,
        )

    def list_calculated_channel_versions(
        self,
        calculated_channel_id: Optional[str] = None,
        client_key: Optional[str] = None,
        page_size: Optional[int] = None,
        page_token: Optional[str] = None,
        filter: Optional[str] = None,
        order_by: Optional[str] = None,
    ) -> Tuple[List[CalculatedChannelConfig], str]:
        """
        List versions of Calculated Channel. See `Sift docs`_
        for more information on available arguments.

        Returns a tuple of a list of `CalculatedChannel` objects and a next page token.

        .. _Sift docs: https://docs.siftstack.com/docs/api/grpc/protocol-buffers/calculated_channels
        """
        if not calculated_channel_id and not client_key:
            raise ValueError("Must provide either `id` or `client_key`")

        request_kwargs: Dict[str, Any] = {}
        if calculated_channel_id is not None:
            request_kwargs["calculated_channel_id"] = calculated_channel_id
        else:
            request_kwargs["client_key"] = client_key

        if page_size is not None:
            request_kwargs["page_size"] = page_size
        if page_token is not None:
            request_kwargs["page_token"] = page_token
        if filter is not None:
            request_kwargs["filter"] = filter
        if order_by is not None:
            request_kwargs["order_by"] = order_by

        req = ListCalculatedChannelVersionsRequest(**request_kwargs)
        resp = self._calculated_channel_service_stub.ListCalculatedChannelVersions(req)
        return (
            [
                self._calculated_channel_to_config(cast(CalculatedChannel, chan))
                for chan in resp.calculated_channel_versions
            ],
            resp.next_page_token,
        )

    def create_calculated_channel(
        self, config: CalculatedChannelConfig
    ) -> Tuple[CalculatedChannelConfig, CalculatedChannelValidationResult]:
        """
        Create a `CalculatedChannel` from a `CalculatedChannelConfig`. See
        `sift_py.calculated_channels.config.CalculatedChannelConfig` for more information on available
        fields to configure.
        """
        asset_configuration = CalculatedChannelAssetConfiguration(
            all_assets=config.all_assets,
            selection=CalculatedChannelAssetConfiguration.AssetSelection(
                asset_ids=[asset.asset_id for asset in self._get_assets(names=config.asset_names)]
                if config.asset_names
                else None,
                tag_ids=config.tag_names,
            )
            if not config.all_assets
            else None,
        )
        query_configuration = CalculatedChannelQueryConfiguration(
            sel=CalculatedChannelQueryConfiguration.Sel(
                expression=config.expression,
                expression_channel_references=[
                    CalculatedChannelAbstractChannelReference(**ch)
                    for ch in config.channel_references
                ],
            ),
        )
        calculated_channel_configuration = CalculatedChannelConfiguration(
            asset_configuration=asset_configuration, query_configuration=query_configuration
        )
        req = CreateCalculatedChannelRequest(
            name=config.name,
            description=config.description,
            units=config.units,
            client_key=config.client_key,
            calculated_channel_configuration=calculated_channel_configuration,
        )
        resp = cast(
            CreateCalculatedChannelResponse,
            self._calculated_channel_service_stub.CreateCalculatedChannel(req),
        )
        return self._calculated_channel_to_config(
            cast(CalculatedChannel, resp.calculated_channel)
        ), cast(CalculatedChannelValidationResult, resp.inapplicable_assets)

    def update_calculated_channel(
        self,
        calculated_channel_config: CalculatedChannelConfig,
        updates: CalculatedChannelUpdate,
        update_notes: str = "",
    ) -> Tuple[CalculatedChannelConfig, CalculatedChannelValidationResult]:
        """
        Revise a `CalculatedChannel` from a `CalculatedChannelUpdate`.  See
        `sift_py.calculated_channels.config.CalculatedChannelUpdate` for more information on available
        fields to update.

        `revision_notes` may be provided to document the reason for revision.

        """
        calculated_channel = self._get_calculated_channel(
            calculated_channel_id=calculated_channel_config.calculated_channel_id,
            client_key=calculated_channel_config.client_key,
        )

        update_map: Dict[str, Any] = {}
        if "name" in updates:
            update_map["name"] = updates["name"]
        if "description" in updates:
            update_map["description"] = updates["description"]
        if "units" in updates:
            update_map["units"] = updates["units"]

        if "expression" in updates or "channel_references" in updates:
            expression = (
                updates.get("expression")
                or calculated_channel.calculated_channel_configuration.query_configuration.sel.expression
            )
            channel_reference_dicts = _channel_references_from_dicts(
                updates.get("channel_references") or []
            )
            channel_references = (
                [CalculatedChannelAbstractChannelReference(**ch) for ch in channel_reference_dicts]
                if channel_reference_dicts
                else calculated_channel.calculated_channel_configuration.query_configuration.sel.expression_channel_references
            )
            update_map["query_configuration"] = CalculatedChannelQueryConfiguration(
                sel=CalculatedChannelQueryConfiguration.Sel(
                    expression=expression,
                    expression_channel_references=channel_references,
                )
            )
        if "asset_names" in updates or "tag_names" in updates or "all_assets" in updates:
            asset_ids = (
                [asset.asset_id for asset in self._get_assets(names=updates.get("asset_names"))]
                if "asset_names" in updates
                else calculated_channel.calculated_channel_configuration.asset_configuration.selection.asset_ids
            )

            tag_ids = (
                updates.get("tag_names")
                if "tag_names" in updates
                else calculated_channel.calculated_channel_configuration.asset_configuration.selection.tag_ids
            )
            # TODO: add full support for tags
            if "tag_names" in updates and updates.get("tag_names") is not None:
                raise NotImplementedError(
                    "Modifying `tag_names` (other than removing them by setting to None) is not currently supported."
                )

            all_assets = (
                updates.get("all_assets")
                if "all_assets" in updates
                else calculated_channel.calculated_channel_configuration.asset_configuration.all_assets
            )
            update_map["asset_configuration"] = CalculatedChannelAssetConfiguration(
                all_assets=all_assets,  # type: ignore
                selection=None
                if all_assets
                else CalculatedChannelAssetConfiguration.AssetSelection(
                    asset_ids=asset_ids, tag_ids=tag_ids
                ),
            )

        if "archived" in updates:
            ts = Timestamp()
            ts.GetCurrentTime()
            update_map["archived_date"] = None if not updates["archived"] else ts

        channel_updater = CalculatedChannel(
            calculated_channel_id=calculated_channel.calculated_channel_id,
            name=update_map.get("name", calculated_channel.name),
            description=update_map.get("description", calculated_channel.description),
            units=update_map.get("units", calculated_channel.units),
            calculated_channel_configuration=CalculatedChannelConfiguration(
                asset_configuration=update_map.get(
                    "asset_configuration",
                    calculated_channel.calculated_channel_configuration.asset_configuration,
                ),
                query_configuration=update_map.get(
                    "query_configuration",
                    calculated_channel.calculated_channel_configuration.query_configuration,
                ),
            ),
            archived_date=update_map.get("archived_date", calculated_channel.archived_date),
        )
        update_mask = FieldMask(paths=list(update_map.keys()))

        req = UpdateCalculatedChannelRequest(
            calculated_channel=channel_updater, update_mask=update_mask, user_notes=update_notes
        )
        resp = self._calculated_channel_service_stub.UpdateCalculatedChannel(req)
        return self._calculated_channel_to_config(
            cast(CalculatedChannel, resp.calculated_channel)
        ), cast(CalculatedChannelValidationResult, resp.inapplicable_assets)

    def create_or_update_calculated_channel_from_yaml(
        self, paths: Union[Path, List[Path]]
    ) -> List[Tuple[CalculatedChannelConfig, CalculatedChannelValidationResult]]:
        """
        Creates or updates calculated channel from provided yaml files.
        """
        calculated_channel_configs = load_calculated_channels(
            paths if isinstance(paths, list) else [paths]
        )
        created_or_updated_configs: List[
            Tuple[CalculatedChannelConfig, CalculatedChannelValidationResult]
        ] = []
        for config in calculated_channel_configs:
            if config.client_key is not None:
                try:
                    found_channel = self.get_calculated_channel(client_key=config.client_key)
                    config.calculated_channel_id = found_channel.calculated_channel_id
                except Exception:
                    pass

            if config.calculated_channel_id is not None:
                updates: CalculatedChannelUpdate = {}
                if config.name is not None:
                    updates["name"] = config.name
                if config.description is not None:
                    updates["description"] = config.description
                if config.units is not None:
                    updates["units"] = cast(config.units, str)  # type: ignore[name-defined]
                if config.expression is not None:
                    updates["expression"] = config.expression
                if config.channel_references is not None:
                    updates["channel_references"] = config.channel_references
                if config.asset_names is not None:
                    updates["asset_names"] = cast(config.asset_names, List[str])  # type: ignore[name-defined]
                if config.tag_names is not None:
                    updates["tag_names"] = cast(config.tag_names, List[str])  # type: ignore[name-defined]
                if config.all_assets is not None:
                    updates["all_assets"] = config.all_assets

                created_or_updated_configs.append(
                    self.update_calculated_channel(
                        calculated_channel_config=config, updates=updates
                    )
                )
            else:
                created_or_updated_configs.append(self.create_calculated_channel(config=config))
        return created_or_updated_configs

    @staticmethod
    def _calculated_channel_to_config(
        calculated_channel: CalculatedChannel,
    ) -> CalculatedChannelConfig:
        return CalculatedChannelConfig(
            calculated_channel_id=calculated_channel.calculated_channel_id,
            name=calculated_channel.name,
            description=calculated_channel.description,
            expression=calculated_channel.calculated_channel_configuration.query_configuration.sel.expression,
            channel_references=[
                {
                    "channel_reference": ref.channel_reference,
                    "channel_identifier": ref.channel_identifier,
                }
                for ref in calculated_channel.calculated_channel_configuration.query_configuration.sel.expression_channel_references
            ],
            units=calculated_channel.units,
            client_key=calculated_channel.client_key,
            asset_names=[
                asset_id
                for asset_id in calculated_channel.calculated_channel_configuration.asset_configuration.selection.asset_ids
            ]
            if not calculated_channel.calculated_channel_configuration.asset_configuration.all_assets
            else None,
            tag_names=[
                tag_id
                for tag_id in calculated_channel.calculated_channel_configuration.asset_configuration.selection.tag_ids
            ]
            if not calculated_channel.calculated_channel_configuration.asset_configuration.all_assets
            else None,
            all_assets=calculated_channel.calculated_channel_configuration.asset_configuration.all_assets,
        )

    def _get_assets(
        self, names: Optional[List[str]] = None, ids: Optional[List[str]] = None
    ) -> List[Asset]:
        if names is None:
            names = []
        if ids is None:
            ids = []

        def get_assets_with_filter(cel_filter: str):
            assets: List[Asset] = []
            next_page_token = ""
            while True:
                req = ListAssetsRequest(
                    filter=cel_filter,
                    page_size=1_000,
                    page_token=next_page_token,
                )
                res = cast(ListAssetsResponse, self._asset_service_stub.ListAssets(req))
                assets.extend(res.assets)

                if not res.next_page_token:
                    break
                next_page_token = res.next_page_token

            return assets

        if names:
            names_cel = cel_in("name", names)
            return get_assets_with_filter(names_cel)
        elif ids:
            ids_cel = cel_in("asset_id", ids)
            return get_assets_with_filter(ids_cel)
        else:
            return []
