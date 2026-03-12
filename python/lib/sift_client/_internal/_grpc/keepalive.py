from typing import TypedDict

DEFAULT_KEEPALIVE_TIME_MS = 20_000
"""Interval with which to send keepalive pings"""

DEFAULT_KEEPALIVE_TIMEOUT_MS = 20_000
"""Timeout while waiting for server to acknowledge keepalive ping"""

DEFAULT_KEEPALIVE_PERMIT_WITHOUT_CALLS = 1
"""Allows connection without any active RPCs"""

DEFAULT_MAX_PINGS_WITHOUT_DATA = 0
"""Disabled"""


# https://github.com/grpc/grpc/blob/master/doc/keepalive.md
class KeepaliveConfig(TypedDict):
    """
    Make make this public in the future to allow folks to configure their own keepalive settings
    if there is demand for it.
    """

    keepalive_time_ms: int
    keepalive_timeout_ms: int
    keepalive_permit_without_calls: int
    max_pings_without_data: int


DEFAULT_KEEPALIVE_CONFIG: KeepaliveConfig = {
    "keepalive_time_ms": DEFAULT_KEEPALIVE_TIME_MS,
    "keepalive_timeout_ms": DEFAULT_KEEPALIVE_TIMEOUT_MS,
    "keepalive_permit_without_calls": DEFAULT_KEEPALIVE_PERMIT_WITHOUT_CALLS,
    "max_pings_without_data": DEFAULT_MAX_PINGS_WITHOUT_DATA,
}
