from __future__ import annotations

import uuid
from datetime import datetime, timedelta, timezone
from pathlib import Path
from typing import ClassVar
from unittest.mock import MagicMock

import grpc
import pytest
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


def compare_test_measurement_fields(simulated: TestMeasurement, actual: TestMeasurement) -> None:
    """Compare simulated and actual TestMeasurement fields (excluding id_)."""
    assert simulated.name == actual.name
    assert simulated.measurement_type == actual.measurement_type
    assert simulated.numeric_value == actual.numeric_value
    assert simulated.string_value == actual.string_value
    assert simulated.boolean_value == actual.boolean_value
    assert simulated.passed == actual.passed
    assert simulated.timestamp == actual.timestamp


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
            {"description": "Error demo w/ updated description"},
        )
        assert step3.status == TestStatus.PASSED
        assert step3_1.description == "Error demo w/ updated description"

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
        """Malformed lines are skipped; a file with no valid entries raises 'No CreateTestReport'."""
        log_file = tmp_path / "bad.jsonl"
        log_file.write_text(
            '[LogTracking] {"lastUploadedLine":0,"idMap":{}}\nthis is not a valid log line\n'
        )

        client = TestResultsLowLevelClient(grpc_client=MagicMock())
        with pytest.raises(ValueError, match="Invalid log line: this is not a valid log lin"):
            await client.import_log_file(log_file)

    @pytest.mark.asyncio
    async def test_empty_log_file_raises(self, tmp_path):
        """A log file with only a LogTracking header and no entries raises."""
        log_file = tmp_path / "empty.jsonl"
        log_file.write_text('[LogTracking] {"lastUploadedLine":0,"idMap":{}}\n')

        client = TestResultsLowLevelClient(grpc_client=MagicMock())
        with pytest.raises(ValueError, match="No CreateTestReport found"):
            await client.import_log_file(log_file)
