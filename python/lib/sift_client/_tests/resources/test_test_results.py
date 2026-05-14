from __future__ import annotations

import uuid
from datetime import datetime, timedelta, timezone
from pathlib import Path
from typing import ClassVar
from unittest.mock import AsyncMock, MagicMock, patch

import grpc
import pytest
import pytest_asyncio
from grpc import aio as aiogrpc

from sift_client._internal.low_level_wrappers.test_results import TestResultsLowLevelClient
from sift_client.resources import TestResultsAPI, TestResultsAPIAsync
from sift_client.sift_types.test_report import (
    ErrorInfo,
    NumericBounds,
    TestMeasurement,
    TestMeasurementCreate,
    TestMeasurementType,
    TestReport,
    TestReportUpdate,
    TestStatus,
    TestStep,
    TestStepCreate,
    TestStepType,
)
from sift_client.util import cel_utils as cel

pytestmark = pytest.mark.integration


def compare_test_report_fields(simulated: TestReport, actual: TestReport) -> None:
    """Compare simulated and actual TestReport fields (excluding id_)."""
    assert simulated.status == actual.status
    assert simulated.name == actual.name
    assert simulated.test_system_name == actual.test_system_name
    assert simulated.test_case == actual.test_case
    assert simulated.serial_number == actual.serial_number
    assert simulated.part_number == actual.part_number
    assert simulated.system_operator == actual.system_operator
    assert simulated.start_time == actual.start_time
    assert simulated.end_time == actual.end_time


def compare_test_step_fields(simulated: TestStep, actual: TestStep) -> None:
    """Compare simulated and actual TestStep fields (excluding id_)."""
    assert simulated.name == actual.name
    assert simulated.description == actual.description
    assert simulated.step_type == actual.step_type
    assert simulated.step_path == actual.step_path
    assert simulated.status == actual.status
    assert simulated.start_time == actual.start_time
    assert simulated.end_time == actual.end_time
    assert simulated.metadata == actual.metadata


def compare_test_measurement_fields(simulated: TestMeasurement, actual: TestMeasurement) -> None:
    """Compare simulated and actual TestMeasurement fields (excluding id_)."""
    assert simulated.name == actual.name
    assert simulated.measurement_type == actual.measurement_type
    assert simulated.numeric_value == actual.numeric_value
    assert simulated.string_value == actual.string_value
    assert simulated.boolean_value == actual.boolean_value
    assert simulated.passed == actual.passed
    assert simulated.timestamp == actual.timestamp
    assert simulated.description == actual.description
    assert simulated.metadata == actual.metadata
    assert simulated.channel_names == actual.channel_names


def test_client_binding(sift_client):
    assert sift_client.test_results
    assert isinstance(sift_client.test_results, TestResultsAPI)
    assert sift_client.async_.test_results
    assert isinstance(sift_client.async_.test_results, TestResultsAPIAsync)


