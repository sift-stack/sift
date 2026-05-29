#!/usr/bin/env python3
"""
Streams simulated vehicle velocity and temperature telemetry generated using random values to mimic onboard vehicle sensors to Sift indefinitely.

This example demonstrates the complete streaming ingestion lifecycle:
- Authenticate with Sift
- Define a telemetry schema (Flow + Channels)
- Create an Asset and Run
- Open a streaming ingestion session
- Send timestamped flows in real time

The program runs continuously until the user terminates it.
"""

# Import dependencies
# ---------------------------------------------------------------------
# Standard library modules for async execution, randomness, timing,
# and generating unique identifiers.
import asyncio
import random
import uuid
from datetime import datetime, timezone

# Used to securely load environment variables from a .env file.
from dotenv import dotenv_values

# Core Sift client and connection configuration.
from sift_client import SiftClient, SiftConnectionConfig

# Sift ingestion types used to define telemetry structure and runs.
from sift_client.sift_types import (
    RunCreate,  # Represents a telemetry collection session
)

# Use sift_stream_bindings types directly to avoid CPU-bound conversion
# penalties from the ergonomic Flow <-> FlowPy conversions.
from sift_stream_bindings import (
    ChannelConfigPy,
    ChannelDataTypePy,
    ChannelValuePy,
    FlowConfigPy,
    FlowPy,
    IngestionConfigFormPy,  # Used instead of IngestionConfigCreate to work with FlowConfigPy
    TimeValuePy,
    ValuePy,
)

# Define configuration constants
# ---------------------------------------------------------------------
# FLOW_NAME identifies the telemetry schema inside Sift.
# SEND_INTERVAL_SECONDS controls sampling frequency.
FLOW_NAME = "vehicle_metrics"
SEND_INTERVAL_SECONDS = 0.5


# Helper function to generate unique names
# ---------------------------------------------------------------------
# Sift Assets and Runs should have unique names.
# This helper creates a timestamp + short UUID suffix to prevent collisions.
def make_unique_suffix() -> str:
    ts = datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%S")
    short_id = uuid.uuid4().hex[:8]
    return f"{ts}_{short_id}"


