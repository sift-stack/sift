"""
This module contains tools to download telemetry from the Sift data API. The
core component of this module is the `sift_py.data.service.DataService` and the
`sift_py.data.query` module. The former is what's used to execute a data query,
while the latter is what's used to actually construct the query. A typical query could look
something like this:

```python
query = DataQuery(
    asset_name="NostromoLV426",
    start_time="2024-07-04T18:09:08.555-07:00",
    end_time="2024-07-04T18:09:11.556-07:00",
    sample_ms=16,
    channels=[
        ChannelQuery(
            channel_name="voltage",
            run_name="[NostromoLV426].1720141748.047512"
        ),
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
```

This query, once passed to the `sift_py.data.service.DataService.execute` method, will
fetch data between `start_time` and `end_time` at the sampling rate given by `sample_ms`.

> ⚠️ **Warning**: Note on Performance
>
> Currently the results of a query are all buffered in memory, so it it best to be mindful
> about your memory limitations and overall performance requirements when requesting data
> within a large time range and a slow sampling rate. Full-fidelity data is returned
> when the `sample_ms` is set to `0`.

The data API allows you to download telemetry for both channels as well as calculated
channels. The following examples demonstrate how to download data for both channels and
calculated channels, respectively.

* [Regular Channels](#regular-channels)
* [Calculated Channels](#calculated-channels)

## Regular Channels

```python
import asyncio
import functools
import pandas as pd
from sift_py.data.query import ChannelQuery, DataQuery
from sift_py.grpc.transport import SiftChannelConfig, use_sift_async_channel
from sift_py.data.service import DataService


async def channel_demo():
    channel_config: SiftChannelConfig = {
        "apikey": "my-key"
        "uri": "sift-uri"
    }

    async with use_sift_async_channel(channel_config) as channel:
        data_service = DataService(channel)

        query = DataQuery(
            asset_name="NostromoLV426",
            start_time="2024-07-04T18:09:08.555-07:00",
            end_time="2024-07-04T18:09:11.556-07:00",
            channels=[
                ChannelQuery(
                    channel_name="voltage",
                    run_name="[NostromoLV426].1720141748.047512"
                ),
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

        data_frames = [
            pd.DataFrame(data.columns())
            for data in result.channels("voltage", "mainmotors.velocity", "gpio.12v")
        ]

        merged_frame = functools.reduce(
            lambda x, y: pd.merge_asof(x, y, on="time"), data_frames
        )

        merged_frame.to_csv("my_csv.csv")

if __name__ == "__main__":
    asyncio.run(example())
```

## Calculated Channels

```python
import asyncio
import functools
import pandas as pd
from sift_py.data.query import ChannelQuery, DataQuery
from sift_py.grpc.transport import SiftChannelConfig, use_sift_async_channel
from sift_py.data.service import DataService


async def channel_demo():
    channel_config: SiftChannelConfig = {
        "apikey": "my-key"
        "uri": "sift-uri"
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

        merged_frame.to_csv("my_csv.csv")

if __name__ == "__main__":
    asyncio.run(example())
```
"""