class TestResultsTest:
    test_reports: ClassVar[dict[str, TestReport]] = {}
    test_steps: ClassVar[dict[str, TestStep]] = {}
    test_measurements: ClassVar[dict[str, TestMeasurement]] = {}

    def test_create_test_report(self, sift_client, nostromo_run, tmp_path):
        # Create a test report
        simulated_time = datetime.now(timezone.utc)
        report_data = {
            "status": TestStatus.PASSED,
            "name": "Test Report with Steps and Measurements",
            "test_system_name": "Test System",
            "test_case": "Test Case",
            "serial_number": str(uuid.uuid4()),
            "part_number": "1234567890",
            "start_time": simulated_time,
            "end_time": simulated_time,
            "run_id": nostromo_run.id_,
        }

        # First, create with log_file to get simulated response
        log_file = tmp_path / "test_log.jsonl"
        simulated_report = sift_client.test_results.create(report_data, log_file=log_file)

        # Verify log file was created and contains content
        assert log_file.exists()
        log_content = log_file.read_text()
        assert "[CreateTestReport:" in log_content

        # Verify simulated report has an id and expected fields
        assert simulated_report.id_ is not None

        # Now create the real report
        test_report = sift_client.test_results.create(report_data)

        # Compare simulated vs actual (fields should match except for id_)
        compare_test_report_fields(simulated_report, test_report)

        assert test_report.id_ is not None
        assert test_report.run_id == nostromo_run.id_
        self.test_reports["basic_test_report"] = test_report

    def test_create_test_steps(self, sift_client, tmp_path):
        test_report = self.test_reports.get("basic_test_report")
        if not test_report:
            pytest.skip("Need to create a test report first")
        simulated_time = test_report.start_time
        log_file = tmp_path / "test_steps_log.jsonl"

        # Test step 1 with log_file comparison
        step1_data = TestStepCreate(
            test_report_id=test_report.id_,
            name="Step 1: Initialization",
            description="Initialize the test environment",
            step_type=TestStepType.ACTION,
            step_path="1",
            status=TestStatus.PASSED,
            start_time=simulated_time,
            end_time=simulated_time + timedelta(seconds=10),
            metadata={"phase": "init", "iteration": 1},
        )

        # Create simulated step first
        simulated_step1 = sift_client.test_results.create_step(step1_data, log_file=log_file)
        assert simulated_step1.id_ is not None
        assert log_file.exists()
        assert "[CreateTestStep:" in log_file.read_text()

        # Create actual step
        step1 = sift_client.test_results.create_step(step1_data)
        compare_test_step_fields(simulated_step1, step1)

        simulated_time = simulated_time + timedelta(seconds=10.1)
        # Create a step using a dict - test log_file with dict input
        step1_1_data = {
            "test_report_id": test_report.id_,
            "parent_step_id": step1.id_,
            "name": "Step 1.1: Substep 1",
            "description": "Substep 1 of Step 1",
            "step_type": TestStepType.ACTION,
            "step_path": "1.1",
            "status": TestStatus.PASSED,
            "start_time": simulated_time,
            "end_time": simulated_time + timedelta(seconds=10),
        }
        simulated_step1_1 = sift_client.test_results.create_step(step1_1_data, log_file=log_file)
        assert simulated_step1_1.id_ is not None
        step1_1 = sift_client.test_results.create_step(step1_1_data)
        compare_test_step_fields(simulated_step1_1, step1_1)

        simulated_time = simulated_time + timedelta(seconds=10.1)

        step2_data = TestStepCreate(
            test_report_id=test_report.id_,
            name="Step 2: Data Collection",
            description="Collect sensor data",
            step_type=TestStepType.ACTION,
            step_path="2",
            status=TestStatus.PASSED,
            start_time=simulated_time,
            end_time=simulated_time + timedelta(seconds=10),
        )
        simulated_step2 = sift_client.test_results.create_step(step2_data, log_file=log_file)
        assert simulated_step2.id_ is not None
        step2 = sift_client.test_results.create_step(step2_data)
        compare_test_step_fields(simulated_step2, step2)

        simulated_time = simulated_time + timedelta(seconds=10.1)
        step3_data = TestStepCreate(
            test_report_id=test_report.id_,
            name="Step 3: Validation",
            description="Validate collected data",
            step_type=TestStepType.ACTION,
            step_path="3",
            status=TestStatus.IN_PROGRESS,
            start_time=simulated_time,
            end_time=simulated_time + timedelta(seconds=10),
        )
        simulated_step3 = sift_client.test_results.create_step(step3_data, log_file=log_file)
        assert simulated_step3.id_ is not None
        step3 = sift_client.test_results.create_step(step3_data)
        compare_test_step_fields(simulated_step3, step3)

        step3_1_data = TestStepCreate(
            test_report_id=test_report.id_,
            parent_step_id=step3.id_,
            name="Step 3.1: Substep 3.1",
            description="Error demo",
            step_type=TestStepType.ACTION,
            step_path="3.1",
            status=TestStatus.FAILED,
            start_time=simulated_time,
            end_time=simulated_time + timedelta(seconds=11),
            error_info=ErrorInfo(
                error_code=1,
                error_message="Demo error message",
            ),
        )
        simulated_step3_1 = sift_client.test_results.create_step(step3_1_data, log_file=log_file)
        assert simulated_step3_1.id_ is not None
        step3_1 = sift_client.test_results.create_step(step3_1_data)
        compare_test_step_fields(simulated_step3_1, step3_1)
        assert step1.id_ is not None
        assert step1_1.id_ is not None
        assert step2.id_ is not None
        assert step3.id_ is not None
        assert step3_1.id_ is not None
        self.test_steps["step1"] = step1
        self.test_steps["step1_1"] = step1_1
        self.test_steps["step2"] = step2
        self.test_steps["step3"] = step3
        self.test_steps["step3_1"] = step3_1

    def test_update_test_steps(self, sift_client, tmp_path):
        step3 = self.test_steps.get("step3")
        step3_1 = self.test_steps.get("step3_1")
        if not step3 or not step3_1:
            pytest.skip("Need to create a step first")

        log_file = tmp_path / "test_step_update_log.jsonl"

        # Test update with log_file first
        simulated_step3 = sift_client.test_results.update_step(
            step3,
            {"status": TestStatus.PASSED},
            log_file=log_file,
        )
        assert log_file.exists()
        assert "[UpdateTestStep]" in log_file.read_text()
        assert simulated_step3.status == TestStatus.PASSED

        # Now do real update
        step3 = sift_client.test_results.update_step(
            step3,
            {"status": TestStatus.PASSED},
        )

        compare_test_step_fields(simulated_step3, step3)

        # Update the step using class function.
        step3_1 = step3_1.update(
            {
                "description": "Error demo w/ updated description",
                "metadata": {"phase": "validation", "retry": 2},
            },
        )
        assert step3.status == TestStatus.PASSED
        assert step3_1.description == "Error demo w/ updated description"
        assert step3_1.metadata == {"phase": "validation", "retry": 2}

    def test_create_test_measurements(self, sift_client, tmp_path):
        step1 = self.test_steps.get("step1")
        step2 = self.test_steps.get("step2")
        step3 = self.test_steps.get("step3")
        step1_1 = self.test_steps.get("step1_1")
        if not step1 or not step2 or not step3 or not step1_1:
            pytest.skip("Need to create steps first")

        log_file = tmp_path / "test_measurements_log.jsonl"

        # Test measurement creation with log_file comparison
        measurement1_data = TestMeasurementCreate(
            test_step_id=step1.id_,
            name="Temperature Reading",
            measurement_type=TestMeasurementType.DOUBLE,
            numeric_value=25.5,
            numeric_bounds=NumericBounds(
                min=24,
                max=26,
            ),
            unit="Celsius",
            passed=True,
            timestamp=step1.start_time,
            description="Expected nominal: 25.0C",
            metadata={"sensor": "thermocouple_a", "channel_index": 1},
            channel_names=["temperature_celsius"],
        )

        # Create simulated measurement first
        simulated_measurement1 = sift_client.test_results.create_measurement(
            measurement1_data,
            update_step=True,
            log_file=log_file,
        )
        assert simulated_measurement1.id_ is not None
        assert log_file.exists()
        assert "[CreateTestMeasurement:" in log_file.read_text()

        # Create actual measurement
        measurement1 = sift_client.test_results.create_measurement(
            measurement1_data,
            update_step=True,
        )
        compare_test_measurement_fields(simulated_measurement1, measurement1)

        # Create a measurement using a dict
        measurement2 = sift_client.test_results.create_measurement(
            {
                "test_step_id": step2.id_,
                "name": "FW Version",
                "measurement_type": TestMeasurementType.STRING,
                "string_value": "1.10.3",
                "passed": True,
                "timestamp": step2.start_time,
                "unit": "K",
            },
            update_step=True,
        )

        measurement3 = sift_client.test_results.create_measurement(
            TestMeasurementCreate(
                test_step_id=step3.id_,
                name="Status Check",
                measurement_type=TestMeasurementType.BOOLEAN,
                boolean_value=True,
                passed=True,
                timestamp=step3.start_time,
            ),
            update_step=True,
        )

        measurement4 = sift_client.test_results.create_measurement(
            TestMeasurementCreate(
                test_step_id=step1_1.id_,
                name="Substep 1.1: Substep 1.1.1",
                measurement_type=TestMeasurementType.BOOLEAN,
                boolean_value=True,
                passed=True,
                timestamp=step1_1.start_time,
            )
        )

        assert measurement1.id_ is not None
        assert measurement2.id_ is not None
        assert measurement3.id_ is not None
        assert measurement4.id_ is not None
        assert measurement2.unit == "K"
        self.test_measurements["measurement1"] = measurement1
        self.test_measurements["measurement2"] = measurement2
        self.test_measurements["measurement3"] = measurement3
        self.test_measurements["measurement4"] = measurement4

    def test_update_test_measurements(self, sift_client, tmp_path):
        measurement2 = self.test_measurements.get("measurement2")
        measurement4 = self.test_measurements.get("measurement4")
        if not measurement2 or not measurement4:
            pytest.skip("Need to create measurements first")

        log_file = tmp_path / "test_measurement_update_log.jsonl"

        update_data = {
            "passed": False,
            "string_expected_value": "1.10.4",
            "unit": "C",
            "description": "Updated note after recalibration",
            "metadata": {"part_number": "PN-002"},
            "channel_names": ["firmware_version_channel"],
        }

        # Test update with log_file first
        simulated_measurement2 = sift_client.test_results.update_measurement(
            measurement2,
            update=update_data,
            update_step=True,
            log_file=log_file,
        )
        assert log_file.exists()
        assert "[UpdateTestMeasurement]" in log_file.read_text()
        assert simulated_measurement2.passed == False

        # Now do real update
        measurement2 = sift_client.test_results.update_measurement(
            measurement2,
            update=update_data,
            update_step=True,
        )

        compare_test_measurement_fields(simulated_measurement2, measurement2)

        assert measurement2.passed == False
        assert measurement2.string_expected_value == "1.10.4"
        assert measurement2.unit == "C"
        assert measurement2.description == "Updated note after recalibration"
        assert measurement2.metadata == {"part_number": "PN-002"}
        assert measurement2.channel_names == ["firmware_version_channel"]
        # Update the measurement using class function.
        measurement4 = measurement4.update(
            {
                "passed": False,
                "numeric_bounds": NumericBounds(
                    min=10,
                    max=20,
                ),
            },
            update_step=True,
        )
        assert measurement4.passed == False
        assert measurement4.numeric_bounds == NumericBounds(
            min=10,
            max=20,
        )
        # Verify update_step propogated the status.
        updated_step = sift_client.test_results.get_step(test_step=measurement4.test_step_id)
        assert updated_step.status == TestStatus.FAILED

        self.test_measurements["measurement2"] = measurement2
        self.test_measurements["measurement4"] = measurement4

    def test_update_test_report(self, sift_client, tmp_path):
        test_report = self.test_reports.get("basic_test_report")
        if not test_report:
            pytest.skip("Need to create a test report first")
        new_end_time = test_report.start_time + timedelta(seconds=42)
        log_file = tmp_path / "test_report_update_log.jsonl"

        update_kwargs = {
            "metadata": {
                "test_environment": "production",
                "temperature": 22.5,
                "humidity": 45.0,
                "automated": True,
            },
            "end_time": new_end_time,
            "run_id": "",
        }

        # Test update with log_file first (create fresh update object)
        simulated_report = sift_client.test_results.update(
            test_report=test_report,
            update=TestReportUpdate(**update_kwargs),
            log_file=log_file,
        )
        assert log_file.exists()
        assert "[UpdateTestReport]" in log_file.read_text()

        # Update the report with metadata (real call, create fresh update object)
        updated_report = sift_client.test_results.update(
            test_report=test_report,
            update=TestReportUpdate(**update_kwargs),
        )

        compare_test_report_fields(simulated_report, updated_report)

        # Update the report using class function.
        updated_report = updated_report.update(
            {"status": TestStatus.FAILED},
        )
        assert updated_report.metadata == {
            "test_environment": "production",
            "temperature": 22.5,
            "humidity": 45.0,
            "automated": True,
        }
        assert updated_report.status == TestStatus.FAILED
        assert updated_report.end_time == new_end_time
        assert updated_report.run_id is None

        self.test_reports["basic_test_report"] = updated_report

    def test_list_test_reports(self, sift_client):
        reports = sift_client.test_results.list_(
            filter_query=cel.not_(cel.equals("serial_number", ""))
            and cel.not_(cel.equals("part_number", "")),
        )
        existing_report = reports[0]
        assert len(reports)
        existing_report = reports[0]
        reports = sift_client.test_results.list_(
            status=existing_report.status,
            test_system_name=existing_report.test_system_name,
            test_case=existing_report.test_case,
            serial_numbers=[existing_report.serial_number],
            part_numbers=[existing_report.part_number],
            system_operator=existing_report.system_operator,
        )
        assert existing_report in reports

    def test_list_test_steps(self, sift_client):
        steps = sift_client.test_results.list_steps()
        existing_step = None
        for step in steps:
            if step.parent_step_id is not None:
                existing_step = step
                break
        assert len(steps)
        steps = sift_client.test_results.list_steps(
            test_reports=[existing_step.test_report_id],
            parent_steps=[existing_step.parent_step_id],
            name=existing_step.name,
            step_type=existing_step.step_type,
            status=existing_step.status,
        )
        assert existing_step in steps

    def test_list_test_measurements(self, sift_client):
        measurements = sift_client.test_results.list_measurements()
        assert len(measurements)
        existing_measurement = measurements[0]
        measurements = sift_client.test_results.list_measurements(
            test_steps=[existing_measurement.test_step_id],
            name=existing_measurement.name,
            measurement_type=existing_measurement.measurement_type,
            passed=existing_measurement.passed,
        )
        assert existing_measurement in measurements

    def test_archive_and_delete_test_report(self, sift_client):
        test_report = self.test_reports.get("basic_test_report")
        if not test_report:
            pytest.skip("Need to create a test report first")

        # Archive the report
        archived_report = sift_client.test_results.archive(test_report=test_report)
        assert archived_report.is_archived

        sift_client.test_results.delete(test_report=test_report)
        try:
            deleted_report = sift_client.test_results.get(test_report_id=test_report.id_)
            assert deleted_report is None  # Shouldn't reach here so error if we get something.
        except aiogrpc.AioRpcError as e:
            self.test_reports.pop("basic_test_report")
            assert e.code() == grpc.StatusCode.NOT_FOUND  # noqa: PT017

    def test_import_test_report(self, sift_client):
        # Import a test report from a file
        create_time = datetime.now(timezone.utc)
        current_dir = Path(__file__).parent
        test_file = Path(current_dir, "test_files", "demo_test_report.xml")
        test_report = sift_client.test_results.import_(test_file=test_file)

        # Excercise find_report, custom_filter, and filtering by commonon-proto fields such as created_date
        found_report = sift_client.test_results.find(
            filter_query=f"test_report_id == '{test_report.id_}' && created_date >= timestamp('{create_time}')"
        )
        assert found_report is not None
        assert found_report.id_ == test_report.id_
        self.test_reports["imported_test_report"] = found_report

    def test_delete_test_reports(self, sift_client):
        for test_report in self.test_reports.values():
            sift_client.test_results.delete(test_report=test_report)


