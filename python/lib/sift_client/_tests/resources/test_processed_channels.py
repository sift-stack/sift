"""Unit tests for the processed channels API."""

from __future__ import annotations

import warnings
from typing import Any
from unittest.mock import AsyncMock, MagicMock, patch

import grpc
import pandas as pd
import pytest
import requests

from sift_client._internal.low_level_wrappers.processed_channels import (
    ProcessedChannelsLowLevelClient,
)
from sift_client.errors import SiftExperimentalWarning
from sift_client.resources import ProcessedChannelsAPIAsync
from sift_client.sift_types.processed_channel import (
    ProcessedChannelCode,
    ProcessedChannelInput,
    ProcessedChannelRequestError,
    ProcessedChannelUpload,
    ProcessedChannelUploadError,
    ProcessedChannelWrite,
    ProcessedChannelWriteStatus,
)
from sift_client.sift_types.run import Run

ASSET_ID = "6f1d3c1e-4a49-4a43-9b1f-0d1c2b3a4f5e"
RUN_ID = "0c9e2b7a-1d2f-4e3a-8b5c-6d7e8f9a0b1c"
WRITE_IDS = ["a6a1f7f3-6c1f-4f39-a7cc-7d7c3bb9f201", "a6a1f7f3-6c1f-4f39-a7cc-7d7c3bb9f202"]


def _write_json(write_id: str, status: str = "PENDING", **extra: Any) -> dict[str, Any]:
    body = {
        "processedChannelWriteId": write_id,
        "channelId": f"channel-{write_id[-1]}",
        "assetId": ASSET_ID,
        "runId": RUN_ID,
        "status": f"PROCESSED_CHANNEL_WRITE_STATUS_{status}",
        "sequence": 1,
        "isCurrent": False,
        "fingerprint": "f",
        "createdDate": "2026-09-29T13:05:00Z",
        "createdByUserId": "user-1",
    }
    body.update(extra)
    return body


def _writes(status: str = "PENDING", **extra: Any) -> list[ProcessedChannelWrite]:
    return [ProcessedChannelWrite._from_rest(_write_json(i, status, **extra)) for i in WRITE_IDS]


def _df() -> pd.DataFrame:
    return pd.DataFrame(
        {"battery_health": [0.98, 0.97], "battery_health_sigma": [0.01, 0.02]},
        index=pd.to_datetime(["2026-09-29T13:00:00Z", "2026-09-29T13:00:01Z"]),
    )


class TestProcessedChannelWrite:
    def test_from_rest(self):
        write = ProcessedChannelWrite._from_rest(
            _write_json(
                WRITE_IDS[0],
                "COMPLETE",
                sequence=3,
                isCurrent=True,
                pointCount="42",
                dataStartTime="2026-09-29T13:00:00.000000001Z",
                code={"ref": "jobs/health.py@abc", "hash": "h"},
                supersedesProcessedChannelWriteId=WRITE_IDS[1],
            )
        )

        assert write.id_ == WRITE_IDS[0]
        assert write.status == ProcessedChannelWriteStatus.COMPLETE
        assert write.sequence == 3
        assert write.is_current
        assert write.point_count == 42
        assert write.code == ProcessedChannelCode(ref="jobs/health.py@abc", hash="h")
        assert write.supersedes_processed_channel_write_id == WRITE_IDS[1]
        assert write.data_start_time is not None
        assert write.error_message is None

    def test_pending_write_has_no_completion_fields(self):
        write = ProcessedChannelWrite._from_rest(_write_json(WRITE_IDS[0]))

        assert write.status == ProcessedChannelWriteStatus.PENDING
        assert write.point_count is None
        assert write.completed_date is None
        assert write.code is None


