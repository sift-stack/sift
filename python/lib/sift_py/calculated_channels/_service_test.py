from unittest import mock

import pytest
from google.protobuf.field_mask_pb2 import FieldMask
from sift.calculated_channels.v2.calculated_channels_pb2 import (
    CalculatedChannel,
    CalculatedChannelAbstractChannelReference,
    CalculatedChannelAssetConfiguration,
    CalculatedChannelConfiguration,
    CalculatedChannelQueryConfiguration,
    CreateCalculatedChannelRequest,
    CreateCalculatedChannelResponse,
    GetCalculatedChannelRequest,
    GetCalculatedChannelResponse,
    ListCalculatedChannelsRequest,
    ListCalculatedChannelsResponse,
    ListCalculatedChannelVersionsRequest,
    ListCalculatedChannelVersionsResponse,
    UpdateCalculatedChannelRequest,
    UpdateCalculatedChannelResponse,
)

from sift_py._internal.test_util.channel import MockChannel
from sift_py.calculated_channels.config import CalculatedChannelConfig, CalculatedChannelUpdate
from sift_py.calculated_channels.service import CalculatedChannelService


@pytest.fixture
def calculated_channel_service():
    return CalculatedChannelService(MockChannel())


channel_references = [
    {
        "channel_reference": "$x",
        "channel_identifier": "channel1",
    },
    {
        "channel_reference": "$y",
        "channel_identifier": "channel2",
    },
]


def test_get_calculated_channel_by_client_key(calculated_channel_service):
    client_key = "test-channel-key"

    with mock.patch.object(
        calculated_channel_service._calculated_channel_service_stub,
        "GetCalculatedChannel",
        return_value=GetCalculatedChannelResponse(
            calculated_channel=CalculatedChannel(
                name="test-channel",
                calculated_channel_configuration=CalculatedChannelConfiguration(
                    query_configuration=CalculatedChannelQueryConfiguration(
                        sel=CalculatedChannelQueryConfiguration.Sel(expression="$x + $y")
                    ),
                    asset_configuration=CalculatedChannelAssetConfiguration(all_assets=True),
                ),
            )
        ),
    ) as mock_get:
        calculated_channel_service.get_calculated_channel(client_key=client_key)
        mock_get.assert_called_once_with(
            GetCalculatedChannelRequest(
                client_key=client_key,
            )
        )


def test_get_calculated_channel_by_id(calculated_channel_service):
    channel_id = "test-channel-id"

    with mock.patch.object(
        calculated_channel_service._calculated_channel_service_stub,
        "GetCalculatedChannel",
        return_value=GetCalculatedChannelResponse(
            calculated_channel=CalculatedChannel(
                name="test",
                calculated_channel_configuration=CalculatedChannelConfiguration(
                    query_configuration=CalculatedChannelQueryConfiguration(
                        sel=CalculatedChannelQueryConfiguration.Sel(expression="$x + $y")
                    ),
                    asset_configuration=CalculatedChannelAssetConfiguration(all_assets=True),
                ),
            )
        ),
    ) as mock_get:
        calculated_channel_service.get_calculated_channel(calculated_channel_id=channel_id)
        mock_get.assert_called_once_with(
            GetCalculatedChannelRequest(
                calculated_channel_id=channel_id,
            )
        )


def test_get_calculated_channel_missing_params(calculated_channel_service):
    with pytest.raises(ValueError, match="Must provide either `id` or `client_key`"):
        calculated_channel_service.get_calculated_channel()


def test_list_calculated_channels(calculated_channel_service):
    with mock.patch.object(
        calculated_channel_service._calculated_channel_service_stub,
        "ListCalculatedChannels",
        return_value=ListCalculatedChannelsResponse(calculated_channels=[], next_page_token="next"),
    ) as mock_list:
        channels, token = calculated_channel_service.list_calculated_channels(
            page_size=10,
            page_token="token",
            filter="filter",
            order_by="name",
        )

        mock_list.assert_called_once_with(
            ListCalculatedChannelsRequest(
                page_size=10,
                page_token="token",
                filter="filter",
                order_by="name",
            )
        )
        assert token == "next"
        assert channels == []