class TestImportLogFile:
    def test_import_log_file_round_trip(self, sift_client, nostromo_run, tmp_path):
        """Create a report with steps, nested steps, and measurements twice:
        once with a log file and once without. Then replay the log and compare.
        """
        t0 = datetime.now(timezone.utc)
        log_file = tmp_path / "round_trip.jsonl"

        report_data = {
            "status": TestStatus.IN_PROGRESS,
            "name": "Round Trip Test Report",
            "test_system_name": "RT System",
            "test_case": "RT Case",
            "serial_number": str(uuid.uuid4()),
            "part_number": "RT-001",
            "start_time": t0,
            "end_time": t0 + timedelta(seconds=60),
            "run_id": nostromo_run.id_,
        }

        results: list[dict] = []

        for iteration_log_file in [log_file, None]:
            report = sift_client.test_results.create(report_data, log_file=iteration_log_file)

            step1 = sift_client.test_results.create_step(
                TestStepCreate(
                    test_report_id=report.id_,
                    name="RT Step 1",
                    description="Top-level step",
                    step_type=TestStepType.SEQUENCE,
                    step_path="1",
                    status=TestStatus.PASSED,
                    start_time=t0,
                    end_time=t0 + timedelta(seconds=20),
                ),
                log_file=iteration_log_file,
            )

            step1_1 = sift_client.test_results.create_step(
                TestStepCreate(
                    test_report_id=report.id_,
                    parent_step_id=step1.id_,
                    name="RT Step 1.1",
                    description="Nested step",
                    step_type=TestStepType.ACTION,
                    step_path="1.1",
                    status=TestStatus.PASSED,
                    start_time=t0,
                    end_time=t0 + timedelta(seconds=10),
                ),
                log_file=iteration_log_file,
            )

            step2 = sift_client.test_results.create_step(
                TestStepCreate(
                    test_report_id=report.id_,
                    name="RT Step 2",
                    description="Another top-level step",
                    step_type=TestStepType.ACTION,
                    step_path="2",
                    status=TestStatus.IN_PROGRESS,
                    start_time=t0 + timedelta(seconds=20),
                    end_time=t0 + timedelta(seconds=40),
                    error_info=ErrorInfo(error_code=42, error_message="test error"),
                ),
                log_file=iteration_log_file,
            )

            m1 = sift_client.test_results.create_measurement(
                TestMeasurementCreate(
                    test_step_id=step1_1.id_,
                    name="RT Temperature",
                    measurement_type=TestMeasurementType.DOUBLE,
                    numeric_value=98.6,
                    numeric_bounds=NumericBounds(min=97.0, max=100.0),
                    unit="F",
                    passed=True,
                    timestamp=t0 + timedelta(seconds=5),
                ),
                log_file=iteration_log_file,
            )

            m2 = sift_client.test_results.create_measurement(
                TestMeasurementCreate(
                    test_step_id=step2.id_,
                    name="RT Status Flag",
                    measurement_type=TestMeasurementType.BOOLEAN,
                    boolean_value=False,
                    passed=False,
                    timestamp=t0 + timedelta(seconds=30),
                ),
                log_file=iteration_log_file,
            )

            step2 = sift_client.test_results.update_step(
                step2,
                {"status": TestStatus.FAILED},
                log_file=iteration_log_file,
            )

            report = sift_client.test_results.update(
                test_report=report,
                update=TestReportUpdate(status=TestStatus.FAILED),
                log_file=iteration_log_file,
            )

            results.append(
                {
                    "report": report,
                    "steps": {"step1": step1, "step1_1": step1_1, "step2": step2},
                    "measurements": {"m1": m1, "m2": m2},
                }
            )

        # Verify log file has all expected entries
        log_content = log_file.read_text()
        assert "[CreateTestReport:" in log_content
        assert "[CreateTestStep:" in log_content
        assert "[CreateTestMeasurement:" in log_content
        assert "[UpdateTestStep]" in log_content
        assert "[UpdateTestReport]" in log_content

        # Replay the log file to create real resources
        replay_result = sift_client.test_results.import_log_file(log_file)

        assert replay_result.report.id_ is not None
        assert len(replay_result.steps) == 3
        assert len(replay_result.measurements) == 2

        direct = results[1]

        # Report: updates should have been folded in before create
        compare_test_report_fields(replay_result.report, direct["report"])
        assert replay_result.report.status == TestStatus.FAILED

        # Steps (matched by name)
        replayed_steps_by_name = {s.name: s for s in replay_result.steps}
        for direct_step in direct["steps"].values():
            replayed_step = replayed_steps_by_name[direct_step.name]
            compare_test_step_fields(replayed_step, direct_step)

        assert replayed_steps_by_name["RT Step 2"].status == TestStatus.FAILED

        # Nested step parent should point to the replayed step1
        assert replayed_steps_by_name["RT Step 1.1"].parent_step_id == (
            replayed_steps_by_name["RT Step 1"].id_
        )

        # Measurements (matched by name)
        replayed_measurements_by_name = {m.name: m for m in replay_result.measurements}
        for direct_m in direct["measurements"].values():
            replayed_m = replayed_measurements_by_name[direct_m.name]
            compare_test_measurement_fields(replayed_m, direct_m)

    @pytest.mark.asyncio
    async def test_malformed_log_line_skipped(self, tmp_path):
        """Malformed lines raise a ValueError during iteration."""
        log_file = tmp_path / "bad.jsonl"
        log_file.write_text("this is not a valid log line\n")

        client = TestResultsLowLevelClient(grpc_client=MagicMock())
        with pytest.raises(ValueError, match="Invalid log line: this is not a valid log lin"):
            await client.import_log_file(log_file)

    @pytest.mark.asyncio
    async def test_empty_log_file_raises(self, tmp_path):
        """A log file with no entries raises 'No CreateTestReport'."""
        log_file = tmp_path / "empty.jsonl"
        log_file.touch()

        client = TestResultsLowLevelClient(grpc_client=MagicMock())
        with pytest.raises(ValueError, match="No CreateTestReport found"):
            await client.import_log_file(log_file)

    def test_concurrent_append_and_tracking_save_preserves_all_lines(self, tmp_path):
        """Writer appending to the log shares no mutation point with the tracking sidecar.

        With tracking moved out of the main log into ``<log>.tracking``, the log
        file is strictly append-only -- there is no path by which concurrent
        ``LogTracking.save`` calls can clobber appended lines. This test pins
        that invariant: 500 writer appends run alongside a hot updater looping
        on sidecar writes, and every append must survive.
        """
        import threading
        import time

        from sift.test_reports.v1.test_reports_pb2 import CreateTestReportRequest

        from sift_client._internal.low_level_wrappers._test_results_log import (
            LogTracking,
            log_request_to_file,
        )

        log_file = tmp_path / "race.jsonl"

        n_appends = 500
        stop = threading.Event()
        request = CreateTestReportRequest()

        def writer() -> None:
            for i in range(n_appends):
                log_request_to_file(log_file, "CreateTestReport", request, response_id=str(i))

        def updater() -> None:
            tracking = LogTracking(last_uploaded_line=0)
            while not stop.is_set():
                tracking.last_uploaded_line += 1
                tracking.save(log_file)
                time.sleep(0)

        t_updater = threading.Thread(target=updater)
        t_writer = threading.Thread(target=writer)
        t_updater.start()
        t_writer.start()
        t_writer.join()
        stop.set()
        t_updater.join()

        with open(log_file) as f:
            data_lines = [line for line in f if line.strip()]
        assert len(data_lines) == n_appends, (
            f"expected {n_appends} appended data lines, found {len(data_lines)}"
        )

        sidecar = LogTracking.sidecar_path(log_file)
        assert sidecar.exists()
        reloaded = LogTracking.load(log_file)
        assert reloaded.last_uploaded_line >= 1


