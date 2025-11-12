from __future__ import annotations

import uuid
from datetime import datetime
from typing import TYPE_CHECKING, Any

from sift_client._internal.low_level_wrappers.test_results import TestResultsLowLevelClient
from sift_client._internal.low_level_wrappers.upload import UploadLowLevelClient
from sift_client.resources._base import ResourceBase
from sift_client.sift_types.test_report import (
    TestMeasurement,
    TestMeasurementCreate,
    TestMeasurementType,
    TestMeasurementUpdate,
    TestReport,
    TestReportCreate,
    TestReportUpdate,
    TestStatus,
    TestStep,
    TestStepCreate,
    TestStepType,
    TestStepUpdate,
)
from sift_client.util.cel_utils import and_, equals, in_

if TYPE_CHECKING:
    import re
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

    async def import_(self, test_file: str | Path) -> TestReport:
        """Import a test report from an already-uploaded file.

        Args:
            test_file: The path to the test report file to import. We currently only support XML files exported from NI TestStand.

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

    async def create(
        self,
        test_report: TestReportCreate | dict,
    ) -> TestReport:
        """Create a new test report.

        Args:
            test_report: The test report to create (can be TestReport or TestReportCreate).

        Returns:
            The created TestReport.
        """
        if isinstance(test_report, dict):
            test_report = TestReportCreate.model_validate(test_report)
        created_report = await self._low_level_client.create_test_report(
            test_report=test_report,
        )
        return self._apply_client_to_instance(created_report)

    async def get(self, *, test_report_id: str) -> TestReport:
        """Get a TestReport.

        Args:
            test_report_id: The ID of the test report.

        Returns:
            The TestReport.
        """
        test_report = await self._low_level_client.get_test_report(test_report_id=test_report_id)
        return self._apply_client_to_instance(test_report)

    async def list_(
        self,
        *,
        name: str | None = None,
        names: list[str] | None = None,
        name_contains: str | None = None,
        name_regex: str | re.Pattern | None = None,
        test_report_ids: list[str] | None = None,
        status: TestStatus | None = None,
        test_system_name: str | None = None,
        test_case: str | None = None,
        serial_numbers: list[str] | None = None,
        part_numbers: list[str] | None = None,
        system_operator: str | None = None,
        created_by: str | None = None,
        modified_by: str | None = None,
        created_after: datetime | None = None,
        created_before: datetime | None = None,
        modified_after: datetime | None = None,
        modified_before: datetime | None = None,
        metadata: list[Any] | dict[str, Any] | None = None,
        include_archived: bool = False,
        filter_query: str | None = None,
        order_by: str | None = None,
        limit: int | None = None,
    ) -> list[TestReport]:
        """List test reports with optional filtering.

        Args:
            name: Exact name of the test report.
            names: List of test report names to filter by.
            name_contains: Partial name of the test report.
            name_regex: Regular expression string to filter test reports by name.
            test_report_ids: Test report IDs to filter by.
            status: Status to filter by (TestStatus enum).
            test_system_name: Test system name to filter by.
            test_case: Test case to filter by.
            serial_numbers: Serial numbers to filter by.
            part_numbers: Part numbers to filter by.
            system_operator: System operator to filter by.
            created_by: User ID who created the test report.
            modified_by: User ID who last modified the test report.
            created_after: Filter test reports created after this datetime.
            created_before: Filter test reports created before this datetime.
            modified_after: Filter test reports modified after this datetime.
            modified_before: Filter test reports modified before this datetime.
            metadata: Filter test reports by metadata criteria.
            include_archived: Whether to include only archived or non-archived reports.
            filter_query: Custom filter to apply to the test reports.
            order_by: How to order the retrieved test reports. If used, this will override the other filters.
            limit: How many test reports to retrieve. If None, retrieves all matches.

        Returns:
            A list of TestReports that matches the filter.
        """
        # Build CEL filter
        filter_parts = [
            *self._build_name_cel_filters(
                name=name, names=names, name_contains=name_contains, name_regex=name_regex
            ),
            *self._build_time_cel_filters(
                created_after=created_after,
                created_before=created_before,
                modified_after=modified_after,
                modified_before=modified_before,
                created_by=created_by,
                modified_by=modified_by,
            ),
            *self._build_metadata_cel_filters(metadata=metadata),
            *self._build_common_cel_filters(
                include_archived=include_archived,
                filter_query=filter_query,
            ),
        ]

        if test_report_ids:
            filter_parts.append(in_("test_report_id", test_report_ids))

        if status is not None:
            if isinstance(status, TestStatus):
                status = status.name.lower()  # type: ignore
            filter_parts.append(equals("status", status))

        if test_system_name:
            filter_parts.append(equals("test_system_name", test_system_name))

        if test_case:
            filter_parts.append(equals("test_case", test_case))

        if serial_numbers:
            filter_parts.append(in_("serial_number", serial_numbers))

        if part_numbers:
            filter_parts.append(in_("part_number", part_numbers))

        if system_operator:
            filter_parts.append(equals("system_operator", system_operator))

        query_filter = and_(*filter_parts)

        test_reports = await self._low_level_client.list_all_test_reports(
            query_filter=query_filter,
            order_by=order_by,
            max_results=limit,
        )
        return self._apply_client_to_instances(test_reports)

    async def find(self, **kwargs) -> TestReport | None:
        """Find a single test report matching the given query. Takes the same arguments as `list_`. If more than one test report is found,
        raises an error.

        Args:
            **kwargs: Keyword arguments to pass to `list_`.

        Returns:
            The TestReport found or None.
        """
        test_reports = await self.list_(**kwargs)
        if len(test_reports) > 1:
            error_msg = (
                f"Multiple test reports found for query ({', '.join(report.id_ or 'no id' for report in test_reports)})"
                if len(test_reports) < 10
                else f"Multiple ({len(test_reports)} test reports found for query)"
            )
            raise ValueError(error_msg)
        elif len(test_reports) == 1:
            return test_reports[0]
        return None

    async def update(
        self, test_report: str | TestReport, update: TestReportUpdate | dict
    ) -> TestReport:
        """Update a TestReport.

        Args:
            test_report: The TestReport or test report ID to update.
            update: Updates to apply to the TestReport.

        Returns:
            The updated TestReport.
        """
        test_report_id = (
            test_report._id_or_error if isinstance(test_report, TestReport) else test_report
        )
        if isinstance(update, dict):
            update = TestReportUpdate.model_validate(update)

        update.resource_id = test_report_id
        updated_test_report = await self._low_level_client.update_test_report(update)
        return self._apply_client_to_instance(updated_test_report)

    async def archive(self, *, test_report: str | TestReport) -> TestReport:
        """Archive a test report.

        Args:
            test_report: The TestReport or test report ID to archive.
        """
        return await self.update(test_report=test_report, update={"is_archived": True})

    async def unarchive(self, *, test_report: str | TestReport) -> TestReport:
        """Unarchive a test report.

        Args:
            test_report: The TestReport or test report ID to unarchive.
        """
        return await self.update(test_report=test_report, update={"is_archived": False})

    async def delete(self, *, test_report: str | TestReport) -> None:
        """Delete a test report.

        Args:
            test_report: The TestReport or test report ID to delete.
        """
        test_report_id = test_report.id_ if isinstance(test_report, TestReport) else test_report
        if not isinstance(test_report_id, str):
            raise TypeError(f"test_report_id must be a string not {type(test_report_id)}")
        await self._low_level_client.delete_test_report(test_report_id=test_report_id)

    async def create_step(self, test_step: TestStepCreate | dict) -> TestStep:
        """Create a new test step.

        Args:
            test_step: The test step to create (can be TestStep or TestStepCreate).

        Returns:
            The created TestStep.
        """
        if isinstance(test_step, dict):
            test_step = TestStepCreate.model_validate(test_step)
        test_step_result = await self._low_level_client.create_test_step(test_step)
        return self._apply_client_to_instance(test_step_result)

    async def list_steps(
        self,
        *,
        test_steps: list[str] | list[TestStep] | None = None,
        test_reports: list[str] | list[TestReport] | None = None,
        parent_steps: list[str] | list[TestStep] | None = None,
        name: str | None = None,
        names: list[str] | None = None,
        name_contains: str | None = None,
        name_regex: str | re.Pattern | None = None,
        status: TestStatus | None = None,
        step_type: TestStepType | None = None,
        filter_query: str | None = None,
        order_by: str | None = None,
        limit: int | None = None,
    ) -> list[TestStep]:
        """List test steps with optional filtering.

        Args:
            test_steps: Test steps to filter by.
            test_reports: Test reports to filter by.
            parent_steps: Parent steps to filter by.
            name: Exact name of the test step.
            names: List of test step names to filter by.
            name_contains: Partial name of the test step.
            name_regex: Regular expression string to filter test steps by name.
            status: Status to filter by (TestStatus enum).
            step_type: Step type to filter by (TestStepType enum).
            filter_query: Explicit CEL query to filter test steps.
            order_by: How to order the retrieved test steps.
            limit: How many test steps to retrieve. If None, retrieves all matches.

        Returns:
            A list of TestSteps that matches the filter.
        """
        # Build CEL filter
        filter_parts = [
            *self._build_name_cel_filters(
                name=name, names=names, name_contains=name_contains, name_regex=name_regex
            ),
            *self._build_common_cel_filters(
                filter_query=filter_query,
            ),
        ]

        if test_steps:
            test_step_ids: list[str] = [
                test_step.id_ or "" if isinstance(test_step, TestStep) else test_step
                for test_step in test_steps
            ]
            filter_parts.append(in_("test_step_id", test_step_ids))

        if test_reports:
            test_report_ids: list[str] = [
                test_report.id_ or "" if isinstance(test_report, TestReport) else test_report
                for test_report in test_reports
            ]
            filter_parts.append(in_("test_report_id", test_report_ids))

        if parent_steps:
            parent_step_ids: list[str] = [
                parent_step.id_ or "" if isinstance(parent_step, TestStep) else parent_step
                for parent_step in parent_steps
            ]
            filter_parts.append(in_("parent_step_id", parent_step_ids))

        if status is not None:
            filter_parts.append(equals("status", status.name.lower()))

        if step_type is not None:
            filter_parts.append(equals("step_type", step_type.name.lower()))

        query_filter = and_(*filter_parts)

        test_steps = await self._low_level_client.list_all_test_steps(
            query_filter=query_filter,
            order_by=order_by,
            max_results=limit,
        )
        return self._apply_client_to_instances(test_steps)

    async def get_step(self, test_step: str | TestStep) -> TestStep:
        """Get a TestStep.

        Args:
            test_step: The TestStep or test step ID to get.
        """
        step_id = test_step._id_or_error if isinstance(test_step, TestStep) else test_step
        test_steps = await self.list_steps(
            test_steps=[step_id],
            limit=1,
        )
        if not test_steps:
            raise ValueError(f"TestStep with ID {step_id} not found")
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
        test_step_id = test_step._id_or_error if isinstance(test_step, TestStep) else test_step

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
        test_step_id = test_step._id_or_error if isinstance(test_step, TestStep) else test_step
        if not isinstance(test_step_id, str):
            raise TypeError(f"test_step_id must be a string not {type(test_step_id)}")
        await self._low_level_client.delete_test_step(test_step_id=test_step_id)

    async def create_measurement(
        self, test_measurement: TestMeasurementCreate | dict, update_step: bool = False
    ) -> TestMeasurement:
        """Create a new test measurement.

        Args:
            test_measurement: The test measurement to create (can be TestMeasurement or TestMeasurementCreate).
            update_step: Whether to update the step to failed if the measurement is being created is failed.

        Returns:
            The created TestMeasurement.
        """
        if isinstance(test_measurement, dict):
            test_measurement = TestMeasurementCreate.model_validate(test_measurement)
        test_measurement_result = await self._low_level_client.create_test_measurement(
            test_measurement
        )
        measurement = self._apply_client_to_instance(test_measurement_result)
        if update_step:
            step = await self.get_step(test_step=test_measurement_result.test_step_id)
            if step.status == TestStatus.PASSED and not measurement.passed:
                await self.update_step(test_step=step, update={"status": TestStatus.FAILED})
        return measurement

    async def create_measurements(
        self, test_measurements: list[TestMeasurementCreate]
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
        measurements: list[str] | list[TestMeasurement] | None = None,
        test_steps: list[str] | list[TestStep] | None = None,
        test_reports: list[str] | list[TestReport] | None = None,
        name: str | None = None,
        names: list[str] | None = None,
        name_contains: str | None = None,
        name_regex: str | re.Pattern | None = None,
        measurement_type: TestMeasurementType | None = None,
        passed: bool | None = None,
        filter_query: str | None = None,
        order_by: str | None = None,
        limit: int | None = None,
    ) -> list[TestMeasurement]:
        """List test measurements with optional filtering.

        Args:
            measurements: Measurements to filter by.
            test_steps: Test steps to filter by.
            test_reports: Test reports to filter by.
            name: Exact name of the test measurement.
            names: List of test measurement names to filter by.
            name_contains: Partial name of the test measurement.
            name_regex: Regular expression string to filter test measurements by name.
            measurement_type: Measurement type to filter by (TestMeasurementType enum).
            passed: Whether the measurement passed.
            filter_query: Explicit CEL query to filter test measurements.
            order_by: How to order the retrieved test measurements.
            limit: How many test measurements to retrieve. If None, retrieves all matches.

        Returns:
            A list of TestMeasurements that matches the filter.
        """
        # Build CEL filter
        filter_parts = [
            *self._build_name_cel_filters(
                name=name, names=names, name_contains=name_contains, name_regex=name_regex
            ),
            *self._build_common_cel_filters(
                filter_query=filter_query,
            ),
        ]

        if measurements:
            measurement_ids = [
                measurement.id_ or "" if isinstance(measurement, TestMeasurement) else measurement
                for measurement in measurements
            ]
            filter_parts.append(in_("measurement_id", measurement_ids))

        if test_steps:
            test_step_ids: list[str] = [
                test_step.id_ or "" if isinstance(test_step, TestStep) else test_step
                for test_step in test_steps
            ]
            filter_parts.append(in_("test_step_id", test_step_ids))

        if test_reports:
            test_report_ids: list[str] = [
                test_report.id_ or "" if isinstance(test_report, TestReport) else test_report
                for test_report in test_reports
            ]
            filter_parts.append(in_("test_report_id", test_report_ids))

        if measurement_type is not None:
            filter_parts.append(equals("measurement_type", measurement_type.name.lower()))

        if passed is not None:
            filter_parts.append(equals("passed", passed))

        query_filter = and_(*filter_parts)

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
            step = await self.get_step(test_step=updated_test_measurement.test_step_id)
            if step.status == TestStatus.PASSED:
                await self.update_step(test_step=step, update={"status": TestStatus.FAILED})
        return updated_test_measurement

    async def delete_measurement(self, *, test_measurement: str | TestMeasurement) -> None:
        """Delete a test measurement.

        Args:
            test_measurement: The TestMeasurement or measurement ID to delete.
        """
        measurement_id = (
            test_measurement.id_ or ""
            if isinstance(test_measurement, TestMeasurement)
            else test_measurement
        )
        if not isinstance(measurement_id, str):
            raise TypeError(f"measurement_id must be a string not {type(measurement_id)}")
        await self._low_level_client.delete_test_measurement(measurement_id=measurement_id)
