# Changelog fragments

Each pull request that changes `python/` adds one fragment file to this directory. A release
renders the fragments into `CHANGELOG.md` and deletes them. Do not edit `CHANGELOG.md` by hand.

## Add a fragment

Name the file `<PR number>.<type>.md`. From `python/`, run:

```bash
uvx towncrier create 830.feat.md --content "Added \`client.webhooks\`."
```

| Type       | Use for                          | Version bump |
| ---------- | -------------------------------- | ------------ |
| `breaking` | A change that breaks callers     | major        |
| `feat`     | New user-facing behavior         | minor        |
| `fix`      | A bug fix users can see          | patch        |
| `chore`    | No user-visible change           | none         |

The version bump follows the highest type in the release. While the major version is 0, a
`breaking` change bumps the minor version. To release a different version, pass `version` to the
release workflow.

Write the entry for a user of the library. Start with what changed. The renderer adds the PR link. To add a second entry of the same type for one PR, name the file `<PR number>.<type>.1.md`. A fragment has one of two shapes, and the changelog check rejects any other.

### One paragraph

Most entries are one paragraph, with no blank lines, headings, or code blocks. It renders as a bullet under its type.

```markdown
Fixed `client.runs.find` so a run with no assets is returned.
```

### Highlight

Use a highlight when an entry needs more than one paragraph, a code example, or a list. Start the fragment with a `# Title` line, then a blank line and a one-paragraph summary. Put the details after the summary: paragraphs, lists, and code blocks.

The summary renders as a bullet under its type, with a link to the highlight. The details render once under the `#### Title` heading in the release's Highlights section, after the type lists. The summary does not repeat there, so write the first details paragraph to make sense without it.

`````markdown
# Artifact versions

Added `client.artifacts.versions` for browsing the version history of an artifact.

Each version has its own file. Download any version by its id:

```python
version = client.artifacts.versions.list(artifact_id)[0]
client.artifacts.versions.download(version.id, path="report.pdf")
```
`````

### What the changelog check enforces

For each fragment that a pull request adds or changes, the check requires that:

- A `breaking`, `feat`, or `fix` fragment is not empty, and a `chore` fragment is empty.
- A fragment without a title is one paragraph, with no blank lines, headings, or code blocks.
- A highlight has a `# Title` line, one blank line, a summary paragraph, and details after the summary.
- A highlight has no other headings.
- Each code block names its language, such as ```` ```python ````, and is closed.
- A fragment that the pull request adds is named `<PR number>.<type>.md` or `<PR number>.<type>.<n>.md`. Fragments that the pull request only changes keep their names.

To run the check locally, from the repository root:

```bash
python3 python/scripts/release_version.py --lint --pr-number 830 --added python/changelog.d/830.feat.md
```

### What the release looks like

If the one-paragraph example is `831.fix.md` and the highlight example is `830.feat.md`, the release renders them as:

`````markdown
## [v0.26.0] - November 3, 2026

### Features

- Added `client.artifacts.versions` for browsing the version history of an artifact. See [Artifact versions](#highlight-0-26-0-1). ([#830](https://github.com/sift-stack/sift/pull/830))

### Fixes

- Fixed `client.runs.find` so a run with no assets is returned. ([#831](https://github.com/sift-stack/sift/pull/831))

### Highlights

#### <a id="highlight-0-26-0-1"></a>Artifact versions

Each version has its own file. Download any version by its id:

```python
version = client.artifacts.versions.list(artifact_id)[0]
client.artifacts.versions.download(version.id, path="report.pdf")
```
`````

## PRs with no user-visible change

Add an empty file named `<PR number>.chore.md`. It is never rendered. A release made only of
`chore` fragments is rejected.

## No PR number yet

While you draft, you can name the file `+<short-name>.<type>.md`. Before the pull request merges, rename it to `<PR number>.<type>.md`. The changelog check requires that each fragment a pull request adds uses that pull request's number, so the entry links the right pull request.

## Release

Run the Python Release Prep workflow. It computes the next version from the current version in
`pyproject.toml` and the fragment types, sets it in `pyproject.toml` and `uv.lock`, renders
`CHANGELOG.md`, and opens a pull request. After that PR merges, the Python Release Tag workflow
creates the `python/vX.Y.Z` tag and starts the publish workflow.

If another change to `python/` merges while the release pull request is open, that change would
ship without its entry. The changelog check fails the release pull request in that case. Run
Python Release Prep again. For the same version, it replaces the release branch and updates the
open pull request. If the new changes raise the version, it opens a new pull request, so close the
old one. If the release pull request merges anyway, Python Release Tag
refuses to tag. Move the leftover entries into the new version's section of `CHANGELOG.md`, delete
their fragments, and merge that change to release.
