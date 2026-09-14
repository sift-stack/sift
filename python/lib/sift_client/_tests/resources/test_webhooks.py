"""Pytest tests for the Webhooks API.

These tests demonstrate and validate the usage of the Webhooks API including:
- Basic webhook operations (get, list, find)
- Webhook filtering and searching
- Webhook creation, updates, and archiving
- Error handling and edge cases

Note that these tests never call `test()`, because that RPC sends a real HTTP
request to the webhook's target URL.
"""

from datetime import datetime, timezone

import pytest

from sift_client import SiftClient
from sift_client.resources import WebhooksAPI, WebhooksAPIAsync
from sift_client.sift_types import Webhook
from sift_client.sift_types.webhook import (
    WebhookCreate,
    WebhookEventType,
    WebhookHttpHeader,
    WebhookUpdate,
)

pytestmark = pytest.mark.integration

TEST_TARGET_URL = "https://example.com/sift-client-pytest"


def test_client_binding(sift_client):
    assert sift_client.webhooks
    assert isinstance(sift_client.webhooks, WebhooksAPI)
    assert sift_client.async_.webhooks
    assert isinstance(sift_client.async_.webhooks, WebhooksAPIAsync)


@pytest.fixture
def webhooks_api_async(sift_client: SiftClient):
    """Get the async webhooks API instance."""
    return sift_client.async_.webhooks


@pytest.fixture
def webhooks_api_sync(sift_client: SiftClient):
    """Get the synchronous webhooks API instance."""
    return sift_client.webhooks


@pytest.fixture(scope="session")
def test_timestamp_str():
    """A per-session suffix so webhook names stay unique across runs."""
    return datetime.now(timezone.utc).isoformat()


@pytest.fixture(scope="session")
def new_webhook(sift_client, test_timestamp_str):
    """Create a webhook for the session and archive it on teardown."""
    created = sift_client.webhooks.create(
        WebhookCreate(
            name=f"test_webhook_{test_timestamp_str}",
            target_url=TEST_TARGET_URL,
            http_headers={"X-Sift-Client-Pytest": "true"},
        )
    )
    yield created
    sift_client.webhooks.archive(created)


