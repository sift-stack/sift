"""Pytest tests for the Calculated Channels API.

These tests demonstrate and validate the usage of the Calculated Channels API including:
- Basic calculated channel operations (get, list, find)
- Calculated channel filtering and searching
- Calculated channel creation, updates, and archiving
- Version management
- Error handling and edge cases
"""

import uuid
from datetime import datetime, timezone

import pytest

from sift_client import SiftClient
from sift_client.resources import CalculatedChannelsAPI, CalculatedChannelsAPIAsync
from sift_client.sift_types import CalculatedChannel
from sift_client.sift_types.calculated_channel import (
    CalculatedChannelCreate,
    CalculatedChannelUpdate,
)
from sift_client.sift_types.channel import ChannelReference
from sift_client.util import cel_utils as cel

pytestmark = pytest.mark.integration


def test_client_binding(sift_client):
    assert sift_client.calculated_channels
    assert isinstance(sift_client.calculated_channels, CalculatedChannelsAPI)
    assert sift_client.async_.calculated_channels
    assert isinstance(sift_client.async_.calculated_channels, CalculatedChannelsAPIAsync)


@pytest.fixture
def calculated_channels_api_async(sift_client: SiftClient):
    """Get the async calculated channels API instance."""
    return sift_client.async_.calculated_channels


@pytest.fixture
def calculated_channels_api_sync(sift_client: SiftClient):
    """Get the synchronous calculated channels API instance."""
    return sift_client.calculated_channels


@pytest.fixture
def test_calculated_channel(calculated_channels_api_sync):
    calculated_channels = calculated_channels_api_sync.list_(
        limit=1, include_archived=True, filter_query=cel.not_(cel.equals("client_key", ""))
    )
    assert calculated_channels
    assert len(calculated_channels) >= 1
    return calculated_channels[0]


@pytest.fixture
def new_calculated_channel(calculated_channels_api_sync, sift_client):
    """Create a test calculated channel for update tests."""
    from datetime import datetime, timezone

    calc_channel_name = f"test_calc_channel_{datetime.now(timezone.utc).isoformat()}"
    description = "Test calculated channel created by Sift Client pytest"

    # Get some channels to reference
    channels = sift_client.channels.list_(limit=2)
    assert len(channels) >= 2

    created_calc_channel = calculated_channels_api_sync.create(
        CalculatedChannelCreate(
            name=calc_channel_name,
            client_key=f"test_calc_chan_{str(uuid.uuid4())[-8:]}",
            description=description,
            expression="$1 + $2",
            expression_channel_references=[
                ChannelReference(channel_reference="$1", channel_identifier=channels[0].name),
                ChannelReference(channel_reference="$2", channel_identifier=channels[1].name),
            ],
            all_assets=True,
        )
    )
    return created_calc_channel


