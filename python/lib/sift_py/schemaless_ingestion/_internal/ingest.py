from typing import Dict, List, Optional, Union
from urllib.parse import urljoin

from sift_py._internal.rest import create_run, list_runs
from sift_py.rest import SiftRestConfig, _RestService
from sift_py.schemaless_ingestion._internal.payload import SchemalessDataPayload, SchemalessPayload
from sift_py.schemaless_ingestion.data import SchemalessData


class _SchemalessIngestionServiceImpl(_RestService):
    INGEST_ENDPOINT = "/api/v2/ingest"
    RUN_ENDPOINT = "/api/v2/runs"

    asset_name: str
    run_id: Optional[str]
    organization_id: Optional[str]
    _ingest_uri: str
    _run_uri: str

    def __init__(
        self,
        rest_conf: SiftRestConfig,
        asset_name: str,
        run_id: Optional[str],
        organization_id: Optional[str],
    ):
        super().__init__(rest_conf=rest_conf)

        self.asset_name = asset_name
        self.run_id = run_id
        self.organization_id = organization_id
        self._ingest_uri = urljoin(self._base_uri, self.INGEST_ENDPOINT)
        self._run_uri = urljoin(self._base_uri, self.RUN_ENDPOINT)

    def attach_run(
        self,
        run_name: str,
        description: Optional[str],
        organization_id: Optional[str],
        tags: Optional[List[str]] = None,
        metadata: Optional[Dict[str, Union[str, float, bool]]] = None,
        force_new: bool = False,
    ):
        """
        Retrieve an existing run or create one to use during this period of ingestion.

        Include `force_new=True` to force the creation of a new run, which will allow creation of a new run using an existing name.
        """

        if not force_new:
            run_id = self._get_run_id_by_name(run_name)
            if run_id:
                self.run_id = run_id
                return

        run_id = create_run(
            self,
            run_name,
            description,
            organization_id,
            tags,
            metadata,
        )

        self.run_id = run_id

    def detach_run(self):
        """
        Detach run from this period of ingestion. Subsequent data ingested won't be associated with
        the run being detached.
        """
        self.run_id = None

    def ingest(self, *data: SchemalessData):
        """
        This method performs the actual data ingestion of one or more SchemalessData objects.
        """

        rfc3339_data = []
        for item in data:
            if item["timestamp"].tzname() is None:
                raise Exception("Timestamps require a timezone to be defined")

            rfc3339_data.append(
                SchemalessDataPayload(
                    timestamp=item["timestamp"].isoformat(), values=item["values"]
                )
            )

        payload = SchemalessPayload(
            asset_name=self.asset_name,
            run_id=self.run_id,
            organization_id=self.organization_id,
            data=rfc3339_data,
        )

        self.send_payload(payload=payload)

    def send_payload(self, payload: SchemalessDataPayload):
        response = self._session.post(url=self._ingest_uri, json=payload)

        if response.status_code != 200:
            raise Exception(
                f"Schemaless data ingestion request failed with status code {response.status_code}. {response.text}"
            )

    def _get_run_id_by_name(
        self,
        run_name: str,
    ) -> Optional[str]:
        resp = list_runs(
            self,
            page_size=1,
            filter=f'name=="{run_name}"',
        )

        if "runs" not in resp or not isinstance(resp["runs"], list):
            raise Exception("Unexpected response from list_runs request")

        if not resp["runs"]:
            return None

        return resp["runs"][0]["runId"]
