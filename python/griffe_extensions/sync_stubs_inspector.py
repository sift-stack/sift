import griffe

logger = griffe.get_logger("griffe_inspect_specific_objects")


class InspectSpecificObjects(griffe.Extension):
    """Only inspect specific objects (such as ones with stubs)"""

    def __init__(self, paths: list[str]) -> None:
        self.objects = paths

    def on_instance(self, *, obj: griffe.Object, **kwargs) -> None:
        if obj.path not in self.objects:
            return

        # Skip over the stub files themselves
        if str(obj.filepath).endswith(".pyi"):
            return
        # Load the stub file instead of importing the .py
        inspected_module = griffe.inspect(obj.module.path, filepath=obj.filepath)
        obj.parent.set_member(obj.name, inspected_module[obj.name])
