"""Global configuration for the Sift client library."""

from __future__ import annotations

from dataclasses import dataclass, fields


@dataclass
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

    def __setattr__(self, name: str, value: object) -> None:
        if name not in {f.name for f in fields(self)}:
            raise AttributeError(f"Unknown setting: {name!r}")
        super().__setattr__(name, value)


config = Config()
