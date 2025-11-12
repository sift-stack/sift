# ensure generated pyproject.toml extras are up-to-date

# Store the root directory of the repository
REPO_ROOT="$(git rev-parse --show-toplevel)"
PYTHON_DIR="$REPO_ROOT/python"
PYPROJECT_FILE="$PYTHON_DIR/pyproject.toml"

# Function to check if pyproject.toml has changed
check_extras_changes() {
    local target_path="$1"
    local changed_files=$(git status --porcelain "$target_path" || true)

    if [ -n "$changed_files" ]; then
        echo "❌ ERROR: Generated pyproject.toml extras are not up-to-date. Please commit the changed files:"
        echo "$changed_files"
        exit 1
    fi
}

# Function to generate Python extras
generate_python_extras() {
    echo "Generating Python extras..."
    cd "$PYTHON_DIR"

    if [[ ! -d "$PYTHON_DIR/venv" ]]; then
        echo "Running bootstrap script..."
        bash ./scripts/dev bootstrap
    fi

    bash ./scripts/dev gen-extras
    check_extras_changes "$PYPROJECT_FILE"
}

generate_python_extras

echo "All extras are up-to-date."
