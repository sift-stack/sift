"""Pytest tests for the Rules API.

These tests demonstrate and validate the usage of the Rules API including:
- Basic rule operations (get, list, find)
- Rule filtering and searching
- Rule creation, updates, and archiving
- Error handling and edge cases
"""

import uuid
from datetime import datetime, timezone

import pytest

from sift_client import SiftClient
from sift_client.resources import RulesAPI, RulesAPIAsync
from sift_client.sift_types import Rule
from sift_client.sift_types.channel import ChannelReference
from sift_client.sift_types.rule import (
    RuleAction,
    RuleActionType,
    RuleAnnotationType,
    RuleCreate,
    RuleUpdate,
)

pytestmark = pytest.mark.integration


def test_client_binding(sift_client):
    assert sift_client.rules
    assert isinstance(sift_client.rules, RulesAPI)
    assert sift_client.async_.rules
    assert isinstance(sift_client.async_.rules, RulesAPIAsync)


@pytest.fixture
def rules_api_async(sift_client: SiftClient):
    """Get the async rules API instance."""
    return sift_client.async_.rules


@pytest.fixture
def rules_api_sync(sift_client: SiftClient):
    """Get the synchronous rules API instance."""
    return sift_client.rules


@pytest.fixture
def test_rule(rules_api_sync):
    rules = rules_api_sync.list_(limit=1)
    assert rules
    assert len(rules) >= 1
    return rules[0]


@pytest.fixture
def new_rule(rules_api_sync, sift_client):
    """Create a test rule for update tests."""
    from datetime import datetime, timezone

    rule_name = f"test_rule_{datetime.now(timezone.utc).isoformat()}"
    description = "Test rule created by Sift Client pytest"

    # Get some channels to reference
    channels = sift_client.channels.list_(limit=2)
    assert len(channels) >= 2

    # Get an asset to apply the rule to
    assets = sift_client.assets.list_(limit=1)
    assert len(assets) >= 1

    created_rule = rules_api_sync.create(
        RuleCreate(
            name=rule_name,
            client_key=f"test_rule_{str(uuid.uuid4())[-8:]}",
            description=description,
            expression="$1 > $2",
            channel_references=[
                ChannelReference(channel_reference="$1", channel_identifier=channels[0].name),
                ChannelReference(channel_reference="$2", channel_identifier=channels[1].name),
            ],
            action=RuleAction.annotation(
                annotation_type=RuleAnnotationType.DATA_REVIEW,
                tags=[],
            ),
            asset_ids=[assets[0].id_],
        )
    )
    return created_rule


