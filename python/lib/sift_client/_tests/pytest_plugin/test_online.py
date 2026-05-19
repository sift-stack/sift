"""Tests for online mode (the default).

Online mode requires connectivity to Sift. The plugin pings via
``client_has_connection`` at session start and aborts with
``pytest.UsageError`` on failure. Missing ``SIFT_API_KEY`` /
``SIFT_GRPC_URI`` / ``SIFT_REST_URI`` env vars are reported as a usage error
so the failure is actionable.
"""

from __future__ import annotations

from typing import TYPE_CHECKING, Callable

if TYPE_CHECKING:
    import pytest


class TestOnlineMode:
    def test_ping_failure_aborts(
        self,
        pytester: pytest.Pytester,
        clear_sift_env: None,
    ) -> None:
        """Online mode with an unreachable ping aborts the session via UsageError."""
        pytester.makeconftest(
            """
            import pytest
            from unittest.mock import MagicMock

            pytest_plugins = ["sift_client.pytest_plugin"]


            @pytest.fixture(scope="session")
            def sift_client():
                client = MagicMock()
                client.ping.ping.side_effect = ConnectionError("unreachable")
                return client
            """
        )
        pytester.makepyfile(
            """
            import pytest

            @pytest.mark.sift_include
            def test_should_not_run():
                assert True
            """
        )
        result = pytester.runpytest_subprocess()
        assert result.ret != 0
        combined = "\n".join(result.outlines + result.errlines)
        assert "Sift ping failed" in combined, combined

    def test_missing_env_vars_named_in_error(
        self,
        pytester: pytest.Pytester,
        clear_sift_env: None,
        write_plugin_conftest: Callable[[], None],
    ) -> None:
        """The default ``sift_client`` fixture names missing env vars in its error."""
        write_plugin_conftest()
        pytester.makepyfile(
            """
            import pytest

            @pytest.mark.sift_include
            def test_should_not_run():
                pass
            """
        )
        result = pytester.runpytest_subprocess()
        assert result.ret != 0
        combined = "\n".join(result.outlines + result.errlines)
        for var in ("SIFT_API_KEY", "SIFT_GRPC_URI", "SIFT_REST_URI"):
            assert var in combined, combined
