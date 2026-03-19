from __future__ import annotations

import zipfile
from typing import TYPE_CHECKING

import requests

if TYPE_CHECKING:
    from pathlib import Path


def download_and_extract_zip(
    url: str, zip_path: Path, output_dir: Path, *, extract: bool = True
) -> list[Path]:
    """Download a zip file from a URL and optionally extract its contents.

    Downloads the file in streaming 4 MiB chunks. If extract is True,
    extracts all contents to the output directory and removes the zip file.

    Args:
        url: The URL to download the zip file from.
        zip_path: Path where the zip file will be saved.
        output_dir: Directory to extract the zip contents into.
            Created if it doesn't exist.
        extract: If True (default), extract the zip and delete it.
            If False, keep the zip file as-is.

    Returns:
        List of paths to the extracted files (excludes directories),
        or a single-element list containing the zip path if extract is False.

    Raises:
        requests.HTTPError: If the download request fails.
        zipfile.BadZipFile: If the downloaded file is not a valid zip.
    """
    output_dir.mkdir(parents=True, exist_ok=True)
    with requests.get(url=url, stream=True) as response:
        response.raise_for_status()
        with zip_path.open("wb") as file:
            for chunk in response.iter_content(chunk_size=4194304):  # 4 MiB
                if chunk:
                    file.write(chunk)
    if not extract:
        return [zip_path]
    with zipfile.ZipFile(zip_path, "r") as zip_file:
        names = zip_file.namelist()
        zip_file.extractall(output_dir)
    zip_path.unlink()
    return [output_dir / name for name in names if not name.endswith("/")]
