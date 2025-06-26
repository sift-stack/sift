#!/usr/bin/env python3
"""
This test demonstrates the usage of the Runs API.

It creates a new run, updates it, and associates assets with it.
It also lists runs, filters them, and deletes the run.

It uses the SiftClient to interact with the API.
"""

import asyncio
import os
from datetime import datetime, timedelta

from sift_client import SiftClient


async def main():
    """
    Main function demonstrating the Runs API usage.
    """
    # Initialize the client
    # You can set these environment variables or pass them directly
    grpc_url = os.getenv("SIFT_GRPC_URI", "localhost:50051")
    rest_url = os.getenv("SIFT_REST_URI", "localhost:8080")
    api_key = os.getenv("SIFT_API_KEY", "")
    client = SiftClient(
        api_key=api_key,
        grpc_url=grpc_url,
        rest_url=rest_url,
    )

    # Use a known asset to fetch a run.
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

    # Example 1: List all runs
    print("\n1. Listing all runs...")
    runs = client.runs.list(limit=5)
    print(f"   Found {len(runs)} runs:")
    for run in runs:
        print(f"   - {run.name} (ID: {run.id}), Organization ID: {run.organization_id}")

    # Example 2: Test different filter options
    print("\n2. Testing different filter options...")

    # Get a sample run for testing filters
    sample_runs = client.runs.list(limit=3)
    if not sample_runs:
        print("   No runs available for filter testing")
        return

    sample_run = sample_runs[0]

    # 2a: Filter by exact name
    print("\n   2a. Filter by exact name...")
    run_name = sample_run.name
    runs = client.runs.list(name=run_name, limit=5)
    print(f"   Found {len(runs)} runs with exact name '{run_name}':")
    for run in runs:
        print(f"   - {run.name} (ID: {run.id})")

    # 2b: Filter by name containing text
    print("\n   2b. Filter by name containing text...")
    runs = client.runs.list(name_contains="test", limit=5)
    print(f"   Found {len(runs)} runs with 'test' in name:")
    for run in runs:
        print(f"   - {run.name}")

    # 2c: Filter by name using regex
    print("\n   2c. Filter by name using regex...")
    runs = client.runs.list(name_regex=".*test.*", limit=5)
    print(f"   Found {len(runs)} runs with 'test' in name (regex):")
    for run in runs:
        print(f"   - {run.name}")

    # 2d: Filter by exact description
    print("\n   2d. Filter by exact description...")
    if sample_run.description:
        runs = client.runs.list(description=sample_run.description, limit=5)
        print(f"   Found {len(runs)} runs with exact description '{sample_run.description}':")
        for run in runs:
            print(f"   - {run.name}: {run.description}")
    else:
        print("   No description available for testing")

    # 2e: Filter by description containing text
    print("\n   2e. Filter by description containing text...")
    runs = client.runs.list(description_contains="test", limit=5)
    print(f"   Found {len(runs)} runs with 'test' in description:")
    for run in runs:
        print(f"   - {run.name}: {run.description}")

    # 2f: Filter by duration seconds
    print("\n   2f. Filter by duration seconds...")
    # Calculate duration for sample run if it has start and stop times
    if sample_run.start_time and sample_run.stop_time:
        duration_seconds = int((sample_run.stop_time - sample_run.start_time).total_seconds())
        runs = client.runs.list(duration_seconds=duration_seconds, limit=5)
        print(f"   Found {len(runs)} runs with duration {duration_seconds} seconds:")
        for run in runs:
            if run.start_time and run.stop_time:
                run_duration = int((run.stop_time - run.start_time).total_seconds())
                print(f"   - {run.name} (duration: {run_duration}s)")
    else:
        print("   No start/stop times available for duration testing")

    # 2g: Filter by client key
    print("\n   2g. Filter by client key...")
    if sample_run.client_key:
        runs = client.runs.list(client_key=sample_run.client_key, limit=5)
        print(f"   Found {len(runs)} runs with client key '{sample_run.client_key}':")
        for run in runs:
            print(f"   - {run.name} (client_key: {run.client_key})")
    else:
        print("   No client key available for testing")

    # 2h: Filter by asset ID
    print("\n   2h. Filter by asset ID...")
    if sample_run.asset_ids:
        asset_id = sample_run.asset_ids[0]
        runs = client.runs.list(asset_id=asset_id, limit=5)
        print(f"   Found {len(runs)} runs associated with asset {asset_id}:")
        for run in runs:
            print(f"   - {run.name} (asset_ids: {list(run.asset_ids)})")
    else:
        print("   No asset IDs available for testing")

    # 2i: Filter by asset name
    print("\n   2i. Filter by asset name...")
    runs = client.runs.list(asset_name="NostromoLV426", limit=5)
    print(f"   Found {len(runs)} runs associated with asset 'NostromoLV426':")
    for run in runs:
        print(f"   - {run.name}")

    # 2j: Filter by created by user ID
    print("\n   2j. Filter by created by user ID...")
    created_by_user_id = sample_run.created_by_user_id
    runs = client.runs.list(created_by_user_id=created_by_user_id, limit=5)
    print(f"   Found {len(runs)} runs created by user {created_by_user_id}:")
    for run in runs:
        print(f"   - {run.name} (created by: {run.created_by_user_id})")

    # 2l: Test ordering options
    print("\n   2l. Testing ordering options...")

    # Order by name ascending
    runs = client.runs.list(order_by="name", limit=3)
    print("   First 3 runs ordered by name (ascending):")
    for run in runs:
        print(f"   - {run.name}")

    # Order by name descending
    runs = client.runs.list(order_by="name desc", limit=3)
    print("   First 3 runs ordered by name (descending):")
    for run in runs:
        print(f"   - {run.name}")

    # Order by creation date (newest first - default)
    runs = client.runs.list(order_by="created_date desc", limit=3)
    print("   First 3 runs ordered by creation date (newest first):")
    for run in runs:
        print(f"   - {run.name} (created: {run.created_date})")

    # Order by creation date (oldest first)
    runs = client.runs.list(order_by="created_date", limit=3)
    print("   First 3 runs ordered by creation date (oldest first):")
    for run in runs:
        print(f"   - {run.name} (created: {run.created_date})")

    # Example 3: Find a single run by name
    print("\n3. Finding a single run by name...")
    run_name = "test-run"  # Replace with an actual run name
    run = client.runs.find(name=run_name)
    if run:
        print(f"   Found run: {run.name}")
        print(f"   Description: {run.description}")
    else:
        print(f"   No run found with name '{run_name}'")

    # Example 4: Create a new run
    print("\n4. Creating a new run...")
    # Create metadata for the run
    metadata = {
        "environment": "production",
        "test_type": "integration",
    }

    # Create a run with start and stop times
    start_time = datetime.now()
    stop_time = start_time + timedelta(minutes=2)

    previously_created_runs = client.runs.list(name_regex="Example Test Run.*")
    if previously_created_runs:
        print(f"   Deleting previously created runs: {previously_created_runs}")
        for run in previously_created_runs:
            print(f"   Deleting run: {run.name}")
            client.runs.delete(run=run)

    new_run = client.runs.create(
        name=f"Example Test Run {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}",
        description="A test run created via the API",
        tags=["api-created", "test"],
        start_time=start_time,
        stop_time=stop_time,
        # Use a unique client key for each run
        client_key=f"example-run-key-{datetime.now().timestamp()}",
        metadata=metadata,
    )
    print(f"   Created run: {new_run.name} (ID: {new_run.id})")
    print(f"   Client key: {new_run.client_key}")
    print(f"   Tags: {new_run.tags}")

    # Example 5: Update a run
    print("\n5. Updating a run...")

    run_to_update = new_run
    print(f"   Updating run: {run_to_update.name}")

    # Update the run
    new_description = "Updated description via API"
    new_metadata = {
        "test_type": "ci",
    }
    new_tags = ["updated", "api-modified"]
    updated_run = client.runs.update(
        run=run_to_update,
        update={
            "description": new_description,
            "tags": new_tags,
            "metadata": new_metadata,
        },
    )
    print(f"   Updated run: {updated_run.name}")
    print(f"   New description: {updated_run.description}")
    print(f"   New tags: {updated_run.tags}")
    print(f"   New metadata: {updated_run.metadata}")
    assert updated_run.description == new_description
    assert sorted(updated_run.tags) == sorted(new_tags)
    assert updated_run.metadata == new_metadata

    # Example 6: Associate assets with a run
    print("\n6. Associating assets with a run...")
    ongoing_runs = client.runs.list(
        name_regex="Example Test Run.*", include_archived=True, is_stopped=False
    )
    if ongoing_runs:
        print("   Ensuring previously created runs are stopped:")
        for run in ongoing_runs:
            if run.stop_time is None:
                print(f"   Stopping run: {run.name}")
                client.runs.stop(run=run)

    # Get a run to associate assets with
    asset_names = ["asset1", "asset2"]  # Replace with actual asset names
    print(f"   Associating assets {asset_names} with run: {new_run.name}")

    client.runs.create_automatic_association_for_assets(run=new_run, asset_names=asset_names)
    print(f"   Successfully associated assets with run: {new_run.name}")

    # Example 7: Delete a run
    print("\n7. Deleting a run")
    run_to_delete = new_run
    print(f"   Deleting run: {run_to_delete.name}")
    client.runs.delete(run=run_to_delete)
    print(f"   Successfully deleted run: {run_to_delete.name}")


if __name__ == "__main__":
    asyncio.run(main())
