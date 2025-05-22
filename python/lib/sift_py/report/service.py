import time
from typing import Optional, cast

from sift.reports.v1.reports_pb2 import GetReportRequest, GetReportResponse, ReportRuleStatus
from sift.reports.v1.reports_pb2 import Report as Report_pb2
from sift.reports.v1.reports_pb2_grpc import ReportServiceStub
from sift_py.grpc.transport import SiftChannel

NOT_RUNNING_STATUS = [
    ReportRuleStatus.REPORT_RULE_STATUS_CANCELED,
    ReportRuleStatus.REPORT_RULE_STATUS_ERROR,
    ReportRuleStatus.REPORT_RULE_STATUS_FAILED,
    ReportRuleStatus.REPORT_RULE_STATUS_FINISHED,
]


class ReportService:
    """Service to track the status of reports."""

    report_id: str
    _report_stub: ReportServiceStub

    def __init__(self, channel: SiftChannel, report_id: str):
        self._report_stub = ReportServiceStub(channel)
        self.report_id = report_id

    def get_results(self) -> Report_pb2:
        """Returns the results of the report.

        Returns:
            Information about the report.
        """
        req = GetReportRequest(report_id=self.report_id)
        res = cast(GetReportResponse, self._report_stub.GetReport(req))
        return res.report

    def wait_until_done(self, timeout: Optional[float] = None) -> bool:
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

            result = self.get_results()
            all_done = all([summary.status in NOT_RUNNING_STATUS for summary in result.summaries])

            if all_done:
                return True
            else:
                time.sleep(5)
