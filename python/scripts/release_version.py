"""Print the next Python release version, from the version in `python/pyproject.toml` and the
fragment types in `python/changelog.d`. Run from the repository root. Uses only the standard
library."""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

PYPROJECT = Path("python/pyproject.toml")
FRAGMENTS_DIR = Path("python/changelog.d")
FRAGMENT_TYPES = {"breaking", "feat", "fix", "chore"}
STABLE = re.compile(r"^(\d+)\.(\d+)\.(\d+)$")


def current_version() -> tuple[int, int, int]:
    version = re.search(r'^version = "(.+)"$', PYPROJECT.read_text(), re.MULTILINE).group(1)
    match = STABLE.match(version)
    if not match:
        sys.exit(f"Current version {version} is not a stable X.Y.Z version. Pass --version.")
    major, minor, patch = (int(part) for part in match.groups())
    return major, minor, patch


def fragment_types() -> set[str]:
    types = set()
    for path in FRAGMENTS_DIR.glob("*.md"):
        if path.name.lower() == "readme.md":
            continue
        parts = path.name.split(".")
        if len(parts) < 3 or parts[-2] not in FRAGMENT_TYPES:
            sys.exit(f"Bad fragment name {path.name}. Use <PR>.<type>.md with a known type.")
        types.add(parts[-2])
    return types


def next_version(explicit: str) -> str:
    types = fragment_types() - {"chore"}
    if not types:
        sys.exit("No changelog fragments to release. Only `chore` fragments or none found.")
    if explicit:
        if not STABLE.match(explicit):
            sys.exit(f"--version {explicit} is not X.Y.Z.")
        return explicit
    major, minor, patch = current_version()
    if "breaking" in types and major >= 1:
        return f"{major + 1}.0.0"
    if types & {"breaking", "feat"}:
        return f"{major}.{minor + 1}.0"
    return f"{major}.{minor}.{patch + 1}"


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--version", default="", help="Explicit X.Y.Z. Skips the automatic bump.")
    print(next_version(parser.parse_args().version))