def test_create_calculated_channel(calculated_channel_service):
    config = CalculatedChannelConfig(
        name="test-channel",
        description="test description",
        expression="$x + $y",
        channel_references=channel_references,
        units="meters",
        client_key="test-key",
        all_assets=True,
    )

    chan = dict(
        name="test-channel",
        description="test description",
        client_key="test-key",
        units="meters",
        calculated_channel_configuration=CalculatedChannelConfiguration(
            asset_configuration=CalculatedChannelAssetConfiguration(all_assets=True),
            query_configuration=CalculatedChannelQueryConfiguration(
                sel=CalculatedChannelQueryConfiguration.Sel(
                    expression="$x + $y",
                    expression_channel_references=[
                        CalculatedChannelAbstractChannelReference(
                            channel_reference=ch.get("channel_reference"),
                            channel_identifier=ch.get("channel_identifier"),
                        )
                        for ch in channel_references
                    ],
                )
            ),
        ),
    )

    with mock.patch.object(
        calculated_channel_service._calculated_channel_service_stub,
        "CreateCalculatedChannel",
        return_value=CreateCalculatedChannelResponse(
            calculated_channel=CalculatedChannel(**chan),
            inapplicable_assets=None,
        ),
    ) as mock_create:
        channel, validation = calculated_channel_service.create_calculated_channel(config)

        mock_create.assert_called_once_with(CreateCalculatedChannelRequest(**chan))


def test_revise_calculated_channel(calculated_channel_service):
    updates = CalculatedChannelUpdate(name="updated-name", description="updated description")

    chan_dict = dict(
        calculated_channel_id="test-id",
        name="test-channel",
        description="test description",
        calculated_channel_configuration=CalculatedChannelConfiguration(
            asset_configuration=CalculatedChannelAssetConfiguration(all_assets=True),
            query_configuration=CalculatedChannelQueryConfiguration(
                sel=CalculatedChannelQueryConfiguration.Sel(
                    expression="$x + $y",
                    expression_channel_references=[],
                )
            ),
        ),
        units="",
        archived_date={},
    )

    mock_channel = CalculatedChannel(**chan_dict)

    with mock.patch.object(
        calculated_channel_service, "_get_calculated_channel", return_value=mock_channel
    ) as _, mock.patch.object(
        calculated_channel_service._calculated_channel_service_stub,
        "UpdateCalculatedChannel",
        return_value=UpdateCalculatedChannelResponse(
            calculated_channel=mock_channel,
            inapplicable_assets=None,
        ),
    ) as mock_update:
        channel, validation = calculated_channel_service.update_calculated_channel(
            CalculatedChannelConfig(
                calculated_channel_id="test-id",
                name="test-channel",
                description="test description",
                expression="$x + $y * 2",
                channel_references=[],
                all_assets=True,
            ),
            updates,
            update_notes="test revision",
        )

        mock_update.assert_called_once_with(
            UpdateCalculatedChannelRequest(
                calculated_channel=CalculatedChannel(
                    **{
                        **chan_dict,
                        **dict(name=updates.get("name"), description=updates.get("description")),
                    }
                ),
                update_mask=FieldMask(paths=["name", "description"]),
                user_notes="test revision",
            )
        )


def test_list_calculated_channel_versions(calculated_channel_service):
    with mock.patch.object(
        calculated_channel_service._calculated_channel_service_stub,
        "ListCalculatedChannelVersions",
        return_value=ListCalculatedChannelVersionsResponse(
            calculated_channel_versions=[], next_page_token="next"
        ),
    ) as mock_list:
        versions, token = calculated_channel_service.list_calculated_channel_versions(
            calculated_channel_id="test-id",
            page_size=10,
            page_token="token",
            filter="filter",
            order_by="name",
        )

        mock_list.assert_called_once_with(
            ListCalculatedChannelVersionsRequest(
                calculated_channel_id="test-id",
                page_size=10,
                page_token="token",
                filter="filter",
                order_by="name",
            )
        )
        assert token == "next"
        assert versions == []


def test_list_calculated_channel_versions_missing_params(calculated_channel_service):
    with pytest.raises(ValueError, match="Must provide either `id` or `client_key`"):
        calculated_channel_service.list_calculated_channel_versions()
