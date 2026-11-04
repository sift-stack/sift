"""Print the next Python release version, from the version in `python/pyproject.toml` and the
fragment types in `python/changelog.d`. Run from the repository root with Python 3.11+. Uses only
the standard library."""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

import tomllib

PYPROJECT = tomllib.loads(Path("python/pyproject.toml").read_text())
TOWNCRIER = PYPROJECT["tool"]["towncrier"]
FRAGMENTS_DIR = Path("python") / TOWNCRIER["directory"]
FRAGMENT_TYPES = {t["directory"] for t in TOWNCRIER["type"]}
NOT_FRAGMENTS = {"README.md", Path(TOWNCRIER["template"]).name}
STABLE = re.compile(r"^(\d+)\.(\d+)\.(\d+)$")


def current_version() -> str:
    return PYPROJECT["project"]["version"]


def fragments() -> list[Path]:
    return sorted(p for p in FRAGMENTS_DIR.iterdir() if p.is_file() and p.name not in NOT_FRAGMENTS)


def fragment_type(path: Path) -> str:
    # <name>.<type>.md, or <name>.<type>.<counter>.md when towncrier deduplicates a name.
    parts = path.name.removesuffix(".md").split(".")
    if len(parts) > 2 and parts[-1].isdigit():
        parts = parts[:-1]
    if len(parts) < 2 or parts[-1] not in FRAGMENT_TYPES:
        sys.exit(f"Bad fragment name {path.name}. Use <PR>.<type>.md with a known type.")
    return parts[-1]


def next_version(explicit: str) -> str:
    types = {fragment_type(path) for path in fragments()} - {"chore"}
    if not types:
        sys.exit("No changelog fragments to release. Only `chore` fragments or none found.")
    if explicit:
        if not STABLE.match(explicit):
            sys.exit(f"--version {explicit} is not X.Y.Z.")
        return explicit
    match = STABLE.match(current_version())
    if not match:
        sys.exit(f"Current version {current_version()} is not X.Y.Z. Pass --version.")
    major, minor, patch = (int(part) for part in match.groups())
    if "breaking" in types and major >= 1:
        return f"{major + 1}.0.0"
    if types & {"breaking", "feat"}:
        return f"{major}.{minor + 1}.0"
    return f"{major}.{minor}.{patch + 1}"


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    mode = parser.add_mutually_exclusive_group()
    mode.add_argument("--version", default="", help="Explicit X.Y.Z. Skips the automatic bump.")
    mode.add_argument("--current", action="store_true", help="Print the current version.")
    mode.add_argument(
        "--leftover",
        action="store_true",
        help="List unreleased fragments and exit 1 if there are any.",
    )
    args = parser.parse_args()
    if args.current:
        print(current_version())
    elif args.leftover:
        leftover = fragments()
        for path in leftover:
            print(path)
        sys.exit(1 if leftover else 0)
    else:
        print(next_version(args.version))
