"""Tests for the data imports low-level wrapper."""

from __future__ import annotations

import pytest
from sift.data_imports.v2.data_imports_pb2 import CreateDataImportFromUploadRequest

from sift_client._internal.low_level_wrappers.data_imports import _set_config_on_request
from sift_client.sift_types.channel import ChannelDataType
from sift_client.sift_types.data_import import McapDataColumn, McapImportConfig


class TestSetConfigOnRequest:
    def test_mcap_config_sets_mcap_field(self):
        request = CreateDataImportFromUploadRequest()
        config = McapImportConfig(
            asset_name="my_asset",
            data=[
                McapDataColumn(
                    topic="/imu", field_path="orientation.x", data_type=ChannelDataType.DOUBLE
                )
            ],
        )

        _set_config_on_request(request, config)

        assert request.HasField("mcap_config")
        assert request.mcap_config.asset_name == "my_asset"
        assert request.mcap_config.data[0].ros2.field_path == "orientation.x"

    def test_unknown_config_type_raises(self):
        request = CreateDataImportFromUploadRequest()
        with pytest.raises(TypeError, match="Unsupported import config type"):
            _set_config_on_request(request, object())  # type: ignore[arg-type]
