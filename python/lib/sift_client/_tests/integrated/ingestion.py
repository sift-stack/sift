import asyncio
import math
import os
import random
import time
from datetime import datetime, timedelta

from sift_client._tests import setup_logger
from sift_client.client import SiftClient
from sift_client.types.channel import (
    Channel,
    ChannelBitFieldElement,
    ChannelDataType,
)
from sift_client.types.ingestion import Flow

setup_logger()


async def main():
    grpc_url = os.getenv("SIFT_GRPC_URI", "localhost:50051")
    api_key = os.getenv("SIFT_API_KEY", "")
    rest_url = os.getenv("SIFT_REST_URI", "localhost:8080")
    client = SiftClient(grpc_url=grpc_url, api_key=api_key, rest_url=rest_url)

    asset = "ian-test-asset"

    # TODO:Get user id from current user
    previously_created_runs = client.runs.list(
        name_regex="test-run-.*", created_by_user_id="1eba461b-fa36-4e98-8fe8-ff32d3e43a6e"
    )
    if previously_created_runs:
        print(f"   Deleting previously created runs: {previously_created_runs}")
        for run in previously_created_runs:
            print(f"   Deleting run: {run.name}")
            client.runs.archive(run=run)

    run = client.runs.create(
        name=f"test-run-{datetime.now().timestamp()}",
        description="A test run created via the API",
        tags=["api-created", "test"],
    )

    regular_flow = Flow(
        name="test-flow",
        channels=[
            Channel(name="test-channel", data_type=ChannelDataType.DOUBLE),
            Channel(
                name="test-enum-channel",
                data_type=ChannelDataType.ENUM,
                enum_types={"enum1": 1, "enum2": 2},
            ),
        ],
    )
    regular_flow.add_channel(
        Channel(
            name="test-bit-field-channel",
            data_type=ChannelDataType.BIT_FIELD,
            bit_field_elements=[
                ChannelBitFieldElement(name="12v", index=0, bit_count=4),
                ChannelBitFieldElement(name="charge", index=4, bit_count=2),
                ChannelBitFieldElement(name="led", index=6, bit_count=1),
                ChannelBitFieldElement(name="heater", index=7, bit_count=1),
            ],
        )
    )

    highspeed_flow = Flow(
        name="highspeed-flow",
        channels=[
            Channel(name="highspeed-channel", data_type=ChannelDataType.DOUBLE),
        ],
    )
    # This seals the flow and ingestion config
    config_id = await client.async_.ingestion.create_ingestion_config(
        asset_name=asset,
        run_id=run.id_,
        flows=[regular_flow, highspeed_flow],
    )
    print(f"config_id: {config_id}")
    try:
        regular_flow.add_channel(Channel(name="test-channel", data_type=ChannelDataType.DOUBLE))
    except ValueError as e:
        assert repr(e) == "ValueError('Cannot add a channel to a flow after creation')"

    other_asset_flows = [
        Flow(
            name="new-asset-flow",
            channels=[
                # Same channel name as the regular flow, but on a different asset.
                Channel(name="test-channel", data_type=ChannelDataType.DOUBLE),
            ],
        )
    ]
    await client.async_.ingestion.create_ingestion_config(
        asset_name="test-asset-ian2",
        run_id=run.id_,
        flows=other_asset_flows,
    )
    sleep_time = 0.05  # Time between outer loop iterations to simulate real-time latency between ingestion calls.
    simulated_duration = 50
    fake_hs_rate = 50  # Hz
    fake_hs_period = 1 / fake_hs_rate
    start = datetime.now()
    for i in range(simulated_duration):
        now = start + timedelta(seconds=i)
        regular_flow.ingest(
            timestamp=now,
            channel_values={
                "test-channel": 3.0 * math.sin(2 * math.pi * fake_hs_rate * i + 0.05),
                "test-enum-channel": i % 2 + 1,
                "test-bit-field-channel": {
                    "12v": random.randint(3, 13),
                    "charge": random.randint(1, 3),
                    "led": random.choice([0, 1]),
                    "heater": random.choice([0, 1]),
                },
            },
        )
        for j in range(fake_hs_rate):
            val = 3.0 * math.sin(2 * math.pi * fake_hs_rate * (i + j * 0.001) + 0)
            timestamp = now + timedelta(milliseconds=j * fake_hs_period * 1000)
            channel_values = {
                "highspeed-channel": val,
            }
            # Alternative way to ingest
            client.ingestion.ingest(
                flow=highspeed_flow, timestamp=timestamp, channel_values=channel_values
            )
        time.sleep(sleep_time)

    other_asset_flows[0].ingest(
        timestamp=start + timedelta(seconds=simulated_duration),
        channel_values={
            "test-channel": -6.66,
        },
    )

    # Test ingestion of a flow without all channels specified
    try:
        regular_flow.ingest(
            timestamp=start + timedelta(seconds=simulated_duration),
            channel_values={
                "test-channel": 0,
                "test-enum-channel": 2,
                # "test-bit-field-channel": bytes([0b01010101]),
            },
        )
    except ValueError as e:
        assert "Expected all channels in flow to have a data point at same time." in repr(e)

    # Test ingestion of a bad enum value (string and int)
    try:
        regular_flow.ingest(
            timestamp=start + timedelta(seconds=simulated_duration),
            channel_values={
                "test-channel": 0,
                "test-enum-channel": -3,
                "test-bit-field-channel": bytes([0b01010101]),
            },
        )
    except ValueError as e:
        assert "Could not find enum value: -3 in enum options: {'enum1': 1, 'enum2': 2}" in repr(e)
    try:
        regular_flow.ingest(
            timestamp=start + timedelta(seconds=simulated_duration),
            channel_values={
                "test-channel": 0,
                "test-enum-channel": "nonexistent-enum",
                "test-bit-field-channel": bytes([0b01010101]),
            },
        )
    except ValueError as e:
        assert (
            "Could not find enum value: nonexistent-enum in enum options: {'enum1': 1, 'enum2': 2}"
            in repr(e)
        )

    client.async_.ingestion.wait_for_ingestion_to_complete(timeout=2)
    end = datetime.now()
    # Test ingesting more data after letting a thread finish. Also exercise ingesting bitfield values as bytes.
    time.sleep(1)
    print("Restarting ingestion")
    regular_flow.ingest(
        timestamp=start + timedelta(seconds=simulated_duration + 1),
        channel_values={
            "test-channel": 7.77,
            "test-enum-channel": 1,
            "test-bit-field-channel": bytes([0b11111111]),
        },
    )
    # Wait less time than threads nominal no_data_timeout so we can exercise forced cleanup.
    client.async_.ingestion.wait_for_ingestion_to_complete(timeout=0.01)
    client.runs.archive(run=run.id_)

    num_datapoints = fake_hs_rate * len(
        highspeed_flow.channels
    ) * simulated_duration + simulated_duration * len(regular_flow.channels)
    print(f"Ingestion time: {end - start} seconds")
    print(f"Ingested {num_datapoints} datapoints")
    total_time = (end - start).total_seconds()
    print(f"Ingestion rate: {num_datapoints / total_time:.2f} datapoints/second")


if __name__ == "__main__":
    asyncio.run(main())
