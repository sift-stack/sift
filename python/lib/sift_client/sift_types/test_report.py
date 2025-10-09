from __future__ import annotations

from datetime import datetime, timezone
from enum import Enum
from typing import TYPE_CHECKING, ClassVar

from pydantic import ConfigDict
from sift.test_reports.v1.test_reports_pb2 import (
    ErrorInfo as ErrorInfoProto,
)
from sift.test_reports.v1.test_reports_pb2 import (
    NumericBounds as NumericBoundsProto,
)
from sift.test_reports.v1.test_reports_pb2 import (
    StringBounds as StringBoundsProto,
)
from sift.test_reports.v1.test_reports_pb2 import (
    TestMeasurement as TestMeasurementProto,
)
from sift.test_reports.v1.test_reports_pb2 import (
    TestReport as TestReportProto,
)
from sift.test_reports.v1.test_reports_pb2 import (
    TestStep as TestStepProto,
)

from sift_client.sift_types._base import (
    BaseType,
    MappingHelper,
    ModelCreate,
    ModelCreateUpdateBase,
    ModelUpdate,
)
from sift_client.util.metadata import metadata_dict_to_proto, metadata_proto_to_dict

if TYPE_CHECKING:
    from sift_client.client import SiftClient


class TestStatus(Enum):
    """TestStatus enum."""

    UNSPECIFIED = 0
    DRAFT = 1
    PASSED = 2
    FAILED = 3
    ABORTED = 4
    ERROR = 5
    IN_PROGRESS = 6
    SKIPPED = 7


class TestStepType(Enum):
    """TestStepType enum."""

    UNSPECIFIED = 0
    SEQUENCE = 1
    GROUP = 2
    ACTION = 3
    FLOW_CONTROL = 4


class TestMeasurementType(Enum):
    """TestMeasurementType enum."""

    UNSPECIFIED = 0
    DOUBLE = 1
    STRING = 3
    BOOLEAN = 4
    LIMIT = 5


class TestReportBase(ModelCreateUpdateBase):
    """Base model for TestReportUpdate and TestReportCreate. Contains shared fields for all test reports. Update and create models differ mostly in what fields are required vs optional."""

    status: TestStatus | None = None
    metadata: dict[str, str | float | bool] | None = None
    serial_number: str | None = None
    part_number: str | None = None
    system_operator: str | None = None

    _to_proto_helpers: ClassVar = {
        "metadata": MappingHelper(
            proto_attr_path="metadata", update_field="metadata", converter=metadata_dict_to_proto
        ),
    }

    def _get_proto_class(self) -> type[TestReportProto]:
        return TestReportProto


class TestReportUpdate(TestReportBase, ModelUpdate[TestReportProto]):
    """Update model for TestReport."""

    name: str | None = None
    test_system_name: str | None = None
    test_case: str | None = None
    start_time: datetime | None = None
    end_time: datetime | None = None

    is_archived: bool | None = None

    def _add_resource_id_to_proto(self, proto_msg: TestReportProto):
        if self._resource_id is None:
            raise ValueError("Resource ID must be set before adding to proto")
        proto_msg.test_report_id = self._resource_id


class TestReportCreate(TestReportBase, ModelCreate[TestReportProto]):
    """Create model for TestReport."""

    name: str
    test_system_name: str
    test_case: str
    start_time: datetime
    end_time: datetime

    def to_proto(self) -> TestReportProto:
        """Convert to protobuf message with custom logic."""
        proto = TestReportProto(
            status=self.status.value,  # type: ignore
            name=self.name,
            test_system_name=self.test_system_name,
            test_case=self.test_case,
            metadata=metadata_dict_to_proto(self.metadata) if self.metadata else {},
            is_archived=False,
        )

        proto.start_time.FromDatetime(self.start_time)
        proto.end_time.FromDatetime(self.end_time)

        if self.serial_number:
            proto.serial_number = self.serial_number

        if self.part_number:
            proto.part_number = self.part_number

        if self.system_operator:
            proto.system_operator = self.system_operator

        return proto

    def _to_proto(self) -> TestReportProto:
        """Alias for to_proto() for compatibility with low-level client."""
        return self.to_proto()


