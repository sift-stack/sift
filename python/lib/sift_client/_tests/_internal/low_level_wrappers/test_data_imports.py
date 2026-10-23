"""Tests for the data imports low-level wrapper."""

from __future__ import annotations

from unittest.mock import MagicMock

import pytest
from sift.data_imports.v2.data_imports_pb2 import (
    CreateDataImportFromUploadRequest,
    CreateDataImportFromUploadResponse,
)

from sift_client._internal.low_level_wrappers.data_imports import (
    DataImportsLowLevelClient,
    _set_config_on_request,
)
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


@pytest.mark.asyncio
async def test_create_from_upload_sets_leave_run_open():
    """leave_run_open is copied onto the upload request."""
    captured: dict[str, CreateDataImportFromUploadRequest] = {}

    class _Stub:
        async def CreateDataImportFromUpload(self, request):  # noqa: N802
            captured["request"] = request
            return CreateDataImportFromUploadResponse(data_import_id="id", upload_url="url")

    client = DataImportsLowLevelClient(grpc_client=MagicMock())
    client._grpc_client.get_stub = MagicMock(return_value=_Stub())
    config = McapImportConfig(asset_name="a", data=[])

    await client.create_from_upload(config, leave_run_open=True)

    assert captured["request"].leave_run_open is True