class TestRulesAPIAsync:
    """Test suite for the async Rules API functionality."""

    class TestGet:
        """Tests for the async get method."""

        @pytest.mark.asyncio
        async def test_get_by_id(self, rules_api_async, test_rule):
            """Test getting a specific rule by ID."""
            retrieved_rule = await rules_api_async.get(rule_id=test_rule.id_)

            assert isinstance(retrieved_rule, Rule)
            assert retrieved_rule.id_ == test_rule.id_
            assert retrieved_rule.name == test_rule.name

        @pytest.mark.asyncio
        async def test_get_by_client_key(self, rules_api_async, test_rule):
            """Test getting a specific rule by client key."""
            if test_rule.client_key:
                retrieved_rule = await rules_api_async.get(client_key=test_rule.client_key)

                assert retrieved_rule is not None
                assert retrieved_rule.id_ == test_rule.id_

    class TestList:
        """Tests for the async list_ method."""

        @pytest.mark.asyncio
        async def test_basic_list(self, rules_api_async):
            """Test basic rule listing functionality."""
            rules = await rules_api_async.list_(limit=5)

            assert isinstance(rules, list)
            assert len(rules) == 5

            rule = rules[0]
            assert isinstance(rule, Rule)

        @pytest.mark.asyncio
        async def test_list_with_name_filter(self, rules_api_async, test_rule):
            """Test rule listing with name filtering."""
            filtered_rules = await rules_api_async.list_(name=test_rule.name)

            assert isinstance(filtered_rules, list)
            assert len(filtered_rules) >= 1

            for rule in filtered_rules:
                assert rule.name == test_rule.name

        @pytest.mark.asyncio
        async def test_list_with_name_contains_filter(self, rules_api_async):
            """Test rule listing with name contains filtering."""
            rules = await rules_api_async.list_(name_contains="test", limit=5)

            assert isinstance(rules, list)
            assert rules

            for rule in rules:
                assert "test" in rule.name.lower()

        @pytest.mark.asyncio
        async def test_list_with_name_regex_filter(self, rules_api_async):
            """Test rule listing with regex name filtering."""
            rules = await rules_api_async.list_(name_regex=r".*test.*", limit=5)

            assert isinstance(rules, list)
            assert rules

            import re

            pattern = re.compile(r".*test.*", re.IGNORECASE)
            for rule in rules:
                assert pattern.match(rule.name)

        @pytest.mark.asyncio
        async def test_list_with_rule_ids_filter(self, rules_api_async):
            """Test rule listing with rule IDs filter."""
            all_rules = await rules_api_async.list_(limit=3)

            if all_rules:
                rule_ids = [r.id_ for r in all_rules]
                filtered_rules = await rules_api_async.list_(rule_ids=rule_ids)

                assert isinstance(filtered_rules, list)
                assert len(filtered_rules) >= len(all_rules)

                for rule in filtered_rules:
                    assert rule.id_ in rule_ids

        @pytest.mark.asyncio
        async def test_list_with_description_contains_filter(self, rules_api_async):
            """Test rule listing with description contains filtering."""
            rules = await rules_api_async.list_(description_contains="test", limit=5)

            assert isinstance(rules, list)
            assert rules

            for rule in rules:
                assert "test" in rule.description.lower()

        @pytest.mark.asyncio
        async def test_list_with_limit(self, rules_api_async):
            """Test rule listing with different limits."""
            rules_1 = await rules_api_async.list_(limit=1)
            assert isinstance(rules_1, list)
            assert len(rules_1) <= 1

            rules_3 = await rules_api_async.list_(limit=3)
            assert isinstance(rules_3, list)
            assert len(rules_3) <= 3

        @pytest.mark.asyncio
        async def test_list_with_time_filters(self, rules_api_async):
            """Test rule listing with time-based filters."""
            from datetime import datetime, timedelta, timezone

            one_year_ago = datetime.now(timezone.utc) - timedelta(days=365)
            rules = await rules_api_async.list_(created_after=one_year_ago, limit=5)

            assert isinstance(rules, list)
            assert rules

            for rule in rules:
                assert rule.created_date >= one_year_ago

    class TestFind:
        """Tests for the async find method."""

        @pytest.mark.asyncio
        async def test_find_rule(self, rules_api_async, test_rule):
            """Test finding a single rule."""
            found_rule = await rules_api_async.find(rule_ids=[test_rule.id_])

            assert found_rule is not None
            assert found_rule.id_ == test_rule.id_

        @pytest.mark.asyncio
        async def test_find_nonexistent_rule(self, rules_api_async):
            """Test finding a non-existent rule returns None."""
            found_rule = await rules_api_async.find(name="nonexistent-rule-name-12345")
            assert found_rule is None

        @pytest.mark.asyncio
        async def test_find_multiple_raises_error(self, rules_api_async):
            """Test finding multiple rules raises an error."""
            with pytest.raises(ValueError, match="Multiple"):
                await rules_api_async.find(name_contains="test", limit=5)

    class TestCreate:
        """Tests for the async create method."""

        @pytest.mark.asyncio
        async def test_create_basic_rule(self, rules_api_async):
            """Test creating a basic rule with minimal fields."""
            from datetime import datetime, timezone

            rule_name = f"test_rule_create_{datetime.now(timezone.utc).isoformat()}"
            description = "Test rule created by Sift Client pytest"

            channels = await rules_api_async.client.async_.channels.list_(limit=2)
            assert len(channels) >= 2

            assets = await rules_api_async.client.async_.assets.list_(limit=1)
            assert len(assets) >= 1

            rule_create = RuleCreate(
                name=rule_name,
                description=description,
                expression="$1 > $2",
                channel_references=[
                    ChannelReference(channel_reference="$1", channel_identifier=channels[0].name),
                    ChannelReference(channel_reference="$2", channel_identifier=channels[1].name),
                ],
                action=RuleAction.annotation(
                    annotation_type=RuleAnnotationType.DATA_REVIEW,
                    tags=[],
                ),
                asset_ids=[assets[0]._id_or_error],
            )

            created_rule = await rules_api_async.create(rule_create)

            try:
                assert created_rule is not None
                assert isinstance(created_rule, Rule)
                assert created_rule.id_ is not None
                assert created_rule.name == rule_name
                assert created_rule.description == description
                assert created_rule.created_date is not None
                assert created_rule.modified_date is not None
            finally:
                await rules_api_async.archive(created_rule)

        @pytest.mark.asyncio
        async def test_create_rule_with_dict(self, rules_api_async):
            """Test creating a rule using a dictionary."""
            from datetime import datetime, timezone

            rule_name = f"test_rule_dict_{datetime.now(timezone.utc).isoformat()}"
            description = "Test rule created by Sift Client pytest"

            channels = await rules_api_async.client.async_.channels.list_(limit=2)
            assert len(channels) >= 2

            assets = await rules_api_async.client.async_.assets.list_(limit=1)
            assert len(assets) >= 1

            rule_dict = {
                "name": rule_name,
                "description": description,
                "expression": "$1 > $2",
                "channel_references": [
                    {"channel_reference": "$1", "channel_identifier": channels[0].name},
                    {"channel_reference": "$2", "channel_identifier": channels[1].name},
                ],
                "action": {
                    "action_type": RuleActionType.ANNOTATION,
                    "annotation_type": RuleAnnotationType.PHASE,
                    "tags": [],
                },
                "asset_ids": [assets[0].id_],
            }

            created_rule = await rules_api_async.create(rule_dict)

            try:
                assert created_rule.name == rule_name
                assert created_rule.description == description
            finally:
                await rules_api_async.archive(created_rule)

    class TestUpdate:
        """Tests for the async update method."""

        @pytest.mark.asyncio
        async def test_update_rule_description(self, rules_api_async, new_rule):
            """Test updating a rule's description."""
            try:
                update = RuleUpdate(description="Updated description")
                updated_rule = await rules_api_async.update(new_rule, update)

                assert updated_rule.id_ == new_rule.id_
                assert updated_rule.description == "Updated description"
                # Validate that things we didn't intentionally change didn't change
                assert updated_rule.name == new_rule.name
                assert updated_rule.is_external == new_rule.is_external
                assert updated_rule.expression == new_rule.expression
                assert updated_rule.action.action_type == new_rule.action.action_type
                assert updated_rule.client_key == new_rule.client_key
                assert updated_rule.rule_version.created_date > new_rule.rule_version.created_date
            finally:
                await rules_api_async.archive(new_rule.id_)

        @pytest.mark.asyncio
        async def test_update_rule_name(self, rules_api_async, new_rule):
            """Test updating a rule's name."""
            try:
                new_name = f"updated_{new_rule.name}"
                update = RuleUpdate(name=new_name)
                updated_rule = await rules_api_async.update(new_rule, update)

                assert updated_rule.name == new_name
                assert updated_rule.id_ == new_rule.id_
            finally:
                await rules_api_async.archive(new_rule.id_)

        @pytest.mark.asyncio
        async def test_update_with_dict(self, rules_api_async, new_rule):
            """Test updating a rule using a dictionary."""
            try:
                update_dict = {"description": "Updated via dict"}
                updated_rule = await rules_api_async.update(new_rule, update_dict)

                assert updated_rule.description == "Updated via dict"
            finally:
                await rules_api_async.archive(new_rule.id_)

        @pytest.mark.asyncio
        async def test_update_with_id_string(self, rules_api_async, new_rule):
            """Test updating a rule by passing ID as string."""
            try:
                update = RuleUpdate(description="Updated via ID string")
                updated_rule = await rules_api_async.update(new_rule.id_, update)

                assert updated_rule.id_ == new_rule.id_
                assert updated_rule.description == "Updated via ID string"
            finally:
                await rules_api_async.archive(new_rule.id_)

        @pytest.mark.asyncio
        async def test_update_with_version_notes(self, rules_api_async, new_rule):
            """Test updating a rule with version notes."""
            try:
                update = RuleUpdate(description="Updated with version notes")
                updated_rule = await rules_api_async.update(
                    new_rule, update, version_notes="Test version notes"
                )

                assert updated_rule.id_ == new_rule.id_
                assert updated_rule.description == "Updated with version notes"
            finally:
                await rules_api_async.archive(new_rule.id_)

        @pytest.mark.asyncio
        async def test_update_rule_action(self, rules_api_async, new_rule, ci_pytest_tag):
            """Test updating a rule's action including annotation type, tags, and assignee."""
            try:
                # Update the action with new annotation type, tags, and assignee
                update = RuleUpdate(
                    action=RuleAction.annotation(
                        annotation_type=RuleAnnotationType.PHASE,
                        tags=[ci_pytest_tag],
                        default_assignee_user=new_rule.created_by_user_id,
                    ),
                )
                updated_rule = await rules_api_async.update(new_rule, update)

                # Verify the action was updated
                assert updated_rule.id_ == new_rule.id_
                assert updated_rule.action.action_type == RuleActionType.ANNOTATION
                assert updated_rule.action.annotation_type == RuleAnnotationType.PHASE
                assert set(updated_rule.action.tags_ids) == {ci_pytest_tag.id_}
                assert updated_rule.action.default_assignee_user == new_rule.created_by_user_id

                # Verify other fields remain unchanged
                assert updated_rule.name == new_rule.name
                assert updated_rule.expression == new_rule.expression
            finally:
                await rules_api_async.archive(new_rule.id_)

        @pytest.mark.asyncio
        async def test_update_with_complex_expression(self, rules_api_async, sift_client, test_tag):
            """Test updating a rule with a complex expression (range check)."""
            # Get channels and assets
            channels = await sift_client.async_.channels.list_(limit=2)
            assert len(channels) >= 1
            assets = await sift_client.async_.assets.list_(limit=1)
            assert len(assets) >= 1

            # Create a rule with simple expression
            rule_name = f"test_rule_complex_expr_{datetime.now(timezone.utc).isoformat()}"
            rule_create = RuleCreate(
                name=rule_name,
                description="Test rule for complex expression update",
                expression="$1 > 0.5",
                channel_references=[
                    ChannelReference(channel_reference="$1", channel_identifier=channels[0].name),
                ],
                action=RuleAction.annotation(
                    annotation_type=RuleAnnotationType.DATA_REVIEW,
                    tags=[test_tag],
                ),
                asset_ids=[assets[0].id_],
            )
            created_rule = await rules_api_async.create(rule_create)

            try:
                # Update with complex expression (range check)
                update = RuleUpdate(
                    expression="$1 > 0.3 && $1 < 0.8",
                    channel_references=[
                        ChannelReference(
                            channel_reference="$1", channel_identifier=channels[0].name
                        ),
                    ],
                )
                updated_rule = await rules_api_async.update(created_rule, update)

                # Verify the expression was updated
                assert updated_rule.id_ == created_rule.id_
                assert updated_rule.expression == "$1 > 0.3 && $1 < 0.8"
                assert len(updated_rule.channel_references) == 1
                assert updated_rule.channel_references[0].channel_identifier == channels[0].name

                # Verify other fields remain unchanged
                assert updated_rule.name == created_rule.name
                assert updated_rule.action.action_type == created_rule.action.action_type
            finally:
                await rules_api_async.archive(created_rule.id_)

        @pytest.mark.asyncio
        async def test_update_with_multiple_channel_references(
            self, rules_api_async, sift_client, test_tag
        ):
            """Test updating a rule expression to use multiple channel references."""
            # Get channels and assets
            channels = await sift_client.async_.channels.list_(limit=3)
            assert len(channels) >= 3
            assets = await sift_client.async_.assets.list_(limit=1)
            assert len(assets) >= 1

            # Create a rule with simple expression
            rule_name = f"test_rule_multi_refs_{datetime.now(timezone.utc).isoformat()}"
            rule_create = RuleCreate(
                name=rule_name,
                description="Test rule for multiple channel references",
                expression="$1 > $2",
                channel_references=[
                    ChannelReference(channel_reference="$1", channel_identifier=channels[0].name),
                    ChannelReference(channel_reference="$2", channel_identifier=channels[1].name),
                ],
                action=RuleAction.annotation(
                    annotation_type=RuleAnnotationType.DATA_REVIEW,
                    tags=[test_tag],
                ),
                asset_ids=[assets[0].id_],
            )
            created_rule = await rules_api_async.create(rule_create)

            try:
                # Update with expression using three channel references
                update = RuleUpdate(
                    expression="($1 > $2) && ($3 < 100)",
                    channel_references=[
                        ChannelReference(
                            channel_reference="$1", channel_identifier=channels[0].name
                        ),
                        ChannelReference(
                            channel_reference="$2", channel_identifier=channels[1].name
                        ),
                        ChannelReference(
                            channel_reference="$3", channel_identifier=channels[2].name
                        ),
                    ],
                )
                updated_rule = await rules_api_async.update(created_rule, update)

                # Verify the expression and channel references were updated
                assert updated_rule.id_ == created_rule.id_
                assert updated_rule.expression == "($1 > $2) && ($3 < 100)"
                assert len(updated_rule.channel_references) == 3

                # Verify all three channel references are present
                ref_identifiers = {
                    ref.channel_identifier for ref in updated_rule.channel_references
                }
                assert channels[0].name in ref_identifiers
                assert channels[1].name in ref_identifiers
                assert channels[2].name in ref_identifiers
            finally:
                await rules_api_async.archive(created_rule.id_)

        @pytest.mark.asyncio
        async def test_update_with_invalid_expression(self, rules_api_async, new_rule):
            """Test updating a rule with an invalid expression.

            Note: The server may or may not validate expression syntax at update time.
            This test documents the current behavior.
            """
            try:
                # Attempt to update with an invalid expression
                update = RuleUpdate(
                    expression="invalid_expression",
                    channel_references=[
                        ChannelReference(
                            channel_reference="$1",
                            channel_identifier=new_rule.channel_references[0].channel_identifier,
                        ),
                    ],
                )

                # This may succeed or fail depending on server-side validation
                # If it succeeds, the expression is stored but may fail at evaluation time
                try:
                    updated_rule = await rules_api_async.update(new_rule, update)
                    # If update succeeds, verify the expression was set
                    assert updated_rule.expression == "invalid_expression"
                except Exception as e:
                    # If server validates and rejects, that's also acceptable behavior
                    assert (  # noqa: PT017
                        "expression" in str(e).lower() or "invalid" in str(e).lower()
                    )
            finally:
                await rules_api_async.archive(new_rule.id_)

    class TestArchive:
        """Tests for the async archive method."""

        @pytest.mark.asyncio
        async def test_archive_rule(self, rules_api_async, new_rule):
            """Test archiving a rule."""
            rule = await rules_api_async.archive(new_rule)

            assert isinstance(rule, Rule)
            assert rule.id_ == new_rule.id_
            assert rule.is_archived is True

            rules_without_archived = await rules_api_async.list_(
                name=new_rule.name, include_archived=False
            )
            assert len(rules_without_archived) == 0

            rules_with_archived = await rules_api_async.list_(
                name=new_rule.name, include_archived=True
            )
            assert len(rules_with_archived) == 1
            assert rules_with_archived[0].id_ == new_rule.id_
            assert rules_with_archived[0].archived_date is not None

        @pytest.mark.asyncio
        async def test_archive_with_id_string(self, rules_api_async, new_rule):
            """Test archiving a rule by passing ID as string."""
            rule = await rules_api_async.archive(new_rule.id_)

            assert isinstance(rule, Rule)
            assert rule.id_ == new_rule.id_
            assert rule.is_archived is True

    class TestUnarchive:
        """Tests for the async unarchive method."""

        @pytest.mark.asyncio
        async def test_unarchive_rule(self, rules_api_async, new_rule):
            """Test unarchiving a rule."""
            try:
                await rules_api_async.archive(new_rule)

                rule = await rules_api_async.unarchive(new_rule)

                assert isinstance(rule, Rule)
                assert rule.id_ == new_rule.id_
                assert rule.is_archived is False
            finally:
                await rules_api_async.archive(new_rule.id_)

    class TestBatchUpdate:
        """Tests for the async batch_update_rules method."""

        @pytest.mark.asyncio
        async def test_batch_update_or_create_rules(self, rules_api_async, nostromo_asset):
            """Test updating multiple rules with different fields."""
            from datetime import datetime, timezone

            rule1_name = f"test_batch_rule_1_{datetime.now(timezone.utc).isoformat()}"
            rule2_name = f"test_batch_rule_2_{datetime.now(timezone.utc).isoformat()}"

            rule1 = await rules_api_async.create(
                RuleCreate(
                    name=rule1_name,
                    client_key=f"test_batch_1_{str(uuid.uuid4())[-8:]}",
                    description="Test rule 1 for batch update",
                    expression="$1 > $2",
                    channel_references=[
                        ChannelReference(channel_reference="$1", channel_identifier="channel1"),
                        ChannelReference(channel_reference="$2", channel_identifier="channel2"),
                    ],
                    action=RuleAction.annotation(
                        annotation_type=RuleAnnotationType.DATA_REVIEW,
                        tags=[],
                    ),
                    asset_ids=[nostromo_asset.id_],
                )
            )

            rule2 = await rules_api_async.create(
                RuleCreate(
                    name=rule2_name,
                    client_key=f"test_batch_2_{str(uuid.uuid4())[-8:]}",
                    description="Test rule 2 for batch update",
                    expression="$1 > 0.5",
                    channel_references=[
                        ChannelReference(channel_reference="$1", channel_identifier="channel1"),
                    ],
                    action=RuleAction.annotation(
                        annotation_type=RuleAnnotationType.DATA_REVIEW,
                        tags=[],
                    ),
                    asset_ids=[nostromo_asset.id_],
                )
            )

            try:
                # Batch update both rules
                rule1_update = RuleUpdate(description="Updated description 1")
                rule1_update.resource_id = rule1.id_

                rule2_update = RuleUpdate(description="Updated description 2")
                rule2_update.resource_id = rule2.id_

                updates = [rule1_update, rule2_update]

                updated_rules = await rules_api_async.batch_update_or_create_rules(updates)

                assert isinstance(updated_rules, list)
                assert len(updated_rules) == 2

                # Verify updates were applied
                assert updated_rules[0].description == "Updated description 1"
                assert updated_rules[1].description == "Updated description 2"
            finally:
                await rules_api_async.archive(rule1.id_)
                await rules_api_async.archive(rule2.id_)

        @pytest.mark.asyncio
        async def test_batch_update_rules_creates_rules(self, rules_api_async, nostromo_asset):
            """Test batch updating rules that don't already exist."""
            from datetime import datetime, timezone

            rule1_name = f"test_batch_rule_1_{datetime.now(timezone.utc).isoformat()}"
            rule2_name = f"test_batch_rule_2_{datetime.now(timezone.utc).isoformat()}"

            rule1 = RuleCreate(
                name=rule1_name,
                client_key=f"test_batch_1_{str(uuid.uuid4())[-8:]}",
                description="Test rule 1 for batch update",
                expression="$1 > $2",
                channel_references=[
                    ChannelReference(channel_reference="$1", channel_identifier="channel1"),
                    ChannelReference(channel_reference="$2", channel_identifier="channel2"),
                ],
                action=RuleAction.annotation(
                    annotation_type=RuleAnnotationType.DATA_REVIEW,
                    tags=[],
                ),
                asset_ids=[nostromo_asset.id_],
            )

            rule2 = RuleCreate(
                name=rule2_name,
                client_key=f"test_batch_2_{str(uuid.uuid4())[-8:]}",
                description="Test rule 2 for batch update",
                expression="$1 > 0.5",
                channel_references=[
                    ChannelReference(channel_reference="$1", channel_identifier="channel1"),
                ],
                action=RuleAction.annotation(
                    annotation_type=RuleAnnotationType.DATA_REVIEW,
                    tags=[],
                ),
                asset_ids=[nostromo_asset.id_],
            )

            updated_rules: list[Rule] = []
            try:
                # Batch update (actually create) both rules
                updates = [rule1, rule2]
                updated_rules = await rules_api_async.batch_update_or_create_rules(updates)

                assert isinstance(updated_rules, list)
                assert len(updated_rules) == 2

                assert updated_rules[0].client_key == rule1.client_key
                assert updated_rules[1].client_key == rule2.client_key
            finally:
                for rule in updated_rules:
                    await rules_api_async.archive(rule.id_)

        @pytest.mark.asyncio
        async def test_batch_update_rules_empty_list(self, rules_api_async):
            """Test handling empty list."""
            updated_rules = await rules_api_async.batch_update_or_create_rules([])

            assert isinstance(updated_rules, list)
            assert len(updated_rules) == 0


class TestRulesAPISync:
    """Test suite for the synchronous Rules API functionality."""

    class TestGet:
        """Tests for the sync get method."""

        def test_get_by_id(self, rules_api_sync, test_rule):
            """Test getting a specific rule by ID synchronously."""
            retrieved_rule = rules_api_sync.get(rule_id=test_rule.id_)

            assert retrieved_rule is not None
            assert retrieved_rule.id_ == test_rule.id_