T0 = datetime(2026, 1, 1, tzinfo=timezone.utc)


def _make_report(id_: str = "sim-report") -> TestReport:
    return TestReport(
        id_=id_,
        status=TestStatus.IN_PROGRESS,
        name="n",
        test_system_name="s",
        test_case="c",
        start_time=T0,
        end_time=T0,
        metadata={},
        is_archived=False,
    )


def _make_step(id_: str = "sim-step") -> TestStep:
    return TestStep(
        id_=id_,
        test_report_id="sim-report",
        name="step",
        step_type=TestStepType.ACTION,
        step_path="1",
        status=TestStatus.IN_PROGRESS,
        start_time=T0,
        end_time=T0,
    )


def _make_measurement(id_: str = "sim-meas") -> TestMeasurement:
    return TestMeasurement(
        id_=id_,
        measurement_type=TestMeasurementType.BOOLEAN,
        name="m",
        test_step_id="sim-step",
        boolean_value=True,
        passed=True,
        timestamp=T0,
    )


@pytest.fixture
def mock_client():
    client = MagicMock()
    client.grpc_client = MagicMock()
    client.rest_client = MagicMock()
    return client


@pytest_asyncio.fixture
def api(mock_client):
    """Build a TestResultsAPIAsync with mocked low-level + upload clients."""
    with patch(
        "sift_client.resources.test_results.TestResultsLowLevelClient",
        autospec=True,
    ), patch(
        "sift_client.resources.test_results.UploadLowLevelClient",
        autospec=True,
    ):
        return TestResultsAPIAsync(mock_client)


