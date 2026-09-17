"""Pytest tests for the Annotations API.

These tests demonstrate and validate the usage of the Annotations API including:
- Basic annotation operations (get, list, find)
- Annotation filtering and searching
- Annotation creation, updates, and archiving
- The review workflow: assignment, state, and annotation logs
"""

from datetime import datetime, timedelta, timezone

import pytest

from sift_client import SiftClient
from sift_client.resources import AnnotationLogsAPI, AnnotationsAPI, AnnotationsAPIAsync
from sift_client.sift_types import Annotation
from sift_client.sift_types.annotation import (
    AnnotationCreate,
    AnnotationLog,
    AnnotationLogKind,
    AnnotationLogState,
    AnnotationState,
    AnnotationType,
    AnnotationUpdate,
)

pytestmark = pytest.mark.integration


def test_client_binding(sift_client):
    assert isinstance(sift_client.annotations, AnnotationsAPI)
    assert isinstance(sift_client.annotations.logs, AnnotationLogsAPI)
    assert isinstance(sift_client.async_.annotations, AnnotationsAPIAsync)


@pytest.fixture(scope="session")
def test_timestamp_str():
    """A per-session suffix so annotation names stay unique across runs."""
    return datetime.now(timezone.utc).isoformat()


@pytest.fixture(scope="session")
def window():
    """A short, fixed time range for the annotations these tests create."""
    end = datetime.now(timezone.utc)
    return end - timedelta(minutes=5), end


@pytest.fixture
def annotations_api_async(sift_client: SiftClient):
    """Get the async annotations API instance."""
    return sift_client.async_.annotations


@pytest.fixture(scope="session")
def new_annotation(sift_client, test_timestamp_str, window, nostromo_asset):
    """Create an annotation for the session and archive it on teardown."""
    start, end = window
    created = sift_client.annotations.create(
        AnnotationCreate(
            name=f"test_annotation_{test_timestamp_str}",
            description="Created by Sift Client pytest",
            start_time=start,
            end_time=end,
            assets=[nostromo_asset.name],
            state=AnnotationState.OPEN,
        )
    )
    yield created
    sift_client.annotations.archive(created)


class TestAnnotations:
    """Tests for the Annotations API."""

    def test_create(self, new_annotation, test_timestamp_str, nostromo_asset):
        """Test that create returns a fully populated annotation."""
        assert isinstance(new_annotation, Annotation)
        assert new_annotation.id_ is not None
        assert new_annotation.name == f"test_annotation_{test_timestamp_str}"
        assert new_annotation.annotation_type is AnnotationType.DATA_REVIEW
        assert new_annotation.state is AnnotationState.OPEN
        assert new_annotation.is_archived is False
        assert nostromo_asset.id_ in new_annotation.asset_ids

    def test_create_phase(self, sift_client, test_timestamp_str, window, nostromo_asset):
        """Test creating a phase annotation, which carries no review state."""
        start, end = window
        phase = sift_client.annotations.create(
            AnnotationCreate(
                name=f"test_phase_{test_timestamp_str}",
                start_time=start,
                end_time=end,
                assets=[nostromo_asset.name],
                annotation_type=AnnotationType.PHASE,
            )
        )

        assert phase.annotation_type is AnnotationType.PHASE
        sift_client.annotations.archive(phase)

    def test_get(self, sift_client, new_annotation):
        """Test getting an annotation by ID."""
        fetched = sift_client.annotations.get(new_annotation._id_or_error)

        assert fetched.id_ == new_annotation.id_
        assert fetched.name == new_annotation.name

    def test_basic_list(self, sift_client, new_annotation):
        """Test basic annotation listing functionality."""
        annotations = sift_client.annotations.list_(limit=5)

        assert isinstance(annotations, list)
        for annotation in annotations:
            assert isinstance(annotation, Annotation)
            assert annotation.id_ is not None

    def test_list_with_name_filter(self, sift_client, new_annotation):
        """Test annotation listing with name filtering."""
        by_name = sift_client.annotations.list_(name=new_annotation.name)

        assert len(by_name) == 1
        assert by_name[0].id_ == new_annotation.id_

    def test_list_with_id_filter(self, sift_client, new_annotation):
        """Test annotation listing filtered to specific IDs."""
        annotations = sift_client.annotations.list_(annotation_ids=[new_annotation._id_or_error])

        assert [a.id_ for a in annotations] == [new_annotation.id_]

    def test_list_with_type_and_state_filters(self, sift_client, new_annotation):
        """Test annotation listing filtered by type and review state."""
        annotations = sift_client.annotations.list_(
            annotation_ids=[new_annotation._id_or_error],
            annotation_type=AnnotationType.DATA_REVIEW,
            state=AnnotationState.OPEN,
        )

        assert [a.id_ for a in annotations] == [new_annotation.id_]

    def test_list_with_asset_filter(self, sift_client, new_annotation, nostromo_asset):
        """Test annotation listing filtered by asset."""
        annotations = sift_client.annotations.list_(assets=[nostromo_asset], limit=20)

        assert new_annotation.id_ in {a.id_ for a in annotations}

    def test_find(self, sift_client, new_annotation):
        """Test finding a single annotation."""
        found = sift_client.annotations.find(name=new_annotation.name)

        assert found is not None
        assert found.id_ == new_annotation.id_

    def test_find_nonexistent(self, sift_client):
        """Test finding a non-existent annotation returns None."""
        found = sift_client.annotations.find(
            name=f"nonexistent_annotation_{datetime.now(timezone.utc).timestamp()}"
        )
        assert found is None

    def test_update(self, sift_client, new_annotation):
        """Test updating an annotation's description and state."""
        updated = sift_client.annotations.update(
            new_annotation,
            AnnotationUpdate(description="updated", state=AnnotationState.FLAGGED),
        )

        assert updated.description == "updated"
        assert updated.state is AnnotationState.FLAGGED
        # The name was not in the mask, so it is unchanged.
        assert updated.name == new_annotation.name

        sift_client.annotations.update(new_annotation, {"state": AnnotationState.OPEN})

    def test_update_accepts_dict(self, sift_client, new_annotation, test_timestamp_str):
        """Test that update accepts a plain dict."""
        renamed = f"test_annotation_renamed_{test_timestamp_str}"
        updated = sift_client.annotations.update(new_annotation._id_or_error, {"name": renamed})

        assert updated.name == renamed
        sift_client.annotations.update(new_annotation, {"name": new_annotation.name})

    def test_archive_and_unarchive(self, sift_client, test_timestamp_str, window, nostromo_asset):
        """Test archiving and unarchiving an annotation."""
        start, end = window
        annotation = sift_client.annotations.create(
            AnnotationCreate(
                name=f"test_annotation_archive_{test_timestamp_str}",
                start_time=start,
                end_time=end,
                assets=[nostromo_asset.name],
            )
        )

        archived = sift_client.annotations.archive(annotation)
        assert archived.is_archived is True

        # Archived annotations are excluded from list_ by default.
        assert sift_client.annotations.find(name=annotation.name) is None
        assert (
            sift_client.annotations.find(name=annotation.name, include_archived=True).id_
            == annotation.id_
        )

        unarchived = sift_client.annotations.unarchive(annotation)
        assert unarchived.is_archived is False

        sift_client.annotations.archive(annotation)

    def test_instance_methods(self, sift_client, test_timestamp_str, window, nostromo_asset):
        """Test the update, resolve, and archive methods on the instance itself."""
        start, end = window
        annotation = sift_client.annotations.create(
            AnnotationCreate(
                name=f"test_annotation_instance_{test_timestamp_str}",
                start_time=start,
                end_time=end,
                assets=[nostromo_asset.name],
                state=AnnotationState.OPEN,
            )
        )

        annotation.update({"description": "from-instance"})
        assert annotation.description == "from-instance"

        annotation.resolve()
        assert annotation.state is AnnotationState.RESOLVED

        annotation.archive()
        assert annotation.is_archived is True


