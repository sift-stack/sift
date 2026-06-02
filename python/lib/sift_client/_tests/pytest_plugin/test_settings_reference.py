"""Guard rail that pins the docs settings table to the ``_OPTIONS`` registry.

If you add or change a setting in ``lib/sift_client/pytest_plugin.py`` without
regenerating the Markdown table in ``docs/guides/pytest_plugin/configuration.md``,
this test fails with the up-to-date block to paste in.
"""

from __future__ import annotations

from pathlib import Path
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    import pytest


# python/lib/sift_client/_tests/pytest_plugin/test_settings_reference.py -> python/
_REPO_PYTHON_DIR = Path(__file__).resolve().parents[4]
_DOCS_PATH = _REPO_PYTHON_DIR / "docs/guides/pytest_plugin/configuration.md"


def test_settings_reference_docs_in_sync(pytestconfig: pytest.Config) -> None:
    """The Markdown table under '## Settings reference' matches the registry verbatim."""
    if not _DOCS_PATH.exists():
        import pytest

        pytest.skip(f"{_DOCS_PATH} not present in this checkout")
    from sift_client.pytest_plugin import _render_settings_reference

    rendered = _render_settings_reference()
    content = _DOCS_PATH.read_text()
    if rendered not in content:
        import pytest

        pytest.fail(
            "Settings reference is out of sync with the _OPTIONS registry. Replace the "
            "table under '## Settings reference' in "
            "docs/guides/pytest_plugin/configuration.md with:\n\n" + rendered
        )
