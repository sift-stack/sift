from __future__ import annotations

import logging
from typing import TYPE_CHECKING, Any

from sift_client._internal.low_level_wrappers.ingestion import IngestionLowLevelClient
from sift_client.resources._base import ResourceBase

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

    async def create_ingestion_config(
        self,
        *,
        asset_name: str,
        run_id: str | None = None,
        flows: list[Flow],
        client_key: str | None = None,
        organization_id: str | None = None,
    ) -> str:
        """Create an ingestion config.

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

    def ingest(
        self,
        *,
        flow: Flow,
        timestamp: datetime,
        channel_values: dict[str, Any],
    ):
        """Ingest data for a flow.

        Args:
            flow: The flow to ingest data for.
            timestamp: The timestamp of the data.
            channel_values: Dictionary mapping channel names to their values.
        """
        self._low_level_client.ingest_flow(
            flow=flow,
            timestamp=timestamp,
            channel_values=channel_values,
        )

    def wait_for_ingestion_to_complete(self, timeout: float | None = None):
        """Wait for all ingestion to complete.

        Args:
            run_id: The id of the run to wait for.
            timeout: The timeout in seconds to wait for ingestion to complete. If None, will wait forever.
        """
        logger.info("Waiting for ingestion to complete")
        self._low_level_client.wait_for_ingestion_to_complete(timeout)