# Main entry point
# ---------------------------------------------------------------------
# All ingestion logic lives inside this async function.
# Streaming ingestion uses async gRPC under the hood.
async def main() -> None:
    # Create unique Asset and Run names
    # -----------------------------------------------------------------
    # An Asset represents the telemetry-producing system.
    # A Run represents a single data collection session for that Asset.
    suffix = make_unique_suffix()
    asset_name = f"robot_vehicle_{suffix}"
    run_name = f"{asset_name}_run"

    # Load authentication from .env
    # -----------------------------------------------------------------
    # We load credentials from a .env file instead of hardcoding them.
    # These values are required to establish authenticated communication
    # with both the REST and gRPC endpoints of your Sift environment.
    env_vars = dotenv_values(".env")
    api_key = env_vars.get("SIFT_API_KEY")
    grpc_url = env_vars.get("SIFT_GRPC_URL")
    rest_url = env_vars.get("SIFT_REST_URL")

    if not api_key or not grpc_url or not rest_url:
        raise RuntimeError("Missing Sift credentials in .env")

    # Create a client connection to Sift
    # -----------------------------------------------------------------
    # SiftConnectionConfig holds authentication and endpoint details.
    # SiftClient is your primary entry point for interacting with Sift.
    # Streaming ingestion uses the gRPC endpoint defined here.
    connection_config = SiftConnectionConfig(
        api_key=api_key,
        grpc_url=grpc_url,
        rest_url=rest_url,
    )

    client = SiftClient(connection_config=connection_config)

    # Define telemetry signals (Channels) within a Flow
    # -----------------------------------------------------------------
    # A FlowConfigPy defines the telemetry schema using sift_stream_bindings
    # types directly, avoiding the CPU-bound overhead of the ergonomic
    # Flow <-> FlowPy conversions.
    #
    # Each ChannelConfigPy defines:
    #   - name (signal identifier)
    #   - unit (measurement unit)
    #   - data_type (numeric, string, etc.)
    #   - description (a human-readable explanation of what the Channel
    #     (signal) represents and how it should be interpreted)
    #
    # All telemetry sent to Sift must conform to this schema.
    flow_config = FlowConfigPy(
        name=FLOW_NAME,
        channels=[
            ChannelConfigPy(
                name="velocity",
                unit="m/s",
                data_type=ChannelDataTypePy.Double,
                description="The velocity Channel streams real-time speed measurements of the vehicle in meters per second (m/s) as double-precision numeric values.",
                enum_types=[],
                bit_field_elements=[],
            ),
            ChannelConfigPy(
                name="temperature",
                unit="C",
                data_type=ChannelDataTypePy.Double,
                description="The temperature Channel streams real-time temperature readings of the vehicle in degrees Celsius (°C) as double-precision numeric values.",
                enum_types=[],
                bit_field_elements=[],
            ),
        ],
    )

    # Create ingestion configuration
    # -----------------------------------------------------------------
    # IngestionConfigFormPy associates:
    #   - An Asset
    #   - One or more Flow definitions
    # Using IngestionConfigFormPy from sift_stream_bindings to stay compatible
    # with FlowConfigPy, instead of IngestionConfigCreate which only accepts
    # the ergonomic FlowConfig type.
    ingestion_config = IngestionConfigFormPy(
        asset_name=asset_name,
        client_key=asset_name,
        flows=[flow_config],
    )

    # Create Run
    # -----------------------------------------------------------------
    # RunCreate defines the session that will group all incoming flows.
    # While not strictly necessary for ingestion, Runs are useful for organizing
    # data from one or more Assets for a given period of time (such as a specific test,
    # or daily ops).
    # client_key provides a stable unique identifier for the Run.
    run = RunCreate(name=run_name, client_key=run_name)

    # Open a streaming ingestion client
    # -----------------------------------------------------------------
    # This creates a gRPC streaming session tied to:
    #   - The ingestion configuration (Asset + Flows)
    #   - The Run
    #
    # All telemetry sent within this context will appear inside
    # this Run in Sift.
    #
    # WARNING: SiftStream should be reused for the entire program duration.
    # Do NOT recreate the streaming client per batch or per iteration —
    # each instantiation creates a new ingestion config, which introduces
    # significant performance overhead.
    async with await client.async_.ingestion.create_ingestion_config_streaming_client(
        ingestion_config=ingestion_config,
        run=run,
    ) as ingest_client:
        # Continue streaming until the user terminates the program.
        # On Ctrl+C, Python's asyncio raises CancelledError (not
        # KeyboardInterrupt) inside awaiting coroutines.  Catching it
        # here lets the async-with context manager exit cleanly and
        # call finish() via __aexit__.
        try:
            while True:
                now = datetime.now(timezone.utc)

                # Generate mock telemetry values
                # ---------------------------------------------------------
                # In a real system, these would come from sensors,
                # hardware interfaces, or production metrics.
                velocity = random.uniform(0, 10)
                temperature = random.uniform(20, 40)

                # Build and send a FlowPy directly using sift_stream_bindings types
                # ---------------------------------------------------------
                # Using FlowPy, ChannelValuePy, ValuePy, and TimeValuePy directly
                # avoids the CPU-bound conversion overhead of the ergonomic
                # flow_config.as_flow() helper.
                await ingest_client.send(
                    FlowPy(
                        flow_name=FLOW_NAME,
                        timestamp=TimeValuePy.from_timestamp_millis(int(now.timestamp() * 1000)),
                        values=[
                            ChannelValuePy(name="velocity", value=ValuePy.Double(velocity)),
                            ChannelValuePy(name="temperature", value=ValuePy.Double(temperature)),
                        ],
                    )
                )

                print(
                    f"[SENT {now.isoformat()}] "
                    f"velocity={velocity:.2f} m/s | "
                    f"temperature={temperature:.2f} C"
                )

                # Control sampling rate
                await asyncio.sleep(SEND_INTERVAL_SECONDS)
        except (KeyboardInterrupt, asyncio.CancelledError):
            pass

    print("Streaming session closed.")


# Standard Python entry point
# ---------------------------------------------------------------------
# asyncio.run() starts the async ingestion workflow.
# The outer exception handler suppresses noise from the async shutdown
# sequence that may occur after the streaming client has already been
# finished by the context manager.
if __name__ == "__main__":
    try:
        asyncio.run(main())
    except (KeyboardInterrupt, RuntimeError):
        print("\nStreaming stopped by user.")
