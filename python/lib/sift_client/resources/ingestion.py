from __future__ import annotations

import logging
from typing import TYPE_CHECKING, Any

from sift_client._internal.low_level_wrappers.ingestion import (
    IngestionConfigStreamingLowLevelClient,
    IngestionLowLevelClient,
)
from sift_client.resources._base import ResourceBase
from sift_client._internal.util.sift_stream import to_runFormPy
from sift_client.sift_types.ingestion import IngestionConfig
from sift_client.sift_types.run import Run, RunCreate, Tag
from sift_stream_bindings import (
    DurationPy,
    IngestionConfigFormPy,
    MetadataPy,
    RecoveryStrategyPy,
    RetryPolicyPy,
    RunSelectorPy,
    MetadataValuePy,
    IngestWithConfigDataStreamRequestPy,
    SiftStreamMetricsSnapshotPy,
)

if TYPE_CHECKING:
    from datetime import datetime

    from sift_client.client import SiftClient
    from sift_client.sift_types.ingestion import Flow

logger = logging.getLogger(__name__)


class IngestionAPIAsync(ResourceBase):
    """High-level API for interacting with ingestion services.

    This class provides a Pythonic, notebook-friendly interface for interacting with the IngestionAPI.
    It handles automatic handling of gRPC services, seamless type conversion, and clear error handling.

    All methods in this class use the Flow class from the types module, which is a user-friendly
    representation of ingestion flows using standard Python data structures and types.
    """

    def __init__(self, sift_client: SiftClient):
        """Initialize the IngestionAPI.

        Args:
            sift_client: The Sift client to use.
        """
        super().__init__(sift_client)
        self._low_level_client = IngestionLowLevelClient(grpc_client=self.client.grpc_client)


    async def create_ingestion_config_streaming_client(
        self,
        *,
        ingestion_config: IngestionConfig | None = None,
        run: RunCreate | dict | str | Run | None = None,
        asset_tags: list[str] | list[Tag] | None = None,
        asset_metadata: dict[str, str | float | bool] | None = None,
        recovery_strategy: RecoveryStrategyPy | None = None,
        checkpoint_interval_seconds: int | None = None,
        enable_tls: bool = True,
    ) -> IngestionConfigStreamingClient:
        """Create an IngestionConfigStreamingClient.

        Args:
            ingestion_config: The ingestion config.
            run: The run to associate with ingestion. Can be a Run, RunCreate, dict, or run ID string.
            asset_tags: Tags to associate with the asset.
            asset_metadata: Metadata to associate with the asset.
            recovery_strategy: The recovery strategy to use for ingestion.
            checkpoint_interval_seconds: The checkpoint interval in seconds.
            enable_tls: Whether to enable TLS for the connection.

        Returns:
            An initialized IngestionConfigStreamingClient.
        """
        return await IngestionConfigStreamingClient.create(
            self.client,
            ingestion_config=ingestion_config,
            run=run,
            asset_tags=asset_tags,
            asset_metadata=asset_metadata,
            recovery_strategy=recovery_strategy,
            checkpoint_interval_seconds=checkpoint_interval_seconds,
            enable_tls=enable_tls,
        )

    async def create_ingestion_config(
        self,
        *,
        asset_name: str,
        run_id: str | None = None,
        flows: list[Flow],
        client_key: str | None = None,
    ) -> str:
        """Create an ingestion config. This is provided for direct use of the ingestion config API, and not the preferred way to create ingestion configs for streaming through SiftClient.

        Args:
            asset_name: The name of the asset for this ingestion config.
            run_id: Optionally provide a run ID to create a run for the given asset.
            flows: List of flow configurations.
            client_key: Optional client key for identifying this config.
            organization_id: The organization ID.

        Returns:
            The ingestion config ID.

        Raises:
            ValueError: If asset_name is not provided or flows is empty.
        """
        if not asset_name:
            raise ValueError("asset_name must be provided")
        if not flows:
            raise ValueError("flows must not be empty")

        ingestion_config_id = await self._low_level_client.create_ingestion_config(
            asset_name=asset_name,
            flows=flows,
            client_key=client_key,
        )
        for flow in flows:
            flow._apply_client_to_instance(self.client)
            if run_id:
                flow.run_id = run_id

        return ingestion_config_id


class IngestionConfigStreamingClient(ResourceBase):
    """A client for streaming ingestion with an ingestion config.

    This client provides a high-level interface for streaming ingestion using
    an ingestion config. It handles conversion of user-friendly types to the
    low-level Rust bindings.
    """
    def __init__(self, sift_client: SiftClient, low_level_client: IngestionConfigStreamingLowLevelClient):
        """Initialize an IngestionConfigStreamingClient. Users should not initialize this class directly, but rather use the create classmethod."""
        super().__init__(sift_client)
        self._low_level_client = low_level_client

    @classmethod
    async def create(
        cls,
        sift_client: SiftClient,
        *,
        ingestion_config: IngestionConfigFormPy | None = None,
        run: RunCreate | dict | str | Run | None = None,
        asset_tags: list[str] | list[Tag] | None = None,
        asset_metadata: dict[str, str | float | bool] | None = None,
        recovery_strategy: RecoveryStrategyPy | None = None,
        checkpoint_interval_seconds: int | None = None,
        enable_tls: bool = True,
    ) -> IngestionConfigStreamingClient:
        """Create an IngestionConfigStreamingClient.

        Args:
            sift_client: The Sift client to use.
            ingestion_config: The ingestion config (IngestionConfig or IngestionConfigFormPy).
                If IngestionConfig is provided, you must also provide flows separately.
            run: The run to associate with ingestion. Can be a Run, RunCreate, dict, or run ID string.
            asset_tags: Tags to associate with the asset.
            asset_metadata: Metadata to associate with the asset.
            recovery_strategy: The recovery strategy to use for ingestion.
            checkpoint_interval_seconds: The checkpoint interval in seconds.
            enable_tls: Whether to enable TLS for the connection.

        Returns:
            An initialized IngestionConfigStreamingClient.
        """
        instance = cls.__new__(cls)
        instance._sift_client = sift_client

        # Get API key and gRPC URI from the client
        grpc_config = sift_client.grpc_client._config
        api_key = grpc_config.api_key
        grpc_uri = grpc_config.uri

        low_level_client = await IngestionConfigStreamingLowLevelClient.create_sift_stream_instance(
            api_key=api_key,
            grpc_uri=grpc_uri,
            ingestion_config=ingestion_config,
            run=run,
            asset_tags=asset_tags,
            asset_metadata=asset_metadata,
            recovery_strategy=recovery_strategy,
            checkpoint_interval_seconds=checkpoint_interval_seconds,
            enable_tls=enable_tls,
        )

        return cls(sift_client, low_level_client)

    async def send(self, flow: Flow):
        pass

    async def send_requests(self, requests: list[IngestWithConfigDataStreamRequestPy]):
        pass

    async def add_new_flows(self, flow_configs: list[FlowConfigPy]):
        pass

    async def attach_run(self, run: RunFormPy):
        pass

    def detach_run(self):
        pass

    def get_run_id(self) -> str | None:
        pass

    async def finish(self):
        pass

    def get_metrics_snapshot(self) -> SiftStreamMetricsSnapshotPy:
        pass