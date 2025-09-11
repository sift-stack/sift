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

    rules = []
    failed_runs = []
    for run in runs:
        print("run.name: ", run.name)
        print(" client_key: ", run.client_key)
        try:
            report = client.rules.evaluate(
                run_id=run.id_,
                all_applicable_rules=True,
            )
        except Exception as e:
            failed_runs.append(run.id_)
            print(f"Failed to evaluate rules for run {run.id_}: {e}")

    print("Number of successful runs: ", len(runs) - len(failed_runs))
    print("Number of failed runs: ", len(failed_runs))



if __name__ == "__main__":
    asyncio.run(main())
