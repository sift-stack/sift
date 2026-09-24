"""Pytest tests for the Campaigns API.

These tests cover get, list, find, create, update, add_reports,
add_runs, archive/unarchive, and report_summaries.
"""

from datetime import datetime, timezone

import pytest

from sift_client import SiftClient
from sift_client.resources import CampaignsAPI, CampaignsAPIAsync
from sift_client.sift_types import Campaign
from sift_client.sift_types.campaign import CampaignCreate, CampaignUpdate

pytestmark = pytest.mark.integration


def test_client_binding(sift_client):
    assert sift_client.campaigns
    assert isinstance(sift_client.campaigns, CampaignsAPI)
    assert sift_client.async_.campaigns
    assert isinstance(sift_client.async_.campaigns, CampaignsAPIAsync)


@pytest.fixture
def campaigns_api_async(sift_client: SiftClient):
    """Get the async campaigns API instance."""
    return sift_client.async_.campaigns


@pytest.fixture
def campaigns_api_sync(sift_client: SiftClient):
    """Get the synchronous campaigns API instance."""
    return sift_client.campaigns


@pytest.fixture(scope="session")
def test_timestamp_str():
    """A per-session suffix so campaign names stay unique across runs."""
    return datetime.now(timezone.utc).isoformat()


@pytest.fixture(scope="session")
def campaign_run(sift_client, test_timestamp_str):
    """A run with a default report, so it can join a campaign."""
    from sift_client.sift_types.run import RunCreate

    return sift_client.runs.create(
        RunCreate(
            name=f"test_campaign_run_{test_timestamp_str}",
            description="sift_client campaign tests",
            create_default_report=True,
        )
    )


@pytest.fixture(scope="session")
def new_campaign(sift_client, test_timestamp_str):
    """Create a campaign for the session and archive it on teardown."""
    created = sift_client.campaigns.create(
        CampaignCreate(
            name=f"test_campaign_{test_timestamp_str}",
            description="sift_client campaign tests",
            client_key=f"test_campaign_key_{test_timestamp_str}",
            tags=["sift-client-pytest"],
            metadata={"suite": "campaigns"},
        )
    )
    yield created
    sift_client.campaigns.archive(created)


