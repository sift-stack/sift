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

Write the entry for a user of the library. Start with what changed, in one sentence. The
renderer adds the PR link. A fragment can have more paragraphs and code blocks after the first
sentence. They are indented under the bullet.

## PRs with no user-visible change

Add an empty file named `<PR number>.chore.md`. It is never rendered. A release made only of
`chore` fragments is rejected.

## No PR number yet

Name the file `+<short-name>.<type>.md`. The entry has no PR link.

## Release

Run the Python Release Prep workflow. It computes the next version from the current version in
`pyproject.toml` and the fragment types, sets it in `pyproject.toml` and `uv.lock`, renders
`CHANGELOG.md`, and opens a pull request. After that PR merges, the Python Release Tag workflow
creates the `python/vX.Y.Z` tag and starts the publish workflow.

If another change to `python/` merges while the release pull request is open, that change would
ship without its entry. The changelog check fails the release pull request in that case, and the
publish workflow refuses a tag with leftover fragments. Close the release pull request and run
Python Release Prep again.
