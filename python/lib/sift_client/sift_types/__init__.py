"""Sift Types - Pydantic models for Sift resources.

This module provides strongly-typed Pydantic models for interacting with Sift resources.
These models are used throughout the Sift client to provide type safety, validation,
and convenient methods for working with Sift objects.

## Resource BaseTypes

Resource BaseTypes are immutable Pydantic models that represent specific Sift objects
retrieved from the API. They provide:

- **Type-safe access** to all resource properties
- **Convenience methods** for common operations (update, archive, etc.)
- **Related resource access** via properties (e.g., `asset.runs`, `run.assets`)
- **Rich integration** with IDEs for autocomplete and type checking

### Available Resource Types

- `Asset` - Physical or logical entities (vehicles, machines, devices)
- `Run` - Time-bounded operational periods for an asset
- `Channel` - Time-series data streams (sensor readings, telemetry)
-  etc.


## Create and Update Types

Create and Update types are Pydantic models used to create new resources or modify
existing ones. They can be used directly as typed objects or as dictionaries for
convenience.

### Create Types

Create types define the required and optional fields for creating new resources. For example:

- `RunCreate` - Create a new run
- `CalculatedChannelCreate` - Create a new calculated channel
- `RuleCreate` - Create a new rule
- etc.

### Update Types

Update types define which fields can be modified on existing resources. For example:

- `AssetUpdate` - Update asset properties
- `RunUpdate` - Update run properties
- `CalculatedChannelUpdate` - Update calculated channel properties
- etc.

### Example Usage

```python
from sift_client import SiftClient
from sift_client.sift_types import RunCreate, AssetUpdate

client = SiftClient(api_key="...", grpc_url="...", rest_url="...")

# Using Create types - typed approach
run = client.runs.create(
    RunCreate(
        name="Test Run",
        description="A test run",
        asset_ids=["asset123"],
        start_time=datetime.now(),
    )
)

# Using Create types - dict approach (more convenient)
run = client.runs.create({
    "name": "Test Run",
    "description": "A test run",
    "asset_ids": ["asset123"],
    "start_time": datetime.now(),
})

# Using Update types - typed approach
asset = client.assets.update(
    asset="asset123",
    update=AssetUpdate(tags=["production", "v2"])
)

# Using Update types - dict approach (more convenient)
asset = client.assets.update(
    asset="asset123",
    update={"tags": ["production", "v2"]}
)

# Using convenience methods on resource instances
asset.update({"tags": ["production", "v3"]})
```

## Helper Types

Additional types are provided for specific use cases. For example:

- `ChannelReference` - Reference to a channel in expressions
- `ChannelDataType` - Enum for channel data types
- `ChannelBitFieldElement` - Bit field element definition
- `RuleActionType` - Enum for rule action types
- `RuleAnnotationType` - Enum for annotation types
- `ChannelConfig` - Configuration for data ingestion channels
- `Flow` - Data flow configuration for ingestion
- `IngestionConfig` - Complete ingestion configuration
- etc.

## Type Validation

All types use Pydantic for validation, ensuring:

- **Required fields** are present
- **Field types** are correct
- **Datetime fields** have timezone information
- **Enum values** are valid

Validation errors are raised immediately with clear error messages.

## Immutability

Resource BaseTypes (Asset, Run, etc.) are immutable by default. To update a resource,
use the `update()` method, which will update the instance in-place by replacing its
internal state with the updated values from the API.

```python
asset = client.assets.get(asset_id="asset123")
# This will raise an error - assets are immutable
# asset.name = "New Name"

# Instead, use the update method
asset.update({"tags": ["new-tag"]})  # Updates the instance in-place
```
"""

import sys

from sift_client.sift_types.asset import Asset, AssetUpdate
from sift_client.sift_types.calculated_channel import (
    CalculatedChannel,
    CalculatedChannelCreate,
    CalculatedChannelUpdate,
)
from sift_client.sift_types.channel import (
    Channel,
    ChannelBitFieldElement,
    ChannelDataType,
    ChannelReference,
)
from sift_client.sift_types.ingestion import (
    ChannelConfig,
    Flow,
    FlowConfig,
    IngestionConfig,
    IngestionConfigCreate,
)
from sift_client.sift_types.job import (
    DataExportDetails,
    DataExportStatusDetails,
    DataImportDetails,
    DataImportStatusDetails,
    Job,
    JobDetails,
    JobStatus,
    JobStatusDetails,
    JobType,
    RuleEvaluationDetails,
    RuleEvaluationStatusDetails,
)
from sift_client.sift_types.report import Report, ReportRuleStatus, ReportRuleSummary, ReportUpdate
from sift_client.sift_types.rule import (
    Rule,
    RuleAction,
    RuleActionType,
    RuleAnnotationType,
    RuleCreate,
    RuleUpdate,
    RuleVersion,
)
from sift_client.sift_types.run import Run, RunCreate, RunUpdate
from sift_client.sift_types.tag import Tag, TagCreate, TagUpdate
from sift_client.sift_types.test_report import (
    TestMeasurement,
    TestMeasurementCreate,
    TestMeasurementType,
    TestMeasurementUpdate,
    TestReport,
    TestReportCreate,
    TestReportUpdate,
    TestStatus,
    TestStep,
    TestStepCreate,
    TestStepType,
)

if "pytest" in sys.modules:
    # These are not test classes, so we need to set __test__ to False to avoid pytest warnings.
    # Do this here because for some reason our docs generation doesn't like it when done in the classes themselves.
    TestStepType.__test__ = False  # type: ignore
    TestMeasurementType.__test__ = False  # type: ignore
    TestMeasurement.__test__ = False  # type: ignore
    TestMeasurementCreate.__test__ = False  # type: ignore
    TestMeasurementUpdate.__test__ = False  # type: ignore
    TestStatus.__test__ = False  # type: ignore
    TestStep.__test__ = False  # type: ignore
    TestStepCreate.__test__ = False  # type: ignore
    TestReport.__test__ = False  # type: ignore
    TestReportCreate.__test__ = False  # type: ignore
    TestReportUpdate.__test__ = False  # type: ignore

__all__ = [
    "Asset",
    "AssetUpdate",
    "CalculatedChannel",
    "CalculatedChannelCreate",
    "CalculatedChannelUpdate",
    "Channel",
    "ChannelBitFieldElement",
    "ChannelConfig",
    "ChannelDataType",
    "ChannelReference",
    "DataExportDetails",
    "DataExportStatusDetails",
    "DataImportDetails",
    "DataImportStatusDetails",
    "Flow",
    "FlowConfig",
    "IngestionConfig",
    "IngestionConfigCreate",
    "Job",
    "JobDetails",
    "JobStatus",
    "JobStatusDetails",
    "JobType",
    "Report",
    "ReportRuleStatus",
    "ReportRuleSummary",
    "ReportUpdate",
    "Rule",
    "RuleAction",
    "RuleActionType",
    "RuleAnnotationType",
    "RuleCreate",
    "RuleEvaluationDetails",
    "RuleEvaluationStatusDetails",
    "RuleUpdate",
    "RuleVersion",
    "Run",
    "RunCreate",
    "RunUpdate",
    "Tag",
    "TagCreate",
    "TagUpdate",
    "TestMeasurement",
    "TestMeasurementCreate",
    "TestMeasurementType",
    "TestReport",
    "TestReportCreate",
    "TestReportUpdate",
    "TestStatus",
    "TestStep",
    "TestStepCreate",
    "TestStepType",
]
