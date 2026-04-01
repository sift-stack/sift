"""Global configuration for the Sift client library."""

from __future__ import annotations

from dataclasses import dataclass


@dataclass(slots=True)
class Config:
    """Global configuration for the Sift client library.

    This is a singleton dataclass, use the module-level ``config`` instance
    rather than creating your own::

        import sift_client

        sift_client.config.show_progress = False

    Setting an attribute that doesn't exist raises ``AttributeError`` so
    typos are caught immediately.

    """

    show_progress: bool | None = None
    """Controls progress-bar display for job polling and file downloads.

    ``None`` (default) shows bars for sync calls and hides them for async.
    Set to ``False`` to disable everywhere.
    """


config = Config()
