import time

import pytest
from sift_stream_bindings import (
    ChannelConfigPy,
    ChannelDataTypePy,
    ChannelValuePy,
    FlowConfigPy,
    FlowPy,
    IngestionConfigFormPy,
    IngestWithConfigDataStreamRequestPy,
    RunFormPy,
    SiftStreamBuilderPy,
    TimeValuePy,
)


class TestFlow:
    """Test FlowPy functionality."""

    def test_create_empty_flow(self):
        """Test creating a flow with no values."""
        timestamp = TimeValuePy.from_timestamp(int(time.time()), 0)
        flow = FlowPy("test_flow", timestamp, [])
        assert flow

    def test_create_flow_with_multiple_values(self):
        """Test creating a flow with multiple channel values."""
        timestamp = TimeValuePy.from_timestamp(int(time.time()), 0)
        values = [
            ChannelValuePy.float("temperature", 23.5),
            ChannelValuePy.bool("active", True),
            ChannelValuePy.string("status", "running"),
            ChannelValuePy.int32("count", 42),
        ]
        flow = FlowPy("test_flow", timestamp, values)
        assert flow


class TestSiftStreamBuilder:
    """Test SiftStreamBuilderPy functionality."""

    def test_create_stream_builder(self):
        """Test creating a stream builder."""
        builder = SiftStreamBuilderPy("https://api.example.com", "test-api-key")
        assert builder is not None
        assert builder.uri == "https://api.example.com"
        assert builder.apikey == "test-api-key"
        assert builder.enable_tls is True
        assert builder.ingestion_config is None
        assert builder.recovery_strategy is None
        assert builder.checkpoint_interval.secs == 60
        assert builder.checkpoint_interval.nanos == 0

    def test_set_ingestion_config(self):
        """Test setting ingestion config on builder."""
        builder = SiftStreamBuilderPy("https://api.example.com", "test-api-key")

        channel = ChannelConfigPy(
            name="test_channel",
            data_type=ChannelDataTypePy.Float,
            unit="m/s",
            description="Test channel",
            enum_types=[],
            bit_field_elements=[],
        )

        flow_config = FlowConfigPy(
            name="test_flow",
            channels=[channel],
        )

        ingestion_config = IngestionConfigFormPy(
            asset_name="test-asset",
            client_key="test-client-key",
            flows=[flow_config],
        )

        builder.ingestion_config = ingestion_config
        assert builder.ingestion_config is not None
        assert builder.ingestion_config.asset_name == "test-asset"
        assert builder.ingestion_config.client_key == "test-client-key"
        assert len(builder.ingestion_config.flows) == 1
        assert builder.ingestion_config.flows[0].name == "test_flow"
        assert len(builder.ingestion_config.flows[0].channels) == 1
        assert builder.ingestion_config.flows[0].channels[0].name == "test_channel"

    def test_set_run_form(self):
        """Test setting run form on builder."""
        builder = SiftStreamBuilderPy("https://api.example.com", "test-api-key")

        run_form = RunFormPy(
            name="Test Run",
            description="Test run description",
            client_key="test-run-key",
            tags=[],
        )

        builder.run = run_form
        assert builder.run is not None
        assert builder.run.name == "Test Run"
        assert builder.run.description == "Test run description"
        assert builder.run.client_key == "test-run-key"
        assert builder.run.tags == []

    @pytest.mark.asyncio
    async def test_build_stream_no_ingestion_config(self):
        """Test building a stream with no ingestion config."""
        builder = SiftStreamBuilderPy("https://api.example.com", "test-api-key")
        with pytest.raises(RuntimeError, match="ingestion_config is required"):
            await builder.build()


class TestIngestWithConfigDataStreamRequest:
    """Test IngestWithConfigDataStreamRequestPy functionality."""

    def test_create_ingest_request(self):
        """Test creating an ingest request."""
        ingest_request = IngestWithConfigDataStreamRequestPy(
            ingestion_config_id="test-ingestion-config-id",
            flow="test-flow",
            timestamp=TimeValuePy.from_timestamp(int(time.time()), 0),
            channel_values=[],
            run_id="test-run-id",
            end_stream_on_validation_error=False,
            organization_id="test-organization-id",
        )
        assert ingest_request is not None
        assert ingest_request.ingestion_config_id == "test-ingestion-config-id"
        assert ingest_request.flow == "test-flow"
        assert ingest_request.timestamp
        assert ingest_request.channel_values == []
        assert ingest_request.run_id == "test-run-id"
        assert ingest_request.end_stream_on_validation_error is False
        assert ingest_request.organization_id == "test-organization-id"
