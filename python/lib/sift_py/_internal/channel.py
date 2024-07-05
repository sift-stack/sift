from typing import Optional


def channel_fqn(name: str, component: Optional[str]) -> str:
    if component is None or len(component) == 0:
        return name
    else:
        return f"{component}.{name}"
