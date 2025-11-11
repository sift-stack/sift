"""Tests for sift_types.Ingestion models."""

from datetime import datetime, timezone
from unittest.mock import MagicMock

import pytest

from sift_client.sift_types.channel import ChannelBitFieldElement, ChannelDataType
from sift_client.sift_types.ingestion import (
    ChannelConfig,
    FlowConfig,
    IngestionConfig,
)


class TestChannelConfig:
    """Unit tests for ChannelConfig model - tests validators."""

    def test_enum_validator_rejects_enum_without_enum_types(self):
        """Test validator rejects ENUM data_type without enum_types."""
        with pytest.raises(
            ValueError,
            match="Channel 'test_channel' has data_type ENUM but enum_types is not provided",
        ):
            ChannelConfig(
                name="test_channel",
                data_type=ChannelDataType.ENUM,
            )

    def test_enum_validator_accepts_enum_with_enum_types(self):
        """Test validator accepts ENUM data_type with enum_types."""
        # Should not raise
        channel = ChannelConfig(
            name="test_channel",
            data_type=ChannelDataType.ENUM,
            enum_types={"LOW": 0, "HIGH": 1},
        )
        assert channel.data_type == ChannelDataType.ENUM
        assert channel.enum_types == {"LOW": 0, "HIGH": 1}

    def test_bitfield_validator_rejects_bitfield_without_elements(self):
        """Test validator rejects BIT_FIELD data_type without bit_field_elements."""
        with pytest.raises(
            ValueError,
            match="Channel 'test_channel' has data_type BIT_FIELD but bit_field_elements is not provided",
        ):
            ChannelConfig(
                name="test_channel",
                data_type=ChannelDataType.BIT_FIELD,
            )

    def test_bitfield_validator_accepts_bitfield_with_elements(self):
        """Test validator accepts BIT_FIELD data_type with bit_field_elements."""
        # Should not raise
        channel = ChannelConfig(
            name="test_channel",
            data_type=ChannelDataType.BIT_FIELD,
            bit_field_elements=[
                ChannelBitFieldElement(name="field1", index=0, bit_count=4),
                ChannelBitFieldElement(name="field2", index=1, bit_count=4),
            ],
        )
        assert channel.data_type == ChannelDataType.BIT_FIELD
        assert len(channel.bit_field_elements) == 2

    def test_other_data_types_dont_require_special_fields(self):
        """Test that other data types don't require enum_types or bit_field_elements."""
        # Should not raise for DOUBLE
        channel = ChannelConfig(
            name="test_channel",
            data_type=ChannelDataType.DOUBLE,
        )
        assert channel.data_type == ChannelDataType.DOUBLE


class TestFlowConfig:
    """Unit tests for FlowConfig model."""

    def test_as_flow_creates_flow_with_values(self):
        """Test that as_flow creates a Flow with correct channel values."""
        flow_config = FlowConfig(
            name="test_flow",
            channels=[
                ChannelConfig(name="channel1", data_type=ChannelDataType.DOUBLE),
                ChannelConfig(name="channel2", data_type=ChannelDataType.INT_64),
            ],
        )

        timestamp = datetime(2024, 1, 1, 12, 0, 0, tzinfo=timezone.utc)
        values = {"channel1": 42.5, "channel2": 100}

        flow = flow_config.as_flow(timestamp=timestamp, values=values)

        assert flow.flow == "test_flow"
        assert flow.timestamp == timestamp
        assert len(flow.channel_values) == 2
        assert flow.channel_values[0].name == "channel1"
        assert flow.channel_values[0].value == 42.5
        assert flow.channel_values[1].name == "channel2"
        assert flow.channel_values[1].value == 100

    def test_as_flow_raises_on_unknown_channel(self):
        """Test that as_flow raises ValueError for unknown channel values."""
        flow_config = FlowConfig(
            name="test_flow",
            channels=[ChannelConfig(name="channel1", data_type=ChannelDataType.DOUBLE)],
        )

        timestamp = datetime(2024, 1, 1, 12, 0, 0, tzinfo=timezone.utc)
        values = {"channel1": 42.5, "unknown_channel": 100}

        with pytest.raises(
            ValueError,
            match="Provided channel values which do not exist in the flow config",
        ):
            flow_config.as_flow(timestamp=timestamp, values=values)

    def test_as_flow_only_includes_provided_channels(self):
        """Test that as_flow only includes channels with provided values."""
        flow_config = FlowConfig(
            name="test_flow",
            channels=[
                ChannelConfig(name="channel1", data_type=ChannelDataType.DOUBLE),
                ChannelConfig(name="channel2", data_type=ChannelDataType.FLOAT),
                ChannelConfig(name="channel3", data_type=ChannelDataType.INT_64),
            ],
        )

        timestamp = datetime(2024, 1, 1, 12, 0, 0, tzinfo=timezone.utc)
        values = {"channel1": 42.5, "channel3": 100}

        flow = flow_config.as_flow(timestamp=timestamp, values=values)

        assert len(flow.channel_values) == 2
        assert flow.channel_values[0].name == "channel1"
        assert flow.channel_values[1].name == "channel3"


class TestIngestionConfig:
    """Unit tests for IngestionConfig model."""

    def test_ingestion_config_has_required_fields(self):
        """Test that IngestionConfig can be created with required fields."""
        config = IngestionConfig(
            proto=MagicMock(),
            id_="config123",
            asset_id="asset123",
            client_key="client_key_123",
        )

        assert config.id_ == "config123"
        assert config.asset_id == "asset123"
        assert config.client_key == "client_key_123"
