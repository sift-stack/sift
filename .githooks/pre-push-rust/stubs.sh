# ensure generated python stubs are up-to-date, from sift_stream_bindings

# Store the root directory of the repository
REPO_ROOT="$(git rev-parse --show-toplevel)"
BINDINGS_DIR="$REPO_ROOT/rust/crates/sift_stream_bindings"

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

# Function to generate bindings stubs
generate_bindings_stubs() {
    echo "Generating bindings stubs..."
    cd "$BINDINGS_DIR"
    cargo run --bin stub_gen

    # The stub file is generated in the bindings directory
    local stub_file="$BINDINGS_DIR/sift_stream_bindings.pyi"
    check_stub_changes "$stub_file"
}

generate_bindings_stubs

echo "All stubs are up-to-date."