class TestLowLevelClient:
    @pytest.mark.asyncio
    async def test_create_sends_camel_case_json(self):
        rest_client = MagicMock()
        response = MagicMock(status_code=200)
        response.json.return_value = {
            "uploadUrl": "https://api/upload",
            "dataImportId": "import-1",
            "writes": [_write_json(i) for i in WRITE_IDS],
        }
        rest_client.post.return_value = response
        from sift.data_imports.v2.data_imports_pb2 import ParquetFlatDatasetConfig

        client = ProcessedChannelsLowLevelClient(rest_client=rest_client)
        upload_url, data_import_id, writes = await client.create(
            asset_id=ASSET_ID,
            run_id=RUN_ID,
            parquet=ParquetFlatDatasetConfig(),
            footer_offset=123,
            footer_length=45,
            code=ProcessedChannelCode(ref="r", hash="h"),
            inputs=[ProcessedChannelInput(channel_id="c", run_id=RUN_ID)],
            metadata={"window": 5},
        )

        endpoint = rest_client.post.call_args.args[0]
        body = rest_client.post.call_args.kwargs["json"]
        assert endpoint == "/api/v1/processed-channels"
        assert body["assetId"] == ASSET_ID
        assert body["runId"] == RUN_ID
        assert body["footerOffset"] == "123"
        assert body["footerLength"] == 45
        assert body["code"] == {"ref": "r", "hash": "h"}
        assert body["inputs"] == [{"channelId": "c", "runId": RUN_ID}]
        assert body["metadata"][0]["key"]["name"] == "window"
        assert (upload_url, data_import_id) == ("https://api/upload", "import-1")
        assert [w.id_ for w in writes] == WRITE_IDS

    @pytest.mark.asyncio
    async def test_list_writes_pages(self):
        rest_client = MagicMock()
        first = MagicMock(status_code=200)
        first.json.return_value = {"writes": [_write_json(WRITE_IDS[0])], "nextPageToken": "t"}
        second = MagicMock(status_code=200)
        second.json.return_value = {"writes": [_write_json(WRITE_IDS[1])], "nextPageToken": ""}
        rest_client.get.side_effect = [first, second]

        client = ProcessedChannelsLowLevelClient(rest_client=rest_client)
        writes = await client.list_all_writes(query_filter="status == 'x'")

        assert [w.id_ for w in writes] == WRITE_IDS
        params = [c.kwargs["params"] for c in rest_client.get.call_args_list]
        assert params[0]["filter"] == "status == 'x'"
        assert params[1]["pageToken"] == "t"

    @pytest.mark.asyncio
    async def test_refusal_raises_request_error(self):
        response = _response(
            400,
            b'{"code":9,"message":"failed precondition: processed channels are not enabled '
            b'for this organization (org-1)","details":[]}',
        )
        rest_client = MagicMock()
        rest_client.post.return_value = response
        from sift.data_imports.v2.data_imports_pb2 import ParquetFlatDatasetConfig

        client = ProcessedChannelsLowLevelClient(rest_client=rest_client)
        with pytest.raises(ProcessedChannelRequestError) as refused:
            await client.create(
                asset_id=ASSET_ID,
                run_id=RUN_ID,
                parquet=ParquetFlatDatasetConfig(),
                footer_offset=1,
                footer_length=1,
                code=ProcessedChannelCode(ref="r"),
            )

        assert refused.value.code == grpc.StatusCode.FAILED_PRECONDITION
        assert refused.value.server_message.endswith("not enabled for this organization (org-1)")
        assert str(refused.value) == (
            "Sift refused the request (HTTP 400): failed precondition: processed channels are "
            "not enabled for this organization (org-1)"
        )
        assert isinstance(refused.value, requests.HTTPError)
        assert refused.value.response is response
        assert refused.value.__cause__ is None

    @pytest.mark.asyncio
    async def test_error_without_status_raises_http_error(self):
        rest_client = MagicMock()
        rest_client.get.return_value = _response(502, b"bad gateway")

        client = ProcessedChannelsLowLevelClient(rest_client=rest_client)
        with pytest.raises(requests.HTTPError, match="bad gateway") as failed:
            await client.get_write(WRITE_IDS[0])

        assert not isinstance(failed.value, ProcessedChannelRequestError)


def _response(status_code: int, content: bytes) -> requests.Response:
    response = requests.Response()
    response.status_code = status_code
    response._content = content
    response.url = "https://api/api/v1/processed-channels"
    return response


