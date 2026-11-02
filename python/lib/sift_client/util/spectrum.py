"""SPF1 v1 spectrum frames for bytes channels, as read by Spectrum View.

One value is one spectrum: a 32-byte little-endian header, then float32 magnitudes. The timestamp is
the window start; bin ``i`` is centered at ``start_hz + i * step_hz``.

| Offset | Type          | Field                                                  |
| ------ | ------------- | ------------------------------------------------------ |
| 0      | 4 bytes       | Magic ``SPF1``                                         |
| 4      | u8            | Version, 1                                             |
| 5      | u8            | Magnitude unit, a [MagnitudeUnit][sift_client.util.spectrum.MagnitudeUnit] value              |
| 6      | u8            | Encoding, 0 = float32                                  |
| 7      | u8            | Reserved, 0                                            |
| 8      | u32           | Bin count                                              |
| 12     | f64           | Start frequency in Hz                                  |
| 20     | f64           | Bin step in Hz                                         |
| 28     | f32           | Window in seconds, 0 = none                            |
| 32     | count x f32   | Magnitudes                                             |

Spectrum View skips frames whose grid or unit differs from the first frame.

Example:
    ```python
    from sift_client.sift_types import FlowConfig
    from sift_client.util.spectrum import SpectrumEncoder, spectrum_channel

    flow = FlowConfig(name="rf", channels=[spectrum_channel("spectrum")])
    encoder = SpectrumEncoder(sample_rate_hz=10e6, center_hz=2.4e9, frame_interval_s=0.1)
    # per capture, with a streaming client for flow:
    await stream.send(flow.as_flow(timestamp=t, values={"spectrum": encoder.encode(iq)}))
    ```

    Already have magnitudes? Use ``encode_spectrum_frame(mags, start_hz=..., step_hz=..., unit="dBm")``.
"""

from __future__ import annotations

import math
import struct
from dataclasses import dataclass
from enum import IntEnum
from typing import TYPE_CHECKING, Union

import numpy as np
import numpy.typing as npt

if TYPE_CHECKING:
    from sift_client.sift_types.ingestion import ChannelConfig

__all__ = [
    "SPECTRUM_FRAME_HEADER_BYTES",
    "SPECTRUM_FRAME_MAGIC",
    "SPECTRUM_FRAME_VERSION",
    "MagnitudeUnit",
    "SpectrumEncoder",
    "SpectrumFrame",
    "decode_spectrum_frame",
    "encode_spectrum_frame",
    "spectrum_channel",
    "spectrum_frame_from_samples",
]

SPECTRUM_FRAME_MAGIC = b"SPF1"
SPECTRUM_FRAME_VERSION = 1
SPECTRUM_FRAME_HEADER_BYTES = 32

_HEADER = struct.Struct("<4sBBBBIddf")
_ENCODING_FLOAT32 = 0
_MAX_BIN_COUNT = 2**32 - 1
_MAX_FLOAT32 = float(np.finfo(np.float32).max)
# -300 dBFS, so empty bins aren't -inf.
_MIN_AMPLITUDE = 1e-15


class MagnitudeUnit(IntEnum):
    """Magnitude unit; the value is the header byte."""

    DBM = 0
    DBW = 1
    DBFS = 2
    DB = 3
    DBUV = 4

    @property
    def label(self) -> str:
        """Display label, e.g. ``dBm``."""
        return _UNIT_LABELS[self]

    @classmethod
    def from_label(cls, label: str) -> MagnitudeUnit:
        """Unit for a display label, e.g. ``dBFS``."""
        for unit, unit_label in _UNIT_LABELS.items():
            if unit_label == label:
                return unit
        raise ValueError(
            f"unknown magnitude unit {label!r}, expected one of {list(_UNIT_LABELS.values())}"
        )


_UNIT_LABELS = {
    MagnitudeUnit.DBM: "dBm",
    MagnitudeUnit.DBW: "dBW",
    MagnitudeUnit.DBFS: "dBFS",
    MagnitudeUnit.DB: "dB",
    MagnitudeUnit.DBUV: "dBuV",
}

MagnitudeUnitLike = Union[MagnitudeUnit, int, str]


def _to_unit(unit: MagnitudeUnitLike) -> MagnitudeUnit:
    if isinstance(unit, str):
        return MagnitudeUnit.from_label(unit)
    try:
        return MagnitudeUnit(unit)
    except ValueError:
        raise ValueError(f"unknown magnitude unit {unit}") from None


@dataclass(frozen=True)
class SpectrumFrame:
    """One spectrum on a uniform grid.

    Attributes:
        magnitudes: float32 per bin.
        start_hz: Bin 0 center, Hz.
        step_hz: Bin spacing, Hz.
        unit: Magnitude unit.
        window_s: Time covered in seconds, 0 = unset.
    """

    magnitudes: np.ndarray
    start_hz: float
    step_hz: float
    unit: MagnitudeUnit = MagnitudeUnit.DBM
    window_s: float = 0.0

    @property
    def frequencies_hz(self) -> np.ndarray:
        """Bin centers, Hz."""
        return self.start_hz + self.step_hz * np.arange(len(self.magnitudes))

    def to_bytes(self) -> bytes:
        """Encode as SPF1 v1."""
        return encode_spectrum_frame(
            self.magnitudes,
            start_hz=self.start_hz,
            step_hz=self.step_hz,
            unit=self.unit,
            window_s=self.window_s,
        )