class ErrorInfo(BaseType[ErrorInfoProto, "ErrorInfo"]):
    """ErrorInfo model representing error information in a test step."""

    error_code: int
    error_message: str

    @classmethod
    def _from_proto(cls, proto: ErrorInfoProto, sift_client: SiftClient | None = None) -> ErrorInfo:
        return cls(
            id_=None,
            error_code=proto.error_code,
            error_message=proto.error_message,
            _client=sift_client,
        )

    def _to_proto(self) -> ErrorInfoProto:
        """Convert to protobuf message."""
        return ErrorInfoProto(
            error_code=self.error_code,
            error_message=self.error_message,
        )


class TestStepBase(ModelCreateUpdateBase):
    """Base model for TestStepUpdate and TestStepCreate. Contains shared fields for all test steps. Update and create models differ mostly in what fields are required vs optional."""

    parent_step_id: str | None = None
    description: str | None = None
    error_info: ErrorInfo | None = None

    def _get_proto_class(self) -> type[TestStepProto]:
        return TestStepProto


class TestStepUpdate(TestStepBase, ModelUpdate[TestStepProto]):
    """Update model for TestStep."""

    name: str | None = None
    step_type: TestStepType | None = None
    step_path: str | None = None
    status: TestStatus | None = None
    start_time: datetime | None = None
    end_time: datetime | None = None

    def _add_resource_id_to_proto(self, proto_msg: TestStepProto):
        if self._resource_id is None:
            raise ValueError("Resource ID must be set before adding to proto")
        proto_msg.test_step_id = self._resource_id


class TestStepCreate(TestStepBase, ModelCreate[TestStepProto]):
    """Create model for TestStep."""

    test_report_id: str
    name: str
    step_type: TestStepType
    step_path: str
    status: TestStatus
    start_time: datetime
    end_time: datetime

    def to_proto(self) -> TestStepProto:
        """Convert to protobuf message with custom logic."""
        proto = TestStepProto(
            test_report_id=self.test_report_id,
            name=self.name,
            step_type=self.step_type.value,  # type: ignore
            step_path=self.step_path,
            status=self.status.value,  # type: ignore
        )

        proto.start_time.FromDatetime(self.start_time)
        proto.end_time.FromDatetime(self.end_time)

        if self.parent_step_id:
            proto.parent_step_id = self.parent_step_id

        if self.description:
            proto.description = self.description

        if self.error_info:
            proto.error_info.CopyFrom(self.error_info._to_proto())

        return proto


class TestStep(BaseType[TestStepProto, "TestStep"]):
    """TestStep model representing a step in a test."""

    test_report_id: str
    parent_step_id: str | None = None
    name: str
    description: str | None = None
    step_type: TestStepType
    step_path: str
    status: TestStatus
    start_time: datetime
    end_time: datetime
    error_info: ErrorInfo | None = None

    @classmethod
    def _from_proto(cls, proto: TestStepProto, sift_client: SiftClient | None = None) -> TestStep:
        return cls(
            id_=proto.test_step_id,
            test_report_id=proto.test_report_id,
            parent_step_id=proto.parent_step_id if proto.parent_step_id else None,
            name=proto.name,
            description=proto.description if proto.description else None,
            step_type=TestStepType(proto.step_type),
            step_path=proto.step_path,
            status=TestStatus(proto.status),
            start_time=proto.start_time.ToDatetime(tzinfo=timezone.utc),
            end_time=proto.end_time.ToDatetime(tzinfo=timezone.utc),
            error_info=ErrorInfo._from_proto(proto.error_info, sift_client)
            if proto.HasField("error_info")
            else None,
            _client=sift_client,
        )

    def _to_proto(self) -> TestStepProto:
        """Convert to protobuf message."""
        proto = TestStepProto(
            test_step_id=self.id_ or "",
            test_report_id=self.test_report_id,
            name=self.name,
            step_type=self.step_type.value,  # type: ignore
            step_path=self.step_path,
            status=self.status.value,  # type: ignore
        )

        proto.start_time.FromDatetime(self.start_time)
        proto.end_time.FromDatetime(self.end_time)

        if self.parent_step_id:
            proto.parent_step_id = self.parent_step_id

        if self.description:
            proto.description = self.description

        if self.error_info:
            proto.error_info.CopyFrom(self.error_info._to_proto())

        return proto

    def update(self, update: TestStepUpdate | dict) -> TestStep:
        """Update the TestStep."""
        updated_test_step = self._client.test_results.update_step(test_step=self, update=update)
        self._update(updated_test_step)
        return self


