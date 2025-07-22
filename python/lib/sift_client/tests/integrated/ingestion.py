import asyncio
import math
import os
from datetime import datetime, timedelta

from sift_client.client import SiftClient
from sift_client.types.channel import (
    Channel,
    ChannelDataType,
    Flow,
)


async def main():
    client = SiftClient(
        grpc_url=os.getenv("SIFT_GRPC_URI", "localhost:50051"),
        api_key=os.getenv("SIFT_API_KEY", ""),
        rest_url=os.getenv("SIFT_REST_URI", "localhost:8080"),
    )

    # asset = client.assets.get(name="NostromoLV426")
    asset = "ian-test-asset"

    # TODO:Get user id from current user
    previously_created_runs = client.runs.list(
        name_regex="test-run-.*", created_by_user_id="1eba461b-fa36-4e98-8fe8-ff32d3e43a6e"
    )
    if previously_created_runs:
        print(f"   Deleting previously created runs: {previously_created_runs}")
        for run in previously_created_runs:
            print(f"   Deleting run: {run.name}")
            client.runs.delete(run=run)

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
    # regular_flow.add_channel(
    #     Channel(
    #         name="test-bit-field-channel",
    #         data_type=ChannelDataType.BIT_FIELD,
    #         bit_field_elements=[
    #             ChannelBitFieldElement(name="field1", index=0, bit_count=1),
    #             ChannelBitFieldElement(name="field2", index=1, bit_count=1),
    #         ],
    #     )
    # )

    highspeed_flow = Flow(
        name="highspeed-flow",
        channels=[
            Channel(name="highspeed-channel", data_type=ChannelDataType.DOUBLE),
        ],
    )
    # ingestion_config_id = await client.async_.ingestion.create_ingestion_config(
    #     asset_name=asset,
    #     flows=[regular_flow, highspeed_flow],
    # )
    await run.add_flows(flows=[regular_flow, highspeed_flow], asset=asset)

    await run.add_flows(
        flows=[
            Flow(
                name="new-asset-flow",
                channels=[
                    # Same channel name as the regular flow, but on a different asset.
                    Channel(name="test-channel", data_type=ChannelDataType.DOUBLE),
                ],
            )
        ],
        asset="test-asset-ian2",
    )
    simulated_duration = 50
    fake_hs_rate = 50  # Hz
    fake_hs_period = 1 / fake_hs_rate
    start = datetime.now()
    for i in range(simulated_duration):
        now = start + timedelta(seconds=i)
        regular_flow.ingest(
            timestamp=now,
            channel_values={
                "test-channel": 3.0 * math.sin(2 * math.pi * fake_hs_rate * i + 0),
                "test-enum-channel": i % 2 + 1,
                # "test-bit-field-channel": 0b10,
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

    # TODO: Test ingestion of a flow with a channel that has no value
    # TODO: Test ingestion of a bad enum value (string and int)
    # TODO: Check assert adding a channel after data has been ingested causes an error?
    # TODO: Add rule
    run.wait_for_ingestion_to_complete(timeout=61)
    client.runs.delete(run=run.id)

    num_datapoints = fake_hs_rate * len(
        highspeed_flow.channels
    ) * simulated_duration + simulated_duration * len(regular_flow.channels)
    print(f"Ingestion time: {datetime.now() - start} seconds")
    print(f"Ingested {num_datapoints} datapoints")
    print(
        f"Ingestion rate: {num_datapoints / (datetime.now() - start).total_seconds():.2f} datapoints/second"
    )


if __name__ == "__main__":
    asyncio.run(main())
