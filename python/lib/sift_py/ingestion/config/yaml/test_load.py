import pytest

from sift_py.ingestion.config.yaml import load
from sift_py.ingestion.config.yaml.error import YamlConfigError


def test__validate_flow():
    load._validate_flow(
        {
            "name": "reading",
            "channels": [
                {"name": "voltage", "data_type": "double"},
            ],
        }
    )

    with pytest.raises(YamlConfigError):
        load._validate_flow(
            {
                "name": "reading",
                "channels": [
                    {"name": "voltage", "data_type": "dubble"},
                ],
            }
        )
