from __future__ import annotations

import warnings
from typing import NoReturn


class SiftWarning(UserWarning):
    """Base warning for Sift generated warnings."""


class SiftExperimentalWarning(SiftWarning):
    """Warning for experimental features."""


_sift_client_experimental_warned = False


def _sift_client_experimental_warning():
    # Ensure this warning has only been emitted once, even if used in different places.
    global _sift_client_experimental_warned
    if not _sift_client_experimental_warned:
        warnings.warn(
            "`sift_client` is experimental and is subject to change. Use with caution.",
            SiftExperimentalWarning,
            stacklevel=2,
        )
        _sift_client_experimental_warned = True


def _sift_stream_bindings_import_error(original_error: ImportError) -> NoReturn:
    # Returns NoReturn to satisfy pyright
    raise ImportError(
        "sift_stream_bindings is required for ingestion streaming functionality. "
        "Install it with: pip install sift-stack-py[sift-stream]"
    ) from original_error
