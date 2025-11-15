from __future__ import annotations

import uuid
from datetime import datetime, timedelta, timezone
from pathlib import Path
from typing import ClassVar

import grpc
import pytest
from grpc import aio as aiogrpc

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


def test_client_binding(sift_client):
    assert sift_client.test_results
    assert isinstance(sift_client.test_results, TestResultsAPI)
    assert sift_client.async_.test_results
    assert isinstance(sift_client.async_.test_results, TestResultsAPIAsync)


class TestResultsTest:
    test_reports: ClassVar[dict[str, TestReport]] = {}
    test_steps: ClassVar[dict[str, TestStep]] = {}
    test_measurements: ClassVar[dict[str, TestMeasurement]] = {}

    def test_create_test_report(self, sift_client, nostromo_run):
        # Create a test report
        simulated_time = datetime.now(timezone.utc)
        test_report = sift_client.test_results.create(
            {
                "status": TestStatus.PASSED,
                "name": "Test Report with Steps and Measurements",
                "test_system_name": "Test System",
                "test_case": "Test Case",
                "serial_number": str(uuid.uuid4()),
                "part_number": "1234567890",
                "start_time": simulated_time,
                "end_time": simulated_time,
                "run_id": nostromo_run.id_,
            },
        )
        assert test_report.id_ is not None
        assert test_report.run_id == nostromo_run.id_
        self.test_reports["basic_test_report"] = test_report

    def test_create_test_steps(self, sift_client):
        test_report = self.test_reports.get("basic_test_report")
        if not test_report:
            pytest.skip("Need to create a test report first")
        simulated_time = test_report.start_time

        # Create multiple test steps using TestStepCreate
        step1 = sift_client.test_results.create_step(
            TestStepCreate(
                test_report_id=test_report.id_,
                name="Step 1: Initialization",
                description="Initialize the test environment",
                step_type=TestStepType.ACTION,
                step_path="1",
                status=TestStatus.PASSED,
                start_time=simulated_time,
                end_time=simulated_time + timedelta(seconds=10),
            ),
        )
        simulated_time = simulated_time + timedelta(seconds=10.1)
        # Create a step using a dict
        step1_1 = sift_client.test_results.create_step(
            {
                "test_report_id": test_report.id_,
                "parent_step_id": step1.id_,
                "name": "Step 1.1: Substep 1",
                "description": "Substep 1 of Step 1",
                "step_type": TestStepType.ACTION,
                "step_path": "1.1",
                "status": TestStatus.PASSED,
                "start_time": simulated_time,
                "end_time": simulated_time + timedelta(seconds=10),
            },
        )
        simulated_time = simulated_time + timedelta(seconds=10.1)

        step2 = sift_client.test_results.create_step(
            TestStepCreate(
                test_report_id=test_report.id_,
                name="Step 2: Data Collection",
                description="Collect sensor data",
                step_type=TestStepType.ACTION,
                step_path="2",
                status=TestStatus.PASSED,
                start_time=simulated_time,
                end_time=simulated_time + timedelta(seconds=10),
            )
        )
        simulated_time = simulated_time + timedelta(seconds=10.1)
        step3 = sift_client.test_results.create_step(
            TestStepCreate(
                test_report_id=test_report.id_,
                name="Step 3: Validation",
                description="Validate collected data",
                step_type=TestStepType.ACTION,
                step_path="3",
                status=TestStatus.IN_PROGRESS,
                start_time=simulated_time,
                end_time=simulated_time + timedelta(seconds=10),
            ),
        )

        step3_1 = sift_client.test_results.create_step(
            TestStepCreate(
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
            ),
        )
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

    def test_update_test_steps(self, sift_client):
        step3 = self.test_steps.get("step3")
        step3_1 = self.test_steps.get("step3_1")
        if not step3 or not step3_1:
            pytest.skip("Need to create a step first")
        step3 = sift_client.test_results.update_step(
            step3,
            {"status": TestStatus.PASSED},
        )
        # Update the step using class function.
        step3_1 = step3_1.update(
            {"description": "Error demo w/ updated description"},
        )
        assert step3.status == TestStatus.PASSED
        assert step3_1.description == "Error demo w/ updated description"

    def test_create_test_measurements(self, sift_client):
        step1 = self.test_steps.get("step1")
        step2 = self.test_steps.get("step2")
        step3 = self.test_steps.get("step3")
        step1_1 = self.test_steps.get("step1_1")
        if not step1 or not step2 or not step3 or not step1_1:
            pytest.skip("Need to create steps first")

        # Create measurements for each step using TestMeasurementCreate
        measurement1 = sift_client.test_results.create_measurement(
            TestMeasurementCreate(
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
            ),
            update_step=True,
        )

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

    def test_update_test_measurements(self, sift_client):
        measurement2 = self.test_measurements.get("measurement2")
        measurement4 = self.test_measurements.get("measurement4")
        if not measurement2 or not measurement4:
            pytest.skip("Need to create measurements first")

        measurement2 = sift_client.test_results.update_measurement(
            measurement2,
            update={
                "passed": False,
                "string_expected_value": "1.10.4",
                "unit": "C",
            },
            update_step=True,
        )
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

    def test_update_test_report(self, sift_client):
        test_report = self.test_reports.get("basic_test_report")
        if not test_report:
            pytest.skip("Need to create a test report first")
        new_end_time = test_report.start_time + timedelta(seconds=42)
        # Update the report with metadata
        updated_report = sift_client.test_results.update(
            test_report=test_report,
            update=TestReportUpdate(
                metadata={
                    "test_environment": "production",
                    "temperature": 22.5,
                    "humidity": 45.0,
                    "automated": True,
                },
                end_time=new_end_time,
                run_id="",
            ),
        )

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
