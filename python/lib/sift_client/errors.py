from __future__ import annotations

from typing import NoReturn


class SiftWarning(UserWarning):
    """Base warning for Sift generated warnings."""


class SiftExperimentalWarning(SiftWarning):
    """Warning for experimental features."""


def _sift_stream_bindings_import_error(original_error: ImportError) -> NoReturn:
    # Returns NoReturn to satisfy pyright
    raise ImportError(
        "sift_stream_bindings is required for ingestion streaming functionality. "
        "Install it with: pip install sift-stack-py[sift-stream]"
    ) from original_error
