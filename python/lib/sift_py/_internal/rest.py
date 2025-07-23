import json
from typing import Dict, List, Optional, Union
from urllib.parse import urljoin

from sift_py.rest import _RestService

INGEST_ENDPOINT = "/api/v2/ingest"
RUN_ENDPOINT = "/api/v2/runs"


def create_run(
    rest_svc: _RestService,
    run_name: str,
    description: Optional[str] = None,
    organization_id: Optional[str] = None,
    tags: Optional[List[str]] = None,
    metadata: Optional[Dict[str, Union[str, float, bool]]] = None,
) -> str:
    """
    Retrieve an existing run or create one to use during this period of ingestion.

    Include `force_new=True` to force the creation of a new run, which will allow creation of a new run using an existing name.
    """

    payload = {
        "name": run_name,
        "description": description,
    }
    if organization_id:
        payload["organizationId"] = organization_id
    if tags:
        payload["tags"] = tags
    if metadata:
        payload["metadata"] = metadata

    response = rest_svc._session.post(url=urljoin(rest_svc._base_uri, RUN_ENDPOINT), json=payload)
    if response.status_code != 200:
        raise Exception(
            f"Run creation failed with status code {response.status_code}. {response.text}"
        )

    try:
        run_info = response.json()
    except (json.decoder.JSONDecodeError, KeyError):
        raise Exception(f"Invalid response: {response.text}")

    if "run" not in run_info:
        raise Exception("Response missing key: run")
    if "runId" not in run_info["run"]:
        raise Exception("Response missing key: runId")

    return run_info["run"]["runId"]


def list_runs(
    rest_svc: _RestService,
    page_size: Optional[int] = None,
    page_token: Optional[str] = None,
    filter: Optional[str] = None,
    order_by: Optional[str] = None,
) -> Dict:
    uri = urljoin(rest_svc._base_uri, RUN_ENDPOINT)

    query_items = []
    if page_size:
        query_items.append(f"pageSize={page_size}")
    if page_token:
        query_items.append(f"pageToken={page_token}")
    if filter:
        query_items.append(f"filter={filter}")
    if order_by:
        query_items.append(f"orderBy={order_by}")

    if query_items:
        query = "?" + "&".join(query_items)
        uri += query

    response = rest_svc._session.get(url=uri)

    if response.status_code != 200:
        raise Exception(
            f"List runs failed with status code {response.status_code}. {response.text}"
        )

    return response.json()
