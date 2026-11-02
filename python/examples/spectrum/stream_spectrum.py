#!/usr/bin/env python3
"""Stream simulated IQ spectra to a bytes channel as SPF1 frames for Spectrum View.

A carrier sweeps across a 2 MHz band at 915 MHz over noise. Each capture is turned into one
frame with ``spectrum_frame_from_samples`` and sent to the ``spectrum`` channel at 10 Hz.
Open the channel in Spectrum View to see the live spectrum and waterfall. Stop with Ctrl+C.
"""

import asyncio
import uuid
from datetime import datetime, timezone

import numpy as np
from dotenv import dotenv_values
from sift_client import SiftClient, SiftConnectionConfig
from sift_client.sift_types import (
    ChannelConfig,
    ChannelDataType,
    FlowConfig,
    IngestionConfigCreate,
    RunCreate,
)
from sift_client.util.spectrum import spectrum_frame_from_samples

SAMPLE_RATE_HZ = 2e6
CENTER_HZ = 915e6
CAPTURE_LEN = 4096
SEND_INTERVAL_S = 0.1
SWEEP_PERIOD_S = 20.0


def capture(t: float, rng: np.random.Generator) -> np.ndarray:
    """One IQ capture: a carrier sweeping +/-800 kHz plus noise."""
    offset_hz = 800e3 * np.sin(2 * np.pi * t / SWEEP_PERIOD_S)
    n = np.arange(CAPTURE_LEN) / SAMPLE_RATE_HZ
    carrier = 0.5 * np.exp(2j * np.pi * offset_hz * n)
    noise = 1e-3 * (rng.standard_normal(CAPTURE_LEN) + 1j * rng.standard_normal(CAPTURE_LEN))
    return carrier + noise


async def main() -> None:
    env = dotenv_values(".env")
    client = SiftClient(
        connection_config=SiftConnectionConfig(
            api_key=env["SIFT_API_KEY"],
            grpc_url=env["SIFT_GRPC_URL"],
            rest_url=env["SIFT_REST_URL"],
        )
    )

    asset_name = f"spectrum_demo_{uuid.uuid4().hex[:8]}"
    flow_config = FlowConfig(
        name="rf",
        channels=[
            ChannelConfig(
                name="spectrum",
                data_type=ChannelDataType.BYTES,
                description="SPF1 v1 spectrum frames, dBFS",
            )
        ],
    )
    ingestion_config = IngestionConfigCreate(
        asset_name=asset_name, client_key=asset_name, flows=[flow_config]
    )
    run = RunCreate(name=f"{asset_name}_run", client_key=f"{asset_name}_run")

    rng = np.random.default_rng()
    start = datetime.now(timezone.utc)
    async with await client.async_.ingestion.create_ingestion_config_streaming_client(
        ingestion_config=ingestion_config, run=run
    ) as ingest_client:
        print(f"Streaming to asset {asset_name}, channel 'spectrum'. Ctrl+C to stop.")
        while True:
            now = datetime.now(timezone.utc)
            frame = spectrum_frame_from_samples(
                capture((now - start).total_seconds(), rng),
                sample_rate_hz=SAMPLE_RATE_HZ,
                center_hz=CENTER_HZ,
                # Each frame stands for the whole send interval, not just the 2 ms capture.
                window_s=SEND_INTERVAL_S,
            )
            await ingest_client.send(flow_config.as_flow(timestamp=now, values={"spectrum": frame}))
            await asyncio.sleep(SEND_INTERVAL_S)


if __name__ == "__main__":
    try:
        asyncio.run(main())
    except KeyboardInterrupt:
        print("\nStopped.")
