"""Project-level conftest for the pytest plugin demo.

A single ``pytest_plugins`` declaration is enough to load the plugin — its
fixtures, hooks, and CLI options register through standard pytest machinery
from there. ``load_dotenv()`` is optional; it just lets the default
``sift_client`` fixture pick up ``SIFT_API_KEY`` / ``SIFT_GRPC_URI`` /
``SIFT_REST_URI`` from a local ``.env`` when running against a real Sift org.
These can also be set as environment variables using your preferred method.
"""

from dotenv import load_dotenv

load_dotenv()

pytest_plugins = ["sift_client.pytest_plugin"]
