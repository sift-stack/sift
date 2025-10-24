from sift_stream_bindings import (
    ChannelConfigPy,
    ChannelDataTypePy,
    ChannelEnumTypePy,
    FlowConfigPy,
    IngestionConfigFormPy,
    RunFormPy,
)


class TestFlowConfig:
    """Test FlowConfigPy functionality."""

    def test_create_empty_flow_config(self):
        """Test creating a flow config with no channels."""
        flow_config = FlowConfigPy(
            name="test_flow",
            channels=[],
        )
        assert flow_config.name == flow_config.name
        assert flow_config.channels == flow_config.channels

    def test_create_flow_config_with_multiple_channels(self):
        """Test creating a flow config with multiple channels like in experiment.py."""
        channels = []

        # Add boolean channel
        channels.append(
            ChannelConfigPy(
                name="bool_value_0",
                data_type=ChannelDataTypePy.Bool,
                unit="",
                description="Boolean value 0",
                enum_types=[],
                bit_field_elements=[],
            )
        )

        # Add string channel
        channels.append(
            ChannelConfigPy(
                name="string_value_0",
                data_type=ChannelDataTypePy.String,
                unit="",
                description="String value 0",
                enum_types=[],
                bit_field_elements=[],
            )
        )

        # Add numeric channels
        numeric_types = [
            ChannelDataTypePy.Float,
            ChannelDataTypePy.Double,
            ChannelDataTypePy.Int32,
            ChannelDataTypePy.Uint32,
            ChannelDataTypePy.Int64,
            ChannelDataTypePy.Uint64,
        ]

        for i, data_type in enumerate(numeric_types):
            channels.append(
                ChannelConfigPy(
                    name=f"numeric_value_{i}",
                    data_type=data_type,
                    unit="units",
                    description=f"Numeric value {i}",
                    enum_types=[],
                    bit_field_elements=[],
                )
            )

        # Add enum channel
        channels.append(
            ChannelConfigPy(
                name="enum_value_0",
                data_type=ChannelDataTypePy.Enum,
                unit="",
                description="Enum value 0",
                enum_types=[
                    ChannelEnumTypePy("STATE_1", 0),
                    ChannelEnumTypePy("STATE_2", 1),
                    ChannelEnumTypePy("STATE_3", 2),
                ],
                bit_field_elements=[],
            )
        )

        flow_config = FlowConfigPy(
            name="test_flow_multi_channel",
            channels=channels,
        )
        assert flow_config.name == flow_config.name
        assert flow_config.channels[0].name == "bool_value_0"
        assert flow_config.channels[1].name == "string_value_0"
        assert flow_config.channels[2].name == "numeric_value_0"
        assert flow_config.channels[3].name == "numeric_value_1"
        assert flow_config.channels[4].name == "numeric_value_2"
        assert flow_config.channels[5].name == "numeric_value_3"
        assert flow_config.channels[6].name == "numeric_value_4"
        assert flow_config.channels[7].name == "numeric_value_5"
        assert flow_config.channels[8].name == "enum_value_0"
        # 1 bool + 1 string + 6 numeric + 1 enum = 9 channels
        assert len(flow_config.channels) == 9


class TestIngestionConfig:
    """Test IngestionConfigFormPy functionality."""

    def test_create_empty_ingestion_config(self):
        """Test creating an ingestion config with no flows."""
        config = IngestionConfigFormPy(
            asset_name="test-asset",
            client_key="test-client-key",
            flows=[],
        )
        assert config.asset_name == config.asset_name
        assert config.client_key == config.client_key
        assert config.flows == config.flows

    def test_create_ingestion_config_with_multiple_flows(self):
        """Test creating an ingestion config with multiple flows."""
        flows = []

        # Create first flow
        channel1 = ChannelConfigPy(
            name="channel_1",
            data_type=ChannelDataTypePy.Float,
            unit="m/s",
            description="Channel 1",
            enum_types=[],
            bit_field_elements=[],
        )
        flows.append(
            FlowConfigPy(
                name="flow_1",
                channels=[channel1],
            )
        )

        # Create second flow
        channel2 = ChannelConfigPy(
            name="channel_2",
            data_type=ChannelDataTypePy.String,
            unit="",
            description="Channel 2",
            enum_types=[],
            bit_field_elements=[],
        )
        flows.append(
            FlowConfigPy(
                name="flow_2",
                channels=[channel2],
            )
        )

        ingestion_config = IngestionConfigFormPy(
            asset_name="test-asset",
            client_key="test-client-key",
            flows=flows,
        )
        assert ingestion_config.asset_name == ingestion_config.asset_name
        assert ingestion_config.client_key == ingestion_config.client_key
        assert ingestion_config.flows[0].name == "flow_1"
        assert ingestion_config.flows[1].name == "flow_2"
        assert len(ingestion_config.flows) == 2


class TestRunForm:
    """Test RunFormPy functionality."""

    def test_create_run_form(self):
        """Test creating a run form."""
        from sift_stream_bindings import MetadataPy, MetadataValuePy

        metadata = [
            MetadataPy(key="environment", value=MetadataValuePy("production")),
            MetadataPy(key="version", value=MetadataValuePy(1.5)),
            MetadataPy(key="enabled", value=MetadataValuePy(True)),
        ]

        run_form = RunFormPy(
            name="Test Run",
            client_key="test-run-key",
            description="Test run description",
            tags=["tag1", "tag2", "tag3"],
            metadata=metadata,
        )
        assert run_form.name == "Test Run"
        assert run_form.description == "Test run description"
        assert run_form.client_key == "test-run-key"
        assert run_form.tags == ["tag1", "tag2", "tag3"]
        assert run_form.metadata is not None
        assert len(run_form.metadata) == 3
        assert run_form.metadata[0].key == "environment"
        assert run_form.metadata[1].key == "version"
        assert run_form.metadata[2].key == "enabled"
