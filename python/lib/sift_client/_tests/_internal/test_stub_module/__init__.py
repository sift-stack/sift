from __future__ import annotations

from sift_client._internal.sync_wrapper import generate_sync_api
from sift_client._tests._internal.test_stub_module.test_py import MockClassAsync

MockClass: type = generate_sync_api(MockClassAsync, "MockClass")
