from __future__ import annotations

from abc import ABC
from typing import TYPE_CHECKING, TypeVar

from sift_client.errors import _sift_client_experimental_warning
from sift_client.transport.base_connection import GrpcClient, RestClient

_sift_client_experimental_warning()

if TYPE_CHECKING:
    from sift_client.client import SiftClient
    from sift_client.types._base import BaseType

T = TypeVar("T", bound="BaseType")


class ResourceBase(ABC):
    _sift_client: "SiftClient"

    def __init__(self, sift_client: "SiftClient"):
        self._sift_client = sift_client

    @property
    def client(self) -> "SiftClient":
        return self._sift_client

    @property
    def grpc_client(self) -> GrpcClient:
        return self.client.grpc_client

    @property
    def rest_client(self) -> RestClient:
        return self.client.rest_client

    def _apply_client_to_instance(self, instance: T) -> T:
        instance._apply_client_to_instance(self.client)
        return instance

    def _apply_client_to_instances(self, instances: list[T]) -> list[T]:
        return [self._apply_client_to_instance(i) for i in instances]
