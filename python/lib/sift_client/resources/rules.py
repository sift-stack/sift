from __future__ import annotations

import warnings
from typing import TYPE_CHECKING, Any, Sequence

from sift_client._internal.low_level_wrappers.rules import RulesLowLevelClient
from sift_client.errors import SiftWarning
from sift_client.resources._base import ResourceBase
from sift_client.sift_types.asset import Asset
from sift_client.sift_types.rule import Rule, RuleCreate, RuleUpdate
from sift_client.util import cel_utils as cel

if TYPE_CHECKING:
    import re
    from datetime import datetime

    from sift_client.client import SiftClient
    from sift_client.sift_types.tag import Tag


class RulesAPIAsync(ResourceBase):
    """High-level API for interacting with rules.

    This class provides a Pythonic, notebook-friendly interface for interacting with the RulesAPI.
    It handles automatic handling of gRPC services, seamless type conversion, and clear error handling.

    All methods in this class use the Rule class from the low-level wrapper, which is a user-friendly
    representation of a rule using standard Python data structures and types.
    """

    def __init__(self, sift_client: SiftClient):
        """Initialize the RulesAPI.

        Args:
            sift_client: The Sift client to use.
        """
        super().__init__(sift_client)
        self._low_level_client = RulesLowLevelClient(grpc_client=self.client.grpc_client)

    async def get(
        self,
        *,
        rule_id: str | None = None,
        client_key: str | None = None,
    ) -> Rule:
        """Get a Rule.

        Args:
            rule_id: The ID of the rule.
            client_key: The client key of the rule.

        Returns:
            The Rule.
        """
        rule = await self._low_level_client.get_rule(rule_id=rule_id, client_key=client_key)
        return self._apply_client_to_instance(rule)

    async def list_(
        self,
        *,
        name: str | None = None,
        names: list[str] | None = None,
        name_contains: str | None = None,
        name_regex: str | re.Pattern | None = None,
        # self ids
        rule_ids: list[str] | None = None,
        client_keys: list[str] | None = None,
        # created/modified ranges
        created_after: datetime | None = None,
        created_before: datetime | None = None,
        modified_after: datetime | None = None,
        modified_before: datetime | None = None,
        # created/modified users
        created_by: Any | str | None = None,
        modified_by: Any | str | None = None,
        # metadata
        metadata: list[Any] | None = None,
        # rule specific
        assets: list[str] | list[Asset] | None = None,
        asset_tags: list[str | Tag] | None = None,
        # common filters
        description_contains: str | None = None,
        include_archived: bool = False,
        filter_query: str | None = None,
        order_by: str | None = None,
        limit: int | None = None,
    ) -> list[Rule]:
        """List rules with optional filtering.

        Args:
            name: Exact name of the rule.
            names: List of rule names to filter by.
            name_contains: Partial name of the rule.
            name_regex: Regular expression string to filter rules by name.
            client_keys: Client keys of rules to filter to.
            rule_ids: IDs of rules to filter to.
            created_after: Rules created after this datetime.
            created_before: Rules created before this datetime.
            modified_after: Rules modified after this datetime.
            modified_before: Rules modified before this datetime.
            created_by: Filter rules created by this User or user ID.
            modified_by: Filter rules last modified by this User or user ID.
            metadata: Filter rules by metadata criteria.
            assets: Filter rules associated with any of these Assets.
            asset_tags: Filter rules associated with any Assets that have these Tag IDs.
            description_contains: Partial description of the rule.
            include_archived: If True, include archived rules in results.
            filter_query: Explicit CEL query to filter rules.
            order_by: Field and direction to order results by.
            limit: Maximum number of rules to return. If None, returns all matches.

        Returns:
            A list of Rules that matches the filter.
        """
        filter_parts = [
            *self._build_name_cel_filters(
                name=name,
                names=names,
                name_contains=name_contains,
                name_regex=name_regex,
            ),
            *self._build_time_cel_filters(
                created_after=created_after,
                created_before=created_before,
                modified_after=modified_after,
                modified_before=modified_before,
                created_by=created_by,
                modified_by=modified_by,
            ),
            *self._build_tags_metadata_cel_filters(tag_ids=asset_tags, metadata=metadata),
            *self._build_common_cel_filters(
                description_contains=description_contains,
                include_archived=include_archived,
                filter_query=filter_query,
            ),
        ]
        if rule_ids:
            filter_parts.append(cel.in_("rule_id", rule_ids))
        if client_keys:
            filter_parts.append(cel.in_("client_key", client_keys))
        if assets:
            ids = [a._id_or_error if isinstance(a, Asset) else a or "" for a in assets]
            filter_parts.append(cel.in_("asset_id", ids))
        query_filter = cel.and_(*filter_parts)
        rules = await self._low_level_client.list_all_rules(
            filter_query=query_filter,
            order_by=order_by,
            max_results=limit,
            page_size=limit,
        )
        return self._apply_client_to_instances(rules)

    async def find(self, **kwargs) -> Rule | None:
        """Find a single rule matching the given query. Takes the same arguments as `list`. If more than one rule is found,
        raises an error.

        Args:
            **kwargs: Keyword arguments to pass to `list`.

        Returns:
            The Rule found or None.
        """
        rules = await self.list_(**kwargs)
        if len(rules) > 1:
            raise ValueError("Multiple rules found for query")
        elif len(rules) == 1:
            return rules[0]
        return None

    async def create(
        self,
        create: RuleCreate | dict | Sequence[RuleCreate | dict],
        *,
        override_expression_validation: bool = True,
    ) -> Rule | list[Rule]:
        """Create a new rule.

        Args:
            create: A RuleCreate object, a dictionary with configuration for the new rule, or a list of the previously mentioned objects.
            override_expression_validation: When true, the rule will be created even if the expression is invalid.

        Warnings:
            SiftWarning: If not all rules are created.

        Returns:
            The created Rule (if a single dictionary or RuleCreate was provided) otherwise a list of the created rules.
        """
        rules: list[RuleCreate] = []
        if isinstance(create, Sequence):
            for c in create:
                if isinstance(c, dict):
                    rules.append(RuleCreate.model_validate(c))
                else:
                    rules.append(c)
        elif isinstance(create, dict):
            rules.append(RuleCreate.model_validate(create))
        else:
            rules.append(create)

        created_rules = await self.batch_update_or_create_rules(
            rules=rules, override_expression_validation=override_expression_validation
        )
        if len(created_rules) != len(rules):
            warnings.warn(
                f"Failed to create all rules: got {len(created_rules)} but expected {len(rules)}",
                SiftWarning,
                stacklevel=2,
            )

        # If there is only one rule to create provided as a dict or RuleCreate, return the single rule.
        if len(created_rules) == 1 and not isinstance(create, Sequence):
            return created_rules[0]

        # Otherwise, return the list of created rules.
        return created_rules

    async def update(
        self,
        rule: Rule | str,
        update: RuleUpdate | dict,
        *,
        version_notes: str | None = None,
    ) -> Rule:
        """Update a Rule.

        Args:
            rule: The Rule or rule ID to update.
            update: Updates to apply to the Rule.
            version_notes: Notes to include in the rule version.

        Returns:
            The updated Rule.
        """
        rule_obj: Rule
        if isinstance(rule, str):
            rule_obj = await self.get(rule_id=rule)
        else:
            rule_obj = rule

        if isinstance(update, dict):
            update = RuleUpdate.model_validate(update)

        updated_rule = await self._low_level_client.update_rule(
            rule=rule_obj, update=update, version_notes=version_notes
        )
        return self._apply_client_to_instance(updated_rule)

    async def archive(self, rule: str | Rule) -> Rule:
        """Archive a rule.

        Args:
            rule: The id or Rule object of the rule to archive.

        Returns:
            The archived Rule.
        """
        return await self.update(rule=rule, update=RuleUpdate(is_archived=True))

    async def unarchive(self, rule: str | Rule) -> Rule:
        """Unarchive a rule.

        Args:
            rule: The id or Rule object of the rule to unarchive.

        Returns:
            The unarchived Rule.
        """
        return await self.update(rule=rule, update=RuleUpdate(is_archived=False))

    async def batch_update_or_create_rules(
        self,
        rules: Sequence[RuleCreate | RuleUpdate],
        *,
        override_expression_validation: bool = False,
    ) -> list[Rule]:
        """Batch update or create multiple rules.

        Args:
            rules: List of rule creates or updates to apply. RuleUpdate objects must have resource_id set.
            override_expression_validation: When true, the rules will be created even if the expressions are invalid.

        Warnings:
            SiftWarning: If not all rules are created or updated.

        Returns:
            List of updated or created Rules.

        Raises:
            ValueError: If the update/create fails or if not all rules were updated/created.
        """
        # If there are no rules to update/create, return an empty list immediately
        # to avoid unnecessary RPC calls.
        if not rules:
            return []

        rule_ids: list[str | None] = []
        for rule in rules:
            if isinstance(rule, RuleUpdate):
                rule_ids.append(rule.resource_id)
            else:
                rule_ids.append(None)

        # Update/create the rules.
        response = await self._low_level_client.batch_update_rules(
            rules=rules, override_expression_validation=override_expression_validation
        )

        if not response.success:
            raise ValueError(f"Failed to update/create rules {response.validation_results}")

        # Ensure all rules were updated/created.
        if response.rules_created_count + response.rules_updated_count != len(rules):
            warnings.warn(
                f"Not all rules were updated/created: got {response.rules_created_count + response.rules_updated_count} but expected {len(rules)}",
                SiftWarning,
                stacklevel=2,
            )

        # Collect rule IDs from the response
        final_rule_ids: list[str] = []
        for rule_id in rule_ids:
            if rule_id is not None:
                # RuleUpdate: use the existing resource_id
                final_rule_ids.append(rule_id)
            else:
                final_rule_ids.append(response.created_rule_identifiers.pop(0).rule_id)

        # Fetch the rules.
        updated_rules = await self._low_level_client.batch_get_rules(rule_ids=final_rule_ids)
        return self._apply_client_to_instances(updated_rules)
