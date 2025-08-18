import warnings

import google.protobuf.message


class SiftError(Exception):
    """
    These exceptions are raised when something totally unexpected occurs and is
    meant to indicate that the error is likely not caused by the user, but rather,
    the library itself. These errors should be reported to Sift.
    """

    msg: str

    def __init__(self, msg: str):
        super().__init__(f"{msg}\nPlease notify Sift.")


class SiftAPIDeprecationWarning(FutureWarning):
    """
    Warning used for deprecated API features that may be removed in future updates.
    """

    ...


def _component_deprecation_warning():
    warnings.warn(
        "`component` field of Channel has been deprecated and will be removed in 1.0.0. "
        "See docs for more details: https://docs.siftstack.com/docs/glossary#component",
        SiftAPIDeprecationWarning,
    )


# The default max message size for the Sift gRPC server.
GRPC_MAX_MESSAGE_SIZE = 4_194_304


class ProtobufMaxSizeExceededError(Exception):
    """
    The library limits the size of certain protobufs to prevent gRPC messages from being too big.
    """


def raise_if_too_large(pb: google.protobuf.message.Message):
    size = len(pb.SerializeToString())
    name = getattr(pb, "name", pb.__class__.__name__)
    if size > GRPC_MAX_MESSAGE_SIZE:
        raise ProtobufMaxSizeExceededError(f"{name} too large: {size}")
