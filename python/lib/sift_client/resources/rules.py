from __future__ import annotations

import re
from datetime import datetime
from typing import TYPE_CHECKING

from sift_client._internal.low_level_wrappers.rules import RulesLowLevelClient
from sift_client.resources._base import ResourceBase
from sift_client.types.rule import Rule, RuleUpdate
from sift_client.util import cel_utils

if TYPE_CHECKING:
    from sift_client.client import SiftClient


class RulesAPIAsync(ResourceBase):
    """
    High-level API for interacting with rules.

    This class provides a Pythonic, notebook-friendly interface for interacting with the RulesAPI.
    It handles automatic handling of gRPC services, seamless type conversion, and clear error handling.

    All methods in this class use the Rule class from the low-level wrapper, which is a user-friendly
    representation of a rule using standard Python data structures and types.
    """

    def __init__(self, sift_client: "SiftClient"):
        """
        Initialize the RulesAPI.

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
        """
        Get a Rule.

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
        name_contains: str | None = None,
        name_regex: str | re.Pattern | None = None,
        created_after: datetime | None = None,
        created_before: datetime | None = None,
        modified_after: datetime | None = None,
        modified_before: datetime | None = None,
        asset_ids: list[str] | None = None,
        include_deleted: bool = False,
        filter_query: str | None = None,
        order_by: str | None = None,
        limit: int | None = None,
    ) -> list[Rule]:
        """
        List rules with optional filtering.

        Args:
            name: Exact name of the rule.
            name_contains: Partial name of the rule.
            name_regex: Regular expression string to filter rules by name.
            created_after: Created after this date.
            created_before: Created before this date.
            modified_after: Modified after this date.
            modified_before: Modified before this date.
            asset_ids: Rules associated with these assets.
            include_deleted: Include deleted rules.
            filter_query: Explicit CEL query to filter rules.
            order_by: How to order the retrieved rules.
            limit: How many rules to retrieve. If None, retrieves all matches.

        Returns:
            A list of Rules that matches the filter.
        """
        if not filter_query:
            filters = []
            if name:
                filters.append(cel_utils.equals("name", name))
            if name_contains:
                filters.append(cel_utils.contains("name", name_contains))
            if name_regex:
                filters.append(cel_utils.match("name", name_regex))
            if created_after:
                filters.append(cel_utils.greater_than("created_date", created_after))
            if created_before:
                filters.append(cel_utils.less_than("created_date", created_before))
            if modified_after:
                filters.append(cel_utils.greater_than("modified_date", modified_after))
            if modified_before:
                filters.append(cel_utils.less_than("modified_date", modified_before))
            if asset_ids:
                filters.append(cel_utils.in_("asset_configuration.asset_ids", asset_ids))
            if not include_deleted:
                filters.append(cel_utils.equals_null("deleted_date"))
            filter_query = cel_utils.and_(*filters)

        rules, _ = await self._low_level_client.search_rules(
            name_matches=name,
            case_sensitive=False,
            regexp=bool(name_regex),
            order_by=order_by,
            asset_ids=asset_ids,
            include_deleted=include_deleted,
            limit=limit,
            offset=0,
        )
        return self._apply_client_to_instances(rules)

    async def find(self, **kwargs) -> Rule | None:
        """
        Find a single rule matching the given query. Takes the same arguments as `list_`. If more than one rule is found,
        raises an error.

        Args:
            **kwargs: Keyword arguments to pass to `list_`.

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
        name: str,
        description: str,
        expression: str,
        action: RuleAction,
        channel_references: list[ExpressionChannelReference],
        rule_client_key: str,
        asset_names: list[str],
        contextual_channels: list[str],
        is_external: bool,
    ) -> Rule:
        """
        Create a new rule.
        """
        rule = Rule(
            name=name,
            description=description,
            expression=expression,
            action=action,
            channel_references=channel_references,
            rule_client_key=rule_client_key,
            asset_names=asset_names,
            contextual_channels=contextual_channels,
            is_external=is_external,
        )
        return await self.create_from_model(rule)

    async def create_from_model(self, rule: Rule) -> str:
        """
        Create a new rule.

        Args:
            rule: The rule to create.

        Returns:
            The rule ID of the created rule.
        """
        return await self._low_level_client.create_rule(rule)

    async def update(self, rule: str | Rule, update: RuleUpdate | dict) -> Rule:
        """
        Update a Rule.

        Args:
            rule: The Rule or rule ID to update.
            update: Updates to apply to the Rule.

        Returns:
            The updated Rule.
        """
        rule_id = rule.rule_id if isinstance(rule, Rule) else rule
        rule_obj = await self.get(rule_id=rule_id)

        if isinstance(update, dict):
            update = RuleUpdate.model_validate(update)

        updated_rule = await self._low_level_client.update_rule(rule_obj, update)
        return self._apply_client_to_instance(updated_rule)

    async def delete(
        self,
        *,
        rule: str | Rule | None = None,
        rules: list[Rule] | None = None,
        rule_ids: list[str] | None = None,
        client_keys: list[str] | None = None,
    ) -> None:
        """
        Delete a rule or multiple.

        Args:
            rule: The Rule to delete.
            rules: The Rules to delete.
            rule_ids: The rule IDs to delete.
            client_keys: The client keys to delete.
        """
        if rule:
            if isinstance(rule, Rule):
                await self._low_level_client.delete_rule(rule_id=rule.rule_id)
            else:
                await self._low_level_client.delete_rule(rule_id=rule)
        elif rules:
            if len(rules) == 1:
                await self._low_level_client.delete_rule(rule_id=rules[0].rule_id)
            else:
                await self._low_level_client.batch_delete_rules(rule_ids=[r.rule_id for r in rules])
        elif rule_ids:
            if len(rule_ids) == 1:
                await self._low_level_client.delete_rule(rule_id=rule_ids[0])
            else:
                await self._low_level_client.batch_delete_rules(rule_ids=rule_ids)
        elif client_keys:
            await self._low_level_client.batch_delete_rules(client_keys=client_keys)
        else:
            raise ValueError("Either rules, rule_ids, or client_keys must be provided")

    async def undelete(
        self,
        rule: str | Rule,
        *,
        rule_id: str | None = None,
        client_key: str | None = None,
    ) -> Rule:
        """
        Undelete a rule.

        Args:
            rule: The Rule or rule ID to undelete.
            rule_id: The rule ID to undelete (alternative to rule parameter).
            client_key: The client key to undelete (alternative to rule parameter).

        Returns:
            The undeleted Rule.
        """
        if rule_id or client_key:
            undeleted_rule = await self._low_level_client.undelete_rule(
                rule_id=rule_id, client_key=client_key
            )
        else:
            rule_id = rule.rule_id if isinstance(rule, Rule) else rule
            undeleted_rule = await self._low_level_client.undelete_rule(rule_id=rule_id)

        return self._apply_client_to_instance(undeleted_rule)

    async def batch_undelete(
        self,
        *,
        rule_ids: list[str] | None = None,
        client_keys: list[str] | None = None,
    ) -> None:
        """
        Batch undelete rules.

        Args:
            rule_ids: List of rule IDs to undelete.
            client_keys: List of client keys to undelete.
        """
        await self._low_level_client.batch_undelete_rules(
            rule_ids=rule_ids, client_keys=client_keys
        )

    async def batch_get(
        self,
        *,
        rule_ids: list[str] | None = None,
        client_keys: list[str] | None = None,
    ) -> list[Rule]:
        """
        Get multiple rules by rule IDs or client keys.

        Args:
            rule_ids: List of rule IDs to get.
            client_keys: List of client keys to get.

        Returns:
            List of Rules.
        """
        rules = await self._low_level_client.batch_get_rules(
            rule_ids=rule_ids, client_keys=client_keys
        )
        return self._apply_client_to_instances(rules)

    async def search(
        self,
        *,
        name_matches: str | None = None,
        case_sensitive: bool = False,
        regexp: bool = False,
        order_by: str | None = None,
        rule_ids: list[str] | None = None,
        asset_ids: list[str] | None = None,
        include_deleted: bool = False,
        limit: int | None = None,
        offset: int = 0,
    ) -> tuple[list[Rule], int]:
        """
        Search for rules.

        Args:
            name_matches: Name pattern to match.
            case_sensitive: Whether the search is case sensitive.
            regexp: Whether to use regex matching.
            order_by: Field to order by.
            rule_ids: List of rule IDs to filter by.
            asset_ids: List of asset IDs to filter by.
            include_deleted: Whether to include deleted rules.
            limit: Maximum number of results to return.
            offset: Number of results to skip.

        Returns:
            Tuple of (list of Rules, total count).
        """
        rules, count = await self._low_level_client.search_rules(
            name_matches=name_matches,
            case_sensitive=case_sensitive,
            regexp=regexp,
            order_by=order_by,
            rule_ids=rule_ids,
            asset_ids=asset_ids,
            include_deleted=include_deleted,
            limit=limit,
            offset=offset,
        )
        return self._apply_client_to_instances(rules), count