def _check_grid(start_hz: float, step_hz: float, count: int, window_s: float) -> None:
    if not math.isfinite(start_hz):
        raise ValueError("frequency start must be a number")
    if not (math.isfinite(step_hz) and step_hz > 0):
        raise ValueError("frequency step must be greater than 0")
    if count <= 0:
        raise ValueError("bin count must be a positive integer")
    if count > _MAX_BIN_COUNT:
        raise ValueError(f"bin count {count} does not fit in 32 bits")
    if not (math.isfinite(window_s) and window_s >= 0):
        raise ValueError("window_s must be 0 or greater")


def encode_spectrum_frame(
    magnitudes: npt.ArrayLike,
    *,
    start_hz: float,
    step_hz: float,
    unit: MagnitudeUnitLike = MagnitudeUnit.DBM,
    window_s: float = 0.0,
) -> bytes:
    """Encode one spectrum as an SPF1 v1 bytes value.

    Args:
        magnitudes: 1-D, one per bin, in ``unit``. Stored as float32.
        start_hz: Bin 0 center, Hz.
        step_hz: Bin spacing, Hz. Greater than 0.
        unit: [MagnitudeUnit][sift_client.util.spectrum.MagnitudeUnit], its byte, or its label (``"dBm"``).
        window_s: Waterfall row height in seconds. 0 lets the view pick.

    Returns:
        ``32 + 4 * len(magnitudes)`` bytes.

    Raises:
        ValueError: On any value Spectrum View would reject.
    """
    mags = np.asarray(magnitudes, dtype="<f4")
    if mags.ndim != 1:
        raise ValueError(f"magnitudes must be 1-D, got shape {mags.shape}")
    start_hz = float(start_hz)
    step_hz = float(step_hz)
    window_s = float(window_s)
    if abs(window_s) > _MAX_FLOAT32:
        raise ValueError("window_s must fit in a float32")
    window_s_f32 = float(np.float32(window_s))
    _check_grid(start_hz, step_hz, len(mags), window_s_f32)
    header = _HEADER.pack(
        SPECTRUM_FRAME_MAGIC,
        SPECTRUM_FRAME_VERSION,
        int(_to_unit(unit)),
        _ENCODING_FLOAT32,
        0,
        len(mags),
        start_hz,
        step_hz,
        window_s_f32,
    )
    return header + mags.tobytes()


def decode_spectrum_frame(data: bytes | bytearray | memoryview) -> SpectrumFrame:
    """Decode an SPF1 v1 bytes value.

    Raises:
        ValueError: If invalid; the message gives the reason.
    """
    data = bytes(data)
    if data[: len(SPECTRUM_FRAME_MAGIC)] != SPECTRUM_FRAME_MAGIC:
        raise ValueError("value does not start with the SPF1 magic")
    if len(data) < SPECTRUM_FRAME_HEADER_BYTES:
        raise ValueError("binary frame is shorter than its header")
    _, version, unit_byte, encoding, _, count, start_hz, step_hz, window_s = _HEADER.unpack_from(
        data
    )
    if version != SPECTRUM_FRAME_VERSION:
        raise ValueError(f"unsupported binary frame version {version}")
    unit = _to_unit(unit_byte)
    if encoding != _ENCODING_FLOAT32:
        raise ValueError(f"unsupported magnitude encoding {encoding}")
    _check_grid(start_hz, step_hz, count, window_s)
    data_bytes = len(data) - SPECTRUM_FRAME_HEADER_BYTES
    if data_bytes != count * 4:
        raise ValueError(f"magnitude has {data_bytes} bytes for {count} bins")
    magnitudes = np.frombuffer(data, dtype="<f4", offset=SPECTRUM_FRAME_HEADER_BYTES).astype(
        np.float32
    )
    return SpectrumFrame(
        magnitudes=magnitudes,
        start_hz=start_hz,
        step_hz=step_hz,
        unit=unit,
        window_s=window_s,
    )


