from __future__ import annotations

import zipfile
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from pathlib import Path

    from sift_client.transport.rest_transport import RestClient


def download_file(url: str, dest: Path, *, rest_client: RestClient) -> Path:
    """Download a file from a URL in streaming 4 MiB chunks.

    Args:
        url: The URL to download from.
        dest: Path where the file will be saved. Parent directories are created if needed.
        rest_client: The SDK rest client to use for the download.

    Returns:
        The path to the downloaded file.

    Raises:
        requests.HTTPError: If the download request fails.
    """
    dest.parent.mkdir(parents=True, exist_ok=True)
    # Strip the session's default Authorization header, presigned URLs carry their own auth
    with rest_client.get(url, stream=True, headers={"Authorization": None}) as response:
        response.raise_for_status()
        with dest.open("wb") as file:
            for chunk in response.iter_content(chunk_size=4194304):  # 4 MiB
                if chunk:
                    file.write(chunk)
    return dest


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
        zip_path.unlink()
    return [output_dir / name for name in names if not name.endswith("/")]
