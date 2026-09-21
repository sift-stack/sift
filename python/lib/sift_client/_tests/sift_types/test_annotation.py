"""Tests for sift_types.Annotation model."""

from datetime import datetime, timezone
from unittest.mock import MagicMock

import pytest
from sift.channels.v3.channels_pb2 import Channel as ChannelProto
from sift.common.type.v1.channel_data_type_pb2 import ChannelDataType as ChannelDataTypeProto

from sift_client.sift_types import Annotation
from sift_client.sift_types.annotation import (
    AnnotationCommentElement,
    AnnotationCreate,
    AnnotationState,
    AnnotationType,
    AnnotationUpdate,
    PhaseCreate,
)
from sift_client.sift_types.channel import Channel

START = datetime(2026, 1, 1, tzinfo=timezone.utc)
END = datetime(2026, 1, 2, tzinfo=timezone.utc)


class TestAnnotationEnums:
    """Unit tests for the CEL filter representations."""

    def test_type_filter_str(self):
        assert AnnotationType.PHASE.to_filter_str() == "ANNOTATION_TYPE_PHASE"

    def test_state_filter_str(self):
        assert AnnotationState.ACCEPTED.to_filter_str() == "ANNOTATION_STATE_RESOLVED"


class TestAnnotationCreate:
    """Unit tests for AnnotationCreate - tests _to_proto_helpers and validators."""

    def test_minimal_create(self):
        proto = AnnotationCreate(name="a", start_time=START, end_time=END).to_proto()

        assert proto.name == "a"
        assert proto.annotation_type == AnnotationType.DATA_REVIEW.value

    def test_assets_and_tags_are_names(self):
        create = AnnotationCreate(
            name="a", start_time=START, end_time=END, assets=["Nostromo"], tags=["review"]
        )
        proto = create.to_proto()

        assert list(proto.assets) == ["Nostromo"]
        assert list(proto.tags) == ["review"]

    def test_metadata_converter(self):
        proto = AnnotationCreate(
            name="a", start_time=START, end_time=END, metadata={"k": "v", "n": 1.5}
        ).to_proto()

        by_key = {m.key.name: m for m in proto.metadata}
        assert by_key["k"].string_value == "v"
        assert by_key["n"].number_value == 1.5

    def test_linked_channels_takes_channel_objects(self):
        """linked_channels takes Channel objects, wrapping them for the proto."""
        channel = Channel._from_proto(
            ChannelProto(
                channel_id="ch-1",
                name="pressure",
                data_type=ChannelDataTypeProto.CHANNEL_DATA_TYPE_DOUBLE,
            )
        )

        create = AnnotationCreate(
            name="a", start_time=START, end_time=END, linked_channels=[channel]
        )
        linked = create.linked_channels_to_proto()

        assert len(linked) == 1
        assert linked[0].channel.channel_id == "ch-1"

    def test_phase_has_no_state(self):
        """PhaseCreate carries no review state, so this fails before the call."""
        with pytest.raises(ValueError, match="state"):
            PhaseCreate(name="a", start_time=START, end_time=END, state=AnnotationState.OPEN)

    def test_rejects_inverted_time_range(self):
        with pytest.raises(ValueError, match="start_time must not be after end_time"):
            AnnotationCreate(name="a", start_time=END, end_time=START)


class TestAnnotationUpdate:
    """Unit tests for AnnotationUpdate - tests field masks."""

    def test_update_mask_only_includes_set_fields(self):
        update = AnnotationUpdate(name="renamed", state=AnnotationState.FAILED)
        update.resource_id = "an-1"

        proto, mask = update.to_proto_with_mask()

        assert proto.annotation_id == "an-1"
        assert proto.name == "renamed"
        assert proto.state == AnnotationState.FAILED.value
        assert set(mask.paths) == {"name", "state"}

    def test_assignment(self):
        update = AnnotationUpdate(assigned_to_user_id="user-1")
        update.resource_id = "an-1"

        proto, mask = update.to_proto_with_mask()

        assert proto.assigned_to_user_id == "user-1"
        assert mask.paths == ["assigned_to_user_id"]

    def test_requires_resource_id(self):
        with pytest.raises(ValueError, match="Resource ID must be set"):
            AnnotationUpdate(name="renamed").to_proto_with_mask()