LOG = "/tmp/log.jsonl"


class TestCreateStamping:
    @pytest.mark.asyncio
    async def test_create_stamps_log_file(self, api):
        api._low_level_client.create_test_report = AsyncMock(return_value=_make_report())
        report_data = {
            "status": TestStatus.IN_PROGRESS,
            "name": "n",
            "test_system_name": "s",
            "test_case": "c",
            "start_time": T0,
            "end_time": T0,
        }
        result = await api.create(report_data, log_file=LOG)
        assert result._log_file == LOG
        assert api._low_level_client.create_test_report.call_args.kwargs["log_file"] == LOG

    @pytest.mark.asyncio
    async def test_create_step_stamps_log_file(self, api):
        api._low_level_client.create_test_step = AsyncMock(return_value=_make_step())
        step_data = {
            "test_report_id": "sim-report",
            "name": "step",
            "step_type": TestStepType.ACTION,
            "step_path": "1",
            "status": TestStatus.IN_PROGRESS,
            "start_time": T0,
            "end_time": T0,
        }
        result = await api.create_step(step_data, log_file=LOG)
        assert result._log_file == LOG

    @pytest.mark.asyncio
    async def test_create_measurement_stamps_log_file(self, api):
        api._low_level_client.create_test_measurement = AsyncMock(return_value=_make_measurement())
        meas_data = {
            "test_step_id": "sim-step",
            "name": "m",
            "measurement_type": TestMeasurementType.BOOLEAN,
            "boolean_value": True,
            "passed": True,
            "timestamp": T0,
        }
        result = await api.create_measurement(meas_data, log_file=LOG)
        assert result._log_file == LOG


