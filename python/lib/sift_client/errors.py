from __future__ import annotations

import warnings


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
