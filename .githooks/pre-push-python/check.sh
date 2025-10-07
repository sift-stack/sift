#!/bin/bash
set -e

# Run all Python development checks
# Calls the existing ./scripts/dev check command

echo "Running Python development checks..."

REPO_ROOT="$(git rev-parse --show-toplevel)"
PYTHON_DIR="$REPO_ROOT/python"

cd "$PYTHON_DIR"

# Run all checks using the existing dev script
echo "Running python check..."
./scripts/dev lint