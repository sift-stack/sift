"""Tests for the data imports low-level wrapper."""

from __future__ import annotations

from unittest.mock import MagicMock

import pytest
from sift.data_imports.v2.data_imports_pb2 import (
    DATA_TYPE_KEY_CSV,
    DATA_TYPE_KEY_MCAP,
    CreateDataImportFromUploadRequest,
    CreateDataImportFromUploadResponse,
)

from sift_client._internal.low_level_wrappers.data_imports import (
    DataImportsLowLevelClient,
    _set_config_on_request,
)
from sift_client.sift_types.channel import ChannelDataType
from sift_client.sift_types.data_import import (
    CsvDataColumn,
    CsvImportConfig,
    CsvTimeColumn,
    McapDataColumn,
    McapImportConfig,
    TimeFormat,
    UlogImportConfig,
)


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


def _csv_config() -> CsvImportConfig:
    return CsvImportConfig(
        asset_name="from-file",
        run_name="from-file",
        run_id="from-file",
        time_column=CsvTimeColumn(column=1, format=TimeFormat.ABSOLUTE_UNIX_NANOSECONDS),
        data_columns=[CsvDataColumn(column=2, name="v", data_type=ChannelDataType.DOUBLE)],
    )


@pytest.mark.asyncio
async def test_create_batch_from_upload_sets_batch_config():
    """A batch upload carries one config per archive member and the shared run."""
    captured: dict[str, CreateDataImportFromUploadRequest] = {}

    class _Stub:
        async def CreateDataImportFromUpload(self, request):  # noqa: N802
            captured["request"] = request
            return CreateDataImportFromUploadResponse(data_import_id="id", upload_url="url")

    client = DataImportsLowLevelClient(grpc_client=MagicMock())
    client._grpc_client.get_stub = MagicMock(return_value=_Stub())

    await client.create_batch_from_upload(
        {
            "a.csv": _csv_config(),
            "log.mcap": McapImportConfig(asset_name="from-file", data=[]),
        },
        asset_name="shared-asset",
        run_id="run-1",
        leave_run_open=True,
    )

    request = captured["request"]
    assert request.leave_run_open is True
    assert not request.HasField("csv_config")
    batch = request.batch_config
    assert batch.asset_name == "shared-asset"
    assert batch.run_id == "run-1"
    assert batch.file_configs["a.csv"].type == DATA_TYPE_KEY_CSV
    assert batch.file_configs["a.csv"].csv_config.asset_name == ""
    assert batch.file_configs["a.csv"].csv_config.run_id == ""
    assert batch.file_configs["a.csv"].csv_config.data_columns[2].name == "v"
    assert batch.file_configs["log.mcap"].type == DATA_TYPE_KEY_MCAP
    assert batch.file_configs["log.mcap"].mcap_config.asset_name == ""


@pytest.mark.asyncio
async def test_create_batch_from_upload_rejects_empty_and_run_metadata():
    client = DataImportsLowLevelClient(grpc_client=MagicMock())

    with pytest.raises(ValueError, match="at least one file"):
        await client.create_batch_from_upload({}, asset_name="a")

    with pytest.raises(ValueError, match="info_keys"):
        await client.create_batch_from_upload(
            {"log.ulg": UlogImportConfig(asset_name="a", info_keys=["git_hash"])},
            asset_name="a",
        )
