"""Tests for sift_types.Webhook model."""

from datetime import datetime, timezone
from unittest.mock import MagicMock

import pytest

from sift_client.sift_types import Webhook
from sift_client.sift_types.webhook import (
    WebhookCreate,
    WebhookEventType,
    WebhookHttpHeader,
    WebhookUpdate,
)


class TestWebhookEventType:
    """Unit tests for the WebhookEventType enum."""

    def test_to_filter_str(self):
        """Test the CEL filter representation matches the proto enum value name."""
        assert WebhookEventType.RULE_VIOLATION.to_filter_str() == (
            "WEBHOOK_EVENT_TYPE_RULE_VIOLATION"
        )


class TestWebhookCreate:
    """Unit tests for WebhookCreate model - tests _to_proto_helpers and validators."""

    def test_minimal_create(self):
        """Test a create with only required fields sets a concrete event type."""
        create = WebhookCreate(name="test_webhook", target_url="https://example.com/hook")
        proto = create.to_proto()

        assert proto.name == "test_webhook"
        assert proto.target_url == "https://example.com/hook"
        # Left unset, the proto would default to UNSPECIFIED, which the server rejects.
        assert proto.event_type == WebhookEventType.RULE_VIOLATION.value

    def test_http_headers_list_converter(self):
        """Test that a list of headers is converted using _to_proto_helpers."""
        create = WebhookCreate(
            name="test_webhook",
            target_url="https://example.com/hook",
            http_headers=[WebhookHttpHeader(name="X-Token", value="secret")],
        )
        proto = create.to_proto()

        assert len(proto.http_headers) == 1
        assert proto.http_headers[0].name == "X-Token"
        assert proto.http_headers[0].value == "secret"

    def test_http_headers_dict_is_normalized(self):
        """Test that a name-to-value mapping is accepted in place of a list."""
        create = WebhookCreate(
            name="test_webhook",
            target_url="https://example.com/hook",
            http_headers={"X-Token": "secret", "X-Env": "staging"},
        )

        assert create.http_headers == [
            WebhookHttpHeader(name="X-Token", value="secret"),
            WebhookHttpHeader(name="X-Env", value="staging"),
        ]

        proto = create.to_proto()
        assert {header.name: header.value for header in proto.http_headers} == {
            "X-Token": "secret",
            "X-Env": "staging",
        }

    def test_explicit_event_type_is_preserved(self):
        """Test that an explicitly set event type is not overwritten by the default."""
        create = WebhookCreate(
            name="test_webhook",
            target_url="https://example.com/hook",
            event_type=WebhookEventType.RULE_VIOLATION,
        )
        assert create.event_type == WebhookEventType.RULE_VIOLATION

    def test_payload_is_optional(self):
        """Test that payload is omitted from the proto when not provided."""
        create = WebhookCreate(name="test_webhook", target_url="https://example.com/hook")
        proto = create.to_proto()

        assert not proto.HasField("payload")

    @pytest.mark.parametrize("target_url", ["example.com/hook", "ftp://example.com", ""])
    def test_rejects_non_http_target_url(self, target_url):
        """Test the target URL validator rejects URLs that are not absolute HTTP(S)."""
        with pytest.raises(ValueError, match="target_url must start with"):
            WebhookCreate(name="test_webhook", target_url=target_url)


