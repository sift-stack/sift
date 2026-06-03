from __future__ import annotations

import json
from typing import ClassVar, TypedDict

from grpc import StatusCode
from typing_extensions import NotRequired, Self


def _format_grpc_duration(seconds: float) -> str:
    """Format seconds as a protobuf Duration string, e.g. ``60s`` or ``0.5s``."""
    trimmed = f"{seconds:.9f}".rstrip("0").rstrip(".")
    return f"{trimmed}s"


class RetryPolicy:
    """
    Retry policy meant to be used for `sift_py.grpc.transport.SiftChannel`. Users may have the ability to configure their own
    custom retry policy in the future, but for now this is primarily intended for internal use.

    - [Retry policy schema](https://github.com/grpc/grpc-proto/blob/ec30f589e2519d595688b9a42f88a91bdd6b733f/grpc/service_config/service_config.proto#L136)
    - [Enable gRPC retry option](https://github.com/grpc/grpc/blob/9a5fdfc3d3a7fc575a394360be4532ee09a85620/include/grpc/impl/channel_arg_names.h#L311)
    - [Service config option](https://github.com/grpc/grpc/blob/9a5fdfc3d3a7fc575a394360be4532ee09a85620/include/grpc/impl/channel_arg_names.h#L207)
    """

    config: RetryConfig

    DEFAULT_POLICY: ClassVar[RetryConfig] = {
        "methodConfig": [
            {
                # We can configure this on a per-service and RPC basis but for now we'll
                # apply this across all services and RPCs.
                "name": [{}],
                "retryPolicy": {
                    # gRPC does not allow more than 5 attempts
                    "maxAttempts": 5,
                    "initialBackoff": "0.05s",
                    "maxBackoff": "5s",
                    "backoffMultiplier": 4,
                    "retryableStatusCodes": [
                        StatusCode.INTERNAL.name,
                        StatusCode.UNKNOWN.name,
                        StatusCode.UNAVAILABLE.name,
                        StatusCode.ABORTED.name,
                        StatusCode.DEADLINE_EXCEEDED.name,
                    ],
                },
            }
        ]
    }

    def __init__(self, config: RetryConfig):
        self.config = config

    def as_json(self) -> str:
        return json.dumps(self.config)

    @classmethod
    def default(cls, timeout_seconds: float | None = None) -> Self:
        """Build the default policy, optionally with a per-call deadline.

        When ``timeout_seconds`` is set, every method config carries a
        ``timeout`` so the C-core fails a stalled call with ``DEADLINE_EXCEEDED``
        instead of hanging. The deadline bounds the whole call, retries included,
        and a per-call ``timeout=`` still overrides it.
        """
        if timeout_seconds is None:
            return cls(config=cls.DEFAULT_POLICY)
        duration = _format_grpc_duration(timeout_seconds)
        config: RetryConfig = {
            "methodConfig": [
                {**method_config, "timeout": duration}
                for method_config in cls.DEFAULT_POLICY["methodConfig"]
            ]
        }
        return cls(config=config)


class RetryConfig(TypedDict):
    methodConfig: list[MethodConfigDict]


class MethodConfigDict(TypedDict):
    name: list[dict[str, str]]
    retryPolicy: RetryConfigDict
    # Applies to all methods via the wildcard `name` matcher above. If a
    # server-streaming RPC is ever added to this channel, scope this by method
    # name so the deadline doesn't clip a long-lived stream.
    timeout: NotRequired[str]


class RetryConfigDict(TypedDict):
    maxAttempts: int
    initialBackoff: str
    maxBackoff: str
    backoffMultiplier: int
    retryableStatusCodes: list[str]