@pytest.fixture
def api() -> ProcessedChannelsAPIAsync:
    client = MagicMock()
    client.async_.assets.get = AsyncMock(return_value=MagicMock(_id_or_error=ASSET_ID))
    resource = ProcessedChannelsAPIAsync(client)
    resource._low_level_client = MagicMock()
    resource._low_level_client.create = AsyncMock(
        return_value=("https://api/upload", "import-1", _writes())
    )
    resource._low_level_client.list_all_writes = AsyncMock(return_value=_writes("COMPLETE"))
    return resource


@pytest.fixture
def uploaded():
    with patch(
        "sift_client.resources.processed_channels.upload_file",
        return_value={"jobId": "job-1"},
    ) as upload_file:
        yield upload_file


class TestUpload:
    @pytest.mark.asyncio
    async def test_non_blocking_returns_pending_upload(self, api, uploaded):
        with pytest.warns(SiftExperimentalWarning):
            upload = await api.upload(
                _df(), asset=ASSET_ID, run=RUN_ID, code="r", wait=False, show_progress=False
            )

        assert upload.data_import_id == "import-1"
        assert upload.job_id == "job-1"
        assert not upload.is_complete
        assert uploaded.call_args.args[0] == "https://api/upload"
        api._low_level_client.list_all_writes.assert_not_called()

        create = api._low_level_client.create.call_args.kwargs
        assert create["asset_id"] == ASSET_ID
        assert create["run_id"] == RUN_ID
        assert create["code"] == ProcessedChannelCode(ref="r")
        assert [c.channel_config.name for c in create["parquet"].data_columns] == [
            "battery_health",
            "battery_health_sigma",
        ]
        assert create["footer_length"] > 0

    @pytest.mark.asyncio
    async def test_blocking_waits_for_completion(self, api, uploaded):
        api._low_level_client.list_all_writes.side_effect = [_writes(), _writes("COMPLETE")]

        with pytest.warns(SiftExperimentalWarning):
            upload = await api.upload(
                _df(),
                asset=ASSET_ID,
                run=RUN_ID,
                units={"battery_health": "pct"},
                code="r",
                polling_interval_secs=0,
                show_progress=False,
            )

        assert upload.succeeded
        assert api._low_level_client.list_all_writes.call_count == 2
        query_filter = api._low_level_client.list_all_writes.call_args.kwargs["query_filter"]
        assert query_filter == (
            f"processed_channel_write_id in ['{WRITE_IDS[0]}','{WRITE_IDS[1]}']"
        )
        units = {
            c.channel_config.name: c.channel_config.units
            for c in api._low_level_client.create.call_args.kwargs["parquet"].data_columns
        }
        assert units == {"battery_health": "pct", "battery_health_sigma": ""}

    @pytest.mark.asyncio
    async def test_blocking_raises_on_failed_writes(self, api, uploaded):
        api._low_level_client.list_all_writes.return_value = _writes(
            "FAILED", errorMessage="points outside the ad hoc run"
        )

        with pytest.warns(SiftExperimentalWarning), pytest.raises(
            ProcessedChannelUploadError, match="2 of 2 .* points outside the ad hoc run"
        ) as error:
            await api.upload(_df(), asset=ASSET_ID, run=RUN_ID, code="r", show_progress=False)

        assert len(error.value.upload.failed_writes) == 2

    @pytest.mark.asyncio
    async def test_resolves_asset_name_and_run_object(self, api, uploaded):
        run = MagicMock(spec=Run)
        run._id_or_error = RUN_ID

        with pytest.warns(SiftExperimentalWarning):
            await api.upload(
                _df(), asset="satellite-01", run=run, code="r", wait=False, show_progress=False
            )

        api.client.async_.assets.get.assert_awaited_once_with(name="satellite-01")
        assert api._low_level_client.create.call_args.kwargs["run_id"] == RUN_ID

    @pytest.mark.asyncio
    async def test_default_code(self, api, uploaded):
        with patch(
            "sift_client.resources.processed_channels.default_code",
            return_value=ProcessedChannelCode(ref="script.py@abc", hash="h"),
        ), pytest.warns(SiftExperimentalWarning):
            await api.upload(_df(), asset=ASSET_ID, run=RUN_ID, wait=False, show_progress=False)

        assert api._low_level_client.create.call_args.kwargs["code"].ref == "script.py@abc"

    @pytest.mark.asyncio
    async def test_invalid_dataframe_creates_nothing(self, api, uploaded):
        df = pd.DataFrame({"v": [1.0, 2.0]})

        with pytest.warns(SiftExperimentalWarning), pytest.raises(ValueError, match="datetimes"):
            await api.upload(df, asset=ASSET_ID, run=RUN_ID, code="r", show_progress=False)

        api._low_level_client.create.assert_not_called()
        uploaded.assert_not_called()