class TestCampaigns:
    """Tests for the Campaigns API."""

    def test_create(self, new_campaign, test_timestamp_str):
        """Test that create returns a fully populated campaign."""
        assert isinstance(new_campaign, Campaign)
        assert new_campaign.id_ is not None
        assert new_campaign.name == f"test_campaign_{test_timestamp_str}"
        assert new_campaign.description == "sift_client campaign tests"
        assert new_campaign.client_key == f"test_campaign_key_{test_timestamp_str}"
        assert new_campaign.tags == ["sift-client-pytest"]
        assert new_campaign.metadata == {"suite": "campaigns"}
        assert new_campaign.is_archived is False
        assert new_campaign.organization_id

    def test_get_by_id(self, campaigns_api_sync, new_campaign):
        """Test getting a campaign by ID."""
        fetched = campaigns_api_sync.get(new_campaign._id_or_error)

        assert isinstance(fetched, Campaign)
        assert fetched.id_ == new_campaign.id_

    def test_get_by_client_key(self, campaigns_api_sync, new_campaign):
        """Test getting a campaign by client key."""
        fetched = campaigns_api_sync.get(client_key=new_campaign.client_key)

        assert fetched.id_ == new_campaign.id_

    def test_get_requires_an_identifier(self, campaigns_api_sync):
        """Test that get rejects a call with neither identifier."""
        with pytest.raises(ValueError, match="Either campaign_id or client_key"):
            campaigns_api_sync.get()

    def test_basic_list(self, campaigns_api_sync, new_campaign):
        """Test basic campaign listing functionality."""
        campaigns = campaigns_api_sync.list_(limit=5)

        assert isinstance(campaigns, list)
        assert len(campaigns) >= 1
        for campaign in campaigns:
            assert isinstance(campaign, Campaign)
            assert campaign.id_ is not None

    def test_list_with_name_filter(self, campaigns_api_sync, new_campaign):
        """Test campaign listing with name filtering."""
        by_name = campaigns_api_sync.list_(name=new_campaign.name)
        by_contains = campaigns_api_sync.list_(name_contains=new_campaign.name)

        assert len(by_name) == 1
        assert by_name[0].id_ == new_campaign.id_
        assert by_contains[0].id_ == new_campaign.id_

    def test_list_with_id_filter(self, campaigns_api_sync, new_campaign):
        """Test campaign listing filtered to specific IDs."""
        campaigns = campaigns_api_sync.list_(campaign_ids=[new_campaign._id_or_error])

        assert len(campaigns) == 1
        assert campaigns[0].id_ == new_campaign.id_

    def test_list_with_client_key_filter(self, campaigns_api_sync, new_campaign):
        """Test campaign listing filtered by client key."""
        campaigns = campaigns_api_sync.list_(client_keys=[new_campaign.client_key])

        assert len(campaigns) == 1
        assert campaigns[0].id_ == new_campaign.id_

    def test_list_with_tag_filter(self, campaigns_api_sync, new_campaign):
        """Test campaign listing filtered by tag."""
        campaigns = campaigns_api_sync.list_(
            campaign_ids=[new_campaign._id_or_error], tags=["sift-client-pytest"]
        )

        assert len(campaigns) == 1

    def test_find(self, campaigns_api_sync, new_campaign):
        """Test finding a single campaign."""
        found = campaigns_api_sync.find(name=new_campaign.name)

        assert found is not None
        assert found.id_ == new_campaign.id_

    def test_find_nonexistent(self, campaigns_api_sync):
        """Test finding a non-existent campaign returns None."""
        found = campaigns_api_sync.find(
            name=f"nonexistent_campaign_{datetime.now(timezone.utc).timestamp()}"
        )
        assert found is None

    def test_update(self, campaigns_api_sync, new_campaign, test_timestamp_str):
        """Test updating a campaign's description."""
        updated = campaigns_api_sync.update(
            new_campaign, CampaignUpdate(description="updated description")
        )

        assert updated.id_ == new_campaign.id_
        assert updated.description == "updated description"
        # The name was not in the mask, so it is unchanged.
        assert updated.name == new_campaign.name

        campaigns_api_sync.update(new_campaign, {"description": "sift_client campaign tests"})

    def test_update_accepts_dict(self, campaigns_api_sync, new_campaign, test_timestamp_str):
        """Test that update accepts a plain dict."""
        renamed = f"test_campaign_renamed_{test_timestamp_str}"
        updated = campaigns_api_sync.update(new_campaign._id_or_error, {"name": renamed})

        assert updated.name == renamed

        campaigns_api_sync.update(new_campaign, {"name": new_campaign.name})

    def test_add_reports(self, campaigns_api_sync, campaign_run, test_timestamp_str):
        """Test adding reports to a campaign without dropping the existing ones."""
        campaign = campaigns_api_sync.create(
            CampaignCreate(name=f"test_campaign_reports_{test_timestamp_str}")
        )
        report_id = campaign_run.default_report_id

        added = campaigns_api_sync.add_reports(campaign, [report_id])
        assert [r.report_id for r in added.report_summaries] == [report_id]

        # Adding the same report again is a no-op, not a duplicate.
        again = campaigns_api_sync.add_reports(added, [report_id])
        assert [r.report_id for r in again.report_summaries] == [report_id]

        campaigns_api_sync.archive(campaign)

    def test_add_runs(self, campaigns_api_sync, campaign_run, test_timestamp_str):
        """Test adding runs to a campaign through their default reports."""
        campaign = campaigns_api_sync.create(
            CampaignCreate(name=f"test_campaign_runs_{test_timestamp_str}")
        )

        added = campaigns_api_sync.add_runs(campaign, [campaign_run])
        assert [r.report_id for r in added.report_summaries] == [campaign_run.default_report_id]

        campaigns_api_sync.archive(campaign)

    def test_add_runs_rejects_run_without_report(
        self, campaigns_api_sync, sift_client, new_campaign, test_timestamp_str
    ):
        """Test that add_runs fails clearly when a run has no report at all."""
        from sift_client.sift_types.run import RunCreate

        run = sift_client.runs.create(
            RunCreate(name=f"test_campaign_no_report_{test_timestamp_str}")
        )

        with pytest.raises(ValueError, match="no report"):
            campaigns_api_sync.add_runs(new_campaign, [run])

    def test_add_runs_falls_back_to_a_report_over_the_run(
        self, campaigns_api_sync, campaign_run, test_timestamp_str
    ):
        """Test that a run with no default report joins through a report search."""
        # Hide the default report so add_runs must search for one over the run.
        run = campaign_run.model_copy(update={"default_report_id": None})
        campaign = campaigns_api_sync.create(
            CampaignCreate(name=f"test_campaign_fallback_{test_timestamp_str}")
        )

        added = campaigns_api_sync.add_runs(campaign, [run])

        assert [r.report_id for r in added.report_summaries] == [campaign_run.default_report_id]
        campaigns_api_sync.archive(campaign)

    def test_create_from_runs(self, campaigns_api_sync, campaign_run, test_timestamp_str):
        """Test seeding a new campaign from a run."""
        campaign = campaigns_api_sync.create(
            CampaignCreate(name=f"test_campaign_from_runs_{test_timestamp_str}"),
            runs=[campaign_run],
        )

        assert [r.report_id for r in campaign.report_summaries] == [campaign_run.default_report_id]

        campaigns_api_sync.archive(campaign)

    def test_create_rejects_multiple_seeds(self, campaigns_api_sync, campaign_run):
        """Test that create rejects more than one seed."""
        with pytest.raises(ValueError, match="At most one of"):
            campaigns_api_sync.create(
                CampaignCreate(name="ignored"),
                runs=[campaign_run],
                reports=[campaign_run.default_report_id],
            )

    def test_report_summaries(self, campaigns_api_sync, campaign_run, test_timestamp_str):
        """Test fetching per-report rule counts for a campaign."""
        campaign = campaigns_api_sync.create(
            CampaignCreate(name=f"test_campaign_summaries_{test_timestamp_str}"),
            runs=[campaign_run],
        )

        summaries = campaigns_api_sync.report_summaries([campaign])

        assert campaign._id_or_error in summaries
        assert [r.report_id for r in summaries[campaign._id_or_error]] == [
            r.report_id for r in campaign.report_summaries
        ]

        campaigns_api_sync.archive(campaign)

    def test_archive_and_unarchive(self, campaigns_api_sync, test_timestamp_str):
        """Test archiving and unarchiving a campaign."""
        campaign = campaigns_api_sync.create(
            CampaignCreate(name=f"test_campaign_archive_{test_timestamp_str}")
        )

        archived = campaigns_api_sync.archive(campaign)
        assert archived.is_archived is True

        # Archived campaigns are excluded from list_ by default.
        assert campaigns_api_sync.find(name=campaign.name) is None
        assert (
            campaigns_api_sync.find(name=campaign.name, include_archived=True).id_ == campaign.id_
        )

        unarchived = campaigns_api_sync.unarchive(campaign)
        assert unarchived.is_archived is False

        campaigns_api_sync.archive(campaign)

    def test_instance_methods(self, campaigns_api_sync, test_timestamp_str):
        """Test the update and archive methods on the Campaign instance itself."""
        campaign = campaigns_api_sync.create(
            CampaignCreate(name=f"test_campaign_instance_{test_timestamp_str}")
        )

        campaign.update({"description": "set from the instance"})
        assert campaign.description == "set from the instance"

        campaign.archive()
        assert campaign.is_archived is True

        campaign.unarchive()
        assert campaign.is_archived is False

        campaign.archive()

    @pytest.mark.asyncio
    async def test_async_api(self, campaigns_api_async, new_campaign):
        """Test that the async API returns the same campaign."""
        fetched = await campaigns_api_async.get(new_campaign._id_or_error)

        assert fetched.id_ == new_campaign.id_

    def test_add_reports_reads_the_server_list(
        self, campaigns_api_sync, campaign_run, test_timestamp_str
    ):
        """A stale Campaign handle must not drop reports added since it was fetched."""
        campaign = campaigns_api_sync.create(
            CampaignCreate(name=f"test_campaign_stale_{test_timestamp_str}")
        )
        stale = campaign

        # Another caller adds a report that `stale` knows nothing about.
        campaigns_api_sync.add_reports(campaign._id_or_error, [campaign_run.default_report_id])
        assert [r.report_id for r in stale.report_summaries] == []

        merged = campaigns_api_sync.add_reports(stale, [campaign_run.default_report_id])

        assert [r.report_id for r in merged.report_summaries] == [campaign_run.default_report_id]

        campaigns_api_sync.archive(campaign)

    def test_campaign_runs(self, campaigns_api_sync, campaign_run, test_timestamp_str):
        """Test reading back the runs behind a campaign's reports."""
        campaign = campaigns_api_sync.create(
            CampaignCreate(name=f"test_campaign_runs_prop_{test_timestamp_str}"),
            runs=[campaign_run],
        )

        assert [r.id_ for r in campaign.runs] == [campaign_run.id_]

        campaigns_api_sync.archive(campaign)

    def test_campaign_report_summaries(self, campaigns_api_sync, campaign_run, test_timestamp_str):
        """Test the rollup from the Campaign instance."""
        campaign = campaigns_api_sync.create(
            CampaignCreate(name=f"test_campaign_rollup_{test_timestamp_str}"),
            runs=[campaign_run],
        )

        # The service returns these in no fixed order; _from_proto sorts them.
        ids = [r.report_id for r in campaign.report_summaries]
        assert ids == sorted(ids)
        assert ids == [
            r.report_id for r in campaigns_api_sync.get(campaign._id_or_error).report_summaries
        ]

        campaigns_api_sync.archive(campaign)

    def test_update_with_nothing_set_is_a_noop(self, campaigns_api_sync, new_campaign):
        """The service rejects an empty update mask, so the client must not send one."""
        unchanged = campaigns_api_sync.update(new_campaign, CampaignUpdate())

        assert unchanged.id_ == new_campaign.id_
        assert unchanged.name == new_campaign.name

    def test_report_summaries_with_no_campaigns(self, campaigns_api_sync):
        """An empty request must not reach the service, which rejects it."""
        assert campaigns_api_sync.report_summaries([]) == {}

    def test_report_summaries_includes_empty_campaigns(
        self, campaigns_api_sync, test_timestamp_str
    ):
        """The service omits campaigns with no reports; every ID asked for is returned."""
        campaign = campaigns_api_sync.create(
            CampaignCreate(name=f"test_campaign_empty_rollup_{test_timestamp_str}")
        )

        summaries = campaigns_api_sync.report_summaries([campaign])

        assert summaries[campaign._id_or_error] == []

        campaigns_api_sync.archive(campaign)
