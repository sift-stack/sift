import os
import asyncio
from datetime import datetime
from sift_client.client import SiftClient


async def main():
    grpc_url = os.getenv("BASE_URI", "localhost:50051")
    api_key = os.getenv("SIFT_API_KEY", "")
    rest_url = os.getenv("REST_URI", "localhost:8080")
    api_key = os.getenv("SIFT_DEV_API_KEY", "")
    rest_url = "https://api.development.siftstack.com"
    grpc_url = "grpc-api.development.siftstack.com"
    organization_id = "org-1234567890"
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
    # TODO: Test limit
    # TODO: Should we have timeouts?
    channels = []
    for run in runs:
        channels = asset.channels(run_id=run.id, limit=10)
        if len(channels) > 0:
            print(
                f"Found {len(channels)} channel(s): {[channel.identifier for channel in channels]} for asset {asset.name} on run {run.name}"
            )
            break

    # Pick a channel
    channel = None
    for c in channels:
        if c.name == "voltage":
            channel = c
            break
    # TODO: Show we can query by name
    print(f"Using channel: {channel.name} (ID: {channel.id})")

    # TODO: Search for run w/ data

    # Get the channel data during this run
    # channel_data = channel.data(start_time=datetime(2025, 3, 2, 21, 0, 0), limit=10)
    channel_data = channel.data(run_id="1d5f5c93-eaaa-48f2-94ff-7ec4337faec7", limit=10)
    print(f"Channel data: {channel_data}")


if __name__ == "__main__":
    asyncio.run(main())
