from __future__ import annotations

import logging
import warnings
from typing import TYPE_CHECKING, Mapping

from sift_client._internal.credentials import ResolvedCredentials, resolve_credentials
from sift_client._internal.disk_cache_config import DiskCacheConfig
from sift_client._internal.urls import frontend_origin_for_api
from sift_client.errors import SiftWarning
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
    UsersAPI,
    UsersAPIAsync,
)
from sift_client.resources.access_control import AccessControlAPI, AccessControlAPIAsync
from sift_client.transport import (
    GrpcClient,
    GrpcConfig,
    RestClient,
    RestConfig,
    SiftConnectionConfig,
    WithGrpcClient,
    WithRestClient,
)
from sift_client.util.cache import CacheNamespace
from sift_client.util.util import AsyncAPIs

if TYPE_CHECKING:
    from sift_client._internal.disk_cache import DiskCache

logger = logging.getLogger(__name__)


class SiftClient(
    WithGrpcClient,
    WithRestClient,
):
    """SiftClient is a high-level client for interacting with Sift's APIs.

    It provides both synchronous and asynchronous interfaces, strong type checking, and a Pythonic API design.

    Examples:
        from sift_client import SiftClient
        from datetime import datetime

        # Use the same credentials sift-cli uses, from its default profile
        client = SiftClient()

        # Or a named profile from sift.toml, like `sift-cli --profile staging`
        client = SiftClient(profile="staging")

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

    access_control: AccessControlAPI
    """Access-control APIs for configuring who can access what in Sift."""

    tags: TagsAPI
    """Instance of the Tags API for making synchronous requests."""

    test_results: TestResultsAPI
    """Instance of the Test Results API for making synchronous requests."""

    users: UsersAPI
    """Instance of the Users API for making synchronous requests."""

    data_export: DataExportAPI
    """Instance of the Data Export API for making synchronous requests."""

    data_import: DataImportAPI
    """Instance of the Data Import API for making synchronous requests."""

    cache: CacheNamespace
    """Surface for the shared on-disk cache used by every cache-aware resource."""

    async_: AsyncAPIs
    """Accessor for the asynchronous APIs. All asynchronous APIs are available as attributes on this accessor."""

    def __init__(
        self,
        api_key: str | None = None,
        grpc_url: str | None = None,
        rest_url: str | None = None,
        connection_config: SiftConnectionConfig | None = None,
        app_url: str | None = None,
        profile: str | None = None,
    ):
        """Initialize the SiftClient with specific connection parameters or a connection_config.

        Any argument left unset is resolved from the environment and from the
        ``sift.toml`` profiles that ``sift-cli`` manages, so ``SiftClient()``
        connects to the same place as ``sift-cli`` with no arguments at all.
        See :func:`sift_client.credentials.resolve_credentials` for the full
        precedence order.

        Args:
            api_key: The Sift API key for authentication.
            grpc_url: The Sift gRPC API URL.
            rest_url: The Sift REST API URL.
            connection_config: A SiftConnectionConfig object to configure the connection behavior of the SiftClient.
                When given, it is used as-is and no credential resolution happens.
            app_url: The Sift web-app origin (e.g. ``https://app.siftstack.com``).
                Set this for on-prem or custom deployments whose API host can't be
                mapped to a frontend automatically; see the ``app_url`` property.
                A value here takes precedence over ``connection_config.app_url``.
            profile: Name of a ``sift.toml`` profile to draw credentials from,
                equivalent to ``sift-cli --profile``. Ignored when
                ``connection_config`` is given.

        Raises:
            SiftCredentialsError: No ``connection_config`` was given and the API
                key or either URL could not be resolved.

        """
        self._credentials: ResolvedCredentials | None = None

        if connection_config:
            grpc_client = GrpcClient(connection_config.get_grpc_config())
            rest_client = RestClient(connection_config.get_rest_config())
        else:
            creds = resolve_credentials(
                api_key=api_key,
                grpc_url=grpc_url,
                rest_url=rest_url,
                app_url=app_url,
                profile=profile,
            )
            self._credentials = creds
            # ``use_ssl`` comes from the gRPC URL's scheme: the transport strips
            # the scheme off and would otherwise dial an ``http://`` endpoint
            # over TLS.
            grpc_client = GrpcClient(
                GrpcConfig(creds.grpc_url, creds.api_key, use_ssl=creds.use_ssl)
            )
            rest_client = RestClient(
                RestConfig(creds.rest_url, creds.api_key, use_ssl=creds.use_ssl)
            )
            app_url = creds.app_url

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

        # Shared on-disk cache: user intent in ``_disk_cache_config`` (opt-out
        # default), live handle in ``_disk_cache`` (lazy so importing this
        # module doesn't pay the diskcache cost up front). The
        # ``client.cache`` namespace mutates both.
        self._disk_cache_config = DiskCacheConfig(enabled=True)
        self._disk_cache: DiskCache | None = None
        self.cache = CacheNamespace(self)

        self.ping = PingAPI(self)
        self.assets = AssetsAPI(self)
        self.calculated_channels = CalculatedChannelsAPI(self)
        self.channels = ChannelsAPI(self)
        self.file_attachments = FileAttachmentsAPI(self)
        self.jobs = JobsAPI(self)
        self.rules = RulesAPI(self)
        self.reports = ReportsAPI(self)
        self.runs = RunsAPI(self)
        self.access_control = AccessControlAPI(
            resource_attributes=ResourceAttributesAPI(self),
            principal_attributes=PrincipalAttributesAPI(self),
        )
        self.tags = TagsAPI(self)
        self.test_results = TestResultsAPI(self)
        self.users = UsersAPI(self)
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
            access_control=AccessControlAPIAsync(
                resource_attributes=ResourceAttributesAPIAsync(self),
                principal_attributes=PrincipalAttributesAPIAsync(self),
            ),
            tags=TagsAPIAsync(self),
            test_results=TestResultsAPIAsync(self),
            users=UsersAPIAsync(self),
            data_export=DataExportAPIAsync(self),
            data_import=DataImportAPIAsync(self),
        )

    @classmethod
    def from_profile(cls, profile: str, **kwargs) -> SiftClient:
        """Build a client from a named ``sift.toml`` profile.

        Equivalent to ``SiftClient(profile=...)``; keyword arguments are passed
        through and still take precedence over the profile's values.

        Args:
            profile: Profile name, as used by ``sift-cli --profile``.
            **kwargs: Any other :class:`SiftClient` argument.

        Returns:
            A client connected to the endpoints that profile names.
        """
        return cls(profile=profile, **kwargs)

    @property
    def credential_sources(self) -> Mapping[str, str] | None:
        """Which layer supplied each credential, for diagnosing connections.

        Maps ``api_key`` / ``grpc_url`` / ``rest_url`` / ``app_url`` to
        ``"arg"``, ``"profile:<name>"``, ``"env"``, ``"default"``, or
        ``"unset"``. ``None`` when the client was built from an explicit
        ``connection_config``, which bypasses resolution.
        """
        return self._credentials.sources if self._credentials else None

    @property
    def profile(self) -> str | None:
        """The ``sift.toml`` profile this client resolved its credentials from."""
        return self._credentials.profile if self._credentials else None

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

    def _get_disk_cache(self) -> DiskCache:
        """Lazy accessor for the shared on-disk cache. Internal to resources.

        The cache is built on first use so that importing ``sift_client``
        doesn't pay the ``diskcache``/``sqlite`` cost up front. The opt-out
        default ("disk caching on at the temp-dir path") is applied here,
        along with the silent-fallback-on-default-path failure: if the
        user left :class:`DiskCacheConfig` at its defaults and opening
        fails (read-only ``/tmp``, restricted container, ...), we log a
        warning and return a disabled :class:`DiskCache` so resources can
        still serve requests by going to the wire. An explicit user-
        supplied path that can't be opened propagates so the caller knows
        their request didn't take.

        After the first call this just returns the memoized handle.
        Subsequent ``client.cache.enable(...)`` calls mutate the
        existing handle in place; this method is not re-entered.
        """
        if self._disk_cache is None:
            from sift_client._internal.disk_cache import DiskCache

            config = self._disk_cache_config
            if not config.enabled:
                self._disk_cache = DiskCache()
                return self._disk_cache
            target_path = config.path or DiskCache.DEFAULT_DISK_PATH
            try:
                self._disk_cache = DiskCache(
                    disk_path=target_path,
                    disk_max_bytes=config.max_bytes,
                )
            except Exception:
                if not config.using_default_path:
                    raise
                warnings.warn(
                    f"Could not open the default sift data cache at {target_path}; "
                    "falling back to no caching. Call "
                    "``client.cache.disable()`` to silence this "
                    "warning, or pass an explicit path via "
                    "``client.cache.enable(path=...)``.",
                    SiftWarning,
                    stacklevel=2,
                )
                self._disk_cache = DiskCache()
        return self._disk_cache

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
