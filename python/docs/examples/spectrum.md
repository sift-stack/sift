# Spectrum Streaming

Stream spectrum frames to a bytes channel and watch them live in Spectrum View.

[`SpectrumEncoder`][sift_client.util.spectrum.SpectrumEncoder] turns each capture of real or IQ samples into one frame, and [`spectrum_channel`][sift_client.util.spectrum.spectrum_channel] declares the bytes channel that holds them.

## Run it

1. Install the client with streaming support:
   ```bash
   pip install 'sift-stack-py[sift-stream]' python-dotenv
   ```
2. Create a `.env` file next to the script:
   ```bash
   SIFT_API_KEY="your-api-key"
   SIFT_GRPC_URL="your-sift-grpc-url"
   SIFT_REST_URL="your-sift-rest-url"
   ```
3. Save the script below as `stream_spectrum.py` and run `python stream_spectrum.py`.
4. In Sift, open the `spectrum` channel on the `spectrum_demo` asset in Spectrum View. Stop the script with Ctrl+C.

```python
"""Stream a live spectrum to Sift. Open the "spectrum" channel in Spectrum View to watch it."""

import asyncio
from datetime import datetime, timezone

import numpy as np
from dotenv import dotenv_values
from sift_client import SiftClient, SiftConnectionConfig
from sift_client.sift_types import FlowConfig, IngestionConfigCreate
from sift_client.util.spectrum import SpectrumEncoder, spectrum_channel

SAMPLE_RATE_HZ = 2e6
INTERVAL_S = 0.1


async def main() -> None:
    env = dotenv_values(".env")
    client = SiftClient(
        connection_config=SiftConnectionConfig(
            api_key=env["SIFT_API_KEY"],
            grpc_url=env["SIFT_GRPC_URL"],
            rest_url=env["SIFT_REST_URL"],
        )
    )

    flow = FlowConfig(name="rf", channels=[spectrum_channel("spectrum")])
    config = IngestionConfigCreate(asset_name="spectrum_demo", flows=[flow])
    encoder = SpectrumEncoder(
        sample_rate_hz=SAMPLE_RATE_HZ, center_hz=915e6, frame_interval_s=INTERVAL_S
    )

    t = np.arange(4096) / SAMPLE_RATE_HZ
    async with await client.async_.ingestion.create_ingestion_config_streaming_client(
        config
    ) as stream:
        while True:
            # Replace with samples from your radio: a 300 kHz tone plus noise.
            iq = 0.5 * np.exp(2j * np.pi * 300e3 * t) + 1e-3 * np.random.randn(t.size)
            frame = encoder.encode(iq)
            now = datetime.now(timezone.utc)
            await stream.send(flow.as_flow(timestamp=now, values={"spectrum": frame}))
            await asyncio.sleep(INTERVAL_S)


if __name__ == "__main__":
    asyncio.run(main())
```

## Already have magnitudes?

If your own FFT or a spectrum analyzer gives you magnitudes, encode them directly:

```python
from sift_client.util.spectrum import encode_spectrum_frame

frame = encode_spectrum_frame(mags_dbm, start_hz=900e6, step_hz=10e3, unit="dBm")
```

Every frame on a channel should share one frequency grid and unit. Spectrum View skips frames that differ from the first frame.
