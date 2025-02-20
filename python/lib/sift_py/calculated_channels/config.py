from __future__ import annotations

from typing import List, Optional, Union

from pydantic import BaseModel, ConfigDict, field_validator, model_validator
from typing_extensions import NotRequired, TypedDict

from sift_py.rule.config import (
    ExpressionChannelReference,
    ExpressionChannelReferenceChannelConfig,
    _channel_references_from_dicts,
)


class CalculatedChannelConfig(BaseModel):
    """
    Configuration for a calculated channel.

    - `name`: Name of the calculated channel.
    - `description`: Description of the calculated channel.
    - `expression`: The expression to calculate the value of the calculated channel.
    - `channel_references`: A list of channel references that are used in the expression. Each reference can either
      be an `ExpressionChannelReference` or `ExpressionChannelReferenceChannelConfig`.
    - `units`: Units of the calculated channel.
    - `client_key`: A user defined string that uniquely identifies the calculated channel. Name is unique, but may be changed.
    - `asset_names`: A list of asset names to make the calculation available for.
    - `tags`: A list of tags on assets to make the calculation available for. NOTE: Not yet supported.
    - `all_assets`: A flag that, when set to `True`, associates the calculated channel with all assets.
    """

    model_config = ConfigDict(arbitrary_types_allowed=True)

    name: str
    description: str = ""
    expression: str
    channel_references: List[
        Union[ExpressionChannelReference, ExpressionChannelReferenceChannelConfig]
    ]
    units: Optional[str] = None
    calculated_channel_id: Optional[str] = None
    client_key: Optional[str] = None
    asset_names: Optional[List[str]] = None
    tag_names: Optional[List[str]] = None
    all_assets: bool = False

    @field_validator("tag_names", mode="before")
    @classmethod
    def check_for_unsupported_tags(cls, v):
        if v:
            raise ValueError("`tag_names` is not yet supported.")
        return v

    @field_validator("channel_references", mode="before")
    @classmethod
    def convert_channel_references(
        cls, raw: List[Union[ExpressionChannelReference, ExpressionChannelReferenceChannelConfig]]
    ) -> List[ExpressionChannelReference]:
        if not isinstance(raw, list):
            raise ValueError("`channel_references` must be a list.")

        for ref in raw:
            if not isinstance(ref, dict) or (
                "channel_identifier" not in ref and "channel_config" not in ref
            ):
                raise ValueError(
                    "`channel_references` must be a list of `ExpressionChannelReference` or `ExpressionChannelReferenceChannelConfig`."
                )

        return _channel_references_from_dicts(raw)

    @model_validator(mode="after")
    def validate_assets(self):
        if not self.asset_names and not self.tag_names and not self.all_assets:
            raise ValueError(
                "At least one of `asset_names`, `tag_names` must be specified or `all_assets` must be set to `True`."
            )
        if self.all_assets and (self.asset_names or self.tag_names):
            raise ValueError(
                "`all_assets` cannot be `True` if `asset_names` or `tag_names` are specified."
            )
        return self


class CalculatedChannelUpdate(TypedDict):
    """
    Represents a dictionary for updating properties of a calculated channel. All fields are optional
    and only the provided fields will be updated.

    - `name`: Updated name of the calculated channel.
    - `description`: Updated description of the calculated channel.
    - `units`: String representing the units for the calculated channel.
    - `expression`: Updated expression used to calculate channel values.
    - `channel_references`: A list of channel references which can either be `ExpressionChannelReference`
       or `ExpressionChannelReferenceChannelConfig` used in the expression.
    - `asset_names`: List of assets associated with the calculation.
    - `tags`: List of tags for associating the calculated channel to assets.
    - `all_assets`: Boolean flag indicating if the calculated channel applies to all assets.
    - `archived`: Boolean flag indicating if the calculated channel is archived.
    """

    name: NotRequired[str]
    description: NotRequired[str]
    units: NotRequired[str]
    expression: NotRequired[str]
    channel_references: NotRequired[
        List[Union[ExpressionChannelReference, ExpressionChannelReferenceChannelConfig]]
    ]
    asset_names: NotRequired[List[str]]
    tag_names: NotRequired[List[str]]
    all_assets: NotRequired[bool]
    archived: NotRequired[bool]
