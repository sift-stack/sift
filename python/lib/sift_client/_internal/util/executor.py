from __future__ import annotations

import asyncio
from typing import TYPE_CHECKING, Any, Callable

if TYPE_CHECKING:
    from concurrent.futures import Executor


async def run_sync_function(
    fn: Callable[..., Any], *args: Any, executor: Executor | None = None
) -> Any:
    """Run a synchronous function in a thread pool to avoid blocking the event loop.

    Pass ``executor`` to run on a dedicated pool instead of the loop's default
    one, isolating slow or blocking work from other offloaded calls.
    """
    loop = asyncio.get_running_loop()
    return await loop.run_in_executor(executor, fn, *args)
