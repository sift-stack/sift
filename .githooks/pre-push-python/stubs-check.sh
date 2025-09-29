#!/bin/bash
set -e


# ensure generated python stubs are up-to-date, from sync clients and sift_stream_bindings

REPO_ROOT="$(git rev-parse --show-toplevel)"
PYTHON_DIR="$REPO_ROOT/python"
BINDINGS_DIR="$REPO_ROOT/rust/crates/sift_stream_bindings"
STUBS_DIR="$PYTHON_DIR/lib/sift_client/resources/sync_stubs"

# Function to check if generated stub files have changed
check_stub_changes() {
    local target_path="$1"
    local changed_files=$(git status --porcelain "$target_path" | grep -E '\.pyi$' || true)

    if [ -n "$changed_files" ]; then
        echo "ERROR: Generated python stubs are not up-to-date. Please commit the changed files:"
        echo "$changed_files"
        exit 1
    fi
}

# Function to generate Python stubs
generate_python_stubs() {
    echo "Generating Python stubs..."
    cd "$PYTHON_DIR"

    if [[ ! -d "$PYTHON_DIR/venv" ]]; then
        echo "Running bootstrap script..."
        bash ./scripts/dev bootstrap
    fi

    bash ./scripts/dev gen-stubs
    check_stub_changes "$STUBS_DIR"
}

# Function to generate bindings stubs
generate_bindings_stubs() {
    echo "Generating bindings stubs..."
    cd "$BINDINGS_DIR"
    cargo run --bin stub_gen

    # The stub file is generated in the bindings directory
    local stub_file="$BINDINGS_DIR/sift_stream_bindings.pyi"
    check_stub_changes "$stub_file"
}

# Check for changes in relevant files
python_changed_files=($(git diff --name-only --diff-filter=ACM | grep '^python/lib/sift_client/' || true))
bindings_changed_files=($(git diff --name-only --diff-filter=ACM | grep '^rust/crates/sift_stream_bindings/src/' || true))

# Generate stubs if needed
if [[ -n "$python_changed_files" ]]; then
    generate_python_stubs
fi

if [[ -n "$bindings_changed_files" ]]; then
    generate_bindings_stubs
fi

echo "All stubs are up-to-date."