from typing import Optional


def channel_fqn(name: str, component: Optional[str]) -> str:
    return name if component is None or len(component) == 0 else f"{component}.{name}"