class TestUpdateStamping:
    @pytest.mark.asyncio
    async def test_update_stamps_log_file(self, api):
        existing = _make_report()
        api._low_level_client.update_test_report = AsyncMock(return_value=existing)
        result = await api.update(
            test_report=existing, update={"status": TestStatus.FAILED}, log_file=LOG
        )
        assert result._log_file == LOG
        assert api._low_level_client.update_test_report.call_args.kwargs["log_file"] == LOG

    @pytest.mark.asyncio
    async def test_update_step_stamps_log_file(self, api):
        existing = _make_step()
        api._low_level_client.update_test_step = AsyncMock(return_value=existing)
        result = await api.update_step(
            test_step=existing, update={"description": "x"}, log_file=LOG
        )
        assert result._log_file == LOG
        assert api._low_level_client.update_test_step.call_args.kwargs["log_file"] == LOG

    @pytest.mark.asyncio
    async def test_update_measurement_stamps_log_file(self, api):
        existing = _make_measurement()
        api._low_level_client.update_test_measurement = AsyncMock(return_value=existing)
        result = await api.update_measurement(
            test_measurement=existing, update={"passed": False}, log_file=LOG
        )
        assert result._log_file == LOG
        assert api._low_level_client.update_test_measurement.call_args.kwargs["log_file"] == LOG


