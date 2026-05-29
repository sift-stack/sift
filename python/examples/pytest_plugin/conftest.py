"""Project-level conftest for the pytest plugin demo.

A single ``pytest_plugins`` declaration is all that's needed — the plugin's
fixtures, hooks, and CLI options register through standard pytest machinery
from there.

The default ``sift_client`` fixture reads ``SIFT_API_KEY`` / ``SIFT_GRPC_URI``
/ ``SIFT_REST_URI`` from the environment. Set them however you prefer: your CI
secret store, your shell, or a local ``.env`` loaded by ``pytest-dotenv``
(``pip install pytest-dotenv`` and it auto-loads ``.env`` — no code here).
"""

pytest_plugins = ["sift_client.pytest_plugin"]
