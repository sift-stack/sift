import asyncio
import os
import time
from datetime import datetime, timezone

import numpy as np
import pandas as pd
from sift_client.client import SiftClient


async def main():
    grpc_url = os.getenv("SIFT_GRPC_URI", "localhost:50051")
    rest_url = os.getenv("SIFT_REST_URI", "localhost:8080")
    api_key = os.getenv("SIFT_API_KEY", "")
    client = SiftClient(grpc_url=grpc_url, api_key=api_key, rest_url=rest_url)

    asset = client.assets.find(name="NostromoLV426")
    asset_id = asset.id
    print(f"Using asset: {asset.name} (ID: {asset_id})")

    # List runs for this asset
    runs = asset.runs()
    print(
        f"Found {len(runs)} run(s): {[run.name for run in runs]} for asset {asset.name} (ID: {asset_id})"
    )

    # Pick one.
    run = runs[0]
    run_id = run.id
    print(f"Using run: {run.name} (ID: {run_id})")

    # List other assets for this run.
    all_assets = run.assets()
    other_assets = [asset for asset in all_assets if asset.id != asset_id]
    print(
        f"Found {len(other_assets)} other asset(s): {other_assets} for run {run.name} (ID: {run_id})"
    )

    # List channels for this asset (find a run w/ data)
    channels = []
    for run in runs:
        asset_channels = asset.channels(run_id=run.id, limit=10)
        other_channels = []
        for c in asset_channels:
            if c.name in {"voltage", "gpio", "temperature", "mainmotor.velocity"}:
                channels.append(c)
            else:
                other_channels.append(c)

        if len(channels) > 3:
            print(
                f"Found {len(channels)} channel(s): {[channel.identifier for channel in channels]} for asset {asset.name} on run {run.name}"
            )
            if len(other_channels) > 0:
                print(
                    f"Found {len(other_channels)} other channel(s): {[c.name for c in other_channels]} for asset {asset.name} on run {run.name}"
                )
            break

    # Get the channel data during a specific run
    channel = channels[0]
    channel_data = channel.data(run_id="1d5f5c93-eaaa-48f2-94ff-7ec4337faec7", limit=100)
    print(f"Channel data for {channel.name} has {len(channel_data)} points")

    # Get data for multiple channels
    print("Getting data for multiple channels:")
    perf_start = time.perf_counter()
    channel_data = client.channels.get_data(
        run_id="1d5f5c93-eaaa-48f2-94ff-7ec4337faec7", channels=channels, limit=100
    )
    first_time = time.perf_counter() - perf_start
    start_time = None
    end_time = None
    for i, (channel_name, data) in enumerate(channel_data.items()):
        print(f"{i}: {channel_name}: {len(data)} points. Avg: {np.mean(data[channel_name])}")

        # Pick a random channel and grab the start end times so we can test the cache
        if i == 1:
            start_time = data.index[0]
            end_time = data.index[-1]
            print(f"Start time: {start_time}, End time: {end_time}")

    # Test cache with varying start_time and end_time parameters
    if start_time and end_time:
        print("\n=== Testing cache with varying time ranges ===")

        # Test 1: Exact same time range (should hit cache)
        print("\nTest 1: Exact same time range no run_id (should hit cache)")
        perf_start = time.perf_counter()
        _ = client.channels.get_data(
            channels=channels,
            start_time=start_time,
            end_time=end_time,
        )
        exact_time = time.perf_counter() - perf_start

        # Test 2: Subset of time range (should hit cache if overlapping)
        print("\nTest 2: Subset of time range (should hit cache if overlapping)")
        mid_time = start_time + (end_time - start_time) / 2
        perf_start = time.perf_counter()
        _ = client.channels.get_data(
            channels=channels,
            start_time=start_time,
            end_time=mid_time,
        )
        subset_time = time.perf_counter() - perf_start

        # Test 3: Extended time range (should hit cache for overlapping portion)
        print("\nTest 3: Extended time range earlier (should hit cache for overlapping portion)")
        extended_start = start_time - (end_time - start_time) * 0.1
        perf_start = time.perf_counter()
        _ = client.channels.get_data(
            channels=channels,
            start_time=extended_start,
            end_time=end_time,
        )
        extended_time = time.perf_counter() - perf_start

        # Test 4: Different time range (should not hit cache)
        print("\nTest 4: Different time encompassed range (should hit cache)")
        different_start = extended_start + pd.Timedelta(seconds=2)
        different_end = start_time + pd.Timedelta(seconds=3)
        perf_start = time.perf_counter()
        _ = client.channels.get_data(
            channels=channels,
            start_time=different_start,
            end_time=different_end,
        )
        different_time = time.perf_counter() - perf_start

        # Test 5: No time range specified (should miss cache from original call)
        print("\nTest 5: No time range specified (should miss cache)")
        # Since None end time is treated as now, we capture now so we can repeat it.
        fake_no_end_time = datetime.now(timezone.utc)
        perf_start = time.perf_counter()
        channel_data_no_time = client.channels.get_data(
            channels=channels,
            end_time=fake_no_end_time,
            limit=100,
        )
        no_time_time = time.perf_counter() - perf_start
        for i, (channel_name, data) in enumerate(channel_data_no_time.items()):
            print(f"{i}: {channel_name}: {len(data)} points. Avg: {np.mean(data[channel_name])}")

        # Test 6: No time range specified again (should hit cache)
        # NOTE: We're not comparing the results since limit combines with cache results.
        print("\nTest 6: No time range specified again (should hit cache)")
        perf_start = time.perf_counter()
        channel_data_no_time = client.channels.get_data(
            channels=channels,
            end_time=fake_no_end_time,
            limit=100,
        )
        for i, (channel_name, data) in enumerate(channel_data_no_time.items()):
            print(
                f"{i}: {channel_name}: {len(data)} points. Avg: {np.mean(data[channel_name]) if channel_name in data else np.nan}"
            )
        no_time_time_repeat = time.perf_counter() - perf_start

        # Summary of cache performance
        print("\n=== Cache Performance Summary ===")
        print(f"Original call: {first_time:.4f} seconds")
        print(
            f"Exact time range no run_id: {exact_time:.4f} seconds ({(first_time / exact_time):.1f}x faster)"
        )
        print(
            f"Subset time range: {subset_time:.4f} seconds ({(first_time / subset_time):.1f}x faster)"
        )
        print(
            f"Extended time range earlier: {extended_time:.4f} seconds ({(first_time / extended_time):.1f}x faster)"
        )
        print(
            f"Different time range: {different_time:.4f} seconds ({(first_time / different_time):.1f}x faster)"
        )
        print(
            f"No time range: {no_time_time:.4f} seconds ({(no_time_time / first_time):.1f}x slower)"
        )
        print(
            f"No time range repeat: {no_time_time_repeat:.4f} seconds ({(no_time_time / no_time_time_repeat):.1f}x faster)"
        )
        assert exact_time < first_time
        assert subset_time < first_time
        assert extended_time < first_time
        assert different_time < first_time
        assert no_time_time > first_time
        assert no_time_time_repeat < no_time_time


if __name__ == "__main__":
    asyncio.run(main())
