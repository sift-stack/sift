from __future__ import annotations

import inspect
from pathlib import Path
from typing import NoReturn

_PACKAGE_ROOT = str(Path(__file__).resolve().parent)


def caller_stacklevel() -> int:
    """Return the ``warnings.warn`` stacklevel of the first frame outside the SDK.

    A fixed stacklevel points at SDK or pydantic internals, which tells the
    caller nothing about which of their lines caused the warning.

    Call this from the function that calls ``warnings.warn``.
    """
    frame = inspect.currentframe()
    frame = frame.f_back if frame is not None else None  # the function calling warn()
    level = 1
    while frame is not None:
        filename = frame.f_code.co_filename
        if not filename.startswith(_PACKAGE_ROOT) and "pydantic" not in filename:
            return level
        frame = frame.f_back
        level += 1
    return 2


class SiftWarning(UserWarning):
    """Base warning for Sift generated warnings."""


class SiftExperimentalWarning(SiftWarning):
    """Warning for experimental features."""


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
