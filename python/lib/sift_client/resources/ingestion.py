from __future__ import annotations

import logging
from typing import TYPE_CHECKING, Iterator

from sift_client._internal.low_level_wrappers.ingestion import (
    IngestionConfigStreamingLowLevelClient,
    IngestionLowLevelClient,
)
from sift_client.errors import _sift_stream_bindings_import_error
from sift_client.resources._base import ResourceBase
from sift_client.sift_types.ingestion import Flow, IngestionConfig, IngestionConfigCreate
from sift_client.sift_types.run import Run, RunCreate, Tag

if TYPE_CHECKING:
    from collections.abc import Iterable

    from sift_stream_bindings import (
        DiskBackupPolicyPy,
        DurationPy,
        FlowDescriptorPy,
        FlowPy,
        IngestionConfigFormPy,
        IngestWithConfigDataStreamRequestPy,
        IngestWithConfigDataStreamRequestWrapperPy,
        MetadataPy,
        RecoveryStrategyPy,
        RetryPolicyPy,
        RunFormPy,
        SiftStreamMetricsSnapshotPy,
    )

    from sift_client.client import SiftClient
    from sift_client.sift_types.ingestion import FlowConfig

logger = logging.getLogger(__name__)


class TracingConfig:
    """Configuration for tracing in SiftStream.

    This class provides factory methods to create tracing configurations for use
    with IngestionConfigStreamingClient. Tracing will only be initialized once per process.
    """

    def __init__(
        self,
        is_enabled: bool = True,
        level: str = "info",
        log_dir: str | None = None,
        filename_prefix: str | None = None,
        max_log_files: int | None = None,
    ):
        """Initialize a TracingConfig.

        Args:
            is_enabled: Whether tracing is enabled. Defaults to True.
            level: Logging level as string - one of "trace", "debug", "info", "warn", "error".
                Defaults to "info".
            log_dir: Directory path for log files. Required if using file logging.
                Defaults to "./logs" when using with_file.
            filename_prefix: Prefix for log filenames. Required if using file logging.
                Defaults to "sift_stream_bindings.log" when using with_file.
            max_log_files: Maximum number of log files to keep. Required if using file logging.
                Defaults to 7 when using with_file.
        """
        self.is_enabled = is_enabled
        self.level = level
        self.log_dir = log_dir
        self.filename_prefix = filename_prefix
        self.max_log_files = max_log_files

    @classmethod
    def disabled(cls) -> TracingConfig:
        """Create a configuration that disables tracing.

        Returns:
            A TracingConfig with tracing disabled.
        """
        return cls(is_enabled=False)

    @classmethod
    def console_only(cls, level: str = "info") -> TracingConfig:
        """Create a configuration that enables tracing to stdout/stderr only.

        Args:
            level: Logging level as string - one of "trace", "debug", "info", "warn", "error".
                Defaults to "info".

        Returns:
            A TracingConfig with tracing enabled (outputs to stdout/stderr only).
        """
        return cls(level=level)

    @classmethod
    def with_file(
        cls,
        level: str = "info",
        log_dir: str = "./logs",
        filename_prefix: str = "sift_stream_bindings.log",
        max_log_files: int = 7,
    ) -> TracingConfig:
        """Create a configuration that enables tracing to both stdout and rolling log files.

        Args:
            level: Logging level as string - one of "trace", "debug", "info", "warn", "error".
                Defaults to "info".
            log_dir: Directory path for log files. Defaults to "./logs".
            filename_prefix: Prefix for log filenames. Defaults to "sift_stream_bindings.log".
            max_log_files: Maximum number of log files to keep. Defaults to 7.

        Returns:
            A TracingConfig with tracing enabled for both stdout and file output.
        """
        return cls(
            level=level,
            log_dir=log_dir,
            filename_prefix=filename_prefix,
            max_log_files=max_log_files,
        )


