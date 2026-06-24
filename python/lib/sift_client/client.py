from __future__ import annotations

from sift_client._internal.urls import frontend_origin_for_api
from sift_client.resources import (
    AssetsAPI,
    AssetsAPIAsync,
    CalculatedChannelsAPI,
    CalculatedChannelsAPIAsync,
    ChannelsAPI,
    ChannelsAPIAsync,
    DataExportAPI,
    DataExportAPIAsync,
    DataImportAPI,
    DataImportAPIAsync,
    FileAttachmentsAPI,
    FileAttachmentsAPIAsync,
    IngestionAPIAsync,
    JobsAPI,
    JobsAPIAsync,
    PingAPI,
    PingAPIAsync,
    PrincipalAttributesAPI,
    PrincipalAttributesAPIAsync,
    ReportsAPI,
    ReportsAPIAsync,
    ResourceAttributesAPI,
    ResourceAttributesAPIAsync,
    RulesAPI,
    RulesAPIAsync,
    RunsAPI,
    RunsAPIAsync,
    TagsAPI,
    TagsAPIAsync,
    TestResultsAPI,
    TestResultsAPIAsync,
)
from sift_client.transport import (
    GrpcClient,
    GrpcConfig,
    RestClient,
    RestConfig,
    SiftConnectionConfig,
    WithGrpcClient,
    WithRestClient,
)
from sift_client.util.util import AsyncAPIs


