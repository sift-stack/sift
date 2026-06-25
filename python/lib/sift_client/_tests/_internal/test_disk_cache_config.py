"""Tests for :class:`sift_client._internal.disk_cache_config.DiskCacheConfig`.

The class is a small intent holder; the tests pin three things that
resource lazy-init code relies on:

* Enable / disable round-trips preserve the right state and clear overrides.
* ``using_default_path`` reflects "enabled AND no user override", which
  drives the silent-fallback-vs-loud-raise distinction in resources.
* ``enable`` accepts ``os.PathLike`` and stringifies it eagerly so consumers
  never need to handle ``pathlib.Path`` vs ``str``.
"""

from __future__ import annotations

import pathlib

import pytest

from sift_client._internal.disk_cache_config import DiskCacheConfig


class TestDiskCacheConfig:
    def test_opt_out_initial_state_enabled_no_overrides(self) -> None:
        """``enabled=True`` (opt-out) starts on with no overrides."""
        config = DiskCacheConfig(enabled=True)
        assert config.enabled
        assert config.path is None
        assert config.max_bytes is None
        assert config.using_default_path

    def test_opt_in_initial_state_disabled(self) -> None:
        """``enabled=False`` (opt-in) starts off; ``using_default_path`` is False."""
        config = DiskCacheConfig(enabled=False)
        assert not config.enabled
        assert config.path is None
        assert config.max_bytes is None
        assert not config.using_default_path

    def test_enable_with_no_args_keeps_defaults(self) -> None:
        """``enable()`` with no args turns on and clears any prior overrides."""
        config = DiskCacheConfig(enabled=False)
        config.enable()
        assert config.enabled
        assert config.path is None
        assert config.max_bytes is None
        assert config.using_default_path

    def test_enable_with_path_marks_non_default(self) -> None:
        """A user-supplied path flips ``using_default_path`` off."""
        config = DiskCacheConfig(enabled=True)
        config.enable(path="/custom/path")
        assert config.enabled
        assert config.path == "/custom/path"
        assert not config.using_default_path

    def test_enable_with_max_bytes_keeps_default_path(self) -> None:
        """Setting ``max_bytes`` alone doesn't make the path non-default."""
        config = DiskCacheConfig(enabled=True)
        config.enable(max_bytes=1024)
        assert config.enabled
        assert config.path is None
        assert config.max_bytes == 1024
        assert config.using_default_path

    def test_enable_stringifies_pathlike(self) -> None:
        """``os.PathLike`` inputs are stored as strings so consumers can be dumb."""
        config = DiskCacheConfig(enabled=True)
        config.enable(path=pathlib.Path("/some/path"))
        assert isinstance(config.path, str)
        assert config.path == "/some/path"

    def test_disable_clears_overrides(self) -> None:
        """``disable()`` zeroes path and max_bytes so a future re-enable starts clean."""
        config = DiskCacheConfig(enabled=True)
        config.enable(path="/custom", max_bytes=4096)
        config.disable()
        assert not config.enabled
        assert config.path is None
        assert config.max_bytes is None
        assert not config.using_default_path

    def test_reenable_after_disable_returns_to_defaults(self) -> None:
        """``disable`` then ``enable()`` (no args) restores the opt-out starting state."""
        config = DiskCacheConfig(enabled=True)
        config.enable(path="/custom", max_bytes=4096)
        config.disable()
        config.enable()
        assert config.enabled
        assert config.path is None
        assert config.max_bytes is None
        assert config.using_default_path

    @pytest.mark.parametrize(
        ("enabled", "path", "expected"),
        [
            (True, None, True),
            (True, "/custom", False),
            (False, None, False),
            (False, "/custom", False),  # disabled wins even with a stashed path
        ],
        ids=["enabled+default", "enabled+custom", "disabled+default", "disabled+custom"],
    )
    def test_using_default_path_matrix(
        self, enabled: bool, path: str | None, expected: bool
    ) -> None:
        """``using_default_path`` is the AND of ``enabled`` and ``path is None``."""
        config = DiskCacheConfig(enabled=enabled)
        if path is not None:
            # Bypass enable() so we can exercise the disabled+custom combo
            # without enable() flipping enabled back on.
            config._path = path
        assert config.using_default_path is expected