class RecoveryStrategyConfig:
    """Configuration for the SiftStream recovery strategy.

    This class provides a Python-friendly interface for configuring the recovery strategy used in SiftStream.
    Recovery strategies determine how SiftStream handles failures and retries when ingesting data.

    Recovery strategies control:
    - How frequently to retry a failed connection to Sift.
    - Whether to use per checkpoint backups to allow re-ingestion of data to Sift after a streaming failure.
    - Settings to control the number and size of backup files, and whether to retain backups after verification of successful ingestion into sift.

    Most users should use one of the factory methods:
    - `retry_only()` - Only attempt to reconnect to Sift after a connection failure. Any data which failed to be ingested will be lost.
      - More performant, but with no guarantee of data ingestion.
    - `retry_with_backups()` - Ingestion is checkpointed. If an ingestion issue occurs during a checkpoint, that data will be re-ingested into Sift
      asynchronously along with incoming live data. Backup files are generated and by default, cleared after a successful checkpoint or re-ingestion.
    """

    def __init__(self, recovery_strategy_py: RecoveryStrategyPy | None):
        """Initialize a RecoveryStrategyConfig.

        Args:
            recovery_strategy_py: The underlying RecoveryStrategyPy instance.
                If None, uses the default retry_with_backups strategy.

        Note:
            Most users should use the factory methods (`retry_only()` or `retry_with_backups()`)
            instead of calling this constructor directly.
        """
        # Importing here to allow sift_stream_bindings to be an optional dependancy for non-ingestion users
        try:
            from sift_stream_bindings import DiskBackupPolicyPy, RecoveryStrategyPy, RetryPolicyPy
        except ImportError as e:
            _sift_stream_bindings_import_error(e)

        # Default to retry_with_backups()
        # This is intentionally different from SiftStream, which defaults to retry_only
        self._recovery_strategy_py = recovery_strategy_py or RecoveryStrategyPy.retry_with_backups(
            retry_policy=RetryPolicyPy.default(), disk_backup_policy=DiskBackupPolicyPy.default()
        )

    def _to_rust_config(self) -> RecoveryStrategyPy:
        """Convert to RecoveryStrategyPy for use with the ingestion client.

        Returns:
            A RecoveryStrategyPy instance that can be passed to the ingestion client.
        """
        return self._recovery_strategy_py

    @classmethod
    def retry_only(cls, retry_policy: RetryPolicyPy | None = None) -> RecoveryStrategyConfig:
        """Create a recovery strategy that only retries connection failures.

        Args:
            retry_policy: Retry policy configuration specifying retry attempts, backoff timing, etc.
                If None, uses the default retry policy (5 attempts, 50ms initial backoff,
                5s max backoff, multiplier of 5).

        Returns:
            A RecoveryStrategyConfig configured for retry-only strategy.
        """
        # Importing here to allow sift_stream_bindings to be an optional dependancy for non-ingestion users
        try:
            from sift_stream_bindings import RecoveryStrategyPy, RetryPolicyPy
        except ImportError as e:
            _sift_stream_bindings_import_error(e)

        retry_policy_py = retry_policy or RetryPolicyPy.default()

        recovery_strategy_py = RecoveryStrategyPy.retry_only(retry_policy_py)
        return cls(recovery_strategy_py=recovery_strategy_py)

    @classmethod
    def retry_with_backups(
        cls,
        retry_policy: RetryPolicyPy | None = None,
        disk_backup_policy: DiskBackupPolicyPy | None = None,
    ) -> RecoveryStrategyConfig:
        """Create a recovery strategy with retries re-ingestion using disk based backups.

        Args:
            retry_policy: Retry policy configuration specifying retry attempts, backoff timing, etc.
                If None, uses the default retry policy (5 attempts, 50ms initial backoff,
                5s max backoff, multiplier of 5).
            disk_backup_policy: Disk backup policy configuration specifying backup directory,
                file size limits, etc. If None, uses the default disk backup policy.

        Returns:
            A RecoveryStrategyConfig configured for retry with disk backups.
        """
        # Importing here to allow sift_stream_bindings to be an optional dependancy for non-ingestion users
        try:
            from sift_stream_bindings import DiskBackupPolicyPy, RecoveryStrategyPy, RetryPolicyPy
        except ImportError as e:
            _sift_stream_bindings_import_error(e)

        retry_policy_py = retry_policy or RetryPolicyPy.default()
        disk_backup_policy_py = disk_backup_policy or DiskBackupPolicyPy.default()

        recovery_strategy_py = RecoveryStrategyPy.retry_with_backups(
            retry_policy=retry_policy_py,
            disk_backup_policy=disk_backup_policy_py,
        )
        return cls(recovery_strategy_py=recovery_strategy_py)


