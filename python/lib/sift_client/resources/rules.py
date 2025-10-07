from __future__ import annotations

from typing import TYPE_CHECKING

from sift_client._internal.low_level_wrappers.rules import RulesLowLevelClient
from sift_client.resources._base import ResourceBase
from sift_client.sift_types.rule import Rule, RuleAction, RuleUpdate
from sift_client.util import cel_utils as cel

if TYPE_CHECKING:
    import re

    from sift_client.client import SiftClient
    from sift_client.sift_types.channel import ChannelReference


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
        rule_ids: list[str] | None = None,
        client_keys: list[str] | None = None,
    ) -> Rule | list[Rule]:
        """Get a Rule.

        Args:
            rule_id: The ID of the rule.
            client_key: The client key of the rule.
            rule_ids: List of rule IDs to get.
            client_keys: List of client keys to get.

        Returns:
            The Rule or Rules.
        """
        if rule_id or client_key:
            rule = await self._low_level_client.get_rule(rule_id=rule_id, client_key=client_key)
            return self._apply_client_to_instance(rule)
        else:
            rules = await self._low_level_client.batch_get_rules(
                rule_ids=rule_ids, client_keys=client_keys
            )
            return self._apply_client_to_instances(rules)

    async def list_(
        self,
        *,
        name: str | None = None,
        name_contains: str | None = None,
        name_regex: str | re.Pattern | None = None,
        order_by: str | None = None,
        limit: int | None = None,
        include_deleted: bool = False,
    ) -> list[Rule]:
        """List rules with optional filtering.

        Args:
            name: Exact name of the rule.
            name_contains: Partial name of the rule.
            name_regex: Regular expression string to filter rules by name.
            order_by: How to order the retrieved rules.
            limit: How many rules to retrieve. If None, retrieves all matches.
            include_deleted: Include deleted rules.

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
        name: str,
        description: str,
        expression: str,
        channel_references: list[ChannelReference],
        action: RuleAction,
        organization_id: str | None = None,
        client_key: str | None = None,
        asset_ids: list[str] | None = None,
        contextual_channels: list[str] | None = None,
        is_external: bool = False,
    ) -> Rule:
        """Create a new rule."""
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
        """Update a Rule.

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
        client_key: str | None = None,
        rules: list[Rule] | None = None,
        rule_ids: list[str] | None = None,
        client_keys: list[str] | None = None,
    ) -> None:
        """Archive a rule or multiple.

        Args:
            rule: The Rule to archive.
            client_key: The client key or the Rule to archive.
            rules: The Rules to archive.
            rule_ids: The rule IDs to archive.
            client_keys: The client keys of the Rules tp archive.
        """
        if rule or client_key:
            rule_id = rule._id_or_error if isinstance(rule, Rule) else rule
            return await self._low_level_client.archive_rule(rule_id=rule_id, client_key=client_key)
        elif rules or rule_ids or client_keys:
            rule_ids = rule_ids or [rule._id_or_error for rule in rules]
            return await self._low_level_client.batch_archive_rules(
                rule_ids=rule_ids, client_keys=client_keys
            )
        else:
            raise ValueError("Either rule or rules must be provided")

        async def unarchive(
                self,
                *,
                rule: str | Rule | None = None,
                client_key: str | None = None,
                rules: list[Rule] | None = None,
                rule_ids: list[str] | None = None,
                client_keys: list[str] | None = None,
        ) -> None:
            """Unarchive a Rule or multiple.

            Args:
            rule: The Rule to restore.
            client_key: The client key of the Rule to restore.
            rules: The Rules to restore.
            rule_ids: The rule IDs to restore.
            client_keys: The client keys of the Rules to restore.
            """
        if rule or client_key:
            rule_id = rule._id_or_error if isinstance(rule, Rule) else rule
            return await self._low_level_client.unarchive_rule(rule_id=rule_id, client_key=client_key)
        elif rules or rule_ids or client_keys:
            rule_ids = rule_ids or [rule._id_or_error for rule in rules]
            return await self._low_level_client.batch_unarchive_rules(
                rule_ids=rule_ids, client_keys=client_keys
            )
        else:
            raise ValueError("Either rule or rules must be provided")



