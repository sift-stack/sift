# Repo-wide developer tasks. Language-specific tasks live in each language's
# own justfile, exposed here as modules, e.g. `just python::test`.
#
# Requires `just` (https://github.com/casey/just). Install with one of:
#   uv tool install rust-just      # via uv
#   cargo install just             # via cargo
#   brew install just              # via Homebrew
#
# Run `just` (or `just --list`) to see all recipes.

mod python

# List available recipes
default:
    @just --list

# Generate code from protobuf for all languages, or one: `just gen python`
gen *lang:
    bash scripts/gen.sh {{lang}}

# Point git at the repo's managed hooks (.githooks)
git-hooks:
    git config core.hooksPath .githooks

# Serve the CLI mdBook docs locally
serve-cli-docs:
    mdbook serve ./rust/crates/sift_cli/assets/docs -d ./rust/crates/sift_cli/assets/docs/book-dev

# Build the CLI mdBook docs
build-cli-docs:
    mdbook build ./rust/crates/sift_cli/assets/docs -d ./rust/crates/sift_cli/assets/docs/book-dev

# --- git hooks: invoked by .githooks/*, enabled via `just git-hooks` ---

# pre-commit: lint and format staged protos with buf
pre-commit:
    #!/usr/bin/env bash
    set -uo pipefail
    if ! command -v buf >/dev/null; then
      echo "Could not find 'buf' executable. Ensure that it is installed in your path."
      exit 2
    fi
    changed=$(git diff --cached --name-only --diff-filter=ACM | grep '^protos/' | grep '\.proto$' || true)
    if [[ -n "$changed" ]]; then
      echo "running buf lint protos"
      buf lint protos || { echo "ERROR! protos linting failed. Please fix the errors and recommit."; exit 1; }
      echo "running buf format protos"
      buf format protos -w --exit-code || { echo "ERROR! protos files were reformatted. Please stage the changes and recommit."; exit 1; }
    fi

# pre-push: run checks for the areas that changed vs the upstream branch
pre-push:
    #!/usr/bin/env bash
    set -uo pipefail
    echo "========================================"
    echo "  Pre-Push Checks"
    echo "========================================"
    failed=0

    py_changed=$(git diff --name-only --diff-filter=ACM @{upstream}... | grep '^python/' || true)
    echo "→ Python files changed:"
    if [[ -n "$py_changed" ]]; then
      echo "  ${py_changed//$'\n'/ }"
      echo "  [1/4] Formatting and linting..."; just python::fmt-lint-check || failed=1
      echo "  [2/4] Stub generation...";        just python::stubs-check   || failed=1
      echo "  [3/4] Extras validation...";      just python::extras-check  || failed=1
      echo "  [4/4] Lockfile check...";         just python::lock-check    || failed=1
    else
      echo "  (none)"
    fi

    bindings_changed=$(git diff --name-only --diff-filter=ACM @{upstream}... | grep '^rust/crates/sift_stream_bindings/src/' || true)
    echo "→ Rust binding files changed:"
    if [[ -n "$bindings_changed" ]]; then
      echo "  ${bindings_changed//$'\n'/ }"
      echo "  [1/1] Stub generation..."; just bindings-stubs-check || failed=1
    else
      echo "  (none)"
    fi

    if [[ $failed -eq 1 ]]; then
      echo "  ✗ Some checks failed"
      exit 1
    fi
    echo "  ✓ All checks passed"

# Verify sift_stream_bindings' generated .pyi stub is up to date
bindings-stubs-check:
    #!/usr/bin/env bash
    set -uo pipefail
    cd rust/crates/sift_stream_bindings
    cargo run --bin stub_gen
    changed=$(git status --porcelain sift_stream_bindings.pyi | grep -E '\.pyi$' || true)
    if [[ -n "$changed" ]]; then
      echo "ERROR: generated stubs are not up to date:"
      echo "$changed" | sed 's/^/  /'
      echo "Commit these changes before pushing."
      exit 1
    fi
