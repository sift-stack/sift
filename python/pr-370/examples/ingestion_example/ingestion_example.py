import asyncio
import random
import time
from datetime import datetime, timezone

from sift_client import SiftClient, SiftConnectionConfig
from sift_client.sift_types import (
    ChannelConfig,
    ChannelDataType,
    FlowConfig,
    IngestionConfigCreate,
    RunCreate,
)


async def main():
    connection_config = SiftConnectionConfig(
        api_key="my_api_key",
        grpc_url="sift_grpc_url",
        rest_url="sift_rest_url",
    )

    client = SiftClient(connection_config=connection_config)

    # Ingestion configs are created using SiftClient types
    ingestion_config = IngestionConfigCreate(
        asset_name="sift_rover_1",
        flows=[
            FlowConfig(
                name="onboard_sensors",
                channels=[
                    ChannelConfig(name="motor_temp", unit="C", data_type=ChannelDataType.DOUBLE),
                    ChannelConfig(
                        name="tank_pressure", unit="kPa", data_type=ChannelDataType.DOUBLE
                    ),
                ],
            )
        ],
    )

    run = RunCreate(name="sift_rover-" + str(int(time.time())))

    async with await client.async_.ingestion.create_ingestion_config_streaming_client(
        ingestion_config=ingestion_config,
        run=run,
    ) as ingest_client:
        while True:
            # Flows can be generated easily from the ingest client
            flow_config = ingest_client.get_flow_config(flow_name="onboard_sensors")
            flow = flow_config.as_flow(
                timestamp=datetime.now(timezone.utc),
                values={
                    "motor_temp": 50.0 + random.random() * 5.0,
                    "tank_pressure": 2000.0 + random.random() * 100.0,
                },
            )
            # Ingest the flow with .send()
            await ingest_client.send(flow=flow)

            await asyncio.sleep(1)


if __name__ == "__main__":
    asyncio.run(main())
