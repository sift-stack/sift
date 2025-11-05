from __future__ import annotations

import asyncio
from pathlib import Path
from typing import TYPE_CHECKING

from sift_client._internal.low_level_wrappers.data_imports import DataImportsLowLevelClient
from sift_client.resources._base import ResourceBase

# from sift_client.sift_types.data_import import (
#     Ch10Config,
#     CsvConfig,
#     DataImport,
#     ParquetConfig,
#     TDMSConfig,
# )

if TYPE_CHECKING:
    from sift_client.client import SiftClient


class DataImportsAPIAsync(ResourceBase):
    """High-level API for interacting with data imports.

    This class provides a Pythonic interface for importing data into Sift.
    """

    def __init__(self, sift_client: SiftClient):
        """Initialize the DataImportsAPI.

        Args:
            sift_client: The Sift client to use.
        """
        super().__init__(sift_client)
        self._low_level_client = DataImportsLowLevelClient(grpc_client=self.client.grpc_client)

    async def import_data(
        self,
        source: str | Path,
        *,
        csv_config: CsvConfig | None = None,
        ch10_config: Ch10Config | None = None,
        tdms_config: TDMSConfig | None = None,
        parquet_config: ParquetConfig | None = None,
    ) -> str:
        """Import data from a local file or URL.

        This method handles both local files and remote URLs. For local files, it will
        automatically upload the file before importing. For URLs (HTTP/HTTPS/S3), it will
        import directly from the URL.

        Args:
            source: Path to a local file or a URL (HTTP/HTTPS/S3).
            csv_config: Configuration for CSV files.
            ch10_config: Configuration for CH10 files.
            tdms_config: Configuration for TDMS files.
            parquet_config: Configuration for Parquet files.

        Returns:
            The data import ID (which corresponds to a Job).

        Example:
            ```python
            from sift_client.sift_types import CsvConfig, TimeColumn, TimeFormat
            from pathlib import Path

            # Create CSV configuration
            csv_config = CsvConfig(
                asset_name="my_asset",
                run_name="test_run",
                first_data_row=2,
                time_column=TimeColumn(
                    format=TimeFormat.ABSOLUTE_UNIX_SECONDS
                ).to_csv_time_column(column_number=1)
            )

            # Import from local file
            job_id = await client.data_imports.import_data(
                source=Path("data.csv"),
                csv_config=csv_config
            )

            # Import from URL
            job_id = await client.data_imports.import_data(
                source="https://example.com/data.csv",
                csv_config=csv_config
            )

            # Import from S3
            job_id = await client.data_imports.import_data(
                source="s3://bucket/data.csv",
                csv_config=csv_config
            )
            ```
        """
        # Check if source is a local file path
        if isinstance(source, Path) or (isinstance(source, str) and not self._is_url(source)):
            # Local file - upload it first
            path = Path(source) if isinstance(source, str) else source

            if not path.exists():
                raise FileNotFoundError(f"File not found: {path}")

            if not path.is_file():
                raise ValueError(f"Path is not a file: {path}")

            # Get upload URL and data import ID
            upload_url, data_import_id = await self._low_level_client.create_data_import_from_upload(
                csv_config=csv_config,
                ch10_config=ch10_config,
                tdms_config=tdms_config,
                parquet_config=parquet_config,
            )

            # Upload the file
            await self._upload_file(path, upload_url)

            return data_import_id
        else:
            # URL - import directly
            return await self._low_level_client.create_data_import_from_url(
                url=str(source),
                csv_config=csv_config,
                ch10_config=ch10_config,
                tdms_config=tdms_config,
                parquet_config=parquet_config,
            )

    @staticmethod
    def _is_url(source: str) -> bool:
        """Check if a string is a URL."""
        return source.startswith(("http://", "https://", "s3://"))

    async def _upload_file(self, path: Path, upload_url: str) -> None:
        """Upload a file to the given presigned URL.

        Args:
            path: Path to the file to upload.
            upload_url: The presigned URL to upload to.
        """
        # Run the synchronous file upload in a thread pool to avoid blocking the event loop
        loop = asyncio.get_event_loop()
        await loop.run_in_executor(None, self._upload_file_sync, path, upload_url)

    @staticmethod
    def _upload_file_sync(path: Path, upload_url: str) -> None:
        """Synchronous helper to upload the file to a presigned URL.

        This is called from a thread pool to avoid blocking the async event loop.
        
        Args:
            path: Path to the file to upload.
            upload_url: The presigned URL to upload to.
        """
        import requests

        with open(path, "rb") as f:
            response = requests.put(upload_url, data=f)

        if response.status_code not in (200, 201, 204):
            raise Exception(
                f"File upload failed with status {response.status_code}: {response.text}"
            )

    async def get(self, data_import_id: str) -> DataImport:
        """Get a data import by ID.

        Args:
            data_import_id: The ID of the data import.

        Returns:
            The DataImport object.
        """
        data_import = await self._low_level_client.get_data_import(data_import_id)
        return self._apply_client_to_instance(data_import)

    async def list_import_jobs(
        self,
        *,
        filter_query: str | None = None,
        order_by: str | None = None,
        limit: int | None = None,
    ) -> list[DataImport]:
        """List data import jobs.

        Args:
            filter_query: CEL filter query (e.g., 'status == "SUCCEEDED"').
            order_by: How to order results (e.g., 'created_date desc').
            limit: Maximum number of results to return.

        Returns:
            A list of DataImport objects.

        Example:
            ```python
            # List all successful imports
            imports = await client.data_imports.list_import_jobs(
                filter_query='status == "SUCCEEDED"',
                order_by='created_date desc',
                limit=10
            )
            ```
        """
        data_imports = await self._low_level_client.list_all_data_imports(
            query_filter=filter_query,
            order_by=order_by,
            max_results=limit,
        )
        return [self._apply_client_to_instance(di) for di in data_imports]

    async def list_(
        self,
        *,
        filter_query: str | None = None,
        order_by: str | None = None,
        limit: int | None = None,
    ) -> list[DataImport]:
        """Alias for list_import_jobs()."""
        return await self.list_import_jobs(
            filter_query=filter_query,
            order_by=order_by,
            limit=limit,
        )

    async def retry(self, data_import_id: str) -> None:
        """Retry a failed data import.

        Args:
            data_import_id: The ID of the data import to retry.
        """
        await self._low_level_client.retry_data_import(data_import_id)
