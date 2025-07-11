import asyncio
import math
import os
from datetime import datetime, timedelta

from sift_client.client import SiftClient
from sift_client.types.channel import (
    Channel,
    ChannelBitFieldElement,
    ChannelDataType,
    ChannelEnumType,
    Flow,
)


async def main():
    client = SiftClient(
        grpc_url=os.getenv("SIFT_GRPC_URI", "localhost:50051"),
        api_key=os.getenv("SIFT_API_KEY", ""),
        rest_url=os.getenv("SIFT_REST_URI", "localhost:8080"),
    )

    asset = client.assets.get(name="NostromoLV426")
    asset = "ian-test-asset"

    # TODO:Create asset

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
                enum_types=[
                    ChannelEnumType(name="enum1", key=1),
                    ChannelEnumType(name="enum2", key=2),
                ],
            ),
        ],
    )
    regular_flow.add_channel(
        Channel(
            name="test-bit-field-channel",
            data_type=ChannelDataType.BIT_FIELD,
            bit_field_elements=[
                ChannelBitFieldElement(name="field1", index=0, bit_count=1),
                ChannelBitFieldElement(name="field2", index=1, bit_count=1),
            ],
        )
    )

    highspeed_flow = Flow(
        name="highspeed-flow",
        channels=[
            Channel(name="highspeed-channel", data_type=ChannelDataType.DOUBLE),
        ],
    )
    run.add_flows(flows=[regular_flow, highspeed_flow], asset=asset)

    run.add_flows(
        flows=[
            Flow(
                name="new-asset-flow",
                channels=[
                    # Same channel name as the regular flow, but on a different asset.
                    Channel(name="test-channel", data_type=ChannelDataType.DOUBLE),
                ],
            )
        ],
        asset="test-asset",
    )

    fake_hs_rate = 50  # Hz
    fake_hs_period = 1 / fake_hs_rate
    for i in range(50):
        now = datetime.now()
        regular_flow.ingest(
            time=now + timedelta(seconds=i),
            channel_values={
                "test-channel": 3.0 * math.sin(2 * math.pi * 1 * i + 0),
                "test-enum-channel": "enum2",
                "test-bit-field-channel": 0b10,
            },
        )
        for j in range(fake_hs_rate):
            val = 1.0 * math.sin(2 * math.pi * fake_hs_rate * i + 0)
            timestamp = (
                now + timedelta(seconds=i) + timedelta(milliseconds=j * fake_hs_period * 1000)
            )
            channel_values = {
                "highspeed-channel": val,
            }
            # Alternative way to ingest
            client.ingestion.ingest(
                flow=highspeed_flow, time=timestamp, channel_values=channel_values
            )

    # TODO: Check assert adding a channel after data has been ingested causes an error?
    # TODO: Add rule

    run.stop()
    client.runs.delete(run=run.id)


if __name__ == "__main__":
    asyncio.run(main())
