from typing import cast
from sift.reports.v1.reports_pb2 import GetReportRequest, GetReportResponse, Report as Report_pb2, ReportRuleStatus
from sift.reports.v1.reports_pb2_grpc import ReportServiceStub
from sift_py.grpc.transport import SiftChannel
import time

NOT_RUNNING_STATUS = [
    ReportRuleStatus.REPORT_RULE_STATUS_CANCELED,
    ReportRuleStatus.REPORT_RULE_STATUS_ERROR,
    ReportRuleStatus.REPORT_RULE_STATUS_FAILED,
    ReportRuleStatus.REPORT_RULE_STATUS_FINISHED,
]

class Report:
    """Used to track the status of a Report."""

    report_id: str
    _report_stub: ReportServiceStub

    def __init__(self, channel: SiftChannel, report_id: str):
        self._report_stub = ReportServiceStub(channel)
        self.report_id = report_id

    def get_result(self) -> Report_pb2:
        """Returns the result of the report.

        Returns:
            Information about the report.
        """
        req = GetReportRequest(report_id=self.report_id)
        res = cast(GetReportResponse, self._report_stub.GetReport(req))
        return res.report

    def wait_until_done(self, timeout=None) -> bool:
        """Waits until the report processing is complete or the timeout is reached.

        Args:
            timeout: Timeout to wait for the report to finish. Default is None.

        Returns:
            True if the report finished within the timeout.
        """
        start_time = time.time()
        while True:
            if timeout is not None:
                if time.time() - start_time > timeout:
                    return False

            result = self.get_result()
            all_done = all([
                summary.status in NOT_RUNNING_STATUS
                for summary in result.summaries
            ])

            if all_done:
                return True
            else:
                time.sleep(5)