class TestWaitUntilComplete:
    @pytest.mark.asyncio
    async def test_timeout(self, api):
        api._low_level_client.list_all_writes.return_value = _writes()
        upload = ProcessedChannelUpload(data_import_id="import-1", writes=_writes())

        with pytest.raises(TimeoutError, match="import-1"):
            await api.wait_until_complete(
                upload, polling_interval_secs=0, timeout_secs=0, show_progress=False
            )

    @pytest.mark.asyncio
    async def test_failures_returned_when_not_raising(self, api):
        api._low_level_client.list_all_writes.return_value = _writes("FAILED")
        upload = ProcessedChannelUpload(data_import_id="import-1", writes=_writes())

        finished = await api.wait_until_complete(
            upload, raise_on_failure=False, show_progress=False
        )

        assert finished.is_complete
        assert not finished.succeeded
        assert len(finished.failed_writes) == 2


class TestSyncClient:
    def test_blocking_and_non_blocking_through_the_client(self, uploaded):
        from sift_client import SiftClient
        from sift_client.transport import SiftConnectionConfig

        client = SiftClient(
            connection_config=SiftConnectionConfig(
                api_key="test", grpc_url="test.invalid:0", rest_url="http://test.invalid"
            )
        )
        with patch.object(
            ProcessedChannelsLowLevelClient,
            "create",
            new_callable=AsyncMock,
            return_value=("https://api/upload", "import-1", _writes()),
        ), patch.object(
            ProcessedChannelsLowLevelClient,
            "list_all_writes",
            new_callable=AsyncMock,
            return_value=_writes("COMPLETE"),
        ), warnings.catch_warnings(record=True) as caught:
            warnings.simplefilter("always")
            done = client.processed_channel.upload(
                _df(),
                asset=ASSET_ID,
                run=RUN_ID,
                units={"battery_health": "pct"},
                code="r",
                show_progress=False,
            )
            started = client.processed_channel.upload(
                _df(), asset=ASSET_ID, run=RUN_ID, code="r", wait=False, show_progress=False
            )
            assert not started.is_complete
            started.wait_until_complete(polling_interval_secs=0)

        assert done.succeeded
        assert started.succeeded
        # One warning per client.processed_channel access above, each pointing at this file.
        experimental = [w for w in caught if issubclass(w.category, SiftExperimentalWarning)]
        assert [w.filename for w in experimental] == [__file__, __file__]


class TestListWrites:
    @pytest.mark.asyncio
    async def test_builds_filter(self, api):
        await api.list_writes(
            channels=["c1"],
            runs=[RUN_ID],
            status=ProcessedChannelWriteStatus.COMPLETE,
            is_current=True,
            filter_query="sequence > 1 || sequence == 0",
            limit=5,
        )

        kwargs = api._low_level_client.list_all_writes.call_args.kwargs
        assert kwargs["query_filter"] == (
            "channel_id in ['c1'] && "
            f"run_id in ['{RUN_ID}'] && "
            "status == 'PROCESSED_CHANNEL_WRITE_STATUS_COMPLETE' && "
            "is_current == true && "
            "(sequence > 1 || sequence == 0)"
        )
        assert kwargs["max_results"] == 5
