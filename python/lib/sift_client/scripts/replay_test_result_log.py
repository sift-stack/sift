"""Replay a test result log file, creating real API objects from a simulation log."""

from __future__ import annotations

import argparse
import os

from sift_client import SiftClient, SiftConnectionConfig


def main() -> None:
    """Replay a test result simulation log file against the Sift API."""
    parser = argparse.ArgumentParser(
        description="Replay a test result simulation log file against the Sift API.",
    )
    parser.add_argument("log_file", help="Path to the .jsonl log file to replay.")
    parser.add_argument("--grpc-url", default=os.getenv("SIFT_GRPC_URI", "localhost:50051"))
    parser.add_argument("--rest-url", default=os.getenv("SIFT_REST_URI", "localhost:8080"))
    parser.add_argument("--api-key", default=os.getenv("SIFT_API_KEY", ""))
    args = parser.parse_args()

    use_ssl = "localhost" not in args.grpc_url and "localhost" not in args.rest_url

    client = SiftClient(
        connection_config=SiftConnectionConfig(
            api_key=args.api_key,
            grpc_url=args.grpc_url,
            rest_url=args.rest_url,
            use_ssl=use_ssl,
        )
    )

    result = client.test_results.replay_log_file(args.log_file)

    print(f"Report: {result.report.name} (id={result.report.id_})")
    print(f"Steps:  {len(result.steps)}")
    for step in result.steps:
        print(f"  - {step.step_path} [{step.status}]")
    print(f"Measurements: {len(result.measurements)}")
    for m in result.measurements:
        print(f"  - {m.name}: passed={m.passed}")


if __name__ == "__main__":
    main()
