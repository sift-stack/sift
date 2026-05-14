"""Unit tests for test results models using mocks."""

from __future__ import annotations

import tempfile
import warnings
from datetime import datetime, timedelta, timezone
from unittest.mock import MagicMock, call

import pytest
from sift.test_reports.v1.test_reports_pb2 import (
    TestMeasurement as TestMeasurementProto,
)
from sift.test_reports.v1.test_reports_pb2 import (
    TestStep as TestStepProto,
)

from sift_client.sift_types.channel import Channel, ChannelDataType
from sift_client.sift_types.test_report import (
    ErrorInfo,
    NumericBounds,
    TestMeasurement,
    TestMeasurementCreate,
    TestMeasurementType,
    TestReport,
    TestStatus,
    TestStep,
    TestStepCreate,
    TestStepType,
)


@pytest.fixture
def mock_test_report(mock_client):
    """Create a mock TestReport instance for testing."""
    simulated_time = datetime.now(timezone.utc)
    test_report = TestReport(
        proto=MagicMock(),
        id_="test_report_123",
        name="Test Report with Steps and Measurements",
        test_system_name="Test System",
        test_case="Test Case",
        status=TestStatus.PASSED,
        start_time=simulated_time,
        end_time=simulated_time,
        metadata={},
        serial_number="123456",
        part_number="123456",
        system_operator="test@test.com",
        archived_date=None,
        is_archived=False,
    )
    test_report._apply_client_to_instance(mock_client)
    return test_report


@pytest.fixture
def mock_test_step(mock_client):
    """Create a mock TestStep instance for testing."""
    simulated_time = datetime.now(timezone.utc)
    test_step = TestStep(
        proto=MagicMock(),
        id_="step_123",
        test_report_id="test_report_123",
        parent_step_id=None,
        name="Step 1: Initialization",
        description="Error demo",
        step_type=TestStepType.ACTION,
        step_path="1",
        status=TestStatus.FAILED,
        start_time=simulated_time,
        end_time=simulated_time + timedelta(seconds=11),
        error_info=ErrorInfo(
            error_code=1,
            error_message="Demo error message",
        ),
        metadata={"fixture": "step", "iteration": 1.0},
    )
    test_step._apply_client_to_instance(mock_client)
    return test_step


@pytest.fixture
def mock_test_measurement(mock_client):
    """Create a mock TestMeasurement instance for testing."""
    simulated_time = datetime.now(timezone.utc)
    test_measurement = TestMeasurement(
        proto=MagicMock(),
        id_="measurement_123",
        test_step_id="step_123",
        name="Temperature Reading",
        measurement_type=TestMeasurementType.DOUBLE,
        numeric_value=25.5,
        numeric_bounds=NumericBounds(min=24, max=26),
        unit="Celsius",
        passed=True,
        timestamp=simulated_time,
        description="Expected nominal: 25.0C",
        metadata={"part_number": "PN-001", "serial_number": "SN-42"},
        channel_names=["temperature_celsius"],
    )
    test_measurement._apply_client_to_instance(mock_client)
    return test_measurement


