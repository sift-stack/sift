import asyncio
import os
from datetime import datetime, timedelta, timezone
from pathlib import Path

import grpc
from grpc import aio as aiogrpc

from sift_client.client import SiftClient, SiftConnectionConfig
from sift_client.sift_types import (
    TestMeasurementCreate,
    TestMeasurementType,
    TestStatus,
    TestStepCreate,
    TestStepType,
)
from sift_client.sift_types.test_report import (
    ErrorInfo,
    NumericBounds,
    TestReportUpdate,
)


async def main():
    grpc_url = os.getenv("SIFT_GRPC_URI", "localhost:50051")
    rest_url = os.getenv("SIFT_REST_URI", "localhost:8080")
    api_key = os.getenv("SIFT_LOCAL_API_KEY", "")
    client = SiftClient(
        connection_config=SiftConnectionConfig(
            grpc_url=grpc_url,
            api_key=api_key,
            rest_url=rest_url,
            use_ssl=False,
        )
    )

    # Create a test report
    simulated_time = datetime.now(timezone.utc)
    test_report = client.test_results.create_report(
        {
            "status": TestStatus.PASSED,
            "name": "Test Report with Steps and Measurements",
            "test_system_name": "Test System",
            "test_case": "Test Case",
            "start_time": simulated_time,
            "end_time": simulated_time,
        },
    )
    print(f"Created test report: {test_report.id_}")

    # Create multiple test steps using TestStepCreate
    step1 = client.test_results.create_step(
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
    print(f"Created step 1: {step1.id_}")

    # Create a step using a dict
    step1_1 = client.test_results.create_step(
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
    print(f"Created step 1.1: {step1_1.id_}")
    simulated_time = simulated_time + timedelta(seconds=10.1)

    step2 = client.test_results.create_step(
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
    print(f"Created step 2: {step2.id_}")
    simulated_time = simulated_time + timedelta(seconds=10.1)
    step3 = client.test_results.create_step(
        TestStepCreate(
            test_report_id=test_report.id_,
            name="Step 3: Validation",
            description="Validate collected data",
            step_type=TestStepType.ACTION,
            step_path="3",
            status=TestStatus.PASSED,
            start_time=simulated_time,
            end_time=simulated_time + timedelta(seconds=10),
        ),
    )
    print(f"Created step 3: {step3.id_}")

    step3_1 = client.test_results.create_step(
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
    print(f"Created step 3.1: {step3_1.id_}")

    simulated_time = simulated_time + timedelta(seconds=11.1)
    # Create measurements for each step using TestMeasurementCreate
    measurement1 = client.test_results.create_measurement(
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
            timestamp=simulated_time,
        ),
        update_step=True,
    )
    print(f"Created measurement 1: {measurement1.id_}")

    # Create a measurement using a dict
    measurement2 = client.test_results.create_measurement(
        {
            "test_step_id": step2.id_,
            "name": "FW Version",
            "measurement_type": TestMeasurementType.STRING,
            "string_value": "1.10.3",
            "passed": True,
            "timestamp": step2.start_time,
        },
        update_step=True,
    )
    print(f"Created measurement 2: {measurement2.id_}")

    measurement3 = client.test_results.create_measurement(
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
    print(f"Created measurement 3: {measurement3.id_}")

    measurement4 = client.test_results.create_measurement(
        TestMeasurementCreate(
            test_step_id=step1_1.id_,
            name="Substep 1.1: Substep 1.1.1",
            measurement_type=TestMeasurementType.BOOLEAN,
            boolean_value=True,
            passed=True,
            timestamp=step1_1.start_time,
        )
    )
    print(f"Created measurement 4: {measurement4}")

    measurement2 = client.test_results.update_measurement(
        measurement2,
        update={
            "passed": False,
            "string_expected_value": "1.10.4",
        },
        update_step=True,
    )
    print(f"Updated measurement 2: {measurement2}")
    assert measurement2.passed == False
    assert measurement2.string_expected_value == "1.10.4"

    measurement4 = client.test_results.update_measurement(
        measurement4,
        update={
            "passed": False,
            "numeric_bounds": NumericBounds(
                min=10,
                max=20,
            ),
        },
        update_step=True,
    )
    print(f"Updated measurement 4: {measurement4}")
    assert measurement4.passed == False
    assert measurement4.numeric_bounds == NumericBounds(
        min=10,
        max=20,
    )

    # Verify update_step propogated the status.
    updated_step = client.test_results.get_step(test_step_id=measurement4.test_step_id)
    assert updated_step.status == TestStatus.FAILED

    # Update the report with metadata
    new_end_time = measurement4.timestamp + timedelta(seconds=10)
    updated_report = client.test_results.update_report(
        test_report=test_report,
        update=TestReportUpdate(
            metadata={
                "test_environment": "production",
                "temperature": 22.5,
                "humidity": 45.0,
                "automated": True,
            },
            status=TestStatus.FAILED,
            end_time=new_end_time,
        ),
    )
    print(f"Updated report with metadata: {updated_report.metadata}")
    assert updated_report.metadata == {
        "test_environment": "production",
        "temperature": 22.5,
        "humidity": 45.0,
        "automated": True,
    }
    assert updated_report.status == TestStatus.FAILED
    assert updated_report.end_time == new_end_time

    # Archive the report
    archived_report = client.test_results.archive_report(test_report=test_report)
    assert archived_report.is_archived

    client.test_results.delete_report(test_report=test_report)
    try:
        deleted_report = client.test_results.get_report(test_report_id=test_report.id_)
        assert deleted_report is None  # Shouldn't reach here so error if we get something.
    except aiogrpc.AioRpcError as e:
        print(f"Report deleted: {e}")
        assert e.code() == grpc.StatusCode.NOT_FOUND

    # Import a test report from a file
    create_time = datetime.now(timezone.utc)
    current_dir = Path(__file__).parent
    test_file = Path(current_dir, "test_files", "demo_test_report.xml")
    test_report = client.test_results.import_test_report(test_file=test_file)
    print(f"Imported test report: {test_report.id_}")

    # Excercise find_report, custom_filter, and filtering by commonon-proto fields such as created_date
    found_report = client.test_results.find_report(
        custom_filter=f"test_report_id == '{test_report.id_}' && created_date >= timestamp('{create_time}')"
    )
    assert found_report is not None
    assert found_report.id_ == test_report.id_

    client.test_results.delete_report(test_report=found_report)
    print(f"Found and deleted imported report: {found_report.id_}")

    print("Test completed successfully")


if __name__ == "__main__":
    asyncio.run(main())
