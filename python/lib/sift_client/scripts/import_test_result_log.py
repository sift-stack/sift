"""Replay a test result log file, creating real API objects from a simulation log."""

from __future__ import annotations

import argparse
import logging
import os
from pathlib import Path
from typing import TYPE_CHECKING

from sift_client import SiftClient, SiftConnectionConfig
from sift_client._internal.low_level_wrappers._test_results_log import LogTracking
from sift_client._internal.pytest_plugin.audit_log import log_event
from sift_client._internal.pytest_plugin.replay_worker import cleanup_temp_log
from sift_client.util.test_results.context_manager import log_replay_instructions

if TYPE_CHECKING:
    from sift_client._internal.low_level_wrappers.test_results import ReplayResult

logger = logging.getLogger(__name__)


def _describe_upload(log_file: str, new_report: bool) -> None:
    """Say up front whether this run continues an upload or starts one.

    The sidecar decides, so without this the same command can do several quite
    different things with no way to tell which from the output. The branches
    read the same recorded state the importer routes on, so the two cannot
    drift into disagreeing about what is about to happen.
    """
    if new_report:
        print(f"Uploading {log_file} as a new report.")
        return
    tracking = LogTracking.load(log_file)
    if tracking.complete:
        print(f"{log_file} is already fully uploaded; nothing to do.")
    elif tracking.id_map:
        print(
            f"Resuming the interrupted upload of {log_file} "
            f"({len(tracking.id_map)} already uploaded)."
        )
    else:
        print(f"Uploading {log_file}.")


def _print_result(result: ReplayResult) -> None:
    if result.report is not None:
        print(f"Report: {result.report.name} (id={result.report.id_})")
    print(f"Steps:  {len(result.steps)}")
    for step in result.steps:
        print(f"  - {step.step_path} [{step.status}]")
    print(f"Measurements: {len(result.measurements)}")
    for m in result.measurements:
        print(f"  - {m.name}: passed={m.passed}")


def main() -> None:
    """Replay a test result simulation log file against the Sift API."""
    parser = argparse.ArgumentParser(
        description="Replay a test result simulation log file against the Sift API.",
        epilog=(
            "Runs in one of two modes. With no flags it uploads the log as a new "
            "report, or resumes into the report an interrupted earlier run created, "
            "whichever the tracking sidecar calls for. --new-report forces the first."
        ),
    )
    parser.add_argument("log_file", help="Path to the .jsonl log file to replay.")
    parser.add_argument("--grpc-url", default=os.getenv("SIFT_GRPC_URI"))
    parser.add_argument("--rest-url", default=os.getenv("SIFT_REST_URI"))
    parser.add_argument("--api-key", default=os.getenv("SIFT_API_KEY"))
    parser.add_argument(
        "--new-report",
        action="store_true",
        help="Ignore a partially uploaded report and upload the log as a new one. "
        "By default an interrupted upload is resumed into the report it created.",
    )
    parser.add_argument(
        "--audit-log", default=None, help="Path to the replay worker's DEBUG audit log."
    )
    args = parser.parse_args()

    if args.audit_log:
        from sift_client._internal.pytest_plugin.audit_log import attach_file_handler

        attach_file_handler(Path(args.audit_log))

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

    _describe_upload(args.log_file, args.new_report)
    try:
        result = client.test_results.import_log_file(args.log_file, new_report=args.new_report)
        # An audit log means the run is being traced, so the buffer is retained
        # alongside the trail rather than reclaimed.
        if not args.audit_log:
            cleanup_temp_log(args.log_file)
    except Exception as e:
        log_event(logger, logging.ERROR, "replay.failed", error=repr(e))
        log_replay_instructions(args.log_file)
        raise

    if result:
        _print_result(result)


if __name__ == "__main__":
    main()
