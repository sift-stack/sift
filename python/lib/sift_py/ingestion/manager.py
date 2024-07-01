from __future__ import annotations

from contextlib import contextmanager
from typing import Callable, Dict, Iterator, Optional, TypedDict

from typing_extensions import Self, TypeAlias

from sift_py.grpc.transport import SiftChannel
from sift_py.ingestion.config.telemetry import TelemetryConfig
from sift_py.ingestion.service import IngestionService

IngestionServiceBuilder: TypeAlias = Callable[[SiftChannel], IngestionService]


class IngestionServicesManager:
    """
    Allows for the initialization of multiple instances of `sift_py.ingestion.service.IngestionService` from
    either telemetry configs or builders under a single wrapper class that assists in managing data-ingestion
    for multiple telemetry configs.

    The initializer of this class can be used directly, but prefer to use either `from_builders` or `from_telemetry_configs`.
    Prefer to use `from_builders` if you have custom options that you want to provide to `sift_py.ingestion.service.IngestionService.__init__`.

    Example usage:

    ```python
    manager = IngestionServicesManager.from_telementry_configs(grpc_channel, {
        "config_a": config_a,
        "config_b": config_b,
    })

    with manager.ingestion_service("config_a") as config_a:
        config_a.try_ingest_flow(...)

    with manager.ingestion_service("config_b") as config_b:
        config_b.try_ingest_flow(...)
    ```
    """

    _transport_channel: SiftChannel
    _ingestion_services: Dict[str, IngestionService]

    def __init__(
        self, transport_channel: SiftChannel, ingestion_services: Dict[str, IngestionService]
    ):
        self._transport_channel = transport_channel
        self._ingestion_services = ingestion_services

    @classmethod
    def from_builders(
        cls, channel: SiftChannel, builders: Dict[str, IngestionServiceBuilder]
    ) -> Self:
        return cls(
            transport_channel=channel,
            ingestion_services={key: builder(channel) for key, builder in builders.items()},
        )

    @classmethod
    def from_telemetry_configs(
        cls, channel: SiftChannel, telemetry_configs: Dict[str, TelemetryConfig]
    ) -> Self:
        return cls(
            transport_channel=channel,
            ingestion_services={
                key: IngestionService(channel, config) for key, config in telemetry_configs.items()
            },
        )

    def get_ingestion_service_by_identifier(self, identifier: str) -> Optional[IngestionService]:
        return self._ingestion_services.get(identifier)

    def __getitem__(self, identifier: str) -> Optional[IngestionService]:
        return self.get_ingestion_service_by_identifier(identifier)

    @contextmanager
    def ingestion_service(self, identifier: str) -> Iterator[IngestionService]:
        ingestion_service = self[identifier]

        if ingestion_service is None:
            raise IngestionServiceManagerError(
                f"An ingestion service is not configured for the identifier '{identifier}'."
            )

        yield ingestion_service


class IngestionServiceManagerError(Exception):
    def __init__(self, msg: str):
        return super().__init__(msg)


class TelemetryConfigByIdentifierMap(TypedDict):
    identifier: str
    telemetry_config: TelemetryConfig


class IngestionConfigServiceBuilderIdentifierMap(TypedDict):
    identifier: str
    builder: IngestionServiceBuilder
