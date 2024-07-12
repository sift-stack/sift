import asyncio
import os

import pandas as pd
from dotenv import load_dotenv
from sift_py.data.query import CalculatedChannelQuery, DataQuery
from sift_py.data.service import DataService
from sift_py.grpc.transport import SiftChannelConfig, use_sift_async_channel


async def calculated_channel_demo():
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
                CalculatedChannelQuery(
                    channel_key="calc-voltage",
                    expression="$1 + 10",
                    expression_channel_references=[
                        {
                            "reference": "$1",
                            "channel_name": "voltage",
                        },
                    ],
                    run_name="[NostromoLV426].1720141748.047512",
                ),
                CalculatedChannelQuery(
                    channel_key="calc-velocity",
                    expression="$1 * 2",
                    expression_channel_references=[
                        {
                            "reference": "$1",
                            "channel_name": "velocity",
                            "component": "mainmotors",
                        },
                    ],
                    run_name="[NostromoLV426].1720141748.047512",
                ),
            ],
        )

        result = await data_service.execute(query)
        calc_voltage, calc_velocity = result.channels("calc-voltage", "calc-velocity")

        calc_voltage_df = pd.DataFrame(calc_voltage.columns())
        calc_velocity_df = pd.DataFrame(calc_velocity.columns())

        merged_frame = pd.merge_asof(calc_voltage_df, calc_velocity_df, on="time")

        return merged_frame


if __name__ == "__main__":
    data = asyncio.run(calculated_channel_demo())
    print(data)
