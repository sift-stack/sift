from __future__ import annotations

import warnings


class SiftWarning(UserWarning):
    """Base warning for Sift generated warnings."""


class SiftExperimentalWarning(SiftWarning):
    """Warning for experimental features."""

def _sift_client_experimental_warning():
    # Ensure this warning has only been emitted once, even if used in different places.
    if not getattr(_sift_client_experimental_warning, "_warned", False):
        warnings.warn(
            "`sift_client` is experimental and is subject to change. Use with caution.",
            SiftExperimentalWarning,
            stacklevel=2
        )
        _sift_client_experimental_warning._warned = True
