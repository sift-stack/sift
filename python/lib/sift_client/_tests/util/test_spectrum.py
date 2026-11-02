from __future__ import annotations

import struct
from typing import Any

import numpy as np
import pytest

from sift_client.sift_types import ChannelDataType, FlowConfig
from sift_client.util.spectrum import (
    SPECTRUM_FRAME_HEADER_BYTES,
    MagnitudeUnit,
    SpectrumEncoder,
    SpectrumFrame,
    decode_spectrum_frame,
    encode_spectrum_frame,
    spectrum_channel,
    spectrum_frame_from_samples,
)


def _frame(**overrides) -> bytes:
    args: dict[str, Any] = {
        "start_hz": 1e6,
        "step_hz": 250.0,
        "unit": MagnitudeUnit.DBM,
        "window_s": 0.0,
    }
    args.update(overrides)
    return encode_spectrum_frame([-90.0, -40.5, -88.25], **args)


class TestEncode:
    def test_matches_reference_layout(self):
        mags = np.array([-90.0, -40.5, -88.25], dtype=np.float32)
        expected = (
            struct.pack("<4sBBBBIddf", b"SPF1", 1, 2, 0, 0, 3, 2.4e9, 1e3, 0.25)
            + mags.astype("<f4").tobytes()
        )

        frame = encode_spectrum_frame(
            mags, start_hz=2.4e9, step_hz=1e3, unit=MagnitudeUnit.DBFS, window_s=0.25
        )

        assert frame == expected
        assert len(frame) == SPECTRUM_FRAME_HEADER_BYTES + 4 * len(mags)

    @pytest.mark.parametrize("unit", list(MagnitudeUnit))
    def test_round_trip_each_unit(self, unit: MagnitudeUnit):
        decoded = decode_spectrum_frame(_frame(unit=unit))
        assert decoded.unit == unit
        np.testing.assert_array_equal(decoded.magnitudes, [-90.0, -40.5, -88.25])
        assert decoded.start_hz == 1e6
        assert decoded.step_hz == 250.0
        assert decoded.window_s == 0.0

    def test_accepts_unit_label_and_int(self):
        assert decode_spectrum_frame(_frame(unit="dBuV")).unit == MagnitudeUnit.DBUV
        assert decode_spectrum_frame(_frame(unit=1)).unit == MagnitudeUnit.DBW

    def test_window_round_trips_at_float32_precision(self):
        assert decode_spectrum_frame(_frame(window_s=0.1)).window_s == pytest.approx(0.1, rel=1e-6)

    def test_frame_dataclass_round_trip(self):
        frame = SpectrumFrame(
            magnitudes=np.array([1.0, 2.0], dtype=np.float32),
            start_hz=-5.0,
            step_hz=5.0,
            unit=MagnitudeUnit.DB,
        )
        decoded = decode_spectrum_frame(frame.to_bytes())
        np.testing.assert_array_equal(decoded.frequencies_hz, [-5.0, 0.0])
        np.testing.assert_array_equal(decoded.magnitudes, frame.magnitudes)
        assert decoded.unit == MagnitudeUnit.DB

    @pytest.mark.parametrize(
        ("magnitudes", "overrides", "message"),
        [
            ([], {}, "bin count"),
            ([[1.0, 2.0]], {}, "1-D"),
            ([1.0], {"step_hz": 0.0}, "frequency step"),
            ([1.0], {"step_hz": float("inf")}, "frequency step"),
            ([1.0], {"start_hz": float("nan")}, "frequency start"),
            ([1.0], {"window_s": -1.0}, "window_s"),
            ([1.0], {"window_s": float("nan")}, "window_s"),
            ([1.0], {"window_s": 1e39}, "window_s"),
            ([1.0], {"unit": 9}, "magnitude unit"),
            ([1.0], {"unit": "dbm"}, "magnitude unit"),
        ],
    )
    def test_rejects_frames_spectrum_view_would_skip(self, magnitudes, overrides, message):
        args = {"start_hz": 0.0, "step_hz": 1.0}
        args.update(overrides)
        with pytest.raises(ValueError, match=message):
            encode_spectrum_frame(magnitudes, **args)


def _patched(offset: int, value: int) -> bytes:
    data = bytearray(_frame())
    data[offset] = value
    return bytes(data)


class TestDecode:
    @pytest.mark.parametrize(
        ("make_data", "message"),
        [
            (lambda: b"JSON{}", "magic"),
            (lambda: b"SPF1\x01", "shorter than its header"),
            (lambda: _patched(4, 2), "version 2"),
            (lambda: _patched(5, 7), "magnitude unit 7"),
            (lambda: _patched(6, 1), "encoding 1"),
            (lambda: _frame()[: SPECTRUM_FRAME_HEADER_BYTES + 4], "4 bytes for 3 bins"),
        ],
    )
    def test_rejects_invalid_frames(self, make_data, message):
        with pytest.raises(ValueError, match=message):
            decode_spectrum_frame(make_data())

    def test_accepts_memoryview(self):
        assert decode_spectrum_frame(memoryview(_frame())).step_hz == 250.0


