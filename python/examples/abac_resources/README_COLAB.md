# Using ABAC Resources Script in Google Colab

## Why Colab Can't Import the Package

The package name is **`sift-stack-py`** (not `sift-py`), and it's published to PyPI. However, the new proto files for `user_attributes`, `resource_attribute`, and `policies` need to be:
1. Generated (run `./scripts/gen.sh python`)
2. Committed to the repository
3. Published to PyPI (or available in the GitHub repo)

## Option 1: Install from GitHub Branch (Recommended)

Install directly from the branch with the ABAC proto files:

```python
# Install the package from the specific branch
!pip install git+https://github.com/sift-stack/sift.git@eng-000-abac-test-protos#subdirectory=python
```

Or from the main branch (once merged):

```python
# Install from main branch
!pip install git+https://github.com/sift-stack/sift.git@main#subdirectory=python
```

**Note**: This will only work if:
- The proto files have been generated and committed to the repo
- You have access to the repository (it's public or you have credentials)

## Option 2: Install from PyPI

```python
!pip install sift-stack-py
```

**Note**: This will only work if the proto files for user_attributes, resource_attribute, and policies have been generated and included in the published package.

## Option 3: Standalone Version (No Package Required)

If you can't install from GitHub or PyPI, you can use a standalone version that doesn't require `sift_py`. 
This version uses plain gRPC with minimal dependencies - you only need `grpcio` and `protobuf` from pip.

## Installation Troubleshooting

### If GitHub install fails:
1. **Private repository**: Make sure you have access to the repo, or use a personal access token:
   ```python
   !pip install git+https://<token>@github.com/sift-stack/sift.git@main#subdirectory=python
   ```

2. **Missing generated files**: The proto files need to be generated and committed to the repo first:
   ```bash
   cd /path/to/sift
   ./scripts/gen.sh python
   git add python/lib/sift/user_attributes python/lib/sift/resource_attribute python/lib/sift/policies
   git commit -m "Add generated proto files for ABAC resources"
   git push
   ```
   Then try installing again from GitHub.

3. **Wrong branch**: Make sure you're installing from the correct branch (usually `main` or `master`).

4. **Wrong package name**: The package is `sift-stack-py`, not `sift-py`:
   ```python
   # Correct
   !pip install sift-stack-py
   
   # Wrong
   !pip install sift-py  # This won't work
   ```

### Alternative: Use Standalone Script

If installation is problematic, use `main_standalone.py` which includes minimal gRPC channel setup code directly in the file, requiring only:
- `grpcio` (installable via pip)
- `protobuf` (installable via pip)
- The generated proto files (which you can copy directly)

