"""Test Results Utilities.

This module provides utilities for working with test results.

# Context Managers
- `NewStep` - Context manager to create a new step in a test report.
- `ReportContext` - Context for a new TestReport. Mostly serves as a store to communicate between step context managers since they can be nested or siblings.

### Examples

```python
rc = ReportContext.create(client, name="Example Report", description="Example Report")

with rc.new_step(name="Setup") as step:
    controller_setup(step)
with rc.new_step(name="Example Step", description=desc) as parent_step:
    cmd_interface.cmd("ec1", "rtv.cmd", 75.0)
    sleep(0.01)

    with rc.new_step(name="Substep 1", description="Measure position") as step:
        ec = "ec1"
        pos_channel = "rtv.pos"
        pos = tlm.read(ec, pos_channel)
        success = step.measure(pos, name=f"{ec}.{pos_channel}", bounds=(min=74.9, max=75.1))
        return success # This is optional for other uses, but the step and its parents will be updated correctly i.e. failed if the measurement fails.
```
"""

from .context_manager import NewStep, ReportContext
from .pytest import report_context

__all__ = ["NewStep", "ReportContext", "report_context"]