class TestFromSamples:
    fs = 1024.0
    n = 256

    def _peak(self, frame: SpectrumFrame) -> tuple[float, float]:
        i = int(np.argmax(frame.magnitudes))
        return float(frame.frequencies_hz[i]), float(frame.magnitudes[i])

    def test_real_tone_reads_zero_dbfs_on_one_sided_grid(self):
        t = np.arange(self.n) / self.fs
        x = np.cos(2 * np.pi * 100.0 * t)

        frame = decode_spectrum_frame(spectrum_frame_from_samples(x, sample_rate_hz=self.fs))

        assert frame.unit == MagnitudeUnit.DBFS
        assert frame.start_hz == 0.0
        assert frame.step_hz == self.fs / self.n
        assert len(frame.magnitudes) == self.n // 2 + 1
        freq, mag = self._peak(frame)
        assert freq == 100.0
        assert mag == pytest.approx(0.0, abs=1e-3)

    @pytest.mark.parametrize("window", ["hann", "rect", None])
    def test_window_gain_is_corrected(self, window):
        t = np.arange(self.n) / self.fs
        x = 0.5 * np.cos(2 * np.pi * 64.0 * t)

        frame = decode_spectrum_frame(
            spectrum_frame_from_samples(x, sample_rate_hz=self.fs, window=window)
        )

        assert self._peak(frame)[1] == pytest.approx(20 * np.log10(0.5), abs=1e-3)

    def test_complex_tone_is_centered_on_tuned_frequency(self):
        t = np.arange(self.n) / self.fs
        x = np.exp(2j * np.pi * -48.0 * t)

        frame = decode_spectrum_frame(
            spectrum_frame_from_samples(x, sample_rate_hz=self.fs, center_hz=915e6)
        )

        assert len(frame.magnitudes) == self.n
        assert frame.start_hz == 915e6 - self.fs / 2
        freq, mag = self._peak(frame)
        assert freq == 915e6 - 48.0
        assert mag == pytest.approx(0.0, abs=1e-3)

    def test_odd_length_complex_grid_is_symmetric(self):
        frame = decode_spectrum_frame(
            spectrum_frame_from_samples(np.ones(5, dtype=complex), sample_rate_hz=5.0)
        )
        np.testing.assert_allclose(frame.frequencies_hz, [-2.0, -1.0, 0.0, 1.0, 2.0])

    def test_full_scale_and_silence_floor(self):
        x = np.zeros(self.n)
        x[0] = 1000.0  # impulse
        frame = decode_spectrum_frame(
            spectrum_frame_from_samples(x, sample_rate_hz=self.fs, window=None, full_scale=1000.0)
        )
        assert np.isfinite(frame.magnitudes).all()

        silent = decode_spectrum_frame(spectrum_frame_from_samples(np.zeros(8), sample_rate_hz=8.0))
        np.testing.assert_allclose(silent.magnitudes, -300.0)

    def test_window_s_defaults_to_capture_length(self):
        default = decode_spectrum_frame(
            spectrum_frame_from_samples(np.ones(self.n), sample_rate_hz=self.fs)
        )
        unset = decode_spectrum_frame(
            spectrum_frame_from_samples(np.ones(self.n), sample_rate_hz=self.fs, window_s=0)
        )
        assert default.window_s == self.n / self.fs
        assert unset.window_s == 0.0

    def test_custom_window_array(self):
        frame = decode_spectrum_frame(
            spectrum_frame_from_samples(np.ones(4), sample_rate_hz=4.0, window=[1, 1, 1, 1])
        )
        assert frame.magnitudes[0] == pytest.approx(0.0, abs=1e-5)

    @pytest.mark.parametrize(
        ("samples", "kwargs", "message"),
        [
            (np.ones((2, 2)), {}, "1-D"),
            (np.ones(1), {}, "at least 2"),
            (np.ones(4), {"sample_rate_hz": 0.0}, "sample_rate_hz"),
            (np.ones(4), {"full_scale": -1.0}, "full_scale"),
            (np.ones(4), {"window": "blackman"}, "unknown window"),
            (np.ones(4), {"window": [1, 1]}, "window has shape"),
            (np.ones(4), {"window": [0, 0, 0, 0]}, "sum to more than 0"),
        ],
    )
    def test_rejects_invalid_arguments(self, samples, kwargs, message):
        args = {"sample_rate_hz": 4.0}
        args.update(kwargs)
        with pytest.raises(ValueError, match=message):
            spectrum_frame_from_samples(samples, **args)


class TestEncoder:
    def test_matches_from_samples_with_frame_interval(self):
        iq = np.exp(2j * np.pi * 0.1 * np.arange(64))
        encoder = SpectrumEncoder(sample_rate_hz=64.0, center_hz=100.0, frame_interval_s=0.5)

        assert encoder.encode(iq) == spectrum_frame_from_samples(
            iq, sample_rate_hz=64.0, center_hz=100.0, window_s=0.5
        )
        assert decode_spectrum_frame(encoder.encode(iq)).window_s == 0.5

    def test_window_defaults_to_capture_length(self):
        frame = SpectrumEncoder(sample_rate_hz=64.0).encode(np.ones(32))
        assert decode_spectrum_frame(frame).window_s == 0.5

    @pytest.mark.parametrize(
        ("kwargs", "message"),
        [
            ({"sample_rate_hz": 0.0}, "sample_rate_hz"),
            ({"sample_rate_hz": 1.0, "full_scale": 0.0}, "full_scale"),
            ({"sample_rate_hz": 1.0, "frame_interval_s": 0.0}, "frame_interval_s"),
        ],
    )
    def test_rejects_invalid_settings(self, kwargs, message):
        with pytest.raises(ValueError, match=message):
            SpectrumEncoder(**kwargs)


def test_frame_is_a_bytes_channel_value():
    flow_config = FlowConfig(name="rf", channels=[spectrum_channel()])
    frame = SpectrumEncoder(sample_rate_hz=8.0).encode(np.ones(8))

    flow = flow_config.as_flow(values={"spectrum": frame})

    (value,) = flow.channel_values
    assert value.ty == ChannelDataType.BYTES
    assert value.value == frame
