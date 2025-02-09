import numpy as np
import pandas as pd


def generate_waveforms(
    duration_s: float = 60.0,
    sample_rate_hz: float = 1000.0,
    min_freq_hz: int = 1,
    max_freq_hz: int = 100,
) -> pd.DataFrame:
    """
    Generates a dataset of various waveforms at different frequencies.

    Args:
        duration_s: Duration of the waveforms in seconds
        sample_rate_hz: Number of samples per second
        min_freq_hz: Minimum frequency to generate
        max_freq_hz: Maximum frequency to generate

    Returns:
        DataFrame containing timestamp and waveform data
    """
    # Calculate number of samples
    num_samples: int = int(duration_s * sample_rate_hz)

    # Generate time array
    time_array: np.ndarray = np.linspace(0, duration_s, num_samples)

    # Initialize DataFrame with timestamp column
    df: pd.DataFrame = pd.DataFrame({"timestamp": time_array})

    # Generate waveforms for each frequency
    for freq in range(min_freq_hz, max_freq_hz + 1):
        # Angular frequency
        w: float = 2 * np.pi * freq
        t: np.ndarray = time_array

        # Sine wave
        df[f"sine_{freq}Hz"] = np.sin(w * t)

        # Square wave
        df[f"square_{freq}Hz"] = np.sign(np.sin(w * t))

        # Triangle wave
        df[f"triangle_{freq}Hz"] = 2 * np.abs(2 * (t * freq - np.floor(0.5 + t * freq))) - 1

        # Sawtooth wave
        df[f"sawtooth_{freq}Hz"] = 2 * (t * freq - np.floor(0.5 + t * freq))

    return df


def main() -> None:
    """
    Main function to generate and save waveform dataset.
    """
    # Generate waveforms
    df: pd.DataFrame = generate_waveforms()

    # Format timestamp column to 3 decimal places
    df["timestamp"] = df["timestamp"].map("{:.3f}".format)

    # Save to CSV
    output_file: str = "waveforms.csv"
    df.to_csv(output_file, index=False)
    print(f"Generated waveform dataset with {len(df)} rows and {len(df.columns)} columns")
    print(f"Saved to {output_file}")


if __name__ == "__main__":
    main()
