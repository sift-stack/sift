# ensure generated python stubs are up-to-date, from sync clients

# Store the root directory of the repository
REPO_ROOT="$(git rev-parse --show-toplevel)"
PYTHON_DIR="$REPO_ROOT/python"
STUBS_DIR="$PYTHON_DIR/lib/sift_client/resources/sync_stubs"

# Function to check if generated stub files have changed
check_stub_changes() {
    local target_path="$1"
    local changed_files=$(git status --porcelain "$target_path" | grep -E '\.pyi$' || true)

    if [ -n "$changed_files" ]; then
        echo "     ✗ ERROR: Generated stubs are not up-to-date:"
        echo "$changed_files" | sed 's/^/       /'
        echo "     Please commit these changes before pushing."
        exit 1
    fi
}

# Function to generate Python stubs
generate_python_stubs() {
    echo "     → Generating stubs..."
    cd "$PYTHON_DIR"

    if [[ ! -d "$PYTHON_DIR/venv" ]]; then
        echo "     → Running bootstrap script..."
        bash ./scripts/dev bootstrap
    fi

    bash ./scripts/dev gen-stubs
    check_stub_changes "$STUBS_DIR"
}

generate_python_stubs

echo "     ✓ Stubs are up-to-date"