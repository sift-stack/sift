"""Pytest tests for the Ingestion API.

These tests demonstrate and validate the usage of the Ingestion API including:
- Creating ingestion configurations
- Ingesting data with various channel types (double, enum, bit field)
- Flow management and validation
- High-speed and regular flow ingestion
- Error handling and edge cases
"""

import math
import random
import time
from datetime import datetime, timedelta, timezone

import pytest

from sift_client import SiftClient
from sift_client.sift_types.channel import ChannelBitFieldElement, ChannelDataType
from sift_client.sift_types.ingestion import ChannelConfig, Flow

pytestmark = pytest.mark.integration

ASSET_NAME = "test-ingestion-asset"


def test_client_binding(sift_client):
    assert getattr(sift_client, "ingestion", None) is None  # Only async!
    assert sift_client.async_.ingestion


def test_run(sift_client: SiftClient):
    """Create a test run for ingestion tests."""
    run = sift_client.runs.create(
        {
            "name": f"test-ingestion-run-{datetime.now(tz=timezone.utc).timestamp()}",
            "description": "Test run for ingestion integration tests",
            "tags": ["test", "ingestion", "pytest"],
        }
    )
    yield run
    # Cleanup
    sift_client.runs.archive(run=run)


class TestIngestionAPIAsync:
    """Test suite for the async Ingestion API functionality."""

    class TestCreateIngestionConfig:
        """Tests for creating ingestion configurations."""

        @pytest.mark.asyncio
        async def test_create_basic_config(self, sift_client, test_run):
            """Test creating a basic ingestion configuration."""
            flow = Flow(
                name="test-basic-flow",
                channels=[
                    ChannelConfig(name="test-channel", data_type=ChannelDataType.DOUBLE),
                ],
            )

            config_id = await sift_client.async_.ingestion.create_ingestion_config(
                asset_name=ASSET_NAME,
                run_id=test_run.id_,
                flows=[flow],
            )

            assert config_id is not None
            assert isinstance(config_id, str)

        @pytest.mark.asyncio
        async def test_create_config_with_multiple_flows(self, sift_client, test_run):
            """Test creating an ingestion configuration with multiple flows."""
            regular_flow = Flow(
                name="test-regular-flow",
                channels=[
                    ChannelConfig(name="regular-channel", data_type=ChannelDataType.DOUBLE),
                ],
            )

            highspeed_flow = Flow(
                name="test-highspeed-flow",
                channels=[
                    ChannelConfig(name="highspeed-channel", data_type=ChannelDataType.DOUBLE),
                ],
            )

            config_id = await sift_client.async_.ingestion.create_ingestion_config(
                asset_name=ASSET_NAME,
                run_id=test_run.id_,
                flows=[regular_flow, highspeed_flow],
            )

            assert config_id is not None

        @pytest.mark.asyncio
        async def test_create_config_with_enum_channel(self, sift_client, test_run):
            """Test creating an ingestion configuration with enum channel."""
            flow = Flow(
                name="test-enum-flow",
                channels=[
                    ChannelConfig(
                        name="test-enum-channel",
                        data_type=ChannelDataType.ENUM,
                        enum_types={"state1": 1, "state2": 2, "state3": 3},
                    ),
                ],
            )

            config_id = await sift_client.async_.ingestion.create_ingestion_config(
                asset_name=ASSET_NAME,
                run_id=test_run.id_,
                flows=[flow],
            )

            assert config_id is not None

        @pytest.mark.asyncio
        async def test_create_config_with_bit_field_channel(self, sift_client, test_run):
            """Test creating an ingestion configuration with bit field channel."""
            flow = Flow(
                name="test-bitfield-flow",
                channels=[
                    ChannelConfig(
                        name="test-bit-field-channel",
                        data_type=ChannelDataType.BIT_FIELD,
                        bit_field_elements=[
                            ChannelBitFieldElement(name="voltage", index=0, bit_count=4),
                            ChannelBitFieldElement(name="current", index=4, bit_count=2),
                            ChannelBitFieldElement(name="status", index=6, bit_count=2),
                        ],
                    ),
                ],
            )

            config_id = await sift_client.async_.ingestion.create_ingestion_config(
                asset_name=ASSET_NAME,
                run_id=test_run.id_,
                flows=[flow],
            )

            assert config_id is not None

        @pytest.mark.asyncio
        async def test_flow_sealed_after_config_creation(self, sift_client, test_run):
            """Test that flows are sealed after ingestion config creation."""
            flow = Flow(
                name="test-sealed-flow",
                channels=[
                    ChannelConfig(name="test-channel", data_type=ChannelDataType.DOUBLE),
                ],
            )

            await sift_client.async_.ingestion.create_ingestion_config(
                asset_name=ASSET_NAME,
                run_id=test_run.id_,
                flows=[flow],
            )

            # Try to add a channel after config creation
            with pytest.raises(ValueError, match="Cannot add a channel to a flow after creation"):
                flow.add_channel(
                    ChannelConfig(name="new-channel", data_type=ChannelDataType.DOUBLE)
                )

    class TestIngestData:
        """Tests for ingesting data."""

        @pytest.mark.asyncio
        async def test_ingest_double_data(self, sift_client, test_run):
            """Test ingesting double data."""
            flow = Flow(
                name="test-double-flow",
                channels=[
                    ChannelConfig(name="double-channel", data_type=ChannelDataType.DOUBLE),
                ],
            )

            await sift_client.async_.ingestion.create_ingestion_config(
                asset_name=ASSET_NAME,
                run_id=test_run.id_,
                flows=[flow],
            )

            start_time = datetime.now(tz=timezone.utc)
            for i in range(10):
                timestamp = start_time + timedelta(seconds=i)
                flow.ingest(
                    timestamp=timestamp,
                    channel_values={"double-channel": float(i)},
                )

            sift_client.async_.ingestion.wait_for_ingestion_to_complete(timeout=2)

        @pytest.mark.asyncio
        async def test_ingest_enum_data(self, sift_client, test_run):
            """Test ingesting enum data."""
            flow = Flow(
                name="test-enum-ingest-flow",
                channels=[
                    ChannelConfig(
                        name="enum-channel",
                        data_type=ChannelDataType.ENUM,
                        enum_types={"low": 1, "medium": 2, "high": 3},
                    ),
                ],
            )

            await sift_client.async_.ingestion.create_ingestion_config(
                asset_name=ASSET_NAME,
                run_id=test_run.id_,
                flows=[flow],
            )

            start_time = datetime.now(tz=timezone.utc)
            for i in range(10):
                timestamp = start_time + timedelta(seconds=i)
                flow.ingest(
                    timestamp=timestamp,
                    channel_values={"enum-channel": (i % 3) + 1},
                )

            sift_client.async_.ingestion.wait_for_ingestion_to_complete(timeout=2)

        @pytest.mark.asyncio
        async def test_ingest_bit_field_data_as_dict(self, sift_client, test_run):
            """Test ingesting bit field data as dictionary."""
            flow = Flow(
                name="test-bitfield-ingest-flow",
                channels=[
                    ChannelConfig(
                        name="bitfield-channel",
                        data_type=ChannelDataType.BIT_FIELD,
                        bit_field_elements=[
                            ChannelBitFieldElement(name="voltage", index=0, bit_count=4),
                            ChannelBitFieldElement(name="current", index=4, bit_count=2),
                            ChannelBitFieldElement(name="led", index=6, bit_count=1),
                            ChannelBitFieldElement(name="heater", index=7, bit_count=1),
                        ],
                    ),
                ],
            )

            await sift_client.async_.ingestion.create_ingestion_config(
                asset_name=ASSET_NAME,
                run_id=test_run.id_,
                flows=[flow],
            )

            start_time = datetime.now(tz=timezone.utc)
            for i in range(10):
                timestamp = start_time + timedelta(seconds=i)
                flow.ingest(
                    timestamp=timestamp,
                    channel_values={
                        "bitfield-channel": {
                            "voltage": random.randint(3, 13),
                            "current": random.randint(1, 3),
                            "led": random.choice([0, 1]),
                            "heater": random.choice([0, 1]),
                        }
                    },
                )

            sift_client.async_.ingestion.wait_for_ingestion_to_complete(timeout=2)

        @pytest.mark.asyncio
        async def test_ingest_bit_field_data_as_bytes(self, sift_client, test_run):
            """Test ingesting bit field data as bytes."""
            flow = Flow(
                name="test-bitfield-bytes-flow",
                channels=[
                    ChannelConfig(
                        name="bitfield-channel",
                        data_type=ChannelDataType.BIT_FIELD,
                        bit_field_elements=[
                            ChannelBitFieldElement(name="field1", index=0, bit_count=4),
                            ChannelBitFieldElement(name="field2", index=4, bit_count=4),
                        ],
                    ),
                ],
            )

            await sift_client.async_.ingestion.create_ingestion_config(
                asset_name=ASSET_NAME,
                run_id=test_run.id_,
                flows=[flow],
            )

            timestamp = datetime.now(tz=timezone.utc)
            flow.ingest(
                timestamp=timestamp,
                channel_values={"bitfield-channel": bytes([0b11110000])},
            )

            sift_client.async_.ingestion.wait_for_ingestion_to_complete(timeout=2)

        @pytest.mark.asyncio
        async def test_ingest_multiple_channels(self, sift_client, test_run):
            """Test ingesting data for multiple channels simultaneously."""
            flow = Flow(
                name="test-multi-channel-flow",
                channels=[
                    ChannelConfig(name="channel1", data_type=ChannelDataType.DOUBLE),
                    ChannelConfig(
                        name="channel2",
                        data_type=ChannelDataType.ENUM,
                        enum_types={"a": 1, "b": 2},
                    ),
                    ChannelConfig(
                        name="channel3",
                        data_type=ChannelDataType.BIT_FIELD,
                        bit_field_elements=[
                            ChannelBitFieldElement(name="bit1", index=0, bit_count=4),
                            ChannelBitFieldElement(name="bit2", index=4, bit_count=4),
                        ],
                    ),
                ],
            )

            await sift_client.async_.ingestion.create_ingestion_config(
                asset_name=ASSET_NAME,
                run_id=test_run.id_,
                flows=[flow],
            )

            start_time = datetime.now(tz=timezone.utc)
            for i in range(5):
                timestamp = start_time + timedelta(seconds=i)
                flow.ingest(
                    timestamp=timestamp,
                    channel_values={
                        "channel1": float(i),
                        "channel2": (i % 2) + 1,
                        "channel3": {"bit1": i % 16, "bit2": (i * 2) % 16},
                    },
                )

            sift_client.async_.ingestion.wait_for_ingestion_to_complete(timeout=2)

        @pytest.mark.asyncio
        async def test_ingest_highspeed_data(self, sift_client, test_run):
            """Test ingesting high-speed data."""
            flow = Flow(
                name="test-highspeed-data-flow",
                channels=[
                    ChannelConfig(name="highspeed-channel", data_type=ChannelDataType.DOUBLE),
                ],
            )

            await sift_client.async_.ingestion.create_ingestion_config(
                asset_name=ASSET_NAME,
                run_id=test_run.id_,
                flows=[flow],
            )

            start_time = datetime.now(tz=timezone.utc)
            fake_hs_rate = 50  # Hz
            fake_hs_period = 1 / fake_hs_rate
            duration = 2  # seconds

            for i in range(duration):
                for j in range(fake_hs_rate):
                    val = 3.0 * math.sin(2 * math.pi * fake_hs_rate * (i + j * 0.001))
                    timestamp = start_time + timedelta(
                        seconds=i, milliseconds=j * fake_hs_period * 1000
                    )
                    flow.ingest(
                        timestamp=timestamp,
                        channel_values={"highspeed-channel": val},
                    )
                time.sleep(0.01)

            sift_client.async_.ingestion.wait_for_ingestion_to_complete(timeout=2)

    class TestIngestionValidation:
        """Tests for ingestion validation and error handling."""

        @pytest.mark.asyncio
        async def test_ingest_invalid_enum_value_raises_error(self, sift_client, test_run):
            """Test that ingesting an invalid enum value raises an error."""
            flow = Flow(
                name="test-enum-validation-flow",
                channels=[
                    ChannelConfig(
                        name="enum-channel",
                        data_type=ChannelDataType.ENUM,
                        enum_types={"valid1": 1, "valid2": 2},
                    ),
                ],
            )

            await sift_client.async_.ingestion.create_ingestion_config(
                asset_name=ASSET_NAME,
                run_id=test_run.id_,
                flows=[flow],
            )

            timestamp = datetime.now(tz=timezone.utc)
            # Test with invalid integer
            with pytest.raises(ValueError, match="Could not find enum value"):
                flow.ingest(
                    timestamp=timestamp,
                    channel_values={"enum-channel": 99},
                )

            # Test with invalid string
            with pytest.raises(ValueError, match="Could not find enum value"):
                flow.ingest(
                    timestamp=timestamp,
                    channel_values={"enum-channel": "invalid-enum"},
                )

        @pytest.mark.asyncio
        async def test_resume_ingestion_after_wait(self, sift_client, test_run):
            """Test that ingestion can resume after waiting for completion."""
            flow = Flow(
                name="test-resume-flow",
                channels=[
                    ChannelConfig(name="test-channel", data_type=ChannelDataType.DOUBLE),
                ],
            )

            await sift_client.async_.ingestion.create_ingestion_config(
                asset_name=ASSET_NAME,
                run_id=test_run.id_,
                flows=[flow],
            )

            # First batch
            timestamp1 = datetime.now(tz=timezone.utc)
            flow.ingest(timestamp=timestamp1, channel_values={"test-channel": 1.0})

            sift_client.async_.ingestion.wait_for_ingestion_to_complete(timeout=2)

            # Wait a bit
            time.sleep(0.1)

            # Second batch after wait
            timestamp2 = timestamp1 + timedelta(seconds=2)
            flow.ingest(timestamp=timestamp2, channel_values={"test-channel": 2.0})

            sift_client.async_.ingestion.wait_for_ingestion_to_complete(timeout=2)
