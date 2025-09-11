#!/usr/bin/env python3
"""This test demonstrates the usage of the Runs API.

It creates a new run, updates it, and associates assets with it.
It also lists runs, filters them, and deletes the run.

It uses the SiftClient to interact with the API.
"""

import asyncio
import os
from datetime import datetime

from zoneinfo import ZoneInfo

from sift_client import SiftClient


async def main():
    """Main function demonstrating the Runs API usage."""
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

    runs = client.runs.list_(
        created_date_start=datetime(2025, 9, 10, 9, 50, tzinfo=ZoneInfo("America/Los_Angeles")),
        created_date_end=datetime(2025, 9, 10, 12, 50, tzinfo=ZoneInfo("America/Los_Angeles")),
        limit=100,
    )

    asset_ids = []
    asset_tags_names = []
    rules = []
    reports = []
    for run in runs:
        print("run.name: ", run.name)
        print(" client_key: ", run.client_key)
        if run.client_key:
            # rules = client.rules.list_(
            #     client_key=run.client_key,
            #     limit=100,
            # )
            raise Exception("client_key is not None! Let's add these rules")
        run_assets = run.assets
        print("  assets: ", [asset.name for asset in run_assets])
        asset_ids.extend([asset.id_ for asset in run_assets])
        asset_tags_names.extend([tag for asset in run_assets for tag in asset.tags])
        per_run_reports = client.reports.list_(
            run_id=run.id_,
        )
        print("  reports: ", [report.name for report in per_run_reports])
        reports.extend(per_run_reports)

    asset_ids = list(set(asset_ids))
    asset_tags_names = list(set(asset_tags_names))
    asset_tags = client.tags.list_(
        names=asset_tags_names,
    )
    print("  asset_tags: ", [(tag.name, tag.id_) for tag in asset_tags])
    print("Number of runs: ", len(runs))
    print("Number of assets: ", len(asset_ids))

    rules = client.rules.list_(
        asset_ids=asset_ids,
        # asset_tags_ids=[tag.id_ for tag in asset_tags],
    )
    print("reports: ", [report.name for report in reports])
    if len(rules) < 10:
        print("rules: ", [rule.name for rule in rules])
    else:
        print("number of rules: ", len(rules))



if __name__ == "__main__":
    asyncio.run(main())
