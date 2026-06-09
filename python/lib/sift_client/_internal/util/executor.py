from __future__ import annotations

import asyncio
from typing import Any, Callable


async def run_sync_function(fn: Callable[..., Any], *args: Any) -> Any:
    """Run a synchronous function in a thread pool to avoid blocking the event loop."""
    loop = asyncio.get_running_loop()
    return await loop.run_in_executor(None, fn, *args)
