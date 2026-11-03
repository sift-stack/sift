"""Tests for release_version.py. Run from the repository root with
`python3 -m unittest discover -s python/scripts/tests`."""

import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

SCRIPT = Path(__file__).resolve().parents[1] / "release_version.py"

TOWNCRIER = """
[tool.towncrier]
directory = "changelog.d"
filename = "CHANGELOG.md"
template = "changelog.d/template.md.jinja"
type = [{ directory = "breaking" }, { directory = "feat" }, { directory = "fix" }, { directory = "chore", showcontent = false }]
"""


class ReleaseVersionTest(unittest.TestCase):
    def setUp(self):
        # The script reads python/pyproject.toml relative to the working directory.
        self.root = Path(self.enterContext(tempfile.TemporaryDirectory()))
        self.fragments = self.root / "python" / "changelog.d"
        self.fragments.mkdir(parents=True)
        (self.fragments / "README.md").write_text("Not a fragment.\n")
        (self.fragments / "template.md.jinja").write_text("Not a fragment.\n")
        self.write_package("0.1.6")

    def write_package(self, version):
        (self.root / "python" / "pyproject.toml").write_text(
            f'[project]\nname = "example-pkg"\nversion = "{version}"\n' + TOWNCRIER
        )

    def add_fragments(self, *names):
        for name in names:
            (self.fragments / name).write_text("Entry.\n")

    def run_script(self, *args):
        return subprocess.run(
            [sys.executable, str(SCRIPT), *args],
            capture_output=True,
            text=True,
            cwd=self.root,
        )

    def assert_next(self, expected, *args):
        result = self.run_script(*args)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(result.stdout.strip(), expected)

    def assert_rejected(self, message, *args):
        result = self.run_script(*args)
        self.assertEqual(result.returncode, 1)
        self.assertIn(message, result.stderr)

    def test_fix_bumps_patch(self):
        self.add_fragments("1.fix.md", "2.chore.md")
        self.assert_next("0.1.7")

    def test_feat_bumps_minor(self):
        self.add_fragments("1.fix.md", "2.feat.md")
        self.assert_next("0.2.0")

    def test_breaking_bumps_minor_before_1_0(self):
        self.add_fragments("1.breaking.md")
        self.assert_next("0.2.0")

    def test_breaking_bumps_major_from_1_0(self):
        self.write_package("1.4.2")
        self.add_fragments("1.breaking.md", "2.feat.md")
        self.assert_next("2.0.0")

    def test_feat_bumps_minor_from_1_0(self):
        self.write_package("1.4.2")
        self.add_fragments("1.feat.md")
        self.assert_next("1.5.0")

    def test_counter_and_orphan_fragment_names(self):
        self.add_fragments("1.fix.1.md", "+no-pr.feat.md")
        self.assert_next("0.2.0")

    def test_nothing_to_release_rejected(self):
        self.assert_rejected("No changelog fragments to release")
        self.add_fragments("1.chore.md")
        self.assert_rejected("No changelog fragments to release")

    def test_bad_fragment_name_rejected(self):
        self.add_fragments("notes.md")
        self.assert_rejected("Bad fragment name notes.md")

    def test_explicit_version(self):
        self.add_fragments("1.fix.md")
        self.assert_next("1.0.0", "--version", "1.0.0")

    def test_explicit_version_must_be_stable(self):
        self.add_fragments("1.fix.md")
        self.assert_rejected("is not X.Y.Z", "--version", "1.0")

    def test_current(self):
        self.assert_next("0.1.6", "--current")

    def test_leftover(self):
        self.assertEqual(self.run_script("--leftover").returncode, 0)
        self.add_fragments("1.fix.md")
        result = self.run_script("--leftover")
        self.assertEqual(result.returncode, 1)
        self.assertEqual(result.stdout.strip(), str(Path("python/changelog.d/1.fix.md")))

    def lint(self, name, text, *args):
        path = self.fragments / name
        path.write_text(text)
        return self.run_script("--lint", "--added", str(path), *args)

    def assert_lint_passes(self, name, text, *args):
        result = self.lint(name, text, *args)
        self.assertEqual(result.returncode, 0, result.stderr)

    def assert_lint_problem(self, message, name, text, *args):
        path = self.fragments / name
        path.write_text(text)
        self.assert_rejected(message, "--lint", "--added", str(path), *args)

    def test_lint_accepts_one_paragraph(self):
        self.assert_lint_passes("1.fix.md", "Fixed `client.webhooks.get`\nfor archived webhooks.\n")

    def test_lint_accepts_highlight(self):
        text = (
            "# Webhooks: rule actions\n\n"
            "Added `client.webhooks`.\n\n"
            "Each webhook calls a URL when a rule is violated:\n\n"
            "```python\n"
            "# Not a heading inside a code block.\n"
            "client.webhooks.get(webhook_id)\n"
            "```\n\n"
            "- `list` lists webhooks.\n"
        )
        self.assert_lint_passes("1.feat.md", text)

    def test_lint_accepts_empty_chore(self):
        self.assert_lint_passes("1.chore.md", "")

    def test_lint_skips_readme_and_template_and_checks_changed(self):
        result = self.run_script(
            "--lint",
            "--added",
            str(self.fragments / "README.md"),
            str(self.fragments / "template.md.jinja"),
            "--changed",
        )
        self.assertEqual(result.returncode, 0, result.stderr)
        path = self.fragments / "1.fix.md"
        path.write_text("A.\n\nB.\n")
        self.assert_rejected("1.fix.md:2:", "--lint", "--changed", str(path))

    def test_lint_rejects_empty_visible_fragment(self):
        for name in ("1.feat.md", "1.fix.md", "1.breaking.md"):
            self.assert_lint_problem("fragment needs an entry", name, "\n")

    def test_lint_rejects_chore_with_content(self):
        self.assert_lint_problem("A chore fragment is never rendered", "1.chore.md", "Text.\n")

    def test_lint_rejects_second_paragraph_without_title(self):
        self.assert_lint_problem(
            "1.feat.md:2: A fragment without a title is one paragraph",
            "1.feat.md",
            "Added `client.webhooks`.\n\nA second paragraph.\n",
        )

    def test_lint_rejects_code_block_without_title(self):
        self.assert_lint_problem(
            "1.feat.md:2: Headings and code blocks go in a highlight",
            "1.feat.md",
            "Added `client.webhooks`.\n```python\nx\n```\n",
        )

    def test_lint_rejects_empty_title(self):
        self.assert_lint_problem(
            "1.feat.md:1: Write the title as `# Title`", "1.feat.md", "# \n\nA.\n\nB.\n"
        )

    def test_lint_treats_hash_without_space_as_paragraph(self):
        self.assert_lint_passes("1.fix.md", "#15732 follow-up: fixed the retry.\n")

    def test_lint_rejects_heading_as_first_line(self):
        self.assert_lint_problem(
            "1.feat.md:1: Headings and code blocks go in a highlight",
            "1.feat.md",
            "## Webhooks\n",
        )

    def test_lint_options_need_lint(self):
        result = self.run_script("--pr-number", "900")
        self.assertEqual(result.returncode, 2)
        self.assertIn("need --lint", result.stderr)

    def test_lint_rejects_highlight_without_details(self):
        self.assert_lint_problem(
            "Add details after the summary paragraph",
            "1.feat.md",
            "# Webhooks\n\nAdded them.\n",
        )

    def test_lint_rejects_highlight_without_blank_line(self):
        self.assert_lint_problem(
            "1.feat.md:2: Put one blank line after the title",
            "1.feat.md",
            "# Webhooks\nAdded them.\n\nDetails.\n",
        )

    def test_lint_rejects_highlight_starting_with_code(self):
        self.assert_lint_problem(
            "1.feat.md:3: Start the highlight with a summary paragraph",
            "1.feat.md",
            "# Webhooks\n\n```python\nx\n```\n\nDetails.\n",
        )

    def test_lint_rejects_extra_heading(self):
        self.assert_lint_problem(
            "1.feat.md:7: Remove this heading",
            "1.feat.md",
            "# Webhooks\n\nAdded.\n\nDetails.\n\n## More\n",
        )

    def test_lint_rejects_code_block_without_language_or_close(self):
        text = "# Webhooks\n\nAdded.\n\n```\nx\n"
        self.assert_lint_problem("1.feat.md:5: Name the language", "1.feat.md", text)
        self.assert_lint_problem("1.feat.md:5: Close this code block", "1.feat.md", text)

    def test_lint_rejects_bad_name(self):
        self.assert_lint_problem("Bad fragment name notes.md", "notes.md", "Entry.\n")

    def test_pr_number_accepts_matching_names(self):
        for name in ("900.feat.md", "900.fix.1.md"):
            self.assert_lint_passes(name, "Entry.\n", "--pr-number", "900")

    def test_pr_number_rejects_other_number(self):
        self.assert_lint_problem(
            "Rename this fragment to 900.fix.1.md",
            "830.fix.1.md",
            "Entry.\n",
            "--pr-number",
            "900",
        )

    def test_pr_number_rejects_orphan_name(self):
        self.assert_lint_problem(
            "Rename this fragment to 900.feat.md",
            "+no-pr.feat.md",
            "Entry.\n",
            "--pr-number",
            "900",
        )

    def test_pr_number_ignores_changed_fragments(self):
        path = self.fragments / "830.fix.md"
        path.write_text("Entry.\n")
        result = self.run_script("--lint", "--changed", str(path), "--pr-number", "900")
        self.assertEqual(result.returncode, 0, result.stderr)


if __name__ == "__main__":
    unittest.main()