class NumericBounds(BaseType[NumericBoundsProto, "NumericBounds"]):
    """NumericBounds model representing numeric bounds for test measurements."""

    min: float | None = None
    max: float | None = None

    @classmethod
    def _from_proto(
        cls, proto: NumericBoundsProto, sift_client: SiftClient | None = None
    ) -> NumericBounds:
        return cls(
            min=proto.min if proto.HasField("min") else None,
            max=proto.max if proto.HasField("max") else None,
            _client=sift_client,
        )

    def _to_proto(self) -> NumericBoundsProto:
        """Convert to protobuf message."""
        return NumericBoundsProto(min=self.min, max=self.max)


class TestMeasurementBase(ModelCreateUpdateBase):
    """Base model for TestMeasurementUpdate and TestMeasurementCreate. Contains shared fields for all test measurements. Update and create models differ mostly in what fields are required vs optional."""

    numeric_value: float | None = None
    string_value: str | None = None
    boolean_value: bool | None = None
    unit: str | None = None
    numeric_bounds: NumericBounds | None = None
    string_expected_value: str | None = None

    def _get_proto_class(self) -> type[TestMeasurementProto]:
        return TestMeasurementProto


class TestMeasurementUpdate(TestMeasurementBase, ModelUpdate[TestMeasurementProto]):
    """Update model for TestMeasurement."""

    name: str | None = None
    measurement_type: int | None = None
    passed: bool | None = None
    timestamp: datetime | None = None

    _to_proto_helpers: ClassVar = {
        "string_expected_value": MappingHelper(
            proto_attr_path="string_bounds.expected_value", update_field="string_bounds"
        ),
    }

    def _add_resource_id_to_proto(self, proto_msg: TestMeasurementProto):
        if self._resource_id is None:
            raise ValueError("Resource ID must be set before adding to proto")
        proto_msg.measurement_id = self._resource_id


class TestMeasurementCreate(TestMeasurementBase, ModelCreate[TestMeasurementProto]):
    """Create model for TestMeasurement."""

    measurement_type: TestMeasurementType
    name: str
    test_step_id: str
    passed: bool
    timestamp: datetime

    def to_proto(self) -> TestMeasurementProto:
        """Convert to protobuf message with custom logic."""
        proto = TestMeasurementProto(
            measurement_type=self.measurement_type.value,  # type: ignore
            name=self.name,
            test_step_id=self.test_step_id,
            passed=self.passed,
        )

        proto.timestamp.FromDatetime(self.timestamp)

        if self.numeric_value is not None:
            proto.numeric_value = self.numeric_value
        elif self.string_value is not None:
            proto.string_value = self.string_value
        elif self.boolean_value is not None:
            proto.boolean_value = self.boolean_value

        if self.numeric_bounds:
            proto.numeric_bounds.CopyFrom(self.numeric_bounds._to_proto())

        if self.string_expected_value:
            proto.string_bounds.CopyFrom(
                StringBoundsProto(expected_value=self.string_expected_value)
            )

        return proto


