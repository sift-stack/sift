from typing import List, Optional

from pydantic import BaseModel, Field, model_validator
from typing_extensions import Self

"""
These are the models that are used to validate the input data for the Sift Grafana Plugin queries
"""


class Channel(BaseModel):
    channelId: Optional[str] = None  # noqa: N815
    channelIdentifier: Optional[str] = None  # noqa: N815

    @model_validator(mode="after")
    def check_passwords_match(self) -> Self:
        channel_id = self.channelId
        channel_identifier = self.channelIdentifier
        if channel_id is None and channel_identifier is None:
            raise ValueError("channelId or channelIdentifier must be provided")
        return self


class Asset(BaseModel):
    assetId: Optional[str] = None  # noqa: N815
    assetName: Optional[str] = None  # noqa: N815

    @model_validator(mode="after")
    def check_passwords_match(self) -> Self:
        asset_id = self.assetId
        asset_name = self.assetName
        if asset_id is None and asset_name is None:
            raise ValueError("assetId or assetName must be provided")
        return self


class ChannelReference(Channel):
    # Enforce format "${number}"
    channelReference: str = Field(..., pattern=r"^\$\d+$")  # noqa: N815


class AssetChannelQuery(Asset, Channel):
    pass


class CalculatedChannelQuery(BaseModel):
    name: str
    asset: Asset
    channelReferences: List[ChannelReference]  # noqa: N815
    expression: str


class SiftQuery(BaseModel):
    queries: List[AssetChannelQuery]
    calculatedChannelQuery: Optional[CalculatedChannelQuery] = None  # noqa: N815
    groupByRun: bool  # noqa: N815

    @model_validator(mode="after")
    def check_passwords_match(self) -> Self:
        queries = self.queries
        calculated_channel_query = self.calculatedChannelQuery
        if len(queries) > 0 and calculated_channel_query is not None:
            raise ValueError("queries and calculatedChannelQuery cannot both be provided")
        return self
