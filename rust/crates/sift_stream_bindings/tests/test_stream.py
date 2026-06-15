import time

import pytest
from sift_stream_bindings import (
    ChannelConfigPy,
    ChannelDataTypePy,
    FileBackupBuilderPy,
    FlowConfigPy,
    FlowPy,
    IngestionConfigFormPy,
    LiveOnlyBuilderPy,
    LiveWithBackupsBuilderPy,
    RunFormPy,
    SiftStreamBuilderPy,
    StreamConfigBuilderPy,
    TimeValuePy,
    ValuePy,
    IngestWithConfigDataStreamRequestPy,
)


def _make_ingestion_config() -> IngestionConfigFormPy:
    channel = ChannelConfigPy(
        name="temperature",
        data_type=ChannelDataTypePy.Float,
        unit="celsius",
        description="Temperature sensor",
        enum_types=[],
        bit_field_elements=[],
    )
    flow = FlowConfigPy(name="sensors", channels=[channel])
    return IngestionConfigFormPy(
        asset_name="test-asset",
        client_key="test-client-key",
        flows=[flow],
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
        from sift_stream_bindings import ChannelValuePy
        timestamp = TimeValuePy.from_timestamp(int(time.time()), 0)
        values = [
            ChannelValuePy("temperature", ValuePy.Float(23.5)),
            ChannelValuePy("active", ValuePy.Bool(True)),
            ChannelValuePy("status", ValuePy.String("running")),
            ChannelValuePy("count", ValuePy.Int32(42)),
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

    def test_ingestion_config_returns_stream_config_builder(self):
        """Test that ingestion_config() advances to StreamConfigBuilderPy."""
        builder = SiftStreamBuilderPy("https://api.example.com", "test-api-key")
        config_builder = builder.ingestion_config(_make_ingestion_config())
        assert isinstance(config_builder, StreamConfigBuilderPy)

    def test_set_run_form(self):
        """Test setting run form on builder."""
        from sift_stream_bindings import MetadataPy, MetadataValuePy

        builder = SiftStreamBuilderPy("https://api.example.com", "test-api-key")

        metadata = [
            MetadataPy(key="test_key", value=MetadataValuePy("test_value")),
        ]

        run_form = RunFormPy(
            name="Test Run",
            client_key="test-run-key",
            description="Test run description",
            tags=[],
            metadata=metadata,
        )

        builder.run = run_form
        assert builder.run is not None
        assert builder.run.name == "Test Run"
        assert builder.run.description == "Test run description"
        assert builder.run.client_key == "test-run-key"
        assert builder.run.tags == []
        assert builder.run.metadata is not None
        assert len(builder.run.metadata) == 1

    @pytest.mark.asyncio
    async def test_build_stream_no_ingestion_config(self):
        """Test that build() raises when no ingestion config is set."""
        builder = SiftStreamBuilderPy("https://api.example.com", "test-api-key")
        with pytest.raises((RuntimeError, ValueError), match="ingestion_config"):
            await builder.build()


class TestStreamConfigBuilder:
    """Test StreamConfigBuilderPy and its mode selectors."""

    def _make_config_builder(self) -> StreamConfigBuilderPy:
        return SiftStreamBuilderPy(
            "https://api.example.com", "test-api-key"
        ).ingestion_config(_make_ingestion_config())

    def test_live_only_returns_builder(self):
        """Test that live_only() returns a LiveOnlyBuilderPy."""
        config_builder = self._make_config_builder()
        mode_builder = config_builder.live_only()
        assert isinstance(mode_builder, LiveOnlyBuilderPy)

    def test_live_with_backups_returns_builder(self):
        """Test that live_with_backups() returns a LiveWithBackupsBuilderPy."""
        config_builder = self._make_config_builder()
        mode_builder = config_builder.live_with_backups()
        assert isinstance(mode_builder, LiveWithBackupsBuilderPy)

    def test_file_backup_returns_builder(self):
        """Test that file_backup() returns a FileBackupBuilderPy."""
        config_builder = self._make_config_builder()
        mode_builder = config_builder.file_backup()
        assert isinstance(mode_builder, FileBackupBuilderPy)

    def test_set_run_on_config_builder(self):
        """Test setting run info on StreamConfigBuilderPy."""
        config_builder = self._make_config_builder()
        config_builder.run_id = "my-run-id"
        assert config_builder.run_id == "my-run-id"

    def test_set_asset_tags_on_config_builder(self):
        """Test setting asset tags on StreamConfigBuilderPy."""
        config_builder = self._make_config_builder()
        config_builder.asset_tags = ["tag1", "tag2"]
        assert config_builder.asset_tags == ["tag1", "tag2"]


class TestLiveOnlyBuilder:
    """Test LiveOnlyBuilderPy default fields."""

    def test_default_fields(self):
        """Test that LiveOnlyBuilderPy initializes with expected defaults."""
        mode_builder = (
            SiftStreamBuilderPy("https://api.example.com", "test-api-key")
            .ingestion_config(_make_ingestion_config())
            .live_only()
        )
        assert isinstance(mode_builder.enable_compression_for_ingestion, bool)
        assert mode_builder.enable_compression_for_ingestion is False
        assert mode_builder.ingestion_data_channel_capacity > 0
        assert mode_builder.control_channel_capacity > 0

    def test_set_compression(self):
        """Test toggling compression on LiveOnlyBuilderPy."""
        mode_builder = (
            SiftStreamBuilderPy("https://api.example.com", "test-api-key")
            .ingestion_config(_make_ingestion_config())
            .live_only()
        )
        mode_builder.enable_compression_for_ingestion = True
        assert mode_builder.enable_compression_for_ingestion is True


class TestLiveWithBackupsBuilder:
    """Test LiveWithBackupsBuilderPy default fields."""

    def test_default_fields(self):
        """Test that LiveWithBackupsBuilderPy initializes with expected defaults."""
        from sift_stream_bindings import DurationPy, RetryPolicyPy

        mode_builder = (
            SiftStreamBuilderPy("https://api.example.com", "test-api-key")
            .ingestion_config(_make_ingestion_config())
            .live_with_backups()
        )
        assert isinstance(mode_builder.checkpoint_interval, DurationPy)
        assert isinstance(mode_builder.retry_policy, RetryPolicyPy)
        assert mode_builder.ingestion_data_channel_capacity > 0
        assert mode_builder.backup_data_channel_capacity > 0
        assert mode_builder.control_channel_capacity > 0

    def test_set_checkpoint_interval(self):
        """Test setting checkpoint_interval on LiveWithBackupsBuilderPy."""
        from sift_stream_bindings import DurationPy

        mode_builder = (
            SiftStreamBuilderPy("https://api.example.com", "test-api-key")
            .ingestion_config(_make_ingestion_config())
            .live_with_backups()
        )
        mode_builder.checkpoint_interval = DurationPy(30, 0)
        assert mode_builder.checkpoint_interval.secs == 30


class TestFileBackupBuilder:
    """Test FileBackupBuilderPy default fields."""

    def test_default_fields(self):
        """Test that FileBackupBuilderPy initializes with expected defaults."""
        from sift_stream_bindings import DiskBackupPolicyPy

        mode_builder = (
            SiftStreamBuilderPy("https://api.example.com", "test-api-key")
            .ingestion_config(_make_ingestion_config())
            .file_backup()
        )
        assert isinstance(mode_builder.disk_backup_policy, DiskBackupPolicyPy)
        assert mode_builder.backup_data_channel_capacity > 0
        assert mode_builder.control_channel_capacity > 0

    def test_set_disk_backup_policy(self):
        """Test setting disk_backup_policy on FileBackupBuilderPy."""
        from sift_stream_bindings import DiskBackupPolicyPy, RollingFilePolicyPy

        mode_builder = (
            SiftStreamBuilderPy("https://api.example.com", "test-api-key")
            .ingestion_config(_make_ingestion_config())
            .file_backup()
        )
        policy = DiskBackupPolicyPy(
            backups_dir="/tmp/test-backups",
            max_backup_file_size=10 * 1024 * 1024,
            rolling_file_policy=RollingFilePolicyPy(max_file_count=5),
            retain_backups=False,
        )
        mode_builder.disk_backup_policy = policy
        assert mode_builder.disk_backup_policy is not None


class TestSiftStreamAutoRegister:
    """Test SiftStreamAutoRegisterPy structure and construction helpers."""

    def test_class_is_importable(self):
        """SiftStreamAutoRegisterPy must be exported from the bindings module."""
        from sift_stream_bindings import SiftStreamAutoRegisterPy

        assert SiftStreamAutoRegisterPy is not None

    def test_from_stream_is_static_method(self):
        """from_stream must be a callable on the class (not an instance method)."""
        from sift_stream_bindings import SiftStreamAutoRegisterPy

        assert callable(SiftStreamAutoRegisterPy.from_stream)

    def test_expected_methods_present(self):
        """SiftStreamAutoRegisterPy must expose the full streaming interface."""
        from sift_stream_bindings import SiftStreamAutoRegisterPy

        expected = [
            "from_stream",
            "send",
            "finish",
            "get_flow_descriptor",
            "attach_run",
            "detach_run",
            "run",
            "get_metrics_snapshot",
        ]
        members = dir(SiftStreamAutoRegisterPy)
        for name in expected:
            assert name in members, f"SiftStreamAutoRegisterPy is missing method '{name}'"

    @pytest.mark.asyncio
    async def test_from_stream_rejects_non_stream(self):
        """from_stream must raise TypeError when passed a non-SiftStreamPy value."""
        from sift_stream_bindings import SiftStreamAutoRegisterPy

        with pytest.raises((TypeError, AttributeError)):
            await SiftStreamAutoRegisterPy.from_stream("not-a-stream")

    @pytest.mark.asyncio
    async def test_from_stream_rejects_none(self):
        """from_stream must raise TypeError when passed None."""
        from sift_stream_bindings import SiftStreamAutoRegisterPy

        with pytest.raises((TypeError, AttributeError)):
            await SiftStreamAutoRegisterPy.from_stream(None)

    def test_staged_configs_parameter_accepted(self):
        """from_stream must accept a staged_configs keyword argument."""
        import inspect

        from sift_stream_bindings import SiftStreamAutoRegisterPy

        sig = inspect.signature(SiftStreamAutoRegisterPy.from_stream)
        assert "staged_configs" in sig.parameters, (
            "from_stream is missing the staged_configs parameter — "
            "was it added to the #[pymethods] impl?"
        )

    def test_staged_configs_builder_roundtrip(self):
        """FlowConfigPy objects built for staged_configs are correctly formed."""
        channel = ChannelConfigPy(
            name="velocity",
            data_type=ChannelDataTypePy.Double,
            unit="m/s",
            description="Forward velocity",
            enum_types=[],
            bit_field_elements=[],
        )
        flow_cfg = FlowConfigPy(name="motion", channels=[channel])

        assert flow_cfg.name == "motion"
        assert len(flow_cfg.channels) == 1
        assert flow_cfg.channels[0].name == "velocity"
        assert flow_cfg.channels[0].unit == "m/s"
        assert flow_cfg.channels[0].description == "Forward velocity"


class TestSiftStreamMetricsSnapshot:
    """Test SiftStreamMetricsSnapshotPy fields."""

    def test_grpc_status_counts_attribute_exists(self):
        """SiftStreamMetricsSnapshotPy must expose grpc_status_counts as a readable property."""
        from sift_stream_bindings import SiftStreamMetricsSnapshotPy

        # PyO3 #[pyo3(get)] fields appear as descriptors on the class.
        assert "grpc_status_counts" in dir(SiftStreamMetricsSnapshotPy), (
            "SiftStreamMetricsSnapshotPy is missing grpc_status_counts — "
            "was the field added with #[pyo3(get)]?"
        )


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