class TestAnnotationCommentElement:
    """Unit tests for comment body elements."""

    def test_text_element(self):
        proto = AnnotationCommentElement(text="looks fine")._to_proto()
        assert proto.text == "looks fine"
        assert AnnotationCommentElement._from_proto(proto).text == "looks fine"

    def test_mention_element(self):
        proto = AnnotationCommentElement(user_id="u-1", user_email="a@b.c")._to_proto()
        assert proto.user_mention.user_id == "u-1"
        assert AnnotationCommentElement._from_proto(proto).user_id == "u-1"

    def test_rejects_both(self):
        with pytest.raises(ValueError, match="exactly one"):
            AnnotationCommentElement(text="hi", user_id="u-1")


@pytest.fixture
def mock_annotation(mock_client):
    """Create a mock Annotation instance for testing."""
    annotation = Annotation(
        proto=MagicMock(),
        id_="test_annotation_id",
        name="test_annotation",
        description="test",
        start_time=START,
        end_time=END,
        annotation_type=AnnotationType.DATA_REVIEW,
        organization_id="org1",
        created_date=START,
        modified_date=START,
        created_by_user_id="user1",
        modified_by_user_id="user1",
        tags=[],
        asset_ids=["asset1"],
        linked_channel_ids=[],
        linked_calculated_channel_version_ids=[],
        metadata={},
        is_archived=False,
        pending=False,
        state=AnnotationState.OPEN,
        run_id=None,
        assigned_to_user_id=None,
        legend_config=None,
        archived_date=None,
    )
    annotation._apply_client_to_instance(mock_client)
    return annotation


class TestAnnotation:
    """Unit tests for Annotation model - tests properties and methods."""

    def test_logs_property_calls_client(self, mock_annotation, mock_client):
        mock_client.annotations.logs.list_.return_value = []

        _ = mock_annotation.logs

        mock_client.annotations.logs.list_.assert_called_once_with(annotation="test_annotation_id")

    def test_update_calls_client_and_updates_self(self, mock_annotation, mock_client):
        updated = MagicMock()
        mock_client.annotations.update.return_value = updated

        with MagicMock() as mock_update:
            mock_annotation._update = mock_update

            update = AnnotationUpdate(name="renamed")
            result = mock_annotation.update(update)

            mock_client.annotations.update.assert_called_once_with(
                annotation=mock_annotation, update=update
            )
            mock_update.assert_called_once_with(updated)
            assert result is mock_annotation

    def test_archive_calls_client(self, mock_annotation, mock_client):
        mock_client.annotations.archive.return_value = MagicMock()

        with MagicMock() as mock_update:
            mock_annotation._update = mock_update
            result = mock_annotation.archive()

            mock_client.annotations.archive.assert_called_once_with(annotation=mock_annotation)
            assert result is mock_annotation

    def test_unarchive_calls_client(self, mock_annotation, mock_client):
        mock_client.annotations.unarchive.return_value = MagicMock()

        with MagicMock() as mock_update:
            mock_annotation._update = mock_update
            result = mock_annotation.unarchive()

            mock_client.annotations.unarchive.assert_called_once_with(annotation=mock_annotation)
            assert result is mock_annotation

    def test_assign_forwards_to_the_resource(self, mock_annotation, mock_client):
        mock_client.annotations.assign_to_user.return_value = MagicMock()

        with MagicMock() as mock_update:
            mock_annotation._update = mock_update
            mock_annotation.assign_to_user("user-2")

            mock_client.annotations.assign_to_user.assert_called_once_with(
                annotation=mock_annotation, user="user-2"
            )

    def test_state_verbs_forward_to_the_resource(self, mock_annotation, mock_client):
        for verb in ("set_open", "set_failed", "set_accepted"):
            getattr(mock_client.annotations, verb).return_value = MagicMock()

            with MagicMock() as mock_update:
                mock_annotation._update = mock_update
                getattr(mock_annotation, verb)()

                getattr(mock_client.annotations, verb).assert_called_once_with(
                    annotation=mock_annotation
                )