CACHED = "/tmp/cached.jsonl"
KWARG = "/tmp/kwarg.jsonl"


class TestResourceMethodReadsStampedEntity:
    """Resource-level fallback: when no log_file kwarg is passed, read it off
    the entity. Symmetric with the entity-level convenience method's behavior.
    """

    @pytest.mark.parametrize(
        ("cached", "kwarg", "expected"),
        [
            (None, None, None),
            (CACHED, None, CACHED),  # the fallback
            (CACHED, KWARG, KWARG),  # kwarg wins
        ],
    )
    @pytest.mark.asyncio
    async def test_update_reads_log_file_from_test_report(self, api, cached, kwarg, expected):
        entity = _make_report()
        if cached is not None:
            entity.__dict__["_log_file"] = cached
        api._low_level_client.update_test_report = AsyncMock(return_value=entity)

        await api.update(test_report=entity, update={"status": TestStatus.FAILED}, log_file=kwarg)

        assert api._low_level_client.update_test_report.call_args.kwargs["log_file"] == expected

    @pytest.mark.parametrize(
        ("cached", "kwarg", "expected"),
        [
            (None, None, None),
            (CACHED, None, CACHED),
            (CACHED, KWARG, KWARG),
        ],
    )
    @pytest.mark.asyncio
    async def test_update_step_reads_log_file_from_test_step(self, api, cached, kwarg, expected):
        entity = _make_step()
        if cached is not None:
            entity.__dict__["_log_file"] = cached
        api._low_level_client.update_test_step = AsyncMock(return_value=entity)

        await api.update_step(test_step=entity, update={"description": "x"}, log_file=kwarg)

        assert api._low_level_client.update_test_step.call_args.kwargs["log_file"] == expected

    @pytest.mark.parametrize(
        ("cached", "kwarg", "expected"),
        [
            (None, None, None),
            (CACHED, None, CACHED),
            (CACHED, KWARG, KWARG),
        ],
    )
    @pytest.mark.asyncio
    async def test_update_measurement_reads_log_file_from_test_measurement(
        self, api, cached, kwarg, expected
    ):
        entity = _make_measurement()
        if cached is not None:
            entity.__dict__["_log_file"] = cached
        api._low_level_client.update_test_measurement = AsyncMock(return_value=entity)

        await api.update_measurement(
            test_measurement=entity, update={"passed": False}, log_file=kwarg
        )

        assert (
            api._low_level_client.update_test_measurement.call_args.kwargs["log_file"] == expected
        )

    @pytest.mark.asyncio
    async def test_update_with_string_id_has_no_fallback(self, api):
        """Passing a bare ID (no entity) means no _log_file to read; the resource
        forwards None to the low-level wrapper.
        """
        api._low_level_client.update_test_report = AsyncMock(return_value=_make_report())
        await api.update(test_report="some-id", update={"status": TestStatus.FAILED})
        assert api._low_level_client.update_test_report.call_args.kwargs["log_file"] is None

    @pytest.mark.asyncio
    async def test_update_step_with_string_id_has_no_fallback(self, api):
        api._low_level_client.update_test_step = AsyncMock(return_value=_make_step())
        await api.update_step(test_step="some-id", update={"description": "x"})
        assert api._low_level_client.update_test_step.call_args.kwargs["log_file"] is None


