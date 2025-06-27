from __future__ import annotations

import warnings


class SiftWarning(UserWarning):
    """
    Base warning for Sift generated warnings.
    """


class SiftExperimentalWarning(SiftWarning):
    """
    Warning for experimental features.
    """


def _sift_client_experimental_warning():
    warnings.warn(
        "`sift_client` is experimental and is subject to change. Use with caution.",
        SiftExperimentalWarning,
    )
