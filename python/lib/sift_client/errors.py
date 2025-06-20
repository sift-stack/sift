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


_warned_already = False


def _sift_client_experimental_warning():
    global _warned_already
    if _warned_already:  # Prevent sending this warning too many times
        return
    _warned_already = True
    warnings.warn(
        "`sift_client` is experimental and is subject to change. Use with caution.",
        SiftExperimentalWarning,
    )
