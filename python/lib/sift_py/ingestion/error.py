"""
Errors specific to the `sift_py` ingestion module.
"""


class YamlConfigError(Exception):
    """
    Used when the YAML config has missing or invalid properties.
    """

    message: str

    def __init__(self, message: str):
        super().__init__(message)