class TestCalculatedChannelsAPIAsync:
    """Test suite for the async Calculated Channels API functionality."""

    class TestGet:
        """Tests for the async get method."""

        @pytest.mark.asyncio
        async def test_get_by_id(self, calculated_channels_api_async, test_calculated_channel):
            """Test getting a specific calculated channel by ID."""
            retrieved_calc_channel = await calculated_channels_api_async.get(
                calculated_channel_id=test_calculated_channel.id_
            )

            assert isinstance(retrieved_calc_channel, CalculatedChannel)
            assert retrieved_calc_channel.id_ == test_calculated_channel.id_
            assert retrieved_calc_channel.name == test_calculated_channel.name

        @pytest.mark.asyncio
        async def test_get_by_client_key(
            self, calculated_channels_api_async, test_calculated_channel
        ):
            """Test getting a specific calculated channel by client key."""
            retrieved_calc_channel = await calculated_channels_api_async.get(
                client_key=test_calculated_channel.client_key
            )

            assert retrieved_calc_channel is not None
            assert retrieved_calc_channel.id_ == test_calculated_channel.id_

    class TestList:
        """Tests for the async list_ method."""

        @pytest.mark.asyncio
        async def test_basic_list(self, calculated_channels_api_async):
            """Test basic calculated channel listing functionality."""
            calc_channels = await calculated_channels_api_async.list_(
                limit=5, include_archived=True
            )

            assert isinstance(calc_channels, list)
            assert len(calc_channels) == 5

            calc_channel = calc_channels[0]
            assert isinstance(calc_channel, CalculatedChannel)

        @pytest.mark.asyncio
        async def test_list_with_name_filter(self, calculated_channels_api_async):
            """Test calculated channel listing with name filtering."""
            all_calc_channels = await calculated_channels_api_async.list_(
                limit=10, include_archived=True
            )

            test_calc_channel_name = all_calc_channels[0].name
            filtered_calc_channels = await calculated_channels_api_async.list_(
                name=test_calc_channel_name, include_archived=True
            )

            assert isinstance(filtered_calc_channels, list)
            assert len(filtered_calc_channels) >= 1

            for calc_channel in filtered_calc_channels:
                assert calc_channel.name == test_calc_channel_name

        @pytest.mark.asyncio
        async def test_list_with_name_contains_filter(self, calculated_channels_api_async):
            """Test calculated channel listing with name contains filtering."""
            calc_channels = await calculated_channels_api_async.list_(
                name_contains="test", limit=5, include_archived=True
            )

            assert isinstance(calc_channels, list)
            assert calc_channels

            for calc_channel in calc_channels:
                assert "test" in calc_channel.name.lower()

        @pytest.mark.asyncio
        async def test_list_with_name_regex_filter(self, calculated_channels_api_async):
            """Test calculated channel listing with regex name filtering."""
            calc_channels = await calculated_channels_api_async.list_(
                name_regex=r".*test.*", limit=5, include_archived=True
            )

            assert isinstance(calc_channels, list)
            assert calc_channels

            import re

            pattern = re.compile(r".*test.*", re.IGNORECASE)
            for calc_channel in calc_channels:
                assert pattern.match(calc_channel.name)

        @pytest.mark.asyncio
        async def test_list_with_limit(self, calculated_channels_api_async):
            """Test calculated channel listing with different limits."""
            calc_channels_1 = await calculated_channels_api_async.list_(limit=1)
            assert isinstance(calc_channels_1, list)
            assert len(calc_channels_1) <= 1

            calc_channels_3 = await calculated_channels_api_async.list_(limit=3)
            assert isinstance(calc_channels_3, list)
            assert len(calc_channels_3) <= 3

    class TestFind:
        """Tests for the async find method."""

        @pytest.mark.asyncio
        async def test_find_calculated_channel(
            self, calculated_channels_api_async, test_calculated_channel
        ):
            """Test finding a single calculated channel."""
            found_calc_channel = await calculated_channels_api_async.find(
                name=test_calculated_channel.name, include_archived=True
            )

            assert found_calc_channel is not None
            assert found_calc_channel.id_ == test_calculated_channel.id_

        @pytest.mark.asyncio
        async def test_find_nonexistent_calculated_channel(self, calculated_channels_api_async):
            """Test finding a non-existent calculated channel returns None."""
            found_calc_channel = await calculated_channels_api_async.find(
                name="nonexistent-calculated-channel-name-12345", include_archived=True
            )
            assert found_calc_channel is None

        @pytest.mark.asyncio
        async def test_find_multiple_raises_error(self, calculated_channels_api_async):
            """Test finding multiple calculated channels raises an error."""
            with pytest.raises(ValueError, match="Multiple"):
                await calculated_channels_api_async.find(
                    name_contains="test", limit=5, include_archived=True
                )

    class TestCreate:
        """Tests for the async create method."""

        @pytest.mark.asyncio
        async def test_create_basic_calculated_channel(self, calculated_channels_api_async):
            """Test creating a basic calculated channel with minimal fields."""
            from datetime import datetime, timezone

            calc_channel_name = f"test_calc_channel_create_{datetime.now(timezone.utc).isoformat()}"
            description = "Test calculated channel created by Sift Client pytest"

            channels = await calculated_channels_api_async.client.async_.channels.list_(limit=2)
            assert len(channels) >= 2

            calc_channel_create = CalculatedChannelCreate(
                name=calc_channel_name,
                description=description,
                expression="$1 + $2",
                expression_channel_references=[
                    ChannelReference(channel_reference="$1", channel_identifier=channels[0].name),
                    ChannelReference(channel_reference="$2", channel_identifier=channels[1].name),
                ],
                all_assets=True,
            )

            created_calc_channel = await calculated_channels_api_async.create(calc_channel_create)

            try:
                assert created_calc_channel is not None
                assert isinstance(created_calc_channel, CalculatedChannel)
                assert created_calc_channel.id_ is not None
                assert created_calc_channel.name == calc_channel_name
                assert created_calc_channel.description == description
                assert created_calc_channel.created_date is not None
                assert created_calc_channel.modified_date is not None
            finally:
                await calculated_channels_api_async.archive(created_calc_channel)

        @pytest.mark.asyncio
        async def test_create_calculated_channel_with_dict(self, calculated_channels_api_async):
            """Test creating a calculated channel using a dictionary."""
            from datetime import datetime, timezone

            calc_channel_name = f"test_calc_channel_dict_{datetime.now(timezone.utc).isoformat()}"
            description = "Test calculated channel created by Sift Client pytest"

            channels = await calculated_channels_api_async.client.async_.channels.list_(limit=2)
            assert len(channels) >= 2

            calc_channel_dict = {
                "name": calc_channel_name,
                "description": description,
                "expression": "$1 + $2",
                "expression_channel_references": [
                    {"channel_reference": "$1", "channel_identifier": channels[0].name},
                    {"channel_reference": "$2", "channel_identifier": channels[1].name},
                ],
                "all_assets": True,
            }

            created_calc_channel = await calculated_channels_api_async.create(calc_channel_dict)

            try:
                assert created_calc_channel.name == calc_channel_name
                assert created_calc_channel.description == description
            finally:
                await calculated_channels_api_async.archive(created_calc_channel)

    class TestUpdate:
        """Tests for the async update method."""

        @pytest.mark.asyncio
        async def test_update_calculated_channel_description(
            self, calculated_channels_api_async, new_calculated_channel
        ):
            """Test updating a calculated channel's description."""
            try:
                update = CalculatedChannelUpdate(description="Updated description")
                updated_calc_channel = await calculated_channels_api_async.update(
                    new_calculated_channel, update
                )

                assert updated_calc_channel.id_ == new_calculated_channel.id_
                assert updated_calc_channel.description == "Updated description"
                assert updated_calc_channel.name == new_calculated_channel.name
            finally:
                await calculated_channels_api_async.archive(new_calculated_channel.id_)

        @pytest.mark.asyncio
        async def test_update_calculated_channel_name(
            self, calculated_channels_api_async, new_calculated_channel
        ):
            """Test updating a calculated channel's name."""
            try:
                new_name = f"updated_{new_calculated_channel.name}"
                update = CalculatedChannelUpdate(name=new_name)
                updated_calc_channel = await calculated_channels_api_async.update(
                    new_calculated_channel, update
                )

                assert updated_calc_channel.name == new_name
                assert updated_calc_channel.id_ == new_calculated_channel.id_
            finally:
                await calculated_channels_api_async.archive(new_calculated_channel.id_)

        @pytest.mark.asyncio
        async def test_update_with_dict(
            self, calculated_channels_api_async, new_calculated_channel
        ):
            """Test updating a calculated channel using a dictionary."""
            try:
                update_dict = {"description": "Updated via dict"}
                updated_calc_channel = await calculated_channels_api_async.update(
                    new_calculated_channel, update_dict
                )

                assert updated_calc_channel.description == "Updated via dict"
            finally:
                await calculated_channels_api_async.archive(new_calculated_channel.id_)

        @pytest.mark.asyncio
        async def test_update_with_id_string(
            self, calculated_channels_api_async, new_calculated_channel
        ):
            """Test updating a calculated channel by passing ID as string."""
            try:
                update = CalculatedChannelUpdate(description="Updated via ID string")
                updated_calc_channel = await calculated_channels_api_async.update(
                    new_calculated_channel.id_, update
                )

                assert updated_calc_channel.id_ == new_calculated_channel.id_
                assert updated_calc_channel.description == "Updated via ID string"
            finally:
                await calculated_channels_api_async.archive(new_calculated_channel.id_)

        @pytest.mark.asyncio
        async def test_update_calculated_channel_units(
            self, calculated_channels_api_async, new_calculated_channel
        ):
            """Test updating a calculated channel's units."""
            try:
                update = CalculatedChannelUpdate(units="percentage")
                updated_calc_channel = await calculated_channels_api_async.update(
                    new_calculated_channel, update
                )

                assert updated_calc_channel.id_ == new_calculated_channel.id_
                assert updated_calc_channel.units == "percentage"
                assert updated_calc_channel.name == new_calculated_channel.name
            finally:
                await calculated_channels_api_async.archive(new_calculated_channel.id_)

        @pytest.mark.asyncio
        async def test_update_with_complex_expression(
            self, calculated_channels_api_async, sift_client
        ):
            """Test updating a calculated channel with a complex expression using multiple channel references."""
            # Get channels to reference
            channels = await sift_client.async_.channels.list_(limit=3)
            assert len(channels) >= 3

            # Create a calculated channel
            calc_channel_name = (
                f"test_calc_channel_complex_{datetime.now(timezone.utc).isoformat()}"
            )
            calc_channel_create = CalculatedChannelCreate(
                name=calc_channel_name,
                description="Test calculated channel for complex expression update",
                expression="$1 + $2",
                expression_channel_references=[
                    ChannelReference(channel_reference="$1", channel_identifier=channels[0].name),
                    ChannelReference(channel_reference="$2", channel_identifier=channels[1].name),
                ],
                all_assets=True,
            )
            created_calc_channel = await calculated_channels_api_async.create(calc_channel_create)

            try:
                # Update with complex expression
                update = CalculatedChannelUpdate(
                    expression="($1 / $2) * 100 + ($3 * 0.1)",
                    expression_channel_references=[
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
                updated_calc_channel = await calculated_channels_api_async.update(
                    created_calc_channel, update
                )

                assert updated_calc_channel.id_ == created_calc_channel.id_
                assert updated_calc_channel.expression == "($1 / $2) * 100 + ($3 * 0.1)"
                assert len(updated_calc_channel.channel_references) == 3
                # Verify all three channel references are present
                ref_identifiers = {
                    ref.channel_identifier for ref in updated_calc_channel.channel_references
                }
                assert channels[0].name in ref_identifiers
                assert channels[1].name in ref_identifiers
                assert channels[2].name in ref_identifiers
            finally:
                await calculated_channels_api_async.archive(created_calc_channel.id_)

        @pytest.mark.asyncio
        async def test_update_with_invalid_expression(
            self, calculated_channels_api_async, new_calculated_channel
        ):
            """Test updating a calculated channel with an invalid expression.

            Note: The server may or may not validate expression syntax at update time.
            This test documents the current behavior.
            """
            try:
                # Attempt to update with an invalid expression
                update = CalculatedChannelUpdate(
                    expression="invalid_expression",
                    expression_channel_references=[
                        ChannelReference(
                            channel_reference="$1",
                            channel_identifier=new_calculated_channel.channel_references[
                                0
                            ].channel_identifier,
                        ),
                    ],
                )

                # This may succeed or fail depending on server-side validation
                # If it succeeds, the expression is stored but may fail at evaluation time
                try:
                    updated_calc_channel = await calculated_channels_api_async.update(
                        new_calculated_channel, update
                    )
                    # If update succeeds, verify the expression was set
                    assert updated_calc_channel.expression == "invalid_expression"
                except Exception as e:
                    # If server validates and rejects, that's also acceptable behavior
                    assert (  # noqa: PT017
                        "expression" in str(e).lower() or "invalid" in str(e).lower()
                    )
            finally:
                await calculated_channels_api_async.archive(new_calculated_channel.id_)

    class TestArchive:
        """Tests for the async archive method."""

        @pytest.mark.asyncio
        async def test_archive_calculated_channel(
            self, calculated_channels_api_async, new_calculated_channel
        ):
            """Test archiving a calculated channel."""
            calc_channel = await calculated_channels_api_async.archive(new_calculated_channel)

            assert isinstance(calc_channel, CalculatedChannel)
            assert calc_channel.id_ == new_calculated_channel.id_
            assert calc_channel.is_archived is True

            calc_channels_without_archived = await calculated_channels_api_async.list_(
                name=new_calculated_channel.name, include_archived=False
            )
            assert len(calc_channels_without_archived) == 0

            calc_channels_with_archived = await calculated_channels_api_async.list_(
                name=new_calculated_channel.name, include_archived=True
            )
            assert len(calc_channels_with_archived) == 1
            assert calc_channels_with_archived[0].id_ == new_calculated_channel.id_
            assert calc_channels_with_archived[0].archived_date is not None

        @pytest.mark.asyncio
        async def test_archive_with_id_string(
            self, calculated_channels_api_async, new_calculated_channel
        ):
            """Test archiving a calculated channel by passing ID as string."""
            calc_channel = await calculated_channels_api_async.archive(new_calculated_channel.id_)

            assert isinstance(calc_channel, CalculatedChannel)
            assert calc_channel.id_ == new_calculated_channel.id_
            assert calc_channel.is_archived is True

    class TestUnarchive:
        """Tests for the async unarchive method."""

        @pytest.mark.asyncio
        async def test_unarchive_calculated_channel(
            self, calculated_channels_api_async, new_calculated_channel
        ):
            """Test unarchiving a calculated channel."""
            try:
                await calculated_channels_api_async.archive(new_calculated_channel)

                calc_channel = await calculated_channels_api_async.unarchive(new_calculated_channel)

                assert isinstance(calc_channel, CalculatedChannel)
                assert calc_channel.id_ == new_calculated_channel.id_
                assert calc_channel.is_archived is False
            finally:
                await calculated_channels_api_async.archive(new_calculated_channel.id_)

    class TestListVersions:
        """Tests for the async list_versions method."""

        @pytest.mark.asyncio
        async def test_list_versions(self, calculated_channels_api_async, test_calculated_channel):
            """Test listing versions of a calculated channel."""
            versions = await calculated_channels_api_async.list_versions(
                calculated_channel=test_calculated_channel, include_archived=True
            )

            assert isinstance(versions, list)
            assert len(versions) >= 1

            for version in versions:
                assert isinstance(version, CalculatedChannel)
                assert version.name == test_calculated_channel.name


class TestCalculatedChannelsAPISync:
    """Test suite for the synchronous Calculated Channels API functionality."""

    class TestGet:
        """Tests for the sync get method."""

        def test_get_by_id(self, calculated_channels_api_sync, test_calculated_channel):
            """Test getting a specific calculated channel by ID synchronously."""
            retrieved_calc_channel = calculated_channels_api_sync.get(
                calculated_channel_id=test_calculated_channel.id_
            )

            assert retrieved_calc_channel is not None
            assert retrieved_calc_channel.id_ == test_calculated_channel.id_
