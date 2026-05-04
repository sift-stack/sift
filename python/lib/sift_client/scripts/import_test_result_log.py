"""Replay a test result log file, creating real API objects from a simulation log."""

from __future__ import annotations

import argparse
import logging
import os
import select
import sys
import tempfile
from typing import TYPE_CHECKING

from sift_client import SiftClient, SiftConnectionConfig
from sift_client.util.test_results.context_manager import log_replay_instructions

if TYPE_CHECKING:
    from sift_client._internal.low_level_wrappers.test_results import ReplayResult

logger = logging.getLogger(__name__)


def _print_result(result: ReplayResult) -> None:
    print(f"Report: {result.report.name} (id={result.report.id_})")
    print(f"Steps:  {len(result.steps)}")
    for step in result.steps:
        print(f"  - {step.step_path} [{step.status}]")
    print(f"Measurements: {len(result.measurements)}")
    for m in result.measurements:
        print(f"  - {m.name}: passed={m.passed}")


def _incremental_import_loop(client: SiftClient, log_file: str) -> ReplayResult | None:
    """Replay incrementally in a loop until stdin is closed (EOF)."""
    result = None
    while True:
        received_signal, _, _ = select.select([sys.stdin], [], [], 1.0)
        result = client.test_results.import_log_file(log_file, incremental=True)
        if received_signal:
            break
    logger.info(f"Replay completed: {result}")
    fp = os.path.abspath(log_file)
    if fp.startswith(tempfile.gettempdir()):
        os.remove(fp)
    return result


def main() -> None:
    """Replay a test result simulation log file against the Sift API."""
    parser = argparse.ArgumentParser(
        description="Replay a test result simulation log file against the Sift API.",
    )
    parser.add_argument("log_file", help="Path to the .jsonl log file to replay.")
    parser.add_argument("--grpc-url", default=os.getenv("SIFT_GRPC_URI"))
    parser.add_argument("--rest-url", default=os.getenv("SIFT_REST_URI"))
    parser.add_argument("--api-key", default=os.getenv("SIFT_API_KEY"))
    parser.add_argument(
        "--incremental", action="store_true", help="Import the log file incrementally."
    )
    args = parser.parse_args()

    if not args.grpc_url or not args.rest_url or not args.api_key:
        raise ValueError("SIFT_GRPC_URI, SIFT_REST_URI, and SIFT_API_KEY must be set")

    use_ssl = "localhost" not in args.grpc_url and "localhost" not in args.rest_url

    client = SiftClient(
        connection_config=SiftConnectionConfig(
            api_key=args.api_key,
            grpc_url=args.grpc_url,
            rest_url=args.rest_url,
            use_ssl=use_ssl,
        )
    )

    try:
        if args.incremental:
            result = _incremental_import_loop(client, args.log_file)
        else:
            result = client.test_results.import_log_file(args.log_file)
            fp = os.path.abspath(args.log_file)
            if fp.startswith(tempfile.gettempdir()):
                os.remove(fp)
    except Exception as e:
        logger.error(e)
        log_replay_instructions(args.log_file)
        raise

    if result:
        _print_result(result)


if __name__ == "__main__":
    main()
