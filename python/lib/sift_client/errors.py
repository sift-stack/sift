from __future__ import annotations

from typing import NoReturn


class SiftWarning(UserWarning):
    """Base warning for Sift generated warnings."""


class SiftExperimentalWarning(SiftWarning):
    """Warning for experimental features."""


class SiftIgnoredInputWarning(SiftWarning):
    """Input the SDK accepted but could not act on.

    Raised for a key that is not a field of a create or update model, and for an
    update that names no fields to change. Both are caller mistakes that the SDK
    tolerates rather than fails, so this class exists to let a caller promote
    exactly these to errors without also promoting unrelated Sift warnings::

        filterwarnings = error::sift_client.errors.SiftIgnoredInputWarning
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
