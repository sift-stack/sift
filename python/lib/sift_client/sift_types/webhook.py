from __future__ import annotations

from datetime import datetime, timezone
from enum import Enum
from typing import TYPE_CHECKING, ClassVar

from pydantic import BaseModel, field_validator, model_validator
from sift.webhooks.v1.webhooks_pb2 import CreateWebhookRequest as CreateWebhookRequestProto
from sift.webhooks.v1.webhooks_pb2 import Webhook as WebhookProto
from sift.webhooks.v1.webhooks_pb2 import WebhookEventType as WebhookEventTypeProto
from sift.webhooks.v1.webhooks_pb2 import WebhookHttpHeader as WebhookHttpHeaderProto

from sift_client.sift_types._base import (
    BaseType,
    MappingHelper,
    ModelCreate,
    ModelCreateUpdateBase,
    ModelUpdate,
)

if TYPE_CHECKING:
    from sift_client.client import SiftClient
    from sift_client.sift_types.user import User


class WebhookEventType(Enum):
    """Enum for the events that trigger a webhook."""

    RULE_VIOLATION = WebhookEventTypeProto.WEBHOOK_EVENT_TYPE_RULE_VIOLATION  # 1

    def to_filter_str(self) -> str:
        """Convert to the string used in CEL filters."""
        return f"WEBHOOK_EVENT_TYPE_{self.name}"


class WebhookHttpHeader(BaseModel):
    """An HTTP header sent with every request to the webhook's target URL.

    Attributes:
        name: The header name.
        value: The header value.
    """

    name: str
    value: str

    @classmethod
    def _from_proto(cls, proto: WebhookHttpHeaderProto) -> WebhookHttpHeader:
        return cls(name=proto.name, value=proto.value)


class WebhookTestResult(BaseModel):
    """The response the target URL returned for a test request.

    Attributes:
        http_response_code: The HTTP status code the target returned.
        http_response_body: The raw response body the target returned.
    """

    http_response_code: int
    http_response_body: bytes

    @property
    def text(self) -> str:
        """The response body decoded as text, with undecodable bytes replaced."""
        return self.http_response_body.decode(errors="replace")


class Webhook(BaseType[WebhookProto, "Webhook"]):
    """Webhook model representing an HTTP callback registered with Sift."""

    # Required fields
    name: str
    organization_id: str
    target_url: str
    event_type: WebhookEventType | None
    http_headers: list[WebhookHttpHeader]
    created_date: datetime
    modified_date: datetime
    created_by_user_id: str
    modified_by_user_id: str
    is_archived: bool

    # Optional fields
    payload: str | None
    archived_date: datetime | None

    @classmethod
    def _from_proto(cls, proto: WebhookProto, sift_client: SiftClient | None = None) -> Webhook:
        return cls(
            proto=proto,
            id_=proto.webhook_id,
            name=proto.name,
            organization_id=proto.organization_id,
            target_url=proto.target_url,
            event_type=(WebhookEventType(proto.event_type) if proto.event_type else None),
            http_headers=[WebhookHttpHeader._from_proto(header) for header in proto.http_headers],
            created_date=proto.created_date.ToDatetime(tzinfo=timezone.utc),
            modified_date=proto.modified_date.ToDatetime(tzinfo=timezone.utc),
            created_by_user_id=proto.created_by_user_id,
            modified_by_user_id=proto.modified_by_user_id,
            is_archived=proto.is_archived,
            payload=proto.payload if proto.HasField("payload") else None,
            archived_date=(
                proto.archived_date.ToDatetime(tzinfo=timezone.utc)
                if proto.HasField("archived_date")
                else None
            ),
            _client=sift_client,
        )

    @property
    def created_by(self) -> User:
        """Fetch the User that created this webhook."""
        return self.client.users.get(user_id=self.created_by_user_id)

    @property
    def modified_by(self) -> User:
        """Fetch the User that last modified this webhook."""
        return self.client.users.get(user_id=self.modified_by_user_id)

    def update(self, update: WebhookUpdate | dict) -> Webhook:
        """Update the Webhook.

        Args:
            update: The update to apply to the webhook. See WebhookUpdate for updatable fields.

        Returns:
            The updated webhook.
        """
        updated_webhook = self.client.webhooks.update(webhook=self, update=update)
        self._update(updated_webhook)
        return self

    def archive(self) -> Webhook:
        """Archive the webhook. Archived webhooks stop receiving events."""
        updated_webhook = self.client.webhooks.archive(webhook=self)
        self._update(updated_webhook)
        return self

    def unarchive(self) -> Webhook:
        """Unarchive the webhook."""
        updated_webhook = self.client.webhooks.unarchive(webhook=self)
        self._update(updated_webhook)
        return self

    def send_test_request(self) -> WebhookTestResult:
        """Send a real request to this webhook's target URL and return its response."""
        return self.client.webhooks.send_test_request(webhook=self)


def _check_target_url(value: str | None) -> str | None:
    """Reject target URLs that are not absolute HTTP(S) URLs."""
    if value is not None and not value.startswith(("http://", "https://")):
        raise ValueError("target_url must start with http:// or https://")
    return value


class WebhookBase(ModelCreateUpdateBase):
    """Base class for Webhook create and update models with shared fields and validation."""

    event_type: WebhookEventType | None = None
    payload: str | None = None
    http_headers: list[WebhookHttpHeader] | dict[str, str] | None = None

    _to_proto_helpers: ClassVar[dict[str, MappingHelper]] = {
        "http_headers": MappingHelper(
            proto_attr_path="http_headers",
            update_field="http_headers",
            converter=WebhookHttpHeaderProto,
        ),
    }

    @field_validator("http_headers", mode="before")
    @classmethod
    def _normalize_http_headers(cls, value):
        """Accept a ``{name: value}`` mapping in place of a list of headers."""
        if isinstance(value, dict):
            return [WebhookHttpHeader(name=name, value=header) for name, header in value.items()]
        return value


class WebhookCreate(WebhookBase, ModelCreate[CreateWebhookRequestProto]):
    """Create model for Webhook."""

    name: str
    target_url: str

    _validate_target_url = field_validator("target_url")(_check_target_url)

    def _get_proto_class(self) -> type[CreateWebhookRequestProto]:
        return CreateWebhookRequestProto

    @model_validator(mode="after")
    def _default_event_type(self):
        """Assign rather than default, since `to_proto` drops unset fields."""
        if self.event_type is None:
            self.event_type = WebhookEventType.RULE_VIOLATION
        return self


class WebhookUpdate(WebhookBase, ModelUpdate[WebhookProto]):
    """Update model for Webhook."""

    name: str | None = None
    target_url: str | None = None
    is_archived: bool | None = None

    _validate_target_url = field_validator("target_url")(_check_target_url)

    def _get_proto_class(self) -> type[WebhookProto]:
        return WebhookProto

    def _add_resource_id_to_proto(self, proto_msg: WebhookProto):
        if self._resource_id is None:
            raise ValueError("Resource ID must be set before adding to proto")
        proto_msg.webhook_id = self._resource_id
