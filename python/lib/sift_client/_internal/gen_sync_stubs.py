"""
Generate .pyi stub files for sync API classes created with generate_sync_api.

This script:
1. Imports all modules in sift_client.resources to ensure all @generate_sync_api decorators run
2. Collects all registered (async_cls, sync_cls) pairs from sync_wrapper._registered
3. Generates .pyi stubs in the specified target directory, including all imports from the original file
"""

import ast
import importlib
import inspect
import pathlib
import sys
from typing import Any, Dict, List, Type

# Add the parent directory to sys.path so we can import sift_client
script_dir = pathlib.Path(__file__).parent.parent.parent.parent
sys.path.insert(0, str(script_dir))

# Import the registration list
from sift_client._internal.sync_wrapper import _registered


def make_stub_block(async_cls: Type[Any], cls: Type[Any]) -> str:
    """Returns the `.pyi` text for one class."""
    name = cls.__name__
    doc = cls.__doc__ or ""
    lines = [f"class {name}:", f'    """{doc}"""', ""]

    # constructor
    sig = inspect.signature(async_cls.__init__)
    params = list(sig.parameters.values())[1:]  # drop 'self'
    params_str = ", ".join(str(p) for p in params)
    lines.append(f"    def __init__(self, {params_str}) -> None: ...")

    # methods
    for meth_name, method in inspect.getmembers(async_cls, inspect.iscoroutinefunction):
        if meth_name.startswith("_"):
            continue
        sig = inspect.signature(method)
        params = list(sig.parameters.values())[1:]  # drop 'self'
        params_str = ", ".join(str(p) for p in params)
        ret = ""
        if sig.return_annotation is not inspect._empty:
            ret = f" -> {sig.return_annotation}"

        # Include docstring if available
        if method.__doc__:
            lines.append(f"    def {meth_name}(self, {params_str}){ret}:")
            lines.append(f'        """{method.__doc__.strip()}"""')
            lines.append("        ...")
        else:
            lines.append(f"    def {meth_name}(self, {params_str}){ret}: ...")

    return "\n".join(lines)


def extract_imports(file_path: pathlib.Path) -> List[str]:
    """Extract import statements from a Python file."""
    with open(file_path, "r") as f:
        content = f.read()

    try:
        tree = ast.parse(content)
        import_lines = []

        for node in ast.iter_child_nodes(tree):
            if isinstance(node, (ast.Import, ast.ImportFrom)):
                # Get the line from the source
                start_line = node.lineno - 1
                end_line = node.end_lineno if hasattr(node, "end_lineno") else start_line

                # Extract the import statement from the source
                lines = content.splitlines()[start_line : end_line + 1]
                import_statement = "\n".join(lines)
                import_lines.append(import_statement)

        return import_lines
    except SyntaxError:
        # Fallback to simple parsing if AST parsing fails
        lines = content.splitlines()
        import_lines = []

        for line in lines:
            line = line.strip()
            if line.startswith("import ") or line.startswith("from "):
                import_lines.append(line)

        return import_lines


def import_all_resources():
    """Import all modules in sift_client.resources to ensure all decorators run."""
    try:
        import sift_client.resources

        print("Imported sift_client.resources")
    except ImportError as e:
        print(f"Error importing sift_client.resources: {e}")
        return

    # Import all submodules to ensure decorators run
    import pkgutil

    for _, name, _ in pkgutil.iter_modules(
        sift_client.resources.__path__, sift_client.resources.__name__ + "."
    ):
        try:
            importlib.import_module(name)
            print(f"Imported {name}")
        except ImportError as e:
            print(f"Error importing {name}: {e}")


def generate_stubs(target_dir):
    """Generate stubs for all registered sync classes in the target directory."""
    # Ensure target directory exists
    target_dir = pathlib.Path(target_dir)
    target_dir.mkdir(exist_ok=True, parents=True)

    # Import all resources to populate _registered
    import_all_resources()

    # Generate __init__.pyi in the target directory
    init_pyi = target_dir / "__init__.pyi"
    init_py = target_dir / "__init__.py"

    # Extract imports from the original file
    imports = []
    if init_py.exists():
        imports = extract_imports(init_py)

    # Generate content
    content = "# Generated stubs for sync API classes\n\n"

    # Add imports from the original file
    for import_line in imports:
        content += f"{import_line}\n"

    if imports:
        content += "\n"

    # Add class stubs
    for async_cls, sync_cls in _registered:
        # Only add the sync class stub
        content += make_stub_block(async_cls, sync_cls) + "\n\n"

    # Write stub file
    init_pyi.write_text(content)
    print(f"Generated stub file: {init_pyi}")


def main():
    """Generate stubs for all registered sync classes."""
    import argparse

    parser = argparse.ArgumentParser(description="Generate .pyi stub files for sync API classes")
    parser.add_argument("-t", "--target", required=True, help="Target directory for stub files")

    args = parser.parse_args()
    generate_stubs(args.target)


if __name__ == "__main__":
    main()
