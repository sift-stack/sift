"""
Generate .pyi stub files for sync API classes created with generate_sync_api.

This script:
1. Imports all modules in sift_client.resources to ensure all @generate_sync_api decorators run
2. Collects all registered (async_cls, sync_cls) pairs from sync_wrapper._registered
3. Generates .pyi stubs in a dedicated sync_stubs directory next to resources
"""

import inspect
import importlib
import pathlib
import sys
from typing import Type, Any

# Add the parent directory to sys.path so we can import sift_client
script_dir = pathlib.Path(__file__).parent.parent.parent.parent
sys.path.insert(0, str(script_dir))

# Import the registration list
from sift_client._internal.sync_wrapper import _registered

# Place stubs in a dedicated sync_stubs directory next to resources
STUB_DIR = pathlib.Path(__file__).parent.parent / "resources" / "sync_stubs"


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
            lines.append(f"        ...")
        else:
            lines.append(f"    def {meth_name}(self, {params_str}){ret}: ...")
            
    return "\n".join(lines)


def import_all_resources():
    """Import all modules in sift_client.resources to ensure all decorators run."""
    try:
        import sift_client.resources
        print(f"Imported sift_client.resources")
    except ImportError as e:
        print(f"Error importing sift_client.resources: {e}")
        return
    
    # Import all submodules to ensure decorators run
    import pkgutil
    for _, name, _ in pkgutil.iter_modules(sift_client.resources.__path__, 
                                          sift_client.resources.__name__ + '.'):
        try:
            importlib.import_module(name)
            print(f"Imported {name}")
        except ImportError as e:
            print(f"Error importing {name}: {e}")


def main():
    """Generate stubs for all registered sync classes."""
    # Ensure stub directory exists
    STUB_DIR.mkdir(exist_ok=True, parents=True)
    
    # Import all resources to populate _registered
    import_all_resources()
    
    # Group by module
    module_stubs = {}
    for async_cls, sync_cls in _registered:
        module = async_cls.__module__
        if module not in module_stubs:
            module_stubs[module] = []
        module_stubs[module].append((async_cls, sync_cls))
    
    # Generate stubs by module
    for module, pairs in module_stubs.items():
        # Get module name for stub file
        module_parts = module.split('.')
        stub_name = module_parts[-1] + ".pyi"
        stub_file = STUB_DIR / stub_name
        
        # Generate content
        content = f"# Generated stubs for {module}\n\n"
        
        # Import everything from the original module
        content += f"from {module} import *\n\n"
        
        # Add class stubs
        for async_cls, sync_cls in pairs:
            # Only add the sync class stub
            content += make_stub_block(async_cls, sync_cls) + "\n\n"
        
        # Write stub file
        stub_file.write_text(content)
        print(f"Generated stub file: {stub_file}")


if __name__ == "__main__":
    main()
