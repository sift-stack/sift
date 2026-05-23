# ensure uv.lock is up-to-date with pyproject.toml

# Clear git env vars set by the parent hook so git commands resolve the work tree normally
unset GIT_DIR GIT_WORK_TREE GIT_INDEX_FILE GIT_PREFIX

REPO_ROOT="$(git rev-parse --show-toplevel)"
PYTHON_DIR="$REPO_ROOT/python"

echo "     → Checking uv.lock..."
cd "$PYTHON_DIR"

if ! uv lock --check; then
    echo "     ❌ ERROR: uv.lock is out of date with pyproject.toml."
    echo "     Run 'uv lock' and commit the result before pushing."
    exit 1
fi

echo "     ✓ uv.lock is up-to-date"
