from __future__ import annotations

import re
import uuid
from datetime import datetime
from typing import TYPE_CHECKING

from sift_client._internal.low_level_wrappers.test_results import TestResultsLowLevelClient
from sift_client._internal.low_level_wrappers.upload import UploadLowLevelClient
from sift_client.resources._base import ResourceBase
from sift_client.sift_types.test_report import (
    TestMeasurement,
    TestMeasurementType,
    TestMeasurementUpdate,
    TestReport,
    TestReportUpdate,
    TestStatus,
    TestStep,
    TestStepType,
    TestStepUpdate,
)
from sift_client.util.cel_utils import contains, equals, match

if TYPE_CHECKING:
    from datetime import datetime
    from pathlib import Path

    from sift_client.client import SiftClient


class TestResultsAPIAsync(ResourceBase):
    """High-level API for interacting with test reports, steps, and measurements."""

    def __init__(self, sift_client: SiftClient):
        """Initialize the TestResultsAPI.

        Args:
            sift_client: The Sift client to use.
        """
        super().__init__(sift_client)
        self._low_level_client = TestResultsLowLevelClient(grpc_client=self.client.grpc_client)
        self._upload_client = UploadLowLevelClient(rest_client=self.client.rest_client)

    async def import_test_report(self, test_file: str | Path) -> TestReport:
        """Import a test report from an already-uploaded file.

        Args:
            test_file: The path to the test report file to import.

        Returns:
            The imported TestReport.
        """
        # Generate a temporary UUID for the test report. The report service will override this with the created report ID.
        temp_uuid = str(uuid.uuid4())
        remote_file_id = await self._upload_client.upload_attachment(
            path=test_file,
            entity_id=temp_uuid,
            entity_type="test_reports",
        )
        test_report = await self._low_level_client.import_test_report(remote_file_id=remote_file_id)
        return self._apply_client_to_instance(test_report)

    async def create_report(
        self,
        status: TestStatus,
        name: str,
        test_system_name: str,
        test_case: str,
        start_time: datetime,
        end_time: datetime,
        metadata: dict[str, str | float | bool] | None = None,
        serial_number: str | None = None,
        part_number: str | None = None,
        system_operator: str | None = None,
    ) -> TestReport:
        """Create a new test report.

        Args:
            status: The status of the test run (TestStatus enum).
            name: The name of the test run.
            test_system_name: The name of the test system.
            test_case: The test case that was run.
            start_time: The start time of the test run.
            end_time: The end time of the test run.
            metadata: The metadata values associated with this test run.
            serial_number: The serial number for the DUT.
            part_number: The part number for the DUT.
            system_operator: Unique identifier for user owner.

        Returns:
            The created TestReport.
        """
        test_report = await self._low_level_client.create_test_report(
            status=status,
            name=name,
            test_system_name=test_system_name,
            test_case=test_case,
            start_time=start_time,
            end_time=end_time,
            metadata=metadata,
            serial_number=serial_number,
            part_number=part_number,
            system_operator=system_operator,
        )
        return self._apply_client_to_instance(test_report)

    async def get_report(self, *, test_report_id: str) -> TestReport:
        """Get a TestReport.

        Args:
            test_report_id: The ID of the test report.

        Returns:
            The TestReport.
        """
        test_report = await self._low_level_client.get_test_report(test_report_id=test_report_id)
        return self._apply_client_to_instance(test_report)

    async def list_reports(
        self,
        *,
        name: str | None = None,
        name_contains: str | None = None,
        name_regex: str | re.Pattern | None = None,
        test_report_id: str | None = None,
        status: TestStatus | None = None,
        test_system_name: str | None = None,
        test_case: str | None = None,
        serial_number: str | None = None,
        part_number: str | None = None,
        system_operator: str | None = None,
        created_by_user_id: str | None = None,
        is_archived: bool | None = None,
        custom_filter: str | None = None,
        order_by: str | None = None,
        limit: int | None = None,
    ) -> list[TestReport]:
        """List test reports with optional filtering.

        Args:
            name: Exact name of the test report.
            name_contains: Partial name of the test report.
            name_regex: Regular expression string to filter test reports by name.
            test_report_id: Test report ID to filter by.
            status: Status to filter by (TestStatus enum).
            test_system_name: Test system name to filter by.
            test_case: Test case to filter by.
            serial_number: Serial number to filter by.
            part_number: Part number to filter by.
            system_operator: System operator to filter by.
            created_by_user_id: User ID who created the test report.
            is_archived: Whether to include only archived or non-archived reports.
            custom_filter: Custom filter to apply to the test reports.
            order_by: How to order the retrieved test reports. If used, this will override the other filters.
            limit: How many test reports to retrieve. If None, retrieves all matches.

        Returns:
            A list of TestReports that matches the filter.
        """
        # Build CEL filter
        filter_parts = []

        if name:
            filter_parts.append(equals("name", name))
        elif name_contains:
            filter_parts.append(contains("name", name_contains))
        elif name_regex:
            if isinstance(name_regex, re.Pattern):
                name_regex = name_regex.pattern
            filter_parts.append(match("name", name_regex))  # type: ignore

        if test_report_id:
            filter_parts.append(equals("test_report_id", test_report_id))

        if status is not None:
            filter_parts.append(equals("status", status))

        if test_system_name:
            filter_parts.append(equals("test_system_name", test_system_name))

        if test_case:
            filter_parts.append(equals("test_case", test_case))

        if serial_number:
            filter_parts.append(equals("serial_number", serial_number))

        if part_number:
            filter_parts.append(equals("part_number", part_number))

        if system_operator:
            filter_parts.append(equals("system_operator", system_operator))

        if created_by_user_id:
            filter_parts.append(equals("created_by_user_id", created_by_user_id))

        if is_archived is not None:
            filter_parts.append(equals("is_archived", is_archived))

        query_filter = " && ".join(filter_parts) if filter_parts else None
        if custom_filter:
            if filter_parts:
                raise ValueError("Custom filter cannot be used with other filters")
            query_filter = custom_filter

        test_reports = await self._low_level_client.list_all_test_reports(
            query_filter=query_filter,
            order_by=order_by,
            max_results=limit,
        )
        return self._apply_client_to_instances(test_reports)

    async def find_report(self, **kwargs) -> TestReport | None:
        """Find a single test report matching the given query. Takes the same arguments as `list_`. If more than one test report is found,
        raises an error.

        Args:
            **kwargs: Keyword arguments to pass to `list_reports`.

        Returns:
            The TestReport found or None.
        """
        test_reports = await self.list_reports(**kwargs)
        if len(test_reports) > 1:
            for report in test_reports:
                print(report)
            raise ValueError("Multiple test reports found for query")
        elif len(test_reports) == 1:
            return test_reports[0]
        return None

    async def update_report(
        self, test_report: str | TestReport, update: TestReportUpdate | dict
    ) -> TestReport:
        """Update a TestReport.

        Args:
            test_report: The TestReport or test report ID to update.
            update: Updates to apply to the TestReport.

        Returns:
            The updated TestReport.
        """
        if isinstance(test_report, str):
            test_report = await self.get_report(test_report_id=test_report)

        if isinstance(update, dict):
            update = TestReportUpdate.model_validate(update)

        update.resource_id = test_report.id_
        updated_test_report = await self._low_level_client.update_test_report(update)
        return self._apply_client_to_instance(updated_test_report)

    async def archive_report(self, *, test_report: str | TestReport) -> TestReport:
        """Archive a test report.

        Args:
            test_report: The TestReport or test report ID to archive.
        """
        return await self.update_report(test_report=test_report, update={"is_archived": True})

    async def delete_report(self, *, test_report: str | TestReport) -> None:
        """Delete a test report.

        Args:
            test_report: The TestReport or test report ID to delete.
        """
        test_report_id = test_report.id_ if isinstance(test_report, TestReport) else test_report
        if not isinstance(test_report_id, str):
            raise TypeError(f"test_report_id must be a string not {type(test_report_id)}")
        await self._low_level_client.delete_test_report(test_report_id=test_report_id)

    async def create_step(self, test_step: TestStep) -> TestStep:
        """Create a new test step.

        Args:
            test_step: The test step to create.

        Returns:
            The created TestStep.
        """
        test_step = await self._low_level_client.create_test_step(test_step)
        return self._apply_client_to_instance(test_step)

    async def list_steps(
        self,
        *,
        test_step_id: str | None = None,
        test_report_id: str | None = None,
        parent_step_id: str | None = None,
        name: str | None = None,
        name_contains: str | None = None,
        status: TestStatus | None = None,
        step_type: TestStepType | None = None,
        order_by: str | None = None,
        limit: int | None = None,
    ) -> list[TestStep]:
        """List test steps with optional filtering.

        Args:
            test_step_id: Test step ID to filter by.
            test_report_id: Test report ID to filter by.
            parent_step_id: Parent step ID to filter by.
            name: Exact name of the test step.
            name_contains: Partial name of the test step.
            status: Status to filter by (TestStatus enum).
            step_type: Step type to filter by (TestStepType enum).
            order_by: How to order the retrieved test steps.
            limit: How many test steps to retrieve. If None, retrieves all matches.

        Returns:
            A list of TestSteps that matches the filter.
        """
        # Build CEL filter
        filter_parts = []

        if test_step_id:
            filter_parts.append(equals("test_step_id", test_step_id))

        if test_report_id:
            filter_parts.append(equals("test_report_id", test_report_id))

        if parent_step_id:
            filter_parts.append(equals("parent_step_id", parent_step_id))

        if name:
            filter_parts.append(equals("name", name))
        elif name_contains:
            filter_parts.append(contains("name", name_contains))

        if status is not None:
            filter_parts.append(equals("status", status))

        if step_type is not None:
            filter_parts.append(equals("step_type", step_type))

        query_filter = " && ".join(filter_parts) if filter_parts else None

        test_steps = await self._low_level_client.list_all_test_steps(
            query_filter=query_filter,
            order_by=order_by,
            max_results=limit,
        )
        return self._apply_client_to_instances(test_steps)

    async def get_step(self, test_step_id: str) -> TestStep:
        """Get a TestStep.

        Args:
            test_step_id: The ID of the test step.
        """
        test_steps = await self._low_level_client.list_all_test_steps(
            query_filter=equals("test_step_id", test_step_id), max_results=1
        )
        if not test_steps:
            raise ValueError(f"TestStep with ID {test_step_id} not found")
        test_step = test_steps[0]
        return self._apply_client_to_instance(test_step)

    async def update_step(
        self, test_step: str | TestStep, update: TestStepUpdate | dict
    ) -> TestStep:
        """Update a TestStep.

        Args:
            test_step: The TestStep or test step ID to update.
            update: Updates to apply to the TestStep.

        Returns:
            The updated TestStep.
        """
        test_step_id = test_step.id_ if isinstance(test_step, TestStep) else test_step

        if isinstance(update, dict):
            update = TestStepUpdate.model_validate(update)

        update.resource_id = test_step_id
        updated_test_step = await self._low_level_client.update_test_step(update)
        return self._apply_client_to_instance(updated_test_step)

    async def delete_step(self, *, test_step: str | TestStep) -> None:
        """Delete a test step.

        Args:
            test_step: The TestStep or test step ID to delete.
        """
        test_step_id = test_step.id_ if isinstance(test_step, TestStep) else test_step
        if not isinstance(test_step_id, str):
            raise TypeError(f"test_step_id must be a string not {type(test_step_id)}")
        await self._low_level_client.delete_test_step(test_step_id=test_step_id)

    async def create_measurement(
        self, test_measurement: TestMeasurement, update_step: bool = False
    ) -> TestMeasurement:
        """Create a new test measurement.

        Args:
            test_measurement: The test measurement to create.
            update_step: Whether to update the step to failed if the measurement is being created is failed.

        Returns:
            The created TestMeasurement.
        """
        test_measurement = await self._low_level_client.create_test_measurement(test_measurement)
        measurement = self._apply_client_to_instance(test_measurement)
        if update_step:
            step = await self.get_step(test_step_id=test_measurement.test_step_id)
            if step.status == TestStatus.PASSED and not measurement.passed:
                await self.update_step(test_step=step, update={"status": TestStatus.FAILED})
        return measurement

    async def create_measurements(
        self, test_measurements: list[TestMeasurement]
    ) -> tuple[int, list[str]]:
        """Create multiple test measurements in a single request.

        Args:
            test_measurements: The test measurements to create.

        Returns:
            A tuple of (measurements_created_count, measurement_ids).
        """
        return await self._low_level_client.create_test_measurements(test_measurements)

    async def list_measurements(
        self,
        *,
        measurement_id: str | None = None,
        test_step_id: str | None = None,
        test_report_id: str | None = None,
        name: str | None = None,
        name_contains: str | None = None,
        measurement_type: TestMeasurementType | None = None,
        passed: bool | None = None,
        order_by: str | None = None,
        limit: int | None = None,
    ) -> list[TestMeasurement]:
        """List test measurements with optional filtering.

        Args:
            measurement_id: Measurement ID to filter by.
            test_step_id: Test step ID to filter by.
            test_report_id: Test report ID to filter by.
            name: Exact name of the test measurement.
            name_contains: Partial name of the test measurement.
            measurement_type: Measurement type to filter by (TestMeasurementType enum).
            passed: Whether the measurement passed.
            order_by: How to order the retrieved test measurements.
            limit: How many test measurements to retrieve. If None, retrieves all matches.

        Returns:
            A list of TestMeasurements that matches the filter.
        """
        # Build CEL filter
        filter_parts = []

        if measurement_id:
            filter_parts.append(equals("measurement_id", measurement_id))

        if test_step_id:
            filter_parts.append(equals("test_step_id", test_step_id))

        if test_report_id:
            filter_parts.append(equals("test_report_id", test_report_id))

        if name:
            filter_parts.append(equals("name", name))
        elif name_contains:
            filter_parts.append(contains("name", name_contains))

        if measurement_type is not None:
            filter_parts.append(equals("measurement_type", measurement_type))

        if passed is not None:
            filter_parts.append(equals("passed", passed))

        query_filter = " && ".join(filter_parts) if filter_parts else None

        test_measurements = await self._low_level_client.list_all_test_measurements(
            query_filter=query_filter,
            order_by=order_by,
            max_results=limit,
        )
        return self._apply_client_to_instances(test_measurements)

    async def update_measurement(
        self,
        test_measurement: TestMeasurement,
        update: TestMeasurementUpdate | dict,
        update_step: bool = False,
    ) -> TestMeasurement:
        """Update a TestMeasurement.

        Args:
            test_measurement: The TestMeasurement or measurement ID to update.
            update: Updates to apply to the TestMeasurement.
            update_step: Whether to update the step to failed if the measurement is being updated to failed.

        Returns:
            The updated TestMeasurement.
        """
        if isinstance(update, dict):
            update = TestMeasurementUpdate.model_validate(update)

        update.resource_id = test_measurement.id_
        updated_test_measurement = await self._low_level_client.update_test_measurement(update)
        updated_test_measurement = self._apply_client_to_instance(updated_test_measurement)
        # If measurement is being updated to failed, see if step is passed and update it to failed if so
        if update_step and update.passed is not None and not update.passed:
            step = await self.get_step(test_step_id=updated_test_measurement.test_step_id)
            if step.status == TestStatus.PASSED:
                await self.update_step(test_step=step, update={"status": TestStatus.FAILED})
        return updated_test_measurement

    async def delete_measurement(self, *, test_measurement: str | TestMeasurement) -> None:
        """Delete a test measurement.

        Args:
            test_measurement: The TestMeasurement or measurement ID to delete.
        """
        measurement_id = (
            test_measurement.id_
            if isinstance(test_measurement, TestMeasurement)
            else test_measurement
        )
        if not isinstance(measurement_id, str):
            raise TypeError(f"measurement_id must be a string not {type(measurement_id)}")
        await self._low_level_client.delete_test_measurement(measurement_id=measurement_id)
