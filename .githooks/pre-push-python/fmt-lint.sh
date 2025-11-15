#!/usr/bin/env bash

set -e

# Store the root directory of the repository
REPO_ROOT="$(git rev-parse --show-toplevel)"
PYTHON_DIR="$REPO_ROOT/python"

# Change to Python directory
cd "$PYTHON_DIR"

# Run ruff format (formatter)
echo "     → Running ruff format..."
bash ./scripts/dev fmt

# Run ruff check with --fix (linter)
echo "     → Running ruff check --fix..."
bash ./scripts/dev lint-fix

# Check if any files were modified by formatting/linting
cd "$REPO_ROOT"
changed_files=$(git status --porcelain python/lib/sift_client/ | grep -E '\.py$' || true)

if [ -n "$changed_files" ]; then
    echo ""
    echo "     ❌ ERROR: Formatting/linting made changes:"
    echo "$changed_files" | sed 's/^/       /'
    echo ""
    echo "     Please commit these changes before pushing."
    exit 1
fi

echo "     ✓ Formatting and linting passed"