class TestResultsTest:
    """Unit tests for test results models."""

    def test_update_test_step(self, mock_test_step, mock_client):
        """Test updating a test step."""
        # Create updated step mock
        updated_step = TestStep(
            proto=MagicMock(),
            id_="step_123",
            test_report_id="test_report_123",
            parent_step_id=None,
            name="Step 1: Initialization",
            description="Error demo w/ updated description",
            step_type=TestStepType.ACTION,
            step_path="1",
            status=TestStatus.FAILED,
            start_time=mock_test_step.start_time,
            end_time=mock_test_step.end_time,
            error_info=mock_test_step.error_info,
        )
        updated_step._apply_client_to_instance(mock_client)

        # Configure mock to return updated step
        mock_client.test_results = MagicMock()
        mock_client.test_results.update_step.return_value = updated_step

        # Update the step
        result = mock_test_step.update(
            {"description": "Error demo w/ updated description"},
        )

        # Verify the update method was called
        mock_client.test_results.update_step.assert_called_once()
        assert result.description == "Error demo w/ updated description"

    def test_update_test_measurement(self, mock_test_measurement, mock_client):
        """Test updating a test measurement."""
        updated_measurement = MagicMock()
        updated_measurement.passed = False
        updated_measurement.numeric_bounds = NumericBounds(min=10, max=20)
        mock_client.test_results.update_measurement.return_value = updated_measurement

        with MagicMock() as mock_update:
            mock_test_measurement._update = mock_update
            # Update the measurement
            update = {
                "passed": False,
                "numeric_bounds": NumericBounds(min=10, max=20),
            }
            result = mock_test_measurement.update(
                update,
                update_step=True,
            )

            # Verify the update method was called
            mock_client.test_results.update_measurement.assert_called_once_with(
                test_measurement=mock_test_measurement,
                update=update,
                update_step=True,
                log_file=None,
            )
            mock_update.assert_called_once_with(updated_measurement)
            assert result is mock_test_measurement

    def test_update_test_report(self, mock_test_report, mock_client):
        """Test updating a test report."""
        updated_report = MagicMock()
        updated_report.status = TestStatus.FAILED
        mock_client.test_results.update.return_value = updated_report
        with MagicMock() as mock_update:
            mock_test_report._update = mock_update
            # Update the report
            update = {
                "status": TestStatus.FAILED,
            }
            mock_test_report.update(
                update,
            )

            # Verify the update method was called
            mock_client.test_results.update.assert_called_once_with(
                test_report=mock_test_report,
                update=update,
                log_file=None,
            )
            mock_update.assert_called_once_with(updated_report)

    def test_update_preserves_cached_log_file(self, mock_test_report, mock_client):
        """After .update() returns, the cached _log_file survives — BaseType._update()
        only copies model_fields, and private attrs are excluded.
        """
        mock_test_report.__dict__["_log_file"] = "/tmp/cached.jsonl"

        updated = TestReport(
            id_=mock_test_report.id_,
            status=TestStatus.FAILED,
            name=mock_test_report.name,
            test_system_name=mock_test_report.test_system_name,
            test_case=mock_test_report.test_case,
            start_time=mock_test_report.start_time,
            end_time=mock_test_report.end_time,
            metadata={"k": "v"},
            is_archived=False,
        )
        updated.__dict__["_log_file"] = "/tmp/cached.jsonl"
        mock_client.test_results.update.return_value = updated

        result = mock_test_report.update({"status": TestStatus.FAILED})

        assert result is mock_test_report
        assert mock_test_report._log_file == "/tmp/cached.jsonl"
        assert mock_test_report.status == TestStatus.FAILED
        assert mock_test_report.metadata == {"k": "v"}

    def test_archive_test_report(self, mock_test_report, mock_client):
        """Test archiving a test report."""
        # Create archived report mock
        archived_report = MagicMock()
        archived_report.is_archived = True
        mock_client.test_results.archive.return_value = archived_report
        with MagicMock() as mock_update:
            mock_test_report._update = mock_update
            # Archive the report
            mock_test_report.archive()

            # Verify the archive method was called
            mock_client.test_results.archive.assert_called_once()
            mock_update.assert_called_once_with(archived_report)

    def test_numeric_bounds_eq(self):
        """Test the equality of NumericBounds."""
        bounds1 = NumericBounds(min=10, max=20)
        bounds2 = NumericBounds(min=10, max=20)
        bounds3 = NumericBounds(min=10, max=30)
        assert bounds1 == bounds2
        assert bounds1 != bounds3

    def test_report_steps(self, mock_test_report, mock_test_step, mock_client):
        """Test the steps property of TestReport."""
        mock_client.test_results.list_steps.return_value = [mock_test_step]
        steps = mock_test_report.steps
        assert len(steps) == 1
        assert steps[0] == mock_test_step

    def test_step_measurements(self, mock_test_step, mock_test_measurement, mock_client):
        """Test the measurements property of TestStep."""
        mock_client.test_results.list_measurements.return_value = [mock_test_measurement]
        measurements = mock_test_step.measurements
        assert len(measurements) == 1
        assert measurements[0] == mock_test_measurement

    def test_attachments_property_fetches_files(self, mock_test_report, mock_client):
        """Test that attachments property fetches files from client.file_attachments API."""
        # Create mock remote files
        mock_remote_file = MagicMock()
        mock_remote_file.entity_id = mock_test_report.id_
        mock_remote_files = [mock_remote_file]

        # Mock the file_attachments API
        mock_client.file_attachments.list_.return_value = mock_remote_files

        # Access the attachments property (it's a property, not a method)
        result = mock_test_report.attachments

        # Verify file_attachments.list_ was called with correct parameters
        mock_client.file_attachments.list_.assert_called_once_with(
            entities=[mock_test_report],
        )

        # Verify result
        assert result == mock_remote_files

    def test_upload_attachment(self, mock_test_report, mock_test_step, mock_client):
        """Ensure test report and step have FileAttachmentsMixin and it is called correctly."""
        # Create mock file attachment to be returned
        mock_file_attachment = MagicMock()
        mock_file_attachment.description = "Test upload to test report"
        mock_file_attachment.entity_id = mock_test_report.id_
        mock_client.file_attachments.upload.return_value = mock_file_attachment

        # Create a temporary test file
        with tempfile.NamedTemporaryFile(mode="w", suffix=".txt", delete=False) as tmp:
            tmp.write("Test file content\n")
            tmp_path = tmp.name

        _ = mock_test_report.upload_attachment(
            path=tmp_path, description="Test upload to test report"
        )
        _ = mock_test_step.upload_attachment(path=tmp_path, description="Test upload to test step")

        # Verify file_attachments.upload was called with correct parameters
        mock_client.file_attachments.upload.assert_has_calls(
            [
                call(
                    path=tmp_path,
                    entity=mock_test_report,
                    metadata=None,
                    description="Test upload to test report",
                    organization_id=None,
                ),
                call(
                    path=tmp_path,
                    entity=mock_test_step,
                    metadata=None,
                    description="Test upload to test step",
                    organization_id=None,
                ),
            ]
        )

    def test_measurement_description_truncates_with_warning(self):
        """Description over the server limit is truncated and a UserWarning is raised."""
        over_limit = "x" * 2001
        with pytest.warns(UserWarning, match="exceeds 2000 characters"):
            create = TestMeasurementCreate(
                test_step_id="step_123",
                name="m",
                passed=True,
                timestamp=datetime.now(timezone.utc),
                numeric_value=1.0,
                description=over_limit,
            )
        assert create.description is not None
        assert len(create.description) == 2000

    def test_measurement_description_at_limit_is_not_truncated(self):
        """A description exactly at the limit should not warn or truncate."""
        at_limit = "x" * 2000
        with warnings.catch_warnings():
            warnings.simplefilter("error")  # any warning fails the test
            create = TestMeasurementCreate(
                test_step_id="step_123",
                name="m",
                passed=True,
                timestamp=datetime.now(timezone.utc),
                numeric_value=1.0,
                description=at_limit,
            )
        assert create.description == at_limit

    def test_measurement_channel_names_accepts_strings(self):
        """channel_names accepts a homogeneous list of channel name strings."""
        create = TestMeasurementCreate(
            test_step_id="step_123",
            name="m",
            passed=True,
            timestamp=datetime.now(timezone.utc),
            numeric_value=1.0,
            channel_names=["temperature_celsius", "pressure_psi"],
        )
        assert create.channel_names == ["temperature_celsius", "pressure_psi"]
        proto = create.to_proto()
        assert list(proto.channel_names) == ["temperature_celsius", "pressure_psi"]

    def test_measurement_channel_names_accepts_channels(self):
        """channel_names accepts a homogeneous list of Channel instances; names are extracted at serialization."""
        now = datetime.now(timezone.utc)

        def _channel(name: str) -> Channel:
            return Channel(
                proto=MagicMock(),
                id_=f"channel_{name}",
                name=name,
                data_type=ChannelDataType.DOUBLE,
                description="",
                unit="",
                bit_field_elements=[],
                enum_types={},
                asset_id="asset_1",
                created_date=now,
                modified_date=now,
                created_by_user_id="user1",
                modified_by_user_id="user1",
            )

        create = TestMeasurementCreate(
            test_step_id="step_123",
            name="m",
            passed=True,
            timestamp=now,
            numeric_value=1.0,
            channel_names=[_channel("temperature_celsius"), _channel("pressure_psi")],
        )
        proto = create.to_proto()
        assert list(proto.channel_names) == ["temperature_celsius", "pressure_psi"]

    def test_measurement_create_to_proto_writes_new_fields(self):
        """to_proto carries description, metadata, and channel_names onto the proto."""
        create = TestMeasurementCreate(
            test_step_id="step_123",
            name="m",
            passed=True,
            timestamp=datetime.now(timezone.utc),
            numeric_value=1.0,
            description="note",
            metadata={"pn": "PN-001", "count": 3, "flag": True},
            channel_names=["chan_a", "chan_b"],
        )
        proto = create.to_proto()
        assert proto.description == "note"
        assert list(proto.channel_names) == ["chan_a", "chan_b"]
        proto_keys = {m.key.name for m in proto.metadata}
        assert proto_keys == {"pn", "count", "flag"}

    def test_measurement_from_proto_round_trips_new_fields(self):
        """A proto with the new fields populated round-trips into TestMeasurement."""
        ts = datetime.now(timezone.utc)
        source = TestMeasurementCreate(
            test_step_id="step_123",
            name="m",
            passed=True,
            timestamp=ts,
            numeric_value=1.0,
            description="note",
            metadata={"pn": "PN-001", "count": 3},
            channel_names=["chan_a"],
        ).to_proto()
        source.measurement_id = "measurement_456"
        source.test_report_id = "report_789"

        measurement = TestMeasurement._from_proto(source)

        assert measurement.description == "note"
        assert measurement.metadata == {"pn": "PN-001", "count": 3}
        assert measurement.channel_names == ["chan_a"]

    def test_measurement_from_proto_handles_absent_new_fields(self):
        """Proto with unset description/metadata/channel_names yields None on the model."""
        proto = TestMeasurementProto(
            measurement_id="measurement_abc",
            measurement_type=TestMeasurementType.DOUBLE.value,
            name="m",
            test_step_id="step_123",
            test_report_id="report_789",
            passed=True,
        )
        proto.timestamp.FromDatetime(datetime.now(timezone.utc))
        measurement = TestMeasurement._from_proto(proto)
        assert measurement.description is None
        assert measurement.metadata is None
        assert measurement.channel_names is None

    def test_step_create_to_proto_writes_metadata(self):
        """TestStepCreate.to_proto carries metadata onto the proto."""
        now = datetime.now(timezone.utc)
        create = TestStepCreate(
            test_report_id="report_789",
            name="Step",
            step_type=TestStepType.ACTION,
            step_path="1",
            status=TestStatus.IN_PROGRESS,
            start_time=now,
            end_time=now,
            metadata={"pn": "PN-001", "count": 3, "flag": True},
        )
        proto = create.to_proto()
        proto_keys = {m.key.name for m in proto.metadata}
        assert proto_keys == {"pn", "count", "flag"}

    def test_step_from_proto_round_trips_metadata(self):
        """A proto with metadata populated round-trips into TestStep."""
        now = datetime.now(timezone.utc)
        source = TestStepCreate(
            test_report_id="report_789",
            name="Step",
            step_type=TestStepType.ACTION,
            step_path="1",
            status=TestStatus.IN_PROGRESS,
            start_time=now,
            end_time=now,
            metadata={"pn": "PN-001", "count": 3},
        ).to_proto()
        source.test_step_id = "step_456"

        step = TestStep._from_proto(source)

        assert step.metadata == {"pn": "PN-001", "count": 3}

    def test_step_from_proto_handles_absent_metadata(self):
        """Proto with unset metadata yields None on the model."""
        proto = TestStepProto(
            test_step_id="step_abc",
            test_report_id="report_789",
            name="Step",
            step_type=TestStepType.ACTION.value,
            step_path="1",
            status=TestStatus.IN_PROGRESS.value,
        )
        proto.start_time.FromDatetime(datetime.now(timezone.utc))
        proto.end_time.FromDatetime(datetime.now(timezone.utc))
        step = TestStep._from_proto(proto)
        assert step.metadata is None
