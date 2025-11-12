#!/usr/bin/env bash

set -e

# Store the root directory of the repository
REPO_ROOT="$(git rev-parse --show-toplevel)"
PYTHON_DIR="$REPO_ROOT/python"

echo "Running Python formatting and linting with --fix..."

# Change to Python directory
cd "$PYTHON_DIR"

# Run ruff format (formatter)
echo "Running ruff format..."
bash ./scripts/dev fmt

# Run ruff check with --fix (linter)
echo "Running ruff check --fix..."
bash ./scripts/dev lint-fix

# Check if any files were modified by formatting/linting
cd "$REPO_ROOT"
changed_files=$(git status --porcelain python/lib/sift_client/ | grep -E '\.py$' || true)

if [ -n "$changed_files" ]; then
    echo ""
    echo "❌ ERROR: Formatting/linting made changes to the following files:"
    echo "$changed_files"
    echo ""
    echo "Please commit these changes before pushing."
    exit 1
fi

echo "Python formatting and linting completed successfully."
