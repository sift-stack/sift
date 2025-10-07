from __future__ import annotations

from typing import TYPE_CHECKING

from sift_client._internal.low_level_wrappers.rules import RulesLowLevelClient
from sift_client.resources._base import ResourceBase
from sift_client.sift_types.rule import Rule, RuleCreate, RuleUpdate
from sift_client.util import cel_utils as cel

if TYPE_CHECKING:
    import re

    from sift_client.client import SiftClient


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
        create: RuleCreate | dict,
    ) -> Rule:
        """Create a new rule.

        Args:
            create: A RuleCreate object or dictionary with configuration for the new rule.

        Returns:
            The created Rule.
        """
        if isinstance(create, dict):
            create = RuleCreate.model_validate(create)

        created_rule = await self._low_level_client.create_rule(create=create)
        return self._apply_client_to_instance(created_rule)

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
        if isinstance(rule, str):
            rule = await self.get(rule_id=rule)

        if isinstance(update, dict):
            update = RuleUpdate.model_validate(update)

        updated_rule = await self._low_level_client.update_rule(rule, update, version_notes)
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