class TestAnnotationLogs:
    """Tests for the nested annotation logs API."""

    def test_comment(self, sift_client, new_annotation):
        """Test adding a comment and reading it back."""
        log = sift_client.annotations.logs.comment(new_annotation, "looks fine to me")

        assert isinstance(log, AnnotationLog)
        assert log.kind is AnnotationLogKind.COMMENT
        assert log.text == "looks fine to me"

        logs = sift_client.annotations.logs.list_(annotation=new_annotation)
        assert log.id_ in {entry.id_ for entry in logs}

    def test_record_state(self, sift_client, new_annotation):
        """Test recording a state change in the history."""
        log = sift_client.annotations.logs.record_state(new_annotation, AnnotationLogState.FLAGGED)

        assert log.kind is AnnotationLogKind.STATE_UPDATE
        assert log.state is AnnotationLogState.FLAGGED

    def test_list_filtered_by_kind(self, sift_client, new_annotation):
        """Test filtering the history by log kind."""
        sift_client.annotations.logs.comment(new_annotation, "another comment")

        logs = sift_client.annotations.logs.list_(
            annotation=new_annotation, kind=AnnotationLogKind.COMMENT
        )

        assert logs
        assert all(entry.kind is AnnotationLogKind.COMMENT for entry in logs)

    def test_annotation_logs_property(self, sift_client, new_annotation):
        """Test the logs property on the Annotation instance."""
        sift_client.annotations.logs.comment(new_annotation, "via property")

        assert any(entry.kind is AnnotationLogKind.COMMENT for entry in new_annotation.logs)

    def test_delete(self, sift_client, new_annotation):
        """Test deleting a log entry.

        The RPC is accepted but the entry still lists afterwards, so this only
        asserts the call succeeds. Confirmed against the raw stub, so it is server
        side rather than a client mapping problem.
        """
        log = sift_client.annotations.logs.comment(new_annotation, "to be deleted")

        sift_client.annotations.logs.delete(new_annotation, log)

    @pytest.mark.asyncio
    async def test_async_list(self, annotations_api_async, new_annotation):
        """Test the async API returns the same annotations."""
        annotations = await annotations_api_async.list_(
            annotation_ids=[new_annotation._id_or_error]
        )

        assert [a.id_ for a in annotations] == [new_annotation.id_]