class TestWebhooks:
    """Tests for the Webhooks API."""

    def test_create(self, new_webhook, test_timestamp_str):
        """Test that create returns a fully populated webhook."""
        assert isinstance(new_webhook, Webhook)
        assert new_webhook.id_ is not None
        assert new_webhook.name == f"test_webhook_{test_timestamp_str}"
        assert new_webhook.target_url == TEST_TARGET_URL
        assert new_webhook.event_type == WebhookEventType.RULE_VIOLATION
        assert new_webhook.http_headers == [
            WebhookHttpHeader(name="X-Sift-Client-Pytest", value="true")
        ]
        assert new_webhook.is_archived is False
        assert new_webhook.organization_id

    def test_get(self, webhooks_api_sync, new_webhook):
        """Test getting a webhook by ID."""
        fetched = webhooks_api_sync.get(new_webhook._id_or_error)

        assert isinstance(fetched, Webhook)
        assert fetched.id_ == new_webhook.id_
        assert fetched.name == new_webhook.name

    def test_basic_list(self, webhooks_api_sync, new_webhook):
        """Test basic webhook listing functionality."""
        webhooks = webhooks_api_sync.list_(limit=5)

        assert isinstance(webhooks, list)
        assert len(webhooks) >= 1
        for webhook in webhooks:
            assert isinstance(webhook, Webhook)
            assert webhook.id_ is not None

    def test_list_with_name_filter(self, webhooks_api_sync, new_webhook):
        """Test webhook listing with name filtering."""
        by_name = webhooks_api_sync.list_(name=new_webhook.name)
        by_contains = webhooks_api_sync.list_(name_contains=new_webhook.name)

        assert len(by_name) == 1
        assert by_name[0].id_ == new_webhook.id_
        assert by_contains[0].id_ == new_webhook.id_

    def test_list_with_id_filter(self, webhooks_api_sync, new_webhook):
        """Test webhook listing filtered to specific IDs."""
        webhooks = webhooks_api_sync.list_(webhook_ids=[new_webhook._id_or_error])

        assert len(webhooks) == 1
        assert webhooks[0].id_ == new_webhook.id_

    def test_list_with_event_type_filter(self, webhooks_api_sync, new_webhook):
        """Test webhook listing filtered by event type."""
        webhooks = webhooks_api_sync.list_(
            webhook_ids=[new_webhook._id_or_error],
            event_type=WebhookEventType.RULE_VIOLATION,
        )

        assert len(webhooks) == 1
        assert webhooks[0].id_ == new_webhook.id_

    def test_find(self, webhooks_api_sync, new_webhook):
        """Test finding a single webhook."""
        found = webhooks_api_sync.find(name=new_webhook.name)

        assert found is not None
        assert found.id_ == new_webhook.id_

    def test_find_nonexistent(self, webhooks_api_sync):
        """Test finding a non-existent webhook returns None."""
        found = webhooks_api_sync.find(
            name=f"nonexistent_webhook_{datetime.now(timezone.utc).timestamp()}"
        )
        assert found is None

    def test_update(self, webhooks_api_sync, new_webhook):
        """Test updating a webhook's target URL and headers."""
        updated_url = "https://example.com/sift-client-pytest-updated"
        updated = webhooks_api_sync.update(
            new_webhook,
            WebhookUpdate(
                target_url=updated_url,
                http_headers={"X-Sift-Client-Pytest": "updated"},
            ),
        )

        assert updated.id_ == new_webhook.id_
        assert updated.target_url == updated_url
        assert updated.http_headers == [
            WebhookHttpHeader(name="X-Sift-Client-Pytest", value="updated")
        ]
        # The name was not in the mask, so it is unchanged.
        assert updated.name == new_webhook.name

        # Restore the original target so later tests see a stable fixture.
        webhooks_api_sync.update(new_webhook, {"target_url": TEST_TARGET_URL})

    def test_update_accepts_dict(self, webhooks_api_sync, new_webhook, test_timestamp_str):
        """Test that update accepts a plain dict."""
        renamed = f"test_webhook_renamed_{test_timestamp_str}"
        updated = webhooks_api_sync.update(new_webhook._id_or_error, {"name": renamed})

        assert updated.name == renamed

        webhooks_api_sync.update(new_webhook, {"name": new_webhook.name})

    def test_archive_and_unarchive(self, webhooks_api_sync, test_timestamp_str):
        """Test archiving and unarchiving a webhook."""
        webhook = webhooks_api_sync.create(
            WebhookCreate(
                name=f"test_webhook_archive_{test_timestamp_str}",
                target_url=TEST_TARGET_URL,
            )
        )

        archived = webhooks_api_sync.archive(webhook)
        assert archived.is_archived is True

        # Archived webhooks are excluded from list_ by default.
        assert webhooks_api_sync.find(name=webhook.name) is None
        assert webhooks_api_sync.find(name=webhook.name, include_archived=True).id_ == webhook.id_

        unarchived = webhooks_api_sync.unarchive(webhook)
        assert unarchived.is_archived is False

        webhooks_api_sync.archive(webhook)

    def test_instance_methods(self, webhooks_api_sync, test_timestamp_str):
        """Test the update and archive methods on the Webhook instance itself."""
        webhook = webhooks_api_sync.create(
            WebhookCreate(
                name=f"test_webhook_instance_{test_timestamp_str}",
                target_url=TEST_TARGET_URL,
            )
        )

        webhook.update({"payload": "custom-payload"})
        assert webhook.payload == "custom-payload"

        webhook.archive()
        assert webhook.is_archived is True

    def test_test_requires_exactly_one_argument(self, webhooks_api_sync, new_webhook):
        """Test that test() rejects ambiguous arguments before making any request."""
        with pytest.raises(ValueError, match="Exactly one of webhook or create"):
            webhooks_api_sync.test()

        with pytest.raises(ValueError, match="Exactly one of webhook or create"):
            webhooks_api_sync.test(
                new_webhook,
                create=WebhookCreate(name="ignored", target_url=TEST_TARGET_URL),
            )

    @pytest.mark.asyncio
    async def test_async_list(self, webhooks_api_async, new_webhook):
        """Test the async API returns the same webhooks."""
        webhooks = await webhooks_api_async.list_(webhook_ids=[new_webhook._id_or_error])

        assert len(webhooks) == 1
        assert webhooks[0].id_ == new_webhook.id_