class TestWebhookUpdate:
    """Unit tests for WebhookUpdate model - tests field masks and validators."""

    def test_update_mask_only_includes_set_fields(self):
        """Test that the field mask covers exactly the fields that were set."""
        update = WebhookUpdate(name="renamed", target_url="https://example.com/v2")
        update.resource_id = "test_webhook_id"

        proto, mask = update.to_proto_with_mask()

        assert proto.webhook_id == "test_webhook_id"
        assert proto.name == "renamed"
        assert proto.target_url == "https://example.com/v2"
        assert set(mask.paths) == {"name", "target_url"}

    def test_http_headers_converter(self):
        """Test that headers are converted and named in the field mask."""
        update = WebhookUpdate(http_headers={"X-Token": "rotated"})
        update.resource_id = "test_webhook_id"

        proto, mask = update.to_proto_with_mask()

        assert len(proto.http_headers) == 1
        assert proto.http_headers[0].name == "X-Token"
        assert proto.http_headers[0].value == "rotated"
        assert "http_headers" in mask.paths

    def test_archive_update(self):
        """Test that is_archived is masked so archive() takes effect."""
        update = WebhookUpdate(is_archived=True)
        update.resource_id = "test_webhook_id"

        proto, mask = update.to_proto_with_mask()

        assert proto.is_archived is True
        assert mask.paths == ["is_archived"]

    def test_requires_resource_id(self):
        """Test that converting without a resource ID raises."""
        update = WebhookUpdate(name="renamed")

        with pytest.raises(ValueError, match="Resource ID must be set"):
            update.to_proto_with_mask()

    def test_rejects_non_http_target_url(self):
        """Test the target URL validator also applies to updates."""
        with pytest.raises(ValueError, match="target_url must start with"):
            WebhookUpdate(target_url="example.com/hook")


@pytest.fixture
def mock_webhook(mock_client):
    """Create a mock Webhook instance for testing."""
    webhook = Webhook(
        proto=MagicMock(),
        id_="test_webhook_id",
        name="test_webhook",
        organization_id="org1",
        target_url="https://example.com/hook",
        event_type=WebhookEventType.RULE_VIOLATION,
        http_headers=[WebhookHttpHeader(name="X-Token", value="secret")],
        created_date=datetime.now(timezone.utc),
        modified_date=datetime.now(timezone.utc),
        created_by_user_id="user1",
        modified_by_user_id="user1",
        is_archived=False,
        payload=None,
        archived_date=None,
    )
    webhook._apply_client_to_instance(mock_client)
    return webhook


class TestWebhook:
    """Unit tests for Webhook model - tests methods."""

    def test_update_calls_client_and_updates_self(self, mock_webhook, mock_client):
        """Test that update() calls client.webhooks.update and calls _update."""
        updated_webhook = MagicMock()
        mock_client.webhooks.update.return_value = updated_webhook

        with MagicMock() as mock_update:
            mock_webhook._update = mock_update

            update = WebhookUpdate(name="renamed")
            result = mock_webhook.update(update)

            mock_client.webhooks.update.assert_called_once_with(webhook=mock_webhook, update=update)
            mock_update.assert_called_once_with(updated_webhook)
            assert result is mock_webhook

    def test_archive_calls_client_and_updates_self(self, mock_webhook, mock_client):
        """Test that archive() calls client.webhooks.archive and calls _update."""
        archived_webhook = MagicMock()
        mock_client.webhooks.archive.return_value = archived_webhook

        with MagicMock() as mock_update:
            mock_webhook._update = mock_update

            result = mock_webhook.archive()

            mock_client.webhooks.archive.assert_called_once_with(webhook=mock_webhook)
            mock_update.assert_called_once_with(archived_webhook)
            assert result is mock_webhook

    def test_unarchive_calls_client_and_updates_self(self, mock_webhook, mock_client):
        """Test that unarchive() calls client.webhooks.unarchive and calls _update."""
        unarchived_webhook = MagicMock()
        mock_client.webhooks.unarchive.return_value = unarchived_webhook

        with MagicMock() as mock_update:
            mock_webhook._update = mock_update

            result = mock_webhook.unarchive()

            mock_client.webhooks.unarchive.assert_called_once_with(webhook=mock_webhook)
            mock_update.assert_called_once_with(unarchived_webhook)
            assert result is mock_webhook

    def test_test_calls_client(self, mock_webhook, mock_client):
        """Test that test() delegates to client.webhooks.send_test_request."""
        expected = MagicMock()
        mock_client.webhooks.send_test_request.return_value = expected

        result = mock_webhook.send_test_request()

        mock_client.webhooks.send_test_request.assert_called_once_with(webhook=mock_webhook)
        assert result is expected
