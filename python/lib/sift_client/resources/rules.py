from __future__ import annotations

import re
from typing import TYPE_CHECKING, List

from sift_client._internal.low_level_wrappers.rules import RulesLowLevelClient
from sift_client.resources._base import ResourceBase
from sift_client.sift_types.channel import ChannelReference
from sift_client.sift_types.rule import Rule, RuleAction, RuleUpdate
from sift_client.util import cel_utils as cel

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

    async def list(
        self,
        *,
        name: str | None = None,
        name_contains: str | None = None,
        name_regex: str | re.Pattern | None = None,
        order_by: str | None = None,
        limit: int | None = None,
        include_deleted: bool = False,
    ) -> list[Rule]:
        """
        List rules with optional filtering.

        Args:
            name: Exact name of the rule.
            name_contains: Partial name of the rule.
            name_regex: Regular expression string to filter rules by name.
            order_by: How to order the retrieved rules.
            limit: How many rules to retrieve. If None, retrieves all matches.

        Returns:
            A list of Rules that matches the filter.
        """
        if int(name is not None) + int(name_contains is not None) + int(name_regex is not None) > 1:
            raise ValueError("Must use EITHER name, name_contains, or name_regex, not multiple")

        filters = []
        if name:
            filters.append(cel.equals("name", name))
        if name_contains:
            filters.append(cel.contains("name", name_contains))
        if name_regex:
            filters.append(cel.match("name", name_regex))
        if not include_deleted:
            filters.append(cel.equals_null("deleted_date"))
        filter_str = " && ".join(filters) if filters else ""
        rules = await self._low_level_client.list_all_rules(
            filter_query=filter_str,
            order_by=order_by,
            max_results=limit,
            page_size=limit,
        )
        return self._apply_client_to_instances(rules)

    async def find(self, **kwargs) -> Rule | None:
        """
        Find a single rule matching the given query. Takes the same arguments as `list`. If more than one rule is found,
        raises an error.

        Args:
            **kwargs: Keyword arguments to pass to `list`.

        Returns:
            The Rule found or None.
        """
        rules = await self.list(**kwargs)
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
        channel_references: List[ChannelReference],
        action: RuleAction,
        organization_id: str | None = None,
        client_key: str | None = None,
        asset_ids: List[str] | None = None,
        contextual_channels: List[str] | None = None,
        is_external: bool = False,
    ) -> Rule:
        """
        Create a new rule.
        """
        created_rule = await self._low_level_client.create_rule(
            name=name,
            description=description,
            organization_id=organization_id,
            expression=expression,
            action=action,
            channel_references=channel_references,
            client_key=client_key,
            asset_ids=asset_ids,
            contextual_channels=contextual_channels,
            is_external=is_external,
        )
        return self._apply_client_to_instance(created_rule)

    async def update(
        self, rule: str | Rule, update: RuleUpdate | dict, version_notes: str | None = None
    ) -> Rule:
        """
        Update a Rule.

        Args:
            rule: The Rule or rule ID to update.
            update: Updates to apply to the Rule.
            version_notes: Notes to include in the rule version.
        Returns:
            The updated Rule.
        """
        if isinstance(rule, str):
            rule = await self.get(rule_id=rule)

        if isinstance(update, dict):
            update = RuleUpdate.model_validate(update)

        updated_rule = await self._low_level_client.update_rule(rule, update, version_notes)
        return self._apply_client_to_instance(updated_rule)

    async def archive(
        self,
        *,
        rule: str | Rule | None = None,
        rules: List[Rule] | None = None,
        rule_ids: List[str] | None = None,
        client_keys: List[str] | None = None,
    ) -> None:
        """
        Archive a rule or multiple.

        Args:
            rule: The Rule to archive.
            rules: The Rules to archive.
            rule_ids: The rule IDs to archive.
            client_keys: The client keys to archive.
        """
        if rule:
            if isinstance(rule, Rule):
                await self._low_level_client.archive_rule(rule_id=rule.id_)
            else:
                await self._low_level_client.archive_rule(rule_id=rule)
        elif rules:
            if len(rules) == 1:
                await self._low_level_client.archive_rule(rule_id=rules[0].id_)
            else:
                await self._low_level_client.batch_archive_rules(
                    rule_ids=[r.id_ for r in rules],  # type: ignore
                )
        elif rule_ids:
            if len(rule_ids) == 1:
                await self._low_level_client.archive_rule(rule_id=rule_ids[0])
            else:
                await self._low_level_client.batch_archive_rules(rule_ids=rule_ids)
        elif client_keys:
            await self._low_level_client.batch_archive_rules(client_keys=client_keys)
        else:
            raise ValueError("Either rules, rule_ids, or client_keys must be provided")

    async def restore(
        self,
        *,
        rule: str | Rule,
        rule_id: str | None = None,
        client_key: str | None = None,
    ) -> Rule:
        """
        Restore a rule.

        Args:
            rule: The Rule or rule ID to restore.
            rule_id: The rule ID to restore (alternative to rule parameter).
            client_key: The client key to restore (alternative to rule parameter).

        Returns:
            The restored Rule.
        """
        if rule_id or client_key:
            restored_rule = await self._low_level_client.restore_rule(
                rule_id=rule_id, client_key=client_key
            )
        else:
            rule_id = rule.id_ if isinstance(rule, Rule) else rule
            restored_rule = await self._low_level_client.restore_rule(rule_id=rule_id)

        return self._apply_client_to_instance(restored_rule)

    async def batch_restore(
        self,
        *,
        rule_ids: List[str] | None = None,
        client_keys: List[str] | None = None,
    ) -> None:
        """
        Batch restore rules.

        Args:
            rule_ids: List of rule IDs to restore.
            client_keys: List of client keys to undelete.
        """
        await self._low_level_client.batch_restore_rules(rule_ids=rule_ids, client_keys=client_keys)

    async def batch_get(
        self,
        *,
        rule_ids: List[str] | None = None,
        client_keys: List[str] | None = None,
    ) -> List[Rule]:
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
