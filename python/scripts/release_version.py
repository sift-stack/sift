"""Print the next Python release version, from the version in `python/pyproject.toml` and the
fragment types in `python/changelog.d`, and check the shape of fragments. Run from the repository
root with Python 3.11+. Uses only the standard library."""

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
HIDDEN_TYPES = {t["directory"] for t in TOWNCRIER["type"] if not t.get("showcontent", True)}
NOT_FRAGMENTS = {"README.md", Path(TOWNCRIER["template"]).name}
STABLE = re.compile(r"^(\d+)\.(\d+)\.(\d+)$")
# A fragment name has at least a name and a type: <name>.<type>.
MIN_NAME_PARTS = 2
HEADING = re.compile(r"^#{1,6} ")
FENCE = "```"


def current_version() -> str:
    return PYPROJECT["project"]["version"]


def is_fragment(path: Path) -> bool:
    """Whether a file in the fragment directory is a fragment."""
    return path.name not in NOT_FRAGMENTS


def is_fence(line: str) -> bool:
    """Whether a line opens or closes a code block."""
    return line.lstrip().startswith(FENCE)


def fragments() -> list[Path]:
    return sorted(p for p in FRAGMENTS_DIR.iterdir() if p.is_file() and is_fragment(p))


def parse_name(path: Path) -> tuple[str, str, str]:
    """Return the name, type, and counter suffix of a fragment, or exit on a bad name."""
    # <name>.<type>.md, or <name>.<type>.<counter>.md when towncrier deduplicates a name.
    parts = path.name.removesuffix(".md").split(".")
    counter = ""
    if len(parts) > MIN_NAME_PARTS and parts[-1].isdigit():
        counter = "." + parts.pop()
    if len(parts) < MIN_NAME_PARTS or parts[-1] not in FRAGMENT_TYPES:
        sys.exit(f"Bad fragment name {path.name}. Use <PR>.<type>.md with a known type.")
    return ".".join(parts[:-1]), parts[-1], counter


def fragment_type(path: Path) -> str:
    return parse_name(path)[1]


def lint(path: Path, pr_number: int | None = None) -> list[tuple[int, str]]:
    """Return the (line, message) problems in a fragment.

    A fragment is one paragraph, or a highlight: a `# Title` line, a summary paragraph, and
    details. See `python/changelog.d/README.md`. With pr_number, the fragment name must also
    start with it.
    """
    name, kind, counter = parse_name(path)
    problems = []
    if pr_number is not None and name != str(pr_number):
        problems.append(
            (
                1,
                f"Rename this fragment to {pr_number}.{kind}{counter}.md. A fragment that "
                "this pull request adds must use its number, so the changelog links the right "
                "pull request.",
            )
        )
    lines = path.read_text().strip("\n").splitlines()
    empty = not any(line.strip() for line in lines)
    if kind in HIDDEN_TYPES:
        if empty:
            return problems
        return [*problems, (1, f"A {kind} fragment is never rendered. Leave the file empty.")]
    if empty:
        return [
            *problems,
            (
                1,
                f"A {kind} fragment needs an entry. Write what changed, or make it an "
                "empty chore fragment if users see no change.",
            ),
        ]
    if lines[0].startswith("# "):
        return problems + lint_highlight(lines)
    return problems + lint_paragraph(lines)


def lint_paragraph(lines: list[str]) -> list[tuple[int, str]]:
    """Return the first problem in a fragment without a title."""
    for number, line in enumerate(lines, start=1):
        if not line.strip():
            return [
                (
                    number,
                    "A fragment without a title is one paragraph. Remove the blank line, "
                    "or start the fragment with a `# Title` line to make it a highlight.",
                )
            ]
        if is_fence(line) or HEADING.match(line):
            return [
                (
                    number,
                    "Headings and code blocks go in a highlight. Start the fragment with a "
                    "`# Title` line.",
                )
            ]
    return []


def lint_highlight(lines: list[str]) -> list[tuple[int, str]]:
    """Return the problems in a highlight fragment."""
    problems = []

    def problem(number: int, message: str) -> None:
        problems.append((number, message))

    if not lines[0][2:].strip():
        problem(1, "Write the title as `# Title`.")
    if len(lines) < 3 or lines[1].strip() or not lines[2].strip():
        problem(2, "Put one blank line after the title, then the summary paragraph.")
        return problems
    if (
        HEADING.match(lines[2])
        or is_fence(lines[2])
        or lines[2].lstrip().startswith(("- ", "* ", "1. "))
    ):
        problem(
            3,
            "Start the highlight with a summary paragraph, not a list, heading, or code block.",
        )
    summary_end = next((i for i in range(2, len(lines)) if not lines[i].strip()), len(lines))
    if not any(line.strip() for line in lines[summary_end:]):
        problem(
            len(lines),
            "Add details after the summary paragraph, or remove the title to "
            "make this a one-paragraph entry.",
        )
    fence_start = 0
    for number, line in enumerate(lines[1:], start=2):
        if is_fence(line):
            if fence_start:
                fence_start = 0
            else:
                fence_start = number
                if not line.lstrip().removeprefix(FENCE).strip():
                    problem(number, "Name the language after the opening ```, such as ```python.")
        elif not fence_start and HEADING.match(line):
            problem(
                number,
                "Remove this heading. The title is the highlight's only heading, "
                "and the changelog sets the heading levels.",
            )
    if fence_start:
        problem(fence_start, "Close this code block with ```.")
    return problems


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
    mode.add_argument(
        "--lint",
        action="store_true",
        help="Check the shape of the --added and --changed fragments and exit 1 on a problem.",
    )
    parser.add_argument(
        "--added",
        nargs="*",
        type=Path,
        default=[],
        metavar="FILE",
        help="With --lint, files a pull request adds. Files that are not fragments are skipped.",
    )
    parser.add_argument(
        "--changed",
        nargs="*",
        type=Path,
        default=[],
        metavar="FILE",
        help="With --lint, files a pull request changes. Files that are not fragments are skipped.",
    )
    parser.add_argument(
        "--pr-number",
        type=int,
        help="With --lint, require each --added fragment name to start with this PR number.",
    )
    args = parser.parse_args()
    if (args.added or args.changed or args.pr_number is not None) and not args.lint:
        parser.error("--added, --changed, and --pr-number need --lint.")
    if args.current:
        print(current_version())
    elif args.leftover:
        leftover = fragments()
        for path in leftover:
            print(path)
        sys.exit(1 if leftover else 0)
    elif args.lint:
        checks = [(path, args.pr_number) for path in args.added]
        checks += [(path, None) for path in args.changed]
        problems = [
            f"{path}:{number}: {message}"
            for path, pr_number in checks
            if is_fragment(path)
            for number, message in lint(path, pr_number)
        ]
        for problem in problems:
            print(problem, file=sys.stderr)
        sys.exit(1 if problems else 0)
    else:
        print(next_version(args.version))
