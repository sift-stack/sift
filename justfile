# Repo-wide developer tasks. Language-specific tasks live in each language's
# own justfile, exposed here as modules, e.g. `just python::test`.
#
# Requires `just` (https://github.com/casey/just). Install with one of:
#   uv tool install rust-just      # via uv
#   cargo install just             # via cargo
#   brew install just              # via Homebrew
#
# Git hooks are managed by pre-commit (.pre-commit-config.yaml); install them
# with `just install-hooks`.
#
# Run `just` (or `just --list`) to see all recipes.

mod python

# List available recipes
default:
    @just --list

# Install the pre-commit git hooks (commit + push stages)
install-hooks:
    # Clear any core.hooksPath left by the old .githooks setup; pre-commit
    # refuses to install while it is set.
    git config --unset-all core.hooksPath || true
    cd python && uv run pre-commit install --install-hooks --config ../.pre-commit-config.yaml

# Generate code from protobuf for all languages, or one: `just gen python`
gen *lang:
    bash scripts/gen.sh {{lang}}

# Serve the CLI mdBook docs locally
serve-cli-docs:
    mdbook serve ./rust/crates/sift_cli/assets/docs -d ./rust/crates/sift_cli/assets/docs/book-dev

# Build the CLI mdBook docs
build-cli-docs:
    mdbook build ./rust/crates/sift_cli/assets/docs -d ./rust/crates/sift_cli/assets/docs/book-dev

# Verify sift_stream_bindings' generated .pyi stub is up to date (pre-push hook)
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
