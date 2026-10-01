from __future__ import annotations

from typing import NoReturn


class SiftWarning(UserWarning):
    """Base warning for Sift generated warnings."""


class SiftExperimentalWarning(SiftWarning):
    """Warning for experimental features."""


class SiftIgnoredInputWarning(SiftWarning):
    """Warning for input the SDK ignored.

    Covers an unrecognized field on a Create/Update pydantic model, and an update
    that names no fields to change.
    """


class SiftCredentialsError(ValueError):
    """Raised when Sift credentials cannot be resolved.

    Subclasses ``ValueError`` because that is what ``SiftClient`` raised for
    unusable connection arguments before credential resolution existed.
    """


def _sift_stream_bindings_import_error(original_error: ImportError) -> NoReturn:
    # Returns NoReturn to satisfy pyright
    raise ImportError(
        "sift_stream_bindings is required for ingestion streaming functionality. "
        "Install it with: pip install sift-stack-py[sift-stream]"
    ) from original_error