class TestReadPathsDoNotStamp:
    """get/list_/find/import_log_file return real entities; they must not carry _log_file."""

    @pytest.mark.asyncio
    async def test_get_does_not_stamp(self, api):
        api._low_level_client.get_test_report = AsyncMock(return_value=_make_report("real-id"))
        result = await api.get(test_report_id="real-id")
        assert result._log_file is None

    @pytest.mark.asyncio
    async def test_list_does_not_stamp(self, api):
        api._low_level_client.list_all_test_reports = AsyncMock(
            return_value=[_make_report("a"), _make_report("b")]
        )
        results = await api.list_()
        assert all(r._log_file is None for r in results)

    @pytest.mark.asyncio
    async def test_import_log_file_does_not_stamp(self, api, tmp_path):
        from sift_client._internal.low_level_wrappers.test_results import ReplayResult

        log_path = tmp_path / "log.jsonl"
        log_path.touch()
        replay_result = ReplayResult(
            report=_make_report("real-report"),
            steps=[_make_step("real-step")],
            measurements=[_make_measurement("real-meas")],
        )
        api._low_level_client.import_log_file = AsyncMock(return_value=replay_result)

        result = await api.import_log_file(log_path)

        assert result.report._log_file is None
        assert all(s._log_file is None for s in result.steps)
        assert all(m._log_file is None for m in result.measurements)


class TestEndToEndLogFileRouting:
    """Full pipeline: resource -> real low-level client -> actual file write.

    No mocking of the low-level client; the GrpcClient stub is mocked but is
    never invoked because the file-write branch in the low-level wrapper
    short-circuits before any gRPC call when log_file is set. Proves the
    cached-_log_file plumbing reaches the file on disk.
    """

    @pytest.fixture
    def real_api(self, mock_client):
        """TestResultsAPIAsync wired through a real TestResultsLowLevelClient."""
        return TestResultsAPIAsync(mock_client)

    @pytest.mark.asyncio
    async def test_metadata_update_round_trips_through_log_file(self, real_api, tmp_path):
        """Update with metadata via cached
        _log_file, then read the JSONL line back through the same parser the
        replay path uses and verify every key/value round-trips. Proves the
        user-visible payload (not just an opaque entry) lands on disk.
        """
        from google.protobuf import json_format
        from sift.test_reports.v1.test_reports_pb2 import UpdateTestReportRequest

        from sift_client._internal.low_level_wrappers._test_results_log import (
            iter_log_data_lines,
        )
        from sift_client.util.metadata import metadata_proto_to_dict

        log_file = tmp_path / "metadata.jsonl"
        report_data = {
            "status": TestStatus.IN_PROGRESS,
            "name": "n",
            "test_system_name": "s",
            "test_case": "c",
            "start_time": T0,
            "end_time": T0,
        }
        report = await real_api.create(report_data, log_file=log_file)
        assert report._log_file == log_file

        # Mix of string, number, and boolean to cover all three MetadataValue arms.
        metadata = {
            "run_id": "run-abc-123",
            "operator": "test-user",
            "trial_number": 42.5,
            "is_dry_run": True,
        }
        # No log_file kwarg — the resource layer must read it off the entity.
        await real_api.update(test_report=report, update={"metadata": metadata})

        # Find the UpdateTestReport line and decode it the same way replay does.
        update_entries = [
            (rt, rid, js)
            for rt, rid, js in iter_log_data_lines(log_file)
            if rt == "UpdateTestReport"
        ]
        assert len(update_entries) == 1
        _, _, json_str = update_entries[0]

        request = UpdateTestReportRequest()
        json_format.Parse(json_str, request)

        assert "metadata" in request.update_mask.paths
        round_tripped = metadata_proto_to_dict(list(request.test_report.metadata))
        assert round_tripped == metadata
        # And confirm we never reached the gRPC stub.
        real_api._low_level_client._grpc_client.get_stub.assert_not_called()
