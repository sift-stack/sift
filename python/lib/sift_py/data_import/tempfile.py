import os
import tempfile
from pathlib import Path


class NamedTemporaryFile:
    """
    Created a named temporary file.

    Works on both Windows and Unix systems.

    See https://stackoverflow.com/questions/23212435/permission-denied-to-write-to-my-temporary-file
    for more information on Windows compatibility.
    """

    def __init__(self, mode, suffix=""):
        self.temp_dir = tempfile.mkdtemp()
        self.name = Path(self.temp_dir) / f"tempfile{suffix}"
        self.file = open(self.name, mode)

    def __enter__(self):
        return self.file

    def __exit__(self, exc_type, exc_value, traceback):
        self.file.close()
        try:
            os.remove(self.name)
            os.rmdir(self.temp_dir)
        except FileNotFoundError:
            pass
