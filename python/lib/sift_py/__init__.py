"""
`sift_py` is a Python module built on top of Sift's protocol buffers to ergonomically interface with
Sift's gRPC API, especially with regard to data ingestion and and rule evaluation. If there are any
words or concepts that you find yourself needing to familiarize yourself with, be sure to visit the
official [Sift documentation](https://docs.siftstack.com/glossary).

* [Introduction](#introduction)
    - [Quickstart](#quickstart)
* [Telemetry Config](#telemetry-config)
    - [Telemetry Config from YAML](#telemetry-config-from-yaml)
    - [Telemetry Config YAML Schema](#telemetry-config-yaml-schema)
    - [Named Expression Modules](#named-expression-modules)
* [Ingestion Service](#ingestion-service)
    - [Sending data to Sift](#sending-data-to-sift)
* [More Examples](#more-examples)

## Introduction

The two fundamental components of this module are the following:
- `sift_py.ingestion.config.telemetry.TelemetryConfig` (telemetry config)
- `sift_py.ingestion.service.IngestionService` (ingestion service)

The telemetry config defines the schema of your telemetry. It is where you will declare your asset, channels and their components,
flows, and rules:
- `sift_py.ingestion.channel.ChannelConfig`
- `sift_py.ingestion.rule.config.RuleConfig`
- `sift_py.ingestion.flow.FlowConfig`

Once you have a telemetry config instantiated, you can then proceed to instantiate `sift_py.ingestion.service.IngestionService`
which is what's used to actually send Data to Sift.

### Quickstart

The following example demonstrates how to create a simple telemetry config for an asset with a single channel
and a single rule, afterwhich we'll send a single data point to Sift for that channel.

```python
from sift_py.grpc.transport import SiftChannelConfig, use_sift_channel
from sift_py.ingestion.channel import (
    ChannelBitFieldElement,
    ChannelConfig,
    ChannelDataType,
    ChannelEnumType,
    double_value
)
from sift_py.ingestion.service import IngestionService
from sift_py.ingestion.config.telemetry import FlowConfig, TelemetryConfig
from sift_py.ingestion.rule.config import (
    RuleActionCreateDataReviewAnnotation,
    RuleConfig,
)

# Create a channel config
temperature_channel = ChannelConfig(
    name="temperature",
    component="thruster",
    data_type=ChannelDataType.DOUBLE,
    description="temperature of thruster",
    unit="Kelvin",
)

# Create a rule config referencing the above channel
overheating_rule = RuleConfig(
    name="overheating",
    description="Notify Ripley if thrusters get too hot",
    expression='$1 > 400',
    channel_references=[
        {
            "channel_reference": "$1",
            "channel_config": temperature_channel,
        },
    ],
    action=RuleActionCreateDataReviewAnnotation(
        assignee="ellen.ripley@weylandcorp.com",
        tags=["warning", "thruster"],
    ),
),

# Creating the telemetry config using the rules and channels
# described above
telemetry_config = TelemetryConfig(
    asset_name="NostromoLV426",
    ingestion_client_key="nostromo_lv_426",
    rules=[overheating_rule],
    flows=[
        FlowConfig(
            name="temperature_reading",
            channels=[temperature_channel],
        ),
    ],
)


# Create a gRPC transport channel configured specifically for the Sift API
sift_channel_config = SiftChannelConfig(uri=SIFT_BASE_URI, apikey=SIFT_API_KEY)

with use_sift_channel(sift_channel_config) as channel:
    # Create ingestion service using the telemetry config we just created
    ingestion_service = IngestionService(
        channel,
        telemetry_config,
        # Overwrite any rules created in the Sift UI that isn't in the config
        overwrite_rules=True,
        # End stream if errors occur API-side.
        end_stream_on_error=True,
    )

    # Create an ingestion request for the 'temperature_reading' flow
    temperature_reading = self.ingestion_service.try_create_ingestion_request(
        flow_name="temperature_reading",
        timestamp=timestamp,
        channel_values=[
            {
                "channel_name": "temperature",
                "component": "thruster",
                "value": double_value(327)
            },
        ],
    )

    # Send the data to Sift
    ingestion_service.ingest(temperature_reading)
```

## Telemetry Config

There are currently two methods with which to initialize a telemetry config:
- `sift_py.ingestion.config.telemetry.TelemetryConfig.__init__`
- `sift_py.ingestion.config.telemetry.TelemetryConfig.try_from_yaml`

Both are equally valid and your choice to use one or the other largely depends on you and your team's preferred
workflow. The following sections will cover each initialization method.

### Telemetry Config From Yaml

While the telemetry config can be declaratively initialized using using the telemetry config's initializer, `sift_py` also exposes an API
to initialize a telemetry config from a YAML file. The following is a simple demonstration.

Say that we had the following project structure:

```
 example
 ├─ telemetry_configs
 │  └─ nostromo_lv_426.yml
 ├─ main.py
 ├─ telemetry_config.py
 └─ requirements.txt
 ```

If our telemetry config is defined in the YAML file, `nostromo_lv_426.yml`, one of the ways in which
we might read that YAML file in as a `sift_py.ingestion.config.telemetry.TelemetryConfig` is to do the following:

```python
from pathlib import Path

TELEMETRY_CONFIGS_DIR = Path().joinpath("telemetry_configs")

def nostromos_lv_426() -> TelemetryConfig:
    telemetry_config_path = TELEMETRY_CONFIGS_DIR.joinpath("nostromo_lv_426.yml")
    return TelemetryConfig.try_from_yaml(telemetry_config_path)
```

As for the contents of the `nostromo_lv_426.yml`, file it might look something like this:

```yaml
asset_name: NostromoLV426
ingestion_client_key: nostromo_lv_426

channels:
  temperature_channel: &temperature_channel
    name: temperature
    component: thruster
    data_type: double
    description: temperature of the thruster
    unit: Kelvin

rules:
  - name: overheating
    description: Notify Ripley if thrusters get too hot
    expression: $1 > 400
    channel_references:
      - $1: *temperature_channel
    type: review
    assignee: ellen.ripley@weylandcorp.com
    tags:
        - warning
        - thruster

flows:
  - name: temperature_reading
    channels:
      - <<: *temperature_channel
```

And with the telemetry config that we just created, we can then proceed to create an ingestion service
and begin data ingestion.

### Telemetry Config YAML Schema

The following is the formal schema for a valid telemetry config in YAML. You can also see the `sift_py.ingestion.ingestion.config.yaml.spec` module
to see the schema in the for of Python classes.

```yaml
schema:
  description: |
    A formal specification to create a telemetry config which is used
    to stream data and evaluate rules using Sift's gRPC API.

  asset_name:
    type: string
    description: The name of the asset to telemeter.

  ingestion_client_key:
    type: string
    description: User-defined string-key that uniquely identifies this telemetry config.

  organization_id:
    type: string
    description: Optional ID of user's organization. Required if user belongs to multiple organizations.

  channels:
    type: array
    description: Sensors that send the data.
    items:
      type: object
      properties:
        name:
          type: string
          description: Name of the channel.
        description:
          type: string
          description: Description of the channel.
        unit:
          type: string
          description: Unit of measurement.
        component:
          type: string
          description: Name of the component that the channel belongs to.
        data_type:
          type: string
          enum: ["double", "string", "enum", "bit_field", "bool", "float", "int32", "int64", "uint32", "uint64"]
          description: Type of the data associated with the channel.
        enum_types:
          type: array
          items:
            type: object
            properties:
              name:
                type: string
                description: Name of the enum type.
              key:
                type: integer
                description: Key of the enum type.
          description: Required if `data_type` is `enum`.
        bit_field_elements:
          type: array
          description: Required if `data_type` is `bit_field`.
          items:
            type: object
            properties:
              name:
                type: string
                description: Name of the bit-field element.
              index:
                type: integer
                description: Index of the bit-field element.
              bit_count:
                type: integer
                description: Bit count of the bit-field element.

  rules:
    type: array
    description: Rules that, when evaluated to a true, will perform some sort of action.
    items:
      type: object
      properties:
        name:
          type: string
          description: Name of the rule.
        description:
          type: string
          description: Description of the rule.
        expression:
          oneOf:
            - type: string
              description: A string expression defining the rule logic.
            - type: object
              description: A reference to a named expression.
              properties:
                name:
                  type: string
                  description: Name of the named expression.
        type:
          type: string
          enum: [phase, review]
          description: Determines the action to perform if a rule gets evaluated to true.
        assignee:
          type: string
          description: If 'type' is 'review', determines who to notify. Expects an email.
        tags:
          type: array
          items:
            type: string
          description: Tags to associate with the rule.
        channel_references:
          type: array
          description: A list of channel references that map to an actual channel. Use YAML anchors to reference channels.
          items:
            type: object
            description: |
              Key-value pair of string to channel. The channel should be a YAML anchor to a previously declared channel
              in the top-level 'channels' property. The key should take the form of '$1', '$2', '$11', and do on. In YAML
              it would look something like this:

              ------------------------------------
              channel_references:
                - $1: *vehicle_state_channel
                - $2: *voltage_channel
              ------------------------------------
        sub_expressions:
          type: array
          description: A list of sub-expressions which is a mapping of place-holders to sub-expressions.
          items:
            type: object
            description: |
              A sub-expression is made up of two components: A reference and the actual sub-expression. The sub-expression reference is
              a string with a "$" prepended to another string comprised of characters in the following character set: `[a-zA-Z0-9_]`.
              This reference should be mapped to the actual sub-expression. For example, say you have kinematic equations in `kinematics.yml`,
              and the equation you're interested in using looks like the following:

              ------------------------------------
              kinetic_energy_gt:
                0.5 * $mass * $1 * $1 > $threshold
              ------------------------------------

              To properly use `kinetic_energy_gt` in your rule, it would look like the following:

              ------------------------------------
              rules:
                - name: kinetic_energy
                  description: Tracks high energy output while in motion
                  type: review
                  assignee: bob@example.com
                  expression:
                    name: kinetic_energy_gt
                  channel_references:
                    - $1: *velocity_channel
                  sub_expressions:
                    - $mass: 10
                    - $threshold: 470
                  tags:
                      - nostromo
              ------------------------------------
  flows:
    type: array
    description: A list of named groups of channels that send data together.
    items:
      type: object
      properties:
        name:
          type: string
          description: Name of the flow.
        channels:
          type: array
          items:
            type: object
            description: |
              List of channels included in the flow. Should be a YAML anchor from a previously declared channel
              in the top-level 'channels' property.
```

## Named Expression Modules

Often times you may find yourself needing to re-using more complex rule expressions across different telemetry
configs. If this is the case you might consider leveraging `named expressions` which allow you to reference the name
of an expression defined in another YAML file rather than defining it repeatedly across different telemetry configs.

As an example, say this is our current rule in our YAML telemetry config:

```yaml
rules:
  - name: kinetic_energy_gt
    description: Tracks high energy output while in motion
    type: review
    assignee: cthulhu@rlyeh.com
    expression: 0.5 * 10 * $1 * $1 > 470
    channel_references:
      - $1: *velocity_channel
```

Instead of repeatedly writing that kinetic energy expression across different telemetry configs, you can move that expression
over to it's own named expression module YAML file which we'll call `kinematics.yml`, and then reference it by name in the
telemetry configs:

`kinematics.yml`
```yaml
kinetic_energy_gt:
  0.5 * $mass * $1 * $1 > $threshold
rod_torque_gt:
  (1 / 12) * $mass * $rod_length * $rod_length * $1
```

`telemetry_config.py`
```yaml
rules:
  - name: kinetic_energy
    description: Tracks high energy output while in motion
    type: review
    expression:
      name: kinetic_energy_gt
    channel_references:
      - $1: *velocity_channel
    sub_expressions:
      - $mass: 10
      - $threshold: 470
```

In order for the telemetry configs to load in the named expression modules at run-time, all you need to do is provide the path
to the named expression module(s) wherever it may be. For example, given the following project structure:

```
 example
 ├─ telemetry_configs
 │  └─ nostromo_lv_426.yml
 ├─ main.py
 ├─ telemetry_config.py
 └─ expression_modules
    ├─ string.yml
    └─ kinematics.yml
```

Here is how we might load our telemetry config:

```python
from pathlib import Path

from sift_py.ingestion.service import TelemetryConfig

TELEMETRY_CONFIGS_DIR = Path().joinpath("telemetry_configs")
EXPRESSION_MODULES_DIR = Path().joinpath("expression_modules")


def nostromos_lv_426() -> TelemetryConfig:
    telemetry_config_path = TELEMETRY_CONFIGS_DIR.joinpath("nostromo_lv_426.yml")

    return TelemetryConfig.try_from_yaml(
        telemetry_config_path,
        [
            EXPRESSION_MODULES_DIR.joinpath("kinematics.yml"),
            EXPRESSION_MODULES_DIR.joinpath("string.yml"),
        ],
    )
```

## Ingestion Service

As mentioned previously, whereas a telemetry config defines the schema of your telemetry,
`sift_py.ingestion.service.IngestionService` is what's actually responsible for sending your data to Sift.

There are two methods currently available to generate data to send to Sift.
- `sift_py.ingestion.service.IngestionService.try_create_ingestion_request`
- `sift_py.ingestion.service.IngestionService.create_ingestion_request`

Visit the function definitions to understand the differences between each.

Once you have generated a request using either of those methods
data is then sent to Sift using `sift_py.ingestion.service.IngestionService.ingest`.
The following are some examples illustrating generating data ingestion requests and sending them to Sift.

### Sending Data to Sift

Suppose we have the following telemetry config with four configured instances of `sift_py.ingestion.flow.FlowConfig`.

```python
def nostromos_lv_426() -> TelemetryConfig:
    log_channel = ChannelConfig(
        name="log",
        data_type=ChannelDataType.STRING,
        description="asset logs",
    )
    velocity_channel = ChannelConfig(
        name="velocity",
        data_type=ChannelDataType.DOUBLE,
        description="speed",
        unit="Miles Per Hour",
        component="mainmotor",
    )
    voltage_channel = ChannelConfig(
        name="voltage",
        data_type=ChannelDataType.INT_32,
        description="voltage at source",
        unit="Volts",
    )
    vehicle_state_channel = ChannelConfig(
        name="vehicle_state",
        data_type=ChannelDataType.ENUM,
        description="vehicle state",
        enum_types=[
            ChannelEnumType(name="Accelerating", key=0),
            ChannelEnumType(name="Decelerating", key=1),
            ChannelEnumType(name="Stopped", key=2),
        ],
    )
    gpio_channel = ChannelConfig(
        name="gpio",
        data_type=ChannelDataType.BIT_FIELD,
        description="on/off values for pins on gpio",
        bit_field_elements=[
            ChannelBitFieldElement(name="12v", index=0, bit_count=1),
            ChannelBitFieldElement(name="charge", index=1, bit_count=2),
            ChannelBitFieldElement(name="led", index=3, bit_count=4),
            ChannelBitFieldElement(name="heater", index=7, bit_count=1),
        ],
    )

    return TelemetryConfig(
        asset_name="NostromoLV426",
        ingestion_client_key="nostromo_lv_426",
        flows=[
            FlowConfig(
                name="readings",
                channels=[
                    velocity_channel,
                    voltage_channel,
                    vehicle_state_channel,
                    gpio_channel,
                ],
            ),
            FlowConfig(
                name="voltage",
                channels=[voltage_channel],
            ),
            FlowConfig(
                name="gpio_channel",
                channels=[gpio_channel],
            ),
            FlowConfig(name="logs", channels=[log_channel]),
        ],
    )
```

The following is an example of ingesting data for each flow using `sift_py.ingestion.service.IngestionService.try_create_ingestion_request`:

```python
import time
from datetime import datetime, timezone

from sift_py.grpc.transport import SiftChannelConfig, use_sift_channel
from sift_py.ingestion.channel import (
    ChannelBitFieldElement,
    ChannelConfig,
    ChannelDataType,
    ChannelEnumType,
    bit_field_value,
    double_value,
    enum_value,
    int32_value,
    string_value,
)
from sift_py.ingestion.service import IngestionService
from sift_py.ingestion.config.telemetry import FlowConfig, TelemetryConfig


telemetry_config = nostromos_lv_426()

sift_channel_config = SiftChannelConfig(uri=base_uri, apikey=apikey)

with use_sift_channel(sift_channel_config) as channel:
    ingestion_service = IngestionService(
        channel,
        telemetry_config,
        end_stream_on_error=True,  # End stream if errors occur API-side.
    )

    # Send data for the readings flow
    readings_request = ingestion_service.try_create_ingestion_request(
        flow_name="readings",
        timestamp=datetime.now(timezone.utc),
        channel_values=[
            {
                "channel_name": "velocity",
                "component": "mainmotor",
                "value": double_value(10),
            },
            {
                "channel_name": "voltage",
                "value": int32_value(5),
            },
            {
                "channel_name": "vehicle_state",
                "value": enum_value(2),
            },
            {
                "channel_name": "gpio",
                "value": bit_field_value(bytes(int("00001001", 2)),
            },
        ],
    )
    ingestion_service.ingest(readings_request)

    # Send partial data for the readings flow
    partial_readings_request = ingestion_service.try_create_ingestion_request(
        flow_name="readings",
        timestamp=datetime.now(timezone.utc),
        channel_values=[
            {
                "channel_name": "velocity",
                "component": "mainmotor",
                "value": double_value(10),
            },
            {
                "channel_name": "gpio",
                "value": bit_field_value(bytes(int("00001001", 2)),
            },
        ],
    )
    ingestion_service.ingest(partial_readings_request)

    # Send partial data for the logs flow
    logs_request = ingestion_service.try_create_ingestion_request(
        flow_name="readings",
        timestamp=datetime.now(timezone.utc),
        channel_values=[
            {
                "channel_name": "logs",
                "value": string_value("INFO: some message")
            },
        ],
    )
    ingestion_service.ingest(logs_request)
```

Alternatively, you may also use `sift_py.ingestion.service.IngestionService.create_ingestion_request`, but be sure
to read the documentation for that method to understand how to leverage it correctly. Unlike
`sift_py.ingestion.service.IngestionService.try_create_ingestion_request`, it will not perform any client-side validations.
This is useful when performance is critical.

```python
import time
from datetime import datetime, timezone

from sift_py.grpc.transport import SiftChannelConfig, use_sift_channel
from sift_py.ingestion.channel import (
    ChannelBitFieldElement,
    ChannelConfig,
    ChannelDataType,
    ChannelEnumType,
    bit_field_value,
    empty_value,
    double_value,
    enum_value,
    int32_value,
    string_value,
)
from sift_py.ingestion.service import IngestionService
from sift_py.ingestion.config.telemetry import FlowConfig, TelemetryConfig


telemetry_config = nostromos_lv_426()

sift_channel_config = SiftChannelConfig(uri=base_uri, apikey=apikey)

with use_sift_channel(sift_channel_config) as channel:
    ingestion_service = IngestionService(
        channel,
        telemetry_config,
        end_stream_on_error=True,  # End stream if errors occur API-side.
    )

    # Send data for the readings flow
    readings_request = ingestion_service.create_ingestion_request(
        flow_name="readings",
        timestamp=datetime.now(timezone.utc),
        channel_values=[
            double_value(10),
            int32_value(5),
            enum_value(2),
            bit_field_value(bytes(int("00001001", 2)),
        ],
    )
    ingestion_service.ingest(readings_request)

    # Send partial data for the readings flow
    partial_readings_request = ingestion_service.try_create_ingestion_request(
        flow_name="readings",
        timestamp=datetime.now(timezone.utc),
        channel_values=[
            double_value(10),
            empty_value(),
            empty_value(),
            bit_field_value(bytes(int("00001001", 2)),
        ],
    )
    ingestion_service.ingest(partial_readings_request)

    # Send partial data for the logs flow
    logs_request = ingestion_service.try_create_ingestion_request(
        flow_name="readings",
        timestamp=datetime.now(timezone.utc),
        channel_values=[
            string_value("INFO: some message")
        ],
    )
    ingestion_service.ingest(logs_request)
```

## More Examples

For more comphrensive examples demonstrating a little bit of everything, you may
visit the [examples directory](https://github.com/sift-stack/sift/tree/main/python/examples) in the project repo.
"""
