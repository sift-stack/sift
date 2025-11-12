import sys
from pathlib import Path

import tomlkit

pyproject = Path("../pyproject.toml")
if not pyproject.exists():
    sys.exit(f"❌ No pyproject.toml found at {pyproject.resolve()}")

# Parse preserving comments and formatting
doc = tomlkit.parse(pyproject.read_text())

try:
    tool_sift = doc["tool"]["sift"]["extras"]
except KeyError:
    sys.exit("❌ No [tool.sift.extras] section found in pyproject.toml")

# Split atomic and combined definitions
combine_section = tool_sift.get("combine", {})
atomic_extras = {k: v for k, v in tool_sift.items() if k != "combine"}

# Recursive resolver for nested combines
def resolve(name, stack=None):
    """Recursively resolve nested combine groups."""
    if stack is None:
        stack = []
    if name in stack:
        raise ValueError(f"Cyclic combine detected: {' -> '.join(stack + [name])}")
    if name in atomic_extras:
        return list(atomic_extras[name])
    if name in combine_section:
        deps = []
        for sub in combine_section[name]:
            deps.extend(resolve(sub, stack + [name]))
        return deps
    raise KeyError(f"Unknown group '{name}' referenced in combine")

# Build final extras dictionary
final_extras = {}
for name in list(atomic_extras) + list(combine_section):
    deps = resolve(name)
    final_extras[name] = sorted(set(deps))

# Inject into [project.optional-dependencies]
project = doc.setdefault("project", tomlkit.table())
project["optional-dependencies"] = tomlkit.table()


# Write arrays in sorted order with clean formatting
for name in sorted(final_extras):
    deps = final_extras[name]
    arr = tomlkit.array(deps)
    arr.multiline(True)           # each dependency on its own line
    # arr.indent(2)                 # 4-space indentation
    arr.as_string()
    # arr.trailing_comma(True)      # trailing commas for cleaner diffs
    project["optional-dependencies"][name] = arr

# Dump back to file
pyproject.write_text(tomlkit.dumps(doc))

print("✅ Updated [project.optional-dependencies] with formatted, nested extras")