class SiftClient(
    WithGrpcClient,
    WithRestClient,
):
    """SiftClient is a high-level client for interacting with Sift's APIs.

    It provides both synchronous and asynchronous interfaces, strong type checking, and a Pythonic API design.

    Examples:
        from sift_client import SiftClient
        from datetime import datetime

        # Initialize with individual parameters
        client = SiftClient(
            api_key="your-api-key",
            grpc_url="your-sift-grpc-url",
            rest_url="your-sift-rest-url")

        # Or use a connection configuration to customize connection behavior
        connection_config = SiftConnectionConfig(
            grpc_config=GrpcConfig(),
            rest_config=RestConfig())

        sift = SiftClient(connection_config=connection_config)

        # Use the client to make requests
        response = sift.ping.ping()

        # Or asynchronously
        response = await sift.async_.ping.ping()
    """

    ping: PingAPI
    """Instance of the Ping API for making synchronous requests."""

    assets: AssetsAPI
    """Instance of the Assets API for making synchronous requests."""

    calculated_channels: CalculatedChannelsAPI
    """Instance of the Calculated Channels API for making synchronous requests."""

    channels: ChannelsAPI
    """Instance of the Channels API for making synchronous requests."""

    file_attachments: FileAttachmentsAPI
    """Instance of the File Attachments API for making synchronous requests."""

    ingestion: IngestionAPIAsync
    """Instance of the Ingestion API for making synchronous requests."""

    jobs: JobsAPI
    """Instance of the Jobs API for making synchronous requests."""

    reports: ReportsAPI
    """Instance of the Reports API for making synchronous requests."""

    rules: RulesAPI
    """Instance of the Rules API for making synchronous requests."""

    runs: RunsAPI
    """Instance of the Runs API for making synchronous requests."""

    resource_attributes: ResourceAttributesAPI
    """Instance of the Resource Attributes API for making synchronous requests."""

    principal_attributes: PrincipalAttributesAPI
    """Instance of the Principal Attributes API for making synchronous requests."""

    tags: TagsAPI
    """Instance of the Tags API for making synchronous requests."""

    test_results: TestResultsAPI
    """Instance of the Test Results API for making synchronous requests."""

    data_export: DataExportAPI
    """Instance of the Data Export API for making synchronous requests."""

    data_import: DataImportAPI
    """Instance of the Data Import API for making synchronous requests."""

    async_: AsyncAPIs
    """Accessor for the asynchronous APIs. All asynchronous APIs are available as attributes on this accessor."""

    def __init__(
        self,
        api_key: str | None = None,
        grpc_url: str | None = None,
        rest_url: str | None = None,
        connection_config: SiftConnectionConfig | None = None,
        app_url: str | None = None,
        data_cache_max_bytes: int | None = None,
    ):
        """Initialize the SiftClient with specific connection parameters or a connection_config.

        Args:
            api_key: The Sift API key for authentication.
            grpc_url: The Sift gRPC API URL.
            rest_url: The Sift REST API URL.
            connection_config: A SiftConnectionConfig object to configure the connection behavior of the SiftClient.
            app_url: The Sift web-app origin (e.g. ``https://app.siftstack.com``).
                Set this for on-prem or custom deployments whose API host can't be
                mapped to a frontend automatically; see the ``app_url`` property.
                A value here takes precedence over ``connection_config.app_url``.
            data_cache_max_bytes: Cap on the in-memory channel data cache used
                by ``client.channels.get_data`` (bytes). When the bound is
                reached, the least-recently-used cached channel is evicted.
                Defaults to 512 MiB. Set to ``0`` to disable caching. Must be
                ``>= 0``.
        """
        if data_cache_max_bytes is not None and data_cache_max_bytes < 0:
            raise ValueError(f"data_cache_max_bytes must be >= 0, got {data_cache_max_bytes}")
        if not (api_key and grpc_url and rest_url) and not connection_config:
            raise ValueError(
                "Either api_key, grpc_url and rest_url or connection_config must be provided to establish a connection."
            )

        if connection_config:
            grpc_client = GrpcClient(connection_config.get_grpc_config())
            rest_client = RestClient(connection_config.get_rest_config())
        elif api_key and grpc_url and rest_url:
            grpc_client = GrpcClient(GrpcConfig(grpc_url, api_key))
            rest_client = RestClient(RestConfig(rest_url, api_key))
        else:
            raise ValueError(
                "Invalid connection configuration. Please provide api_key, grpc_uri and rest_uri or a connection_config."
            )

        WithGrpcClient.__init__(self, grpc_client=grpc_client)
        WithRestClient.__init__(self, rest_client=rest_client)

        # Explicit web-app origin override; falls back to the connection config's
        # value, then to host-based derivation in the ``app_url`` property.
        self._app_url: str | None = app_url or (
            connection_config.app_url if connection_config else None
        )

        # When set, test-results writes return synthesized responses without
        # contacting Sift. Read by `TestResultsAPIAsync._simulate`. Used by the
        # pytest plugin's ``--sift-disabled`` mode.
        self._simulate: bool = False

        # Read by ``ChannelsAPIAsync._ensure_data_low_level_client`` when it
        # lazily constructs the data wrapper. ``None`` means "use the wrapper
        # default" so we don't have to import the constant here.
        self._data_cache_max_bytes: int | None = data_cache_max_bytes

        self.ping = PingAPI(self)
        self.assets = AssetsAPI(self)
        self.calculated_channels = CalculatedChannelsAPI(self)
        self.channels = ChannelsAPI(self)
        self.file_attachments = FileAttachmentsAPI(self)
        self.jobs = JobsAPI(self)
        self.rules = RulesAPI(self)
        self.reports = ReportsAPI(self)
        self.runs = RunsAPI(self)
        self.resource_attributes = ResourceAttributesAPI(self)
        self.principal_attributes = PrincipalAttributesAPI(self)
        self.tags = TagsAPI(self)
        self.test_results = TestResultsAPI(self)
        self.data_export = DataExportAPI(self)
        self.data_import = DataImportAPI(self)

        # Accessor for the asynchronous APIs
        self.async_ = AsyncAPIs(
            ping=PingAPIAsync(self),
            assets=AssetsAPIAsync(self),
            calculated_channels=CalculatedChannelsAPIAsync(self),
            channels=ChannelsAPIAsync(self),
            file_attachments=FileAttachmentsAPIAsync(self),
            ingestion=IngestionAPIAsync(self),
            jobs=JobsAPIAsync(self),
            reports=ReportsAPIAsync(self),
            rules=RulesAPIAsync(self),
            runs=RunsAPIAsync(self),
            resource_attributes=ResourceAttributesAPIAsync(self),
            principal_attributes=PrincipalAttributesAPIAsync(self),
            tags=TagsAPIAsync(self),
            test_results=TestResultsAPIAsync(self),
            data_export=DataExportAPIAsync(self),
            data_import=DataImportAPIAsync(self),
        )

    @property
    def grpc_client(self) -> GrpcClient:
        """The gRPC client used by the SiftClient for making gRPC API calls."""
        return self._grpc_client

    @property
    def is_loop_running(self) -> bool:
        """Whether the background event loop is still accepting synchronous API work."""
        return self._grpc_client.is_loop_running

    @property
    def rest_client(self) -> RestClient:
        """The REST client used by the SiftClient for making REST API calls."""
        return self._rest_client

    @property
    def app_url(self) -> str | None:
        """The Sift web-app origin for this client, or None if it can't be determined.

        Uses the explicit override passed at construction when set, otherwise
        derives the origin from the REST host for known Sift deployments (e.g.
        ``https://api.siftstack.com`` -> ``https://app.siftstack.com``). Returns
        None for unrecognized hosts with no override.

        # TODO: Add a ``WithAppPage`` mixin on BaseType so resources (TestReport,
        # Run, ...) can expose their own web-app link from ``_client.app_url`` plus
        # a per-type path, instead of callers assembling paths by hand.
        """
        return frontend_origin_for_api(self.rest_client.base_url, override=self._app_url)
