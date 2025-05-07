from typing import cast
from sift.reports.v1.reports_pb2 import GetReportRequest, GetReportResponse, Report as Report_pb2, ReportRuleStatus
from sift.reports.v1.reports_pb2_grpc import ReportServiceStub
from sift_py.grpc.transport import SiftChannel
import time


class Report:
    """
    A class to interact with and manage reports via the ReportServiceStub.
    """

    _report_stub: ReportServiceStub
    report_id: str

    def __init__(self, channel: SiftChannel, report_id: str):
        self._report_stub = ReportServiceStub(channel)
        self.report_id = report_id

    def get_result(self) -> Report_pb2:
        """Returns the result of the report."""
        req = GetReportRequest(report_id=self.report_id)
        res = cast(GetReportResponse, self._report_stub.GetReport(req))
        return res.report

    def wait_until_done(self, timeout=None) -> bool:
        """Waits until the report processing is complete or the timeout is reached."""
        start_time = time.time()
        while True:
            if timeout is not None:
                if time.time() - start_time > timeout:
                    return False

            result = self.get_result()
            all_done = all(
                [summary.status == ReportRuleStatus.REPORT_RULE_STATUS_FINISHED
                for summary in result.summaries]
            )

            if all_done:
                return True
            else:
                time.sleep(5)
