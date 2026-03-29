from __future__ import annotations

import warnings
import zipfile
from typing import TYPE_CHECKING

from alive_progress import alive_bar

from sift_client.errors import SiftWarning

if TYPE_CHECKING:
    from pathlib import Path

    from sift_client.transport.rest_transport import RestClient


def download_file(
    signed_url: str,
    output_path: Path,
    *,
    rest_client: RestClient,
    show_progress: bool = False,
) -> Path:
    """Download a file from a URL in streaming 4 MiB chunks.

    Args:
        url: The URL to download from.
        dest: Path where the file will be saved. Parent directories are created if needed.
        rest_client: The SDK rest client to use for the download.
        show_progress: If True, display a progress bar during download.
            Defaults to False.

    Returns:
        The path to the downloaded file.

    Raises:
        requests.HTTPError: If the download request fails.
    """
    output_path.parent.mkdir(parents=True, exist_ok=True)
    # Strip the session's default Authorization header, presigned URLs carry their own auth
    with rest_client.get(signed_url, stream=True, headers={"Authorization": None}) as response:
        response.raise_for_status()
        total_bytes = int(response.headers.get("Content-Length", 0)) or None
        with alive_bar(
            total_bytes,
            title="Downloading",
            unit="B",
            scale="SI",
            disable=not show_progress,
        ) as bar:
            with output_path.open("wb") as file:
                for chunk in response.iter_content(chunk_size=4194304):  # 4 MiB
                    if chunk:
                        file.write(chunk)
                        bar(len(chunk))
    return output_path


def extract_zip(zip_path: Path, output_dir: Path, *, delete_zip: bool = True) -> list[Path]:
    """Extract a zip file to a directory.

    Args:
        zip_path: Path to the zip file.
        output_dir: Directory to extract contents into. Created if it doesn't exist.
        delete_zip: If True (default), delete the zip file after extraction.

    Returns:
        List of paths to the extracted files (excludes directories).

    Raises:
        zipfile.BadZipFile: If the file is not a valid zip.
    """
    output_dir.mkdir(parents=True, exist_ok=True)
    with zipfile.ZipFile(zip_path, "r") as zip_file:
        names = zip_file.namelist()
        zip_file.extractall(output_dir)
    if delete_zip:
        try:
            zip_path.unlink()
        except OSError:
            warnings.warn(f"Failed to delete zip file '{zip_path}'", SiftWarning, stacklevel=2)
    return [output_dir / name for name in names if not name.endswith("/")]
