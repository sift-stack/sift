"""Test Results Utilities.

This module provides utilities for working with test results.

# Context Managers
- `ReportContext` - Context manager for a new TestReport.
- `NewStep` - Context manager to create a new step in a test report.

### Examples

```python
with ReportContext(client, name="Example Report", description="Example Report") as rc:
    with rc.new_step(name="Setup") as step:
        controller_setup(step)
    with rc.new_step(name="Example Step", description=desc) as parent_step:
        cmd_interface.cmd("ec1", "rtv.cmd", 75.0)
        sleep(0.01)

        with parent_step.substep(name="Substep 1", description="Measure position") as substep:
            ec = "ec1"
            pos_channel = "rtv.pos"
            pos = tlm.read(ec, pos_channel)
            result = substep.measure(pos, name=f"{ec}.{pos_channel}", bounds=(min=74.9, max=75.1))
            return result # This is optional for other uses, but the step and its parents will be updated correctly i.e. failed if the measurement fails.
```
#### Pytest Fixtures

The report context and steps can also be accessed in pytest via the `report_context` and `step` fixtures.
These fixtures are set to autouse and will automatically create a report and steps for each test function.
If you want each module(file) to be marked as a step w/ each test as a substep, import the `module_substep` fixture.

"""

from .context_manager import NewStep, ReportContext
from .pytest import report_context

__all__ = ["NewStep", "ReportContext", "report_context"]
