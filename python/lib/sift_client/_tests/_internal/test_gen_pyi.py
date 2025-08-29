from __future__ import annotations

import os
import pathlib
import re

import pytest

from sift_client._internal.gen_pyi import generate_stubs_for_module


@pytest.fixture
def generated():
    # Make sure cwd is set up correctly
    cwd = pathlib.Path(__file__).parent.parent.parent.parent  # should be in python/lib
    assert str(cwd).endswith("python/lib")
    os.chdir(cwd)

    generated = generate_stubs_for_module(pathlib.Path(__file__).parent / "test_stub_module")
    assert len(generated) == 1, "test_ file should be excluded"
    return next(iter(generated.values()))


def test_extract_imports(generated):
    sections = generated.split("class")

    import_section = sections[0]

    assert "Auto-generated" in import_section

    assert "from __future__ import annotations" in import_section
    assert "from sift_client.sift_types.asset import Asset" in import_section
    assert "from sift_client.resources._base import ResourceBase" in import_section


def test_generate_stubs(generated):
    # Class
    assert "class MockClass" in generated, (
        "MockClass should be generated from async version with base"
    )

    # async_method
    assert "def async_method(self, param1: str, *, param2: int = 0) -> str:" in generated
    # correct sync def, not async
    assert "async def async_method" not in generated
    # docstring
    assert "Mock async method docstring" in generated
    assert "param1: Param1 docstring." in generated
    assert "A string." in generated

    # sync_method
    assert "def sync_method(self, param1: str, **kwargs) -> Asset | None:" in generated
    # correct sync def, not async
    assert "async def sync_method" not in generated
    # docstring
    assert "Mock sync method docstring" in generated

    # async_prop
    assert re.search(r"@property\s*def async_prop\(self\) -> str:", generated)
    # correct sync def, not async
    assert "async def async_prop" not in generated, "async_prop should be converted to sync"
    # docstring
    assert "Mock async property docstring." in generated

    # sync_prop
    assert re.search(r"@property\s*def sync_prop\(self\) -> int:", generated)
    # correct sync def, not async
    assert "async def sync_prop" not in generated
    # docstring
    assert "Mock sync property docstring." in generated


def test_non_generated_omissions(generated):
    assert "class SecondMockClass" not in generated