class TestMeasurement(BaseType[TestMeasurementProto, "TestMeasurement"]):
    """TestMeasurement model representing a measurement in a test."""

    model_config = ConfigDict(arbitrary_types_allowed=True)

    measurement_type: TestMeasurementType
    name: str
    test_step_id: str
    test_report_id: str | None = None  # Read only
    numeric_value: float | None = None
    string_value: str | None = None
    boolean_value: bool | None = None
    unit: str | None = None
    numeric_bounds: NumericBounds | None = None
    string_expected_value: str | None = None
    passed: bool
    timestamp: datetime

    @classmethod
    def _from_proto(
        cls, proto: TestMeasurementProto, sift_client: SiftClient | None = None
    ) -> TestMeasurement:
        numeric_value = None
        string_value = None
        boolean_value = None

        if proto.HasField("numeric_value"):
            numeric_value = proto.numeric_value
        elif proto.HasField("string_value"):
            string_value = proto.string_value
        elif proto.HasField("boolean_value"):
            boolean_value = proto.boolean_value

        return cls(
            id_=proto.measurement_id,
            measurement_type=TestMeasurementType(proto.measurement_type),
            name=proto.name,
            test_step_id=proto.test_step_id,
            test_report_id=proto.test_report_id,
            numeric_value=numeric_value,
            string_value=string_value,
            boolean_value=boolean_value,
            unit=proto.unit.abbreviated_name if proto.HasField("unit") else None,
            numeric_bounds=NumericBounds._from_proto(proto.numeric_bounds, sift_client)
            if proto.HasField("numeric_bounds")
            else None,
            string_expected_value=proto.string_bounds.expected_value
            if proto.HasField("string_bounds")
            else None,
            passed=proto.passed,
            timestamp=proto.timestamp.ToDatetime(tzinfo=timezone.utc),
            _client=sift_client,
        )

    def _to_proto(self) -> TestMeasurementProto:
        """Convert to protobuf message."""
        proto = TestMeasurementProto(
            measurement_id=self.id_ or "",
            measurement_type=self.measurement_type.value,  # type: ignore
            name=self.name,
            test_step_id=self.test_step_id,
            passed=self.passed,
        )

        proto.timestamp.FromDatetime(self.timestamp)

        if self.numeric_value is not None:
            proto.numeric_value = self.numeric_value
        elif self.string_value is not None:
            proto.string_value = self.string_value
        elif self.boolean_value is not None:
            proto.boolean_value = self.boolean_value

        if self.numeric_bounds:
            proto.numeric_bounds.CopyFrom(self.numeric_bounds._to_proto())

        if self.string_expected_value:
            proto.string_bounds.CopyFrom(
                StringBoundsProto(expected_value=self.string_expected_value)
            )

        return proto

    def update(
        self, update: TestMeasurementUpdate | dict, update_step: bool = False
    ) -> TestMeasurement:
        """Update the TestMeasurement."""
        updated_test_measurement = self._client.test_results.update_measurement(
            test_measurement=self, update=update, update_step=update_step
        )
        self._update(updated_test_measurement)
        return self


class TestReport(BaseType[TestReportProto, "TestReport"]):
    """TestReport model representing a test report."""

    model_config = ConfigDict(arbitrary_types_allowed=True)

    status: TestStatus
    name: str
    test_system_name: str
    test_case: str
    start_time: datetime
    end_time: datetime
    metadata: dict[str, str | float | bool]
    serial_number: str | None = None
    part_number: str | None = None
    system_operator: str | None = None
    archived_date: datetime | None = None
    is_archived: bool

    @classmethod
    def _from_proto(
        cls, proto: TestReportProto, sift_client: SiftClient | None = None
    ) -> TestReport:
        return cls(
            id_=proto.test_report_id,
            status=TestStatus(proto.status),
            name=proto.name,
            test_system_name=proto.test_system_name,
            test_case=proto.test_case,
            start_time=proto.start_time.ToDatetime(tzinfo=timezone.utc),
            end_time=proto.end_time.ToDatetime(tzinfo=timezone.utc),
            metadata=metadata_proto_to_dict(proto.metadata),  # type: ignore
            serial_number=proto.serial_number if proto.serial_number else None,
            part_number=proto.part_number if proto.part_number else None,
            system_operator=proto.system_operator if proto.system_operator else None,
            archived_date=proto.archived_date.ToDatetime(tzinfo=timezone.utc)
            if proto.HasField("archived_date")
            else None,
            is_archived=proto.is_archived,
            _client=sift_client,
        )

    def _to_proto(self) -> TestReportProto:
        """Convert to protobuf message."""
        proto = TestReportProto(
            test_report_id=self.id_ or "",
            status=self.status.value,  # type: ignore
            name=self.name,
            test_system_name=self.test_system_name,
            test_case=self.test_case,
            metadata=metadata_dict_to_proto(self.metadata),
            is_archived=self.is_archived,
        )

        proto.start_time.FromDatetime(self.start_time)
        proto.end_time.FromDatetime(self.end_time)

        if self.serial_number:
            proto.serial_number = self.serial_number

        if self.part_number:
            proto.part_number = self.part_number

        if self.system_operator:
            proto.system_operator = self.system_operator

        if self.archived_date:
            proto.archived_date.FromDatetime(self.archived_date)

        return proto

    def update(self, update: TestReportUpdate | dict) -> TestReport:
        """Update the TestReport."""
        updated_test_report = self._client.test_results.update_report(
            test_report=self, update=update
        )
        self._update(updated_test_report)
        return self

    def archive(self) -> TestReport:
        """Archive the TestReport."""
        updated_test_report = self._client.test_results.archive_report(test_report=self)
        self._update(updated_test_report)
        return self

    def unarchive(self) -> TestReport:
        """Unarchive the TestReport."""
        updated_test_report = self._client.test_results.unarchive_report(test_report=self)
        self._update(updated_test_report)
        return self