class IngestionAPIAsync(ResourceBase):
    """High-level API for interacting with ingestion services.

    This class provides a Pythonic, notebook-friendly interface for interacting with the IngestionAPI.
    It handles automatic handling of gRPC services, seamless type conversion, and clear error handling.

    All methods in this class use the Flow class from the types module, which is a user-friendly
    representation of ingestion flows using standard Python data structures and types.
    """

    def __init__(self, sift_client: SiftClient):
        """Initialize the IngestionAPI.

        Args:
            sift_client: The Sift client to use.
        """
        super().__init__(sift_client)
        self._low_level_client = IngestionLowLevelClient(grpc_client=self.client.grpc_client)

    async def create_ingestion_config_streaming_client(
        self,
        ingestion_config: IngestionConfig | IngestionConfigCreate | IngestionConfigFormPy,
        *,
        run: RunCreate | dict | str | Run | None = None,
        asset_tags: list[str] | list[Tag] | None = None,
        asset_metadata: dict[str, str | float | bool] | None = None,
        recovery_strategy: RecoveryStrategyConfig | RecoveryStrategyPy | None = None,
        checkpoint_interval_seconds: int | None = None,
        enable_tls: bool = True,
        tracing_config: TracingConfig | None = None,
    ) -> IngestionConfigStreamingClient:
        """Create an IngestionConfigStreamingClient.

        Args:
            ingestion_config: The ingestion config. Can be a IngestionConfig or IngestionConfigFormPy.
            run: The run to associate with ingestion. Can be a Run, RunCreate, dict, or run ID string.
            asset_tags: Tags to associate with the asset.
            asset_metadata: Metadata to associate with the asset.
            recovery_strategy: The recovery strategy to use for ingestion.
            checkpoint_interval_seconds: The checkpoint interval in seconds.
            enable_tls: Whether to enable TLS for the connection.
            tracing_config: Configuration for SiftStream tracing. Use TracingConfig.stdout_only()
                to enable tracing to stdout only, or TracingConfig.stdout_with_file() to enable
                tracing to both stdout and rolling log files. Defaults to None (tracing will be
                initialized with default settings if not already initialized).

        Returns:
            An initialized IngestionConfigStreamingClient.
        """
        return await IngestionConfigStreamingClient._create(
            self.client,
            ingestion_config=ingestion_config,
            run=run,
            asset_tags=asset_tags,
            asset_metadata=asset_metadata,
            recovery_strategy=recovery_strategy,
            checkpoint_interval_seconds=checkpoint_interval_seconds,
            enable_tls=enable_tls,
            tracing_config=tracing_config,
        )


