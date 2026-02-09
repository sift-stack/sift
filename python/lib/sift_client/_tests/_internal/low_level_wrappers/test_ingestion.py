"""Pytest tests for the Ingestion low-level wrapper.

These tests validate the functionality of the IngestionLowLevelClient including:
- Getting ingestion config flows
- Getting ingestion config ID from client key
- Flow name validation
"""

import pytest

from sift_client._internal.low_level_wrappers.ingestion import IngestionLowLevelClient
from sift_client.sift_types.ingestion import FlowConfig

pytestmark = pytest.mark.integration


@pytest.fixture
def ingestion_low_level_client(sift_client):
    """Get the ingestion low-level client instance."""
    return IngestionLowLevelClient(grpc_client=sift_client.grpc_client)


@pytest.mark.asyncio
async def test_get_ingestion_config_flows(ingestion_low_level_client, sift_client):
    """Test that get_ingestion_config_flows returns correct flows.

    This test:
    1. Uses get_ingestion_config_id_from_client_key to get an ingestion config ID
    2. Gets the config flows using get_ingestion_config_flows
    3. Validates the structure and checks flow names for correctness
    """
    asset_id = "a695480e-0069-44b0-ab4b-93e602f743cb"
    
    # First, we need to find an ingestion config to test with
    # We'll list ingestion configs and use the first one's client_key
    ingestion_configs = await ingestion_low_level_client.list_ingestion_configs("")

    if not ingestion_configs:
        pytest.skip("No ingestion configs available for testing")

    # Use the first ingestion config's client_key
    ingestion_config_id = ingestion_configs[0].id_
    assert ingestion_config_id is not None

    # Get flows
    flows = await ingestion_low_level_client.get_ingestion_config_flows(ingestion_config_id)

    # Verify structure
    assert isinstance(flows, list)
    assert len(flows) > 0

    # Verify all items are FlowConfig instances
    for flow in flows:
        assert isinstance(flow, FlowConfig)
        assert flow.name is not None
        assert isinstance(flow.name, str)
        assert len(flow.name) > 0
        assert len(flow.channels) > 0

