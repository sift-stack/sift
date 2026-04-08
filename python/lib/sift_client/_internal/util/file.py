from __future__ import annotations

import os
import struct
import warnings
import zipfile
from typing import TYPE_CHECKING

from alive_progress import alive_bar  # type: ignore[import-untyped]

import sift_client as _sift_client_module
from sift_client.errors import SiftWarning

if TYPE_CHECKING:
    from pathlib import Path

    from sift_client.transport.rest_transport import RestClient


def resolve_show_progress(*, is_sync: bool) -> bool:
    """Resolve the show_progress setting from the global config.

    Returns the global ``sift_client.config.show_progress`` value when set,
    otherwise defaults to ``is_sync``.
    """
    global_setting = _sift_client_module.config.show_progress
    if global_setting is not None:
        return global_setting
    return is_sync


def upload_file(
    signed_url: str,
    file_path: Path,
    *,
    rest_client: RestClient,
    show_progress: bool = False,
) -> dict:
    """Upload a file to a presigned URL.

    Args:
        signed_url: The presigned URL to upload to.
        file_path: Path to the file to upload.
        rest_client: The SDK rest client to use for the upload.
        show_progress: If True, display a progress spinner during upload.

    Returns:
        The parsed JSON response from the server.

    Raises:
        ValueError: If the upload request fails.
    """
    with alive_bar(
        title=f"Upload [{file_path.name}]",
        bar=None,
        spinner="dots_waves",
        spinner_length=7,
        monitor=False,
        stats=False,
        disable=not show_progress,
    ):
        with open(file_path, "rb") as f:
            response = rest_client.post(
                signed_url,
                data=f,
                headers={"Content-Disposition": f'attachment; filename="{file_path.name}"'},
            )
            if not response.ok:
                raise ValueError(f"Upload failed ({response.status_code}): {response.text}")
            return response.json()


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
            spinner="dots_waves",
            spinner_length=7,
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


def extract_parquet_footer(path: Path) -> tuple[bytes, int]:
    """Extract the Parquet footer bytes and compute the footer offset.

    Args:
        path: Path to the Parquet file.

    Returns:
        A tuple of (footer_bytes, footer_offset).

    Raises:
        ValueError: If the file is not a valid Parquet file.
    """
    with open(path, "rb") as f:
        f.seek(-8, 2)
        footer_tail = f.read(8)
        footer_len = struct.unpack("<I", footer_tail[:4])[0]
        magic = footer_tail[4:]
        if magic != b"PAR1":
            raise ValueError(f"Invalid Parquet file: missing magic bytes in {path}")
        f.seek(-(footer_len + 8), 2)
        footer_bytes = f.read(footer_len)
    return footer_bytes, os.path.getsize(path) - len(footer_bytes) - 8
