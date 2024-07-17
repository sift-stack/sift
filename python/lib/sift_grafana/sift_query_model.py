from typing import Any, List, Optional
from typing_extensions import Self
from pydantic import BaseModel, Field,  model_validator

# These are the models that are used to validate the input data for the Sift Grafana Plugin queries
class Channel(BaseModel):
    channelId: Optional[str] = None
    channelIdentifier: Optional[str] = None
    
    @model_validator(mode='after')
    def check_passwords_match(self) -> Self:
        channelId = self.channelId
        channelIdentifier = self.channelIdentifier
        if channelId is None and channelIdentifier is None:
            raise ValueError('channelId or channelIdentifier must be provided')
        return self
    
class Asset(BaseModel):
    assetId: Optional[str] = None
    assetName: Optional[str] = None

    @model_validator(mode='after')
    def check_passwords_match(self) -> Self:
        assetId = self.assetId
        assetName = self.assetName
        if assetId is None and assetName is None:
            raise ValueError('assetId or assetName must be provided')
        return self

class ChannelReference(Channel):    
    channelReference: str = Field(..., pattern=r'^\$\d+$')  # Enforce format "${number}"

class AssetChannelQuery(Asset, Channel):
    pass

class CalculatedChannelQuery(BaseModel):
    name: str
    asset: Asset
    channelReferences: List[ChannelReference]
    expression: str

class SiftQuery(BaseModel):
    queries: List[AssetChannelQuery]
    calculatedChannelQuery: Optional[CalculatedChannelQuery] = None
    groupByRun: bool

    @model_validator(mode='after')
    def check_passwords_match(self) -> Self:
        queries = self.queries
        calculatedChannelQuery = self.calculatedChannelQuery
        if len(queries) > 0 and calculatedChannelQuery is not None:
            raise ValueError('queries and calculatedChannelQuery cannot both be provided')
        return self
    