class IngestionConfigStreamingClient(ResourceBase):
    """A client for streaming ingestion with an ingestion config.

    This client provides a high-level interface for streaming data to Sift using
    an ingestion config. Under the hood, this client uses the Rust powered SiftStream library to provide
    a high-performance, low-latency, and reliable streaming interface to Sift.

    This client should be initialized using the create classmethod, and not directly. Once streaming has ended, the client should be shutdown using the finish method.
    """

    def __init__(
        self, sift_client: SiftClient, low_level_client: IngestionConfigStreamingLowLevelClient
    ):
        """Initialize an IngestionConfigStreamingClient. Users should not initialize this class directly, but rather use the create classmethod."""
        super().__init__(sift_client)
        self._low_level_client = low_level_client

    @classmethod
    async def _create(
        cls,
        sift_client: SiftClient,
        ingestion_config: IngestionConfig | IngestionConfigCreate | IngestionConfigFormPy,
        *,
        run: RunCreate | dict | str | Run | RunFormPy | None = None,
        asset_tags: list[str] | list[Tag] | None = None,
        asset_metadata: dict[str, str | float | bool] | None = None,
        recovery_strategy: RecoveryStrategyConfig | RecoveryStrategyPy | None = None,
        checkpoint_interval_seconds: int | None = None,
        enable_tls: bool = True,
        tracing_config: TracingConfig | None = None,
    ) -> IngestionConfigStreamingClient:
        """Create an IngestionConfigStreamingClient.

        Args:
            sift_client: The Sift client to use.
            ingestion_config: The ingestion config to use for streaming.
            run: The run to associate with ingestion. Can be a Run, RunCreate, dict, or run ID string.
            asset_tags: Tags to associate with the asset.
            asset_metadata: Metadata to associate with the asset.
            recovery_strategy: The recovery strategy to use for ingestion.
            checkpoint_interval_seconds: The checkpoint interval in seconds.
            enable_tls: Whether to enable TLS for the connection.
            tracing_config: Configuration for SiftStream tracing. Use TracingConfig.console_only()
                to enable tracing to stdout only, or TracingConfig.with_file() to enable
                tracing to both stdout and rolling log files. Defaults to None (tracing will be
                initialized with default settings for TracingConfig.with_file()).

        Returns:
            An initialized IngestionConfigStreamingClient.
        """
        # Importing here to allow sift_stream_bindings to be an optional dependancy for non-ingestion users
        try:
            from sift_stream_bindings import (
                DurationPy,
                IngestionConfigFormPy,
                MetadataPy,
                MetadataValuePy,
                RecoveryStrategyPy,
                RunFormPy,
            )
        except ImportError as e:
            _sift_stream_bindings_import_error(e)

        instance = cls.__new__(cls)
        instance._sift_client = sift_client

        # Get API key and gRPC URI from the client
        grpc_config = sift_client.grpc_client._config
        api_key = grpc_config.api_key
        grpc_uri = grpc_config.uri

        # Convert the ingestion_config variants to a IngestionConfigFormPy
        if isinstance(ingestion_config, IngestionConfig):
            # SiftStream will retrieve the existing config from the client_key
            asset = sift_client.assets.get(asset_id=ingestion_config.asset_id)
            ingestion_config_form = IngestionConfigFormPy(
                asset_name=asset.name,
                client_key=ingestion_config.client_key,
                flows=[],
            )
        elif isinstance(ingestion_config, IngestionConfigCreate):
            ingestion_config_form = ingestion_config._to_rust_form()
        else:
            ingestion_config_form = ingestion_config

        # Convert the recovery strategy variants
        recovery_strategy_py: RecoveryStrategyPy | None = None
        if isinstance(recovery_strategy, RecoveryStrategyConfig):
            recovery_strategy_py = recovery_strategy._to_rust_config()
        elif isinstance(recovery_strategy, RecoveryStrategyPy):
            recovery_strategy_py = recovery_strategy

        # Convert the run variants to a run or run_id
        run_form: RunFormPy | None = None
        run_id: str | None = None
        if isinstance(run, RunFormPy):
            run_form = run
        elif isinstance(run, str):
            run_id = run
        elif isinstance(run, dict):
            run_create = RunCreate.model_validate(run)
            run_form = run_create._to_rust_form()
        elif isinstance(run, Run):
            run_id = run._id_or_error
        elif isinstance(run, RunCreate):
            run_form = run._to_rust_form()

        # Convert asset_tags to list of strings
        asset_tags_list: list[str] | None = None
        if asset_tags is not None:
            asset_tags_list = [tag.name if isinstance(tag, Tag) else tag for tag in asset_tags]

        # Convert asset_metadata dict to list of MetadataPy
        asset_metadata_list: list[MetadataPy] | None = None
        if asset_metadata is not None:
            asset_metadata_list = [
                MetadataPy(key=key, value=MetadataValuePy(value))
                for key, value in asset_metadata.items()
            ]

        # Convert checkpoint_interval_seconds to DurationPy
        checkpoint_interval: DurationPy | None = None
        if checkpoint_interval_seconds is not None:
            checkpoint_interval = DurationPy(secs=checkpoint_interval_seconds, nanos=0)

        low_level_client = await IngestionConfigStreamingLowLevelClient.create_sift_stream_instance(
            api_key=api_key,
            grpc_uri=grpc_uri,
            ingestion_config=ingestion_config_form,
            run_form=run_form,
            run_id=run_id,
            asset_tags=asset_tags_list,
            asset_metadata=asset_metadata_list,
            recovery_strategy=recovery_strategy_py,
            checkpoint_interval=checkpoint_interval,
            enable_tls=enable_tls,
            tracing_config=tracing_config,
        )

        return cls(sift_client, low_level_client)

    async def send(self, flow: Flow | FlowPy):
        """Send telemetry to Sift in the form of a Flow.

        This is the entry-point to send actual telemetry to Sift. If a message is sent that
        doesn't match any flows that the stream knows about locally, the message will still be
        transmitted and a warning log emitted. If you are certain that the message corresponds
        to an unregistered flow then `add_new_flows` should be called first to register the flow
        before calling `send`; otherwise you should monitor the Sift DLQ either in the Sift UI
        or Sift API to ensure successful transmission.

        When sending messages, if backups are enabled, first the message is sent to the backup system. This system is
        used to backup data to disk until the data is confirmed received by Sift. If streaming
        encounters errors, the backed up data will be re-ingested ensuring all data is received
        by Sift.

        If the backup system has fallen behind and the backup queue/channel is full, it will still
        proceed to sending the message to Sift. This ensures data is sent to Sift even if the
        backup system is lagging.

        Args:
            flow: The flow to send to Sift.
        """
        if isinstance(flow, Flow):
            flow_py = flow._to_rust_form()
        else:
            flow_py = flow
        await self._low_level_client.send(flow_py)

    async def batch_send(self, flows: Iterable[Flow | FlowPy]):
        """Send multiple flows to Sift in a single batch operation.

        This method allows you to send multiple flows efficiently in a single batch,
        which can improve performance by reducing overhead compared to calling `send`
        multiple times.

        Args:
            flows: An iterable of flows to send. Each flow can be either a `Flow` or `FlowPy` instance.
        """

        def normalize_flows(flows: Iterable[Flow | FlowPy]) -> Iterator[FlowPy]:
            for flow in flows:
                if isinstance(flow, Flow):
                    yield flow._to_rust_form()
                else:
                    yield flow

        flows_py = normalize_flows(flows)
        await self._low_level_client.batch_send(flows_py)

    async def send_requests(self, requests: list[IngestWithConfigDataStreamRequestPy]):
        """Send data in a manner identical to the raw gRPC service for ingestion-config based streaming.

        This method offers a way to send data that matches the raw gRPC service interface. You are
        expected to handle channel value ordering as well as empty values correctly.

        Important:
            Most users should prefer to use `send`. This method primarily exists to make it easier
            for existing integrations to utilize sift-stream.

        Args:
            requests: List of ingestion requests to send to Sift.
        """
        await self._low_level_client.send_requests(requests)

    def send_requests_nonblocking(
        self, requests: Iterable[IngestWithConfigDataStreamRequestWrapperPy]
    ):
        """Send data in a manner identical to the raw gRPC service for ingestion-config based streaming.

        This method offers a way to send data that matches the raw gRPC service interface. You are
        expected to handle channel value ordering as well as empty values correctly.

        Important:
            If using this interface, you should use `FlowBuilderPy::request` to ensure proper
            building of the request.

        Args:
            requests: List of ingestion requests to send to Sift.
        """
        self._low_level_client.send_requests_nonblocking(requests)

    def get_flow_descriptor(self, flow_name: str) -> FlowDescriptorPy:
        """Retrieve a flow descriptor by name.

        Args:
            flow_name: The name of the flow descriptor to retrieve.
        """
        return self._low_level_client.get_flow_descriptor(flow_name)

    async def add_new_flows(self, flow_configs: list[FlowConfig]):
        """Modify the existing ingestion config by adding new flows that weren't accounted for during initialization.

        This allows you to dynamically add new flow configurations to the ingestion config after
        the stream has been initialized. The new flows will be registered with Sift and can then
        be used in subsequent `send` calls.

        Args:
            flow_configs: List of flow configurations to add to the ingestion config.
        """
        flow_configs_py = [flow_config._to_rust_config() for flow_config in flow_configs]
        await self._low_level_client.add_new_flows(flow_configs_py)

    async def attach_run(self, run: RunCreate | dict | str | Run | RunFormPy):
        """Attach a run to the stream.

        Any data provided through `send` after this function returns will be associated with
        the run. The run can be specified as a Run object, RunCreate object, dict, run ID string,
        or RunFormPy object.

        Args:
            run: The run to attach. Can be a Run, RunCreate, dict, run ID string, or RunFormPy.
        """
        # Importing here to allow sift_stream_bindings to be an optional dependancy for non-ingestion users
        from sift_stream_bindings import RunFormPy, RunSelectorPy

        if isinstance(run, RunFormPy):
            run_selector_py = RunSelectorPy.by_form(run)
        elif isinstance(run, dict):
            run_create = RunCreate.model_validate(run)
            run_form_py = run_create._to_rust_form()
            run_selector_py = RunSelectorPy.by_form(run_form_py)
        elif isinstance(run, Run):
            if run.id_ is None:
                raise ValueError("The Run object must contain a run_id")
            run_selector_py = RunSelectorPy.by_id(run.id_)
        elif isinstance(run, RunCreate):
            run_form_py = run._to_rust_form()
            run_selector_py = RunSelectorPy.by_form(run_form_py)
        elif isinstance(run, str):
            run_selector_py = RunSelectorPy.by_id(run)

        await self._low_level_client.attach_run(run_selector_py)

    def detach_run(self):
        """Detach the run, if any, associated with the stream.

        Any data provided through `send` after this function is called will not be associated
        with a run.
        """
        self._low_level_client.detach_run()

    def get_run_id(self) -> str | None:
        """Retrieve the ID of the attached run, if one exists.

        Returns:
            The run ID if a run is attached, None otherwise.
        """
        return self._low_level_client.get_run_id()

    async def finish(self):
        """Conclude the stream and return when Sift has sent its final response.

        It is important that this method be called in order to obtain the final checkpoint
        acknowledgement from Sift, otherwise some tail-end data may fail to send. This method
        will gracefully shut down the streaming system and ensure all data has been properly
        sent to Sift.
        """
        await self._low_level_client.finish()

    def get_metrics_snapshot(self) -> SiftStreamMetricsSnapshotPy:
        """Retrieve a snapshot of the current metrics for this stream.

        NOTE: The returned metrics snapshot is currently an unstable feature and may change at any time.

        Metrics are recorded related to the performance and operational status of the stream.
        Snapshots are taken at any time this method is called. Metrics are internally updated
        atomically, and calls to get metric snapshots are non-blocking to stream operation.

        Returns:
            A snapshot of the current stream metrics.
        """
        return self._low_level_client.get_metrics_snapshot()

    async def __aenter__(self):
        return self

    async def __aexit__(self, exc_type, exc_val, exc_tb):
        await self.finish()
