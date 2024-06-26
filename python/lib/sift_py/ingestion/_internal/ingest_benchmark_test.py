from sift_py._internal.test_util.channel import MockChannel
from sift_py.ingestion.channel import ChannelBitFieldElement, ChannelConfig, ChannelDataType, ChannelEnumType
from sift_py.ingestion.config.telemetry import TelemetryConfig
from sift_py.ingestion.flow import FlowConfig
from sift_py.ingestion._internal.ingest import _IngestionServiceImpl


def setup_telemetry_config() -> TelemetryConfig:
    log_channel = ChannelConfig(
        name="log",
        data_type=ChannelDataType.STRING,
        description="asset logs",
    )
    velocity_channel = ChannelConfig(
        name="velocity",
        data_type=ChannelDataType.DOUBLE,
        description="speed",
        unit="Miles Per Hour",
        component="mainmotor",
    )
    voltage_channel = ChannelConfig(
        name="voltage",
        data_type=ChannelDataType.INT_32,
        description="voltage at source",
        unit="Volts",
    )
    vehicle_state_channel = ChannelConfig(
        name="vehicle_state",
        data_type=ChannelDataType.ENUM,
        description="vehicle state",
        enum_types=[
            ChannelEnumType(name="Accelerating", key=0),
            ChannelEnumType(name="Decelerating", key=1),
            ChannelEnumType(name="Stopped", key=2),
        ],
    )
    gpio_channel = ChannelConfig(
        name="gpio",
        data_type=ChannelDataType.BIT_FIELD,
        description="on/off values for pins on gpio",
        bit_field_elements=[
            ChannelBitFieldElement(name="12v", index=0, bit_count=1),
            ChannelBitFieldElement(name="charge", index=1, bit_count=2),
            ChannelBitFieldElement(name="led", index=3, bit_count=4),
            ChannelBitFieldElement(name="heater", index=7, bit_count=1),
        ],
    )

    return TelemetryConfig(
        asset_name="NostromoLV426",
        ingestion_client_key="nostromo_lv_426",
        flows=[
            FlowConfig(
                name="readings",
                channels=[
                    velocity_channel,
                    voltage_channel,
                    vehicle_state_channel,
                    gpio_channel,
                ],
            ),
            FlowConfig(
                name="voltage",
                channels=[voltage_channel],
            ),
            FlowConfig(
                name="gpio_channel",
                channels=[gpio_channel],
            ),
            FlowConfig(name="logs", channels=[log_channel]),
        ],
    )

def create_ingestion_requests():
    foo = 0
    for i in range(100_000):
        foo += i
    return foo

def test_my_stuff(benchmark):
    result = benchmark(something)
    assert result == 4999950000

# def benchmark_ingestion_request_create_ingestion_request(benchmark):

