import io
from pathlib import Path

import requests


def download_remote_file(url: str, out: Path):
    with requests.get(url, stream=True) as req:
        req.raise_for_status()
        with open(out, "wb") as output_file:
            for chunk in req.iter_content(chunk_size=io.DEFAULT_BUFFER_SIZE):
                if chunk:
                    output_file.write(chunk)