def spectrum_frame_from_samples(
    samples: npt.ArrayLike,
    *,
    sample_rate_hz: float,
    center_hz: float = 0.0,
    window: str | npt.ArrayLike | None = "hann",
    full_scale: float = 1.0,
    window_s: float | None = None,
) -> bytes:
    """FFT one capture into an SPF1 v1 frame in dBFS.

    Real samples: one-sided, ``center_hz`` to ``center_hz + fs/2``. Complex (IQ): two-sided around
    ``center_hz``. Step is ``fs / len(samples)``. Window gain is corrected, so a tone of amplitude
    ``full_scale`` reads 0 dBFS; empty bins read -300 dBFS.

    Args:
        samples: 1-D capture, real or complex.
        sample_rate_hz: Sample rate, Hz.
        center_hz: Tuned frequency, Hz.
        window: ``"hann"``, ``"rect"``, ``None``, or weights of length ``len(samples)``.
        full_scale: Amplitude that reads 0 dBFS.
        window_s: Defaults to the capture length; 0 = unset. Pass the frame interval when
            captures are shorter than it, or the waterfall has gaps.

    Raises:
        ValueError: On invalid samples or arguments.
    """
    x = np.asarray(samples)
    if x.ndim != 1:
        raise ValueError(f"samples must be 1-D, got shape {x.shape}")
    n = len(x)
    if n < 2:
        raise ValueError("samples must have at least 2 values")
    if not (math.isfinite(sample_rate_hz) and sample_rate_hz > 0):
        raise ValueError("sample_rate_hz must be greater than 0")
    if not (math.isfinite(full_scale) and full_scale > 0):
        raise ValueError("full_scale must be greater than 0")

    weights = _window_weights(window, n)
    coherent_gain = float(np.sum(weights))
    if coherent_gain <= 0:
        raise ValueError("window weights must sum to more than 0")

    step_hz = sample_rate_hz / n
    if np.iscomplexobj(x):
        spectrum = np.fft.fftshift(np.fft.fft(x * weights))
        amplitude = np.abs(spectrum) / coherent_gain
        start_hz = center_hz + float(np.fft.fftshift(np.fft.fftfreq(n, d=1.0 / sample_rate_hz))[0])
    else:
        spectrum = np.fft.rfft(x * weights)
        amplitude = np.abs(spectrum) / coherent_gain
        # Fold in negative frequencies; DC and Nyquist have no pair.
        last_paired = len(amplitude) - 1 if n % 2 == 0 else len(amplitude)
        amplitude[1:last_paired] *= 2
        start_hz = center_hz

    magnitudes = 20 * np.log10(np.maximum(amplitude / full_scale, _MIN_AMPLITUDE))
    return encode_spectrum_frame(
        magnitudes,
        start_hz=start_hz,
        step_hz=step_hz,
        unit=MagnitudeUnit.DBFS,
        window_s=n / sample_rate_hz if window_s is None else window_s,
    )


def _window_weights(window: str | npt.ArrayLike | None, n: int) -> np.ndarray:
    if window is None or (isinstance(window, str) and window == "rect"):
        return np.ones(n)
    if isinstance(window, str):
        if window == "hann":
            # Periodic Hann.
            return 0.5 - 0.5 * np.cos(2 * np.pi * np.arange(n) / n)
        raise ValueError(f"unknown window {window!r}, expected 'hann', 'rect', None or an array")
    weights = np.asarray(window, dtype=np.float64)
    if weights.shape != (n,):
        raise ValueError(f"window has shape {weights.shape}, expected ({n},)")
    return weights


def spectrum_channel(name: str = "spectrum", *, description: str | None = None) -> ChannelConfig:
    """Channel config for SPF1 frames: a bytes channel."""
    from sift_client.sift_types.ingestion import ChannelConfig, ChannelDataType

    return ChannelConfig(name=name, data_type=ChannelDataType.BYTES, description=description)


@dataclass(frozen=True, eq=False)
class SpectrumEncoder:
    """Capture settings, held once, for turning samples into frames.

    Attributes:
        sample_rate_hz: Sample rate, Hz.
        center_hz: Tuned frequency, Hz.
        frame_interval_s: Time between frames, so waterfall rows have no gaps.
            ``None`` uses each capture's length.
        window: See [spectrum_frame_from_samples][sift_client.util.spectrum.spectrum_frame_from_samples].
        full_scale: Amplitude that reads 0 dBFS.
    """

    sample_rate_hz: float
    center_hz: float = 0.0
    frame_interval_s: float | None = None
    window: str | npt.ArrayLike | None = "hann"
    full_scale: float = 1.0

    def __post_init__(self) -> None:
        if not (math.isfinite(self.sample_rate_hz) and self.sample_rate_hz > 0):
            raise ValueError("sample_rate_hz must be greater than 0")
        if not (math.isfinite(self.full_scale) and self.full_scale > 0):
            raise ValueError("full_scale must be greater than 0")
        if self.frame_interval_s is not None and not (
            math.isfinite(self.frame_interval_s) and self.frame_interval_s > 0
        ):
            raise ValueError("frame_interval_s must be greater than 0")

    def encode(self, samples: npt.ArrayLike) -> bytes:
        """FFT one capture into an SPF1 v1 frame in dBFS."""
        return spectrum_frame_from_samples(
            samples,
            sample_rate_hz=self.sample_rate_hz,
            center_hz=self.center_hz,
            window=self.window,
            full_scale=self.full_scale,
            window_s=self.frame_interval_s,
        )
