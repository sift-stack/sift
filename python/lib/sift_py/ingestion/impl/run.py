from typing import List, Optional, cast

from sift.runs.v2.runs_pb2 import (
    CreateRunRequest,
    CreateRunResponse,
    ListRunsRequest,
    ListRunsResponse,
)
from sift.runs.v2.runs_pb2_grpc import RunServiceStub

from sift_py.grpc.transport import SiftChannel


def get_run_id_by_name(
    channel: SiftChannel,
    run_name: str,
) -> Optional[str]:
    svc = RunServiceStub(channel)
    req = ListRunsRequest(
        filter=f'name=="{run_name}"',
        page_size=1,
    )
    res = cast(ListRunsResponse, svc.ListRuns(req))

    if len(res.runs) == 0:
        return None

    return res.runs[0].run_id


def create_run(
    channel: SiftChannel,
    run_name: str,
    description: str,
    organization_id: str,
    tags: List[str],
) -> str:
    svc = RunServiceStub(channel)
    req = CreateRunRequest(
        name=run_name,
        description=description,
        organization_id=organization_id,
        tags=tags,
    )
    res = cast(CreateRunResponse, svc.CreateRun(req))
    return res.run.run_id
