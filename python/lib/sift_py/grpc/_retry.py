from __future__ import annotations

import json
from typing import Dict, List, TypedDict

from grpc import StatusCode
from typing_extensions import Self


class RetryPolicy:
    """
    Retry policy meant to be used for `sift_py.grpc.transport.SiftChannel`. Users may have the ability to configure their own
    custom retry policy in the future, but for now this is primarily intended for internal use.

    - [Retry policy schema](https://github.com/grpc/grpc-proto/blob/ec30f589e2519d595688b9a42f88a91bdd6b733f/grpc/service_config/service_config.proto#L136)
    - [Enable gRPC retry option](https://github.com/grpc/grpc/blob/9a5fdfc3d3a7fc575a394360be4532ee09a85620/include/grpc/impl/channel_arg_names.h#L311)
    - [Service config option](https://github.com/grpc/grpc/blob/9a5fdfc3d3a7fc575a394360be4532ee09a85620/include/grpc/impl/channel_arg_names.h#L207)
    """

    config: RetryConfig

    DEFAULT_POLICY: RetryConfig = {
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
    def default(cls) -> Self:
        return cls(config=cls.DEFAULT_POLICY)


class RetryConfig(TypedDict):
    methodConfig: List[MethodConfigDict]


class MethodConfigDict(TypedDict):
    name: List[Dict[str, str]]
    retryPolicy: RetryConfigDict


class RetryConfigDict(TypedDict):
    maxAttempts: int
    initialBackoff: str
    maxBackoff: str
    backoffMultiplier: int
    retryableStatusCodes: List[str]
