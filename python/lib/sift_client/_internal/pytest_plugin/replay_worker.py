"""Background replay worker: follow a log that is still being written.

Spawned by the report context (see ``util.test_results.context_manager``) for
the duration of a pytest session, never run by hand. It ticks the incremental
importer against a growing log and exits when its stdin closes, which is the
context manager's signal that the session is over.

The user-facing ``import-test-result-log`` command is the other half: it uploads
or resumes a log that is final. Keeping the follow mode here rather than behind
a flag on that command means there is no way to invoke it by accident, and the
public command has one fewer mode to explain.
"""

from __future__ import annotations

import argparse
import logging
import os
import select
import shutil
import sys
import tempfile
from pathlib import Path
from typing import TYPE_CHECKING

from sift_client import SiftClient, SiftConnectionConfig
from sift_client._internal.low_level_wrappers._test_results_log import LogTracking
from sift_client._internal.pytest_plugin.audit_log import log_event
from sift_client.util.test_results.context_manager import log_replay_instructions

if TYPE_CHECKING:
    from sift_client._internal.low_level_wrappers.test_results import ReplayResult

logger = logging.getLogger(__name__)


def cleanup_temp_log(log_file: str) -> None:
    """Remove temp artifacts after a successful upload when audit logging is off.

    Called only when audit logging is off: without an audit trail there's no
    reason to retain the buffer, so default temp artifacts are reclaimed
    immediately. An explicit ``--sift-output-dir`` (not under the temp dir) is
    the user's to keep and is never touched.

    Session-dir layout (``<tmpdir>/sift_test_results/<random>/``): the whole
    directory is removed, cleaning up the JSONL, tracking sidecar, lock, and
    any audit files in one shot.

    Legacy flat-temp layout (file directly in tmpdir): only the JSONL and its
    tracking sidecars are removed individually.

    Shared with the public replay command, which reclaims the same artifacts
    after a one-shot upload.
    """
    fp = Path(log_file).absolute()
    if not str(fp).startswith(tempfile.gettempdir()):
        return
    session_dir = fp.parent
    if session_dir.parent == Path(tempfile.gettempdir()) / "sift_test_results":
        shutil.rmtree(session_dir, ignore_errors=True)
        log_event(logger, logging.DEBUG, "replay.cleanup", log=str(fp), dir=str(session_dir))
        return
    fp.unlink(missing_ok=True)
    LogTracking.sidecar_path(fp).unlink(missing_ok=True)
    LogTracking.backup_path(fp).unlink(missing_ok=True)
    log_event(logger, logging.DEBUG, "replay.cleanup", log=str(fp))


def _incremental_import_loop(
    client: SiftClient, log_file: str, *, keep_log: bool
) -> ReplayResult | None:
    """Replay incrementally in a loop until stdin is closed (EOF).

    Per-entity upload detail and sidecar advances are logged inside the
    incremental importer (``replay.upload`` / ``replay.error``); idle ticks
    that upload nothing are silent on purpose.

    When ``keep_log`` is False (audit logging off) the temp log is deleted on a
    clean finish; with audit logging on it's retained alongside the audit trail.
    """
    result = None
    while True:
        received_signal, _, _ = select.select([sys.stdin], [], [], 1.0)
        result = client.test_results.import_log_file(log_file, incremental=True)
        if received_signal:
            break
    log_event(logger, logging.INFO, "replay.complete", log=log_file)
    if not keep_log:
        cleanup_temp_log(log_file)
    return result


def main() -> None:
    """Follow a growing test result log, uploading entries as they are written."""
    parser = argparse.ArgumentParser(
        description=(
            "Internal replay worker for the Sift pytest plugin. Follows a log file "
            "while the session that writes it is still running, and exits when its "
            "stdin closes. Not meant to be run by hand: to upload or finish a log "
            "that is final, use import-test-result-log."
        )
    )
    parser.add_argument("log_file", help="Path to the .jsonl log file to follow.")
    parser.add_argument("--grpc-url", default=os.getenv("SIFT_GRPC_URI"))
    parser.add_argument("--rest-url", default=os.getenv("SIFT_REST_URI"))
    parser.add_argument("--api-key", default=os.getenv("SIFT_API_KEY"))
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

    # The worker is spawned with --audit-log only when audit logging is on, so
    # its presence is the signal to retain the buffer after a clean upload.
    try:
        _incremental_import_loop(client, args.log_file, keep_log=bool(args.audit_log))
    except Exception as e:
        log_event(logger, logging.ERROR, "replay.failed", error=repr(e))
        log_replay_instructions(args.log_file)
        raise


if __name__ == "__main__":
    main()
