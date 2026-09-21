from __future__ import annotations

from typing import TYPE_CHECKING

from sift_client._internal.low_level_wrappers.webhooks import WebhooksLowLevelClient
from sift_client.resources._base import ResourceBase
from sift_client.sift_types.webhook import (
    Webhook,
    WebhookCreate,
    WebhookEventType,
    WebhookTestResult,
    WebhookUpdate,
)
from sift_client.util import cel_utils as cel

if TYPE_CHECKING:
    import re

    from sift_client.client import SiftClient


class WebhooksAPIAsync(ResourceBase):
    """High-level API for interacting with webhooks.

    A webhook registers an HTTP endpoint that Sift calls when a rule is violated.
    """

    def __init__(self, sift_client: SiftClient):
        """Initialize the WebhooksAPI.

        Args:
            sift_client: The Sift client to use.
        """
        super().__init__(sift_client)
        self._low_level_client = WebhooksLowLevelClient(grpc_client=self.client.grpc_client)

    async def get(self, webhook_id: str) -> Webhook:
        """Get a Webhook.

        Args:
            webhook_id: The ID of the webhook.

        Returns:
            The Webhook.
        """
        webhook = await self._low_level_client.get_webhook(webhook_id=webhook_id)
        return self._apply_client_to_instance(webhook)

    async def list_(
        self,
        *,
        name: str | None = None,
        names: list[str] | None = None,
        name_contains: str | None = None,
        name_regex: str | re.Pattern | None = None,
        # self ids
        webhook_ids: list[str] | None = None,
        # webhook specific
        event_type: WebhookEventType | None = None,
        # common filters
        include_archived: bool = False,
        filter_query: str | None = None,
        order_by: str | None = None,
        limit: int | None = None,
        page_size: int | None = None,
    ) -> list[Webhook]:
        """List webhooks with optional filtering.

        Args:
            name: Exact name of the webhook.
            names: List of webhook names to filter by.
            name_contains: Partial name of the webhook.
            name_regex: Regular expression to filter webhooks by name.
            webhook_ids: Filter to webhooks with any of these IDs.
            event_type: Filter to webhooks triggered by this event type.
            include_archived: If True, include archived webhooks in results.
            filter_query: Explicit CEL query to filter webhooks.
            order_by: Field and direction to order results by. Only `created_date` is
                supported, e.g. "created_date desc".
            limit: Maximum number of webhooks to return. If None, returns all matches.
            page_size: Number of results to fetch per request. Lower this if you hit gRPC
                message size limits on responses. If None, uses the server default.

        Returns:
            A list of Webhook objects that match the filter criteria.
        """
        filter_parts = [
            *self._build_name_cel_filters(
                name=name, names=names, name_contains=name_contains, name_regex=name_regex
            ),
            *self._build_common_cel_filters(
                include_archived=include_archived,
                filter_query=filter_query,
            ),
        ]
        if webhook_ids:
            filter_parts.append(cel.in_("webhook_id", webhook_ids))
        if event_type:
            filter_parts.append(cel.equals("event_type", event_type.to_filter_str()))
        query_filter = cel.and_(*filter_parts)

        webhooks = await self._low_level_client.list_all_webhooks(
            query_filter=query_filter or None,
            order_by=order_by,
            max_results=limit,
            **({"page_size": page_size} if page_size is not None else {}),
        )
        return self._apply_client_to_instances(webhooks)

    async def find(self, **kwargs) -> Webhook | None:
        """Find a single webhook matching the given query. Takes the same arguments as `list_`.
        If more than one webhook is found, raises an error.

        Args:
            **kwargs: Keyword arguments to pass to `list_`.

        Returns:
            The Webhook found or None.
        """
        webhooks = await self.list_(**kwargs)
        if len(webhooks) > 1:
            raise ValueError(f"Multiple ({len(webhooks)}) webhooks found for query")
        elif len(webhooks) == 1:
            return webhooks[0]
        return None

    async def create(self, create: WebhookCreate | dict) -> Webhook:
        """Create a new webhook.

        Args:
            create: The webhook definition. `http_headers` accepts either a list of
                WebhookHttpHeader or a `{name: value}` mapping.

        Returns:
            The created Webhook.
        """
        if isinstance(create, dict):
            create = WebhookCreate.model_validate(create)
        created_webhook = await self._low_level_client.create_webhook(create=create)
        return self._apply_client_to_instance(created_webhook)

    async def update(self, webhook: str | Webhook, update: WebhookUpdate | dict) -> Webhook:
        """Update a Webhook.

        Note that `http_headers` is replaced wholesale, not merged.

        Args:
            webhook: The Webhook or webhook ID to update.
            update: Updates to apply to the Webhook.

        Returns:
            The updated Webhook.
        """
        webhook_id = webhook._id_or_error if isinstance(webhook, Webhook) else webhook
        if isinstance(update, dict):
            update = WebhookUpdate.model_validate(update)
        update.resource_id = webhook_id
        updated_webhook = await self._low_level_client.update_webhook(update)
        return self._apply_client_to_instance(updated_webhook)

    async def archive(self, webhook: str | Webhook) -> Webhook:
        """Archive a webhook. Archived webhooks stop receiving events.

        Args:
            webhook: The Webhook or webhook ID to archive.

        Returns:
            The archived Webhook.
        """
        return await self.update(webhook, WebhookUpdate(is_archived=True))

    async def unarchive(self, webhook: str | Webhook) -> Webhook:
        """Unarchive a webhook.

        Args:
            webhook: The Webhook or webhook ID to unarchive.

        Returns:
            The unarchived Webhook.
        """
        return await self.update(webhook, WebhookUpdate(is_archived=False))

    async def test(
        self,
        webhook: str | Webhook | None = None,
        *,
        create: WebhookCreate | dict | None = None,
    ) -> WebhookTestResult:
        """Send a real request to a webhook's target URL and return its response.

        Pass exactly one of `webhook` or `create`. Use `create` to check an endpoint
        before saving it.

        Args:
            webhook: The Webhook or webhook ID to test.
            create: An unsaved webhook definition to test.

        Returns:
            The response the target URL returned.

        Raises:
            ValueError: If neither or both arguments are provided.
        """
        if (webhook is None) == (create is None):
            raise ValueError("Exactly one of webhook or create must be provided")

        if webhook is not None:
            webhook_id = webhook._id_or_error if isinstance(webhook, Webhook) else webhook
            return await self._low_level_client.test_webhook(webhook_id=webhook_id)

        if isinstance(create, dict):
            create = WebhookCreate.model_validate(create)
        return await self._low_level_client.test_webhook(create=create)
