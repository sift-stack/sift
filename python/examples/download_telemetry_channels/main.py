import asyncio
import functools
import os

import pandas as pd
from dotenv import load_dotenv
from sift_py.data.query import ChannelQuery, DataQuery
from sift_py.data.service import DataService
from sift_py.grpc.transport import SiftChannelConfig, use_sift_async_channel


async def channel_demo():
    load_dotenv()
    channel_config: SiftChannelConfig = {
        "apikey": os.getenv("SIFT_API_KEY", ""),
        "uri": os.getenv("BASE_URI", ""),
    }

    async with use_sift_async_channel(channel_config) as channel:
        data_service = DataService(channel)

        query = DataQuery(
            asset_name="NostromoLV426",
            start_time="2024-07-04T18:09:08.555-07:00",
            end_time="2024-07-04T18:09:11.556-07:00",
            channels=[
                ChannelQuery(channel_name="voltage", run_name="[NostromoLV426].1720141748.047512"),
                ChannelQuery(
                    channel_name="velocity",
                    component="mainmotors",
                    run_name="[NostromoLV426].1720141748.047512",
                ),
                ChannelQuery(
                    channel_name="gpio",
                    run_name="[NostromoLV426].1720141748.047512",
                ),
            ],
        )

        result = await data_service.execute(query)

        # `result.all_channels` for all channels

        data_frames = [
            pd.DataFrame(data.columns())
            for data in result.channels("voltage", "mainmotors.velocity", "gpio.12v")
        ]

        merged_frame = functools.reduce(lambda x, y: pd.merge_asof(x, y, on="time"), data_frames)

        return merged_frame


if __name__ == "__main__":
    data = asyncio.run(channel_demo())
    print(data)
