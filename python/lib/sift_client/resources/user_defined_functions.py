from __future__ import annotations

from typing import TYPE_CHECKING

from sift_client._internal.low_level_wrappers.user_defined_functions import (
    UserDefinedFunctionsLowLevelClient,
)
from sift_client.resources._base import ResourceBase
from sift_client.sift_types.user_defined_function import (
    FunctionInput,
    FunctionUsage,
    UserDefinedFunction,
    UserDefinedFunctionCreate,
    UserDefinedFunctionUpdate,
    UserDefinedFunctionValidation,
    UserDefinedFunctionVersion,
)
from sift_client.util import cel_utils as cel

if TYPE_CHECKING:
    import re

    from sift_client.client import SiftClient


class UserDefinedFunctionVersionsAPIAsync(ResourceBase):
    """High-level API for a function's version history.

    Every save produces a new version. Reachable as
    `client.user_defined_functions.versions`.
    """

    def __init__(self, sift_client: SiftClient):
        """Initialize the UserDefinedFunctionVersionsAPI.

        Args:
            sift_client: The Sift client to use.
        """
        super().__init__(sift_client)
        self._low_level_client = UserDefinedFunctionsLowLevelClient(
            grpc_client=self.client.grpc_client
        )

    async def get(self, *, version: str | UserDefinedFunctionVersion) -> UserDefinedFunctionVersion:
        """Get one version.

        Args:
            version: The UserDefinedFunctionVersion or version ID.

        Returns:
            The UserDefinedFunctionVersion.
        """
        version_id = (
            version._id_or_error if isinstance(version, UserDefinedFunctionVersion) else version
        )
        version = await self._low_level_client.get_version(version_id=version_id)
        return self._apply_client_to_instance(version)

    async def batch_get(
        self, *, versions: list[str] | list[UserDefinedFunctionVersion]
    ) -> list[UserDefinedFunctionVersion]:
        """Get many versions in one call.

        Args:
            versions: The UserDefinedFunctionVersions or version IDs.

        Returns:
            The UserDefinedFunctionVersions.
        """
        ids = [v._id_or_error if isinstance(v, UserDefinedFunctionVersion) else v for v in versions]
        found = await self._low_level_client.get_versions(version_ids=ids)
        return self._apply_client_to_instances(found)

    async def list_(
        self,
        *,
        user_defined_function: str | UserDefinedFunction | None = None,
        name: str | None = None,
        # version specific
        version: int | None = None,
        # common filters
        include_archived: bool = False,
        filter_query: str | None = None,
        order_by: str | None = None,
        limit: int | None = None,
        page_size: int | None = None,
    ) -> list[UserDefinedFunctionVersion]:
        """List a function's versions.

        Args:
            user_defined_function: The UserDefinedFunction or function ID whose versions to list.
            name: The function name, as an alternative to `user_defined_function`.
            version: Filter to a single version number.
            include_archived: If True, include archived versions in results.
            filter_query: Explicit CEL query to filter versions.
            order_by: Field and direction to order results by.
            limit: Maximum number of versions to return. If None, returns all matches.
            page_size: Number of results to fetch per request.

        Returns:
            A list of UserDefinedFunctionVersion objects in the service's order, which is
            by name unless `order_by` says otherwise. Every version shares the function's
            name, so pass `order_by="version desc"` for newest first.
        """
        filter_parts = self._build_common_cel_filters(
            include_archived=include_archived, filter_query=filter_query
        )
        if version is not None:
            filter_parts.append(cel.equals("version", version))
        query_filter = cel.and_(*filter_parts)

        versions = await self._low_level_client.list_all_versions(
            function_id=(
                user_defined_function._id_or_error
                if isinstance(user_defined_function, UserDefinedFunction)
                else user_defined_function
            ),
            name=name,
            query_filter=query_filter or None,
            order_by=order_by,
            max_results=limit,
            **({"page_size": page_size} if page_size is not None else {}),
        )
        return self._apply_client_to_instances(versions)


class UserDefinedFunctionsAPIAsync(ResourceBase):
    """High-level API for interacting with user-defined functions.

    A user-defined function is a named expression that calculated channels and rules
    can call. Every save produces a new version.
    """

    def __init__(self, sift_client: SiftClient):
        """Initialize the UserDefinedFunctionsAPI.

        Args:
            sift_client: The Sift client to use.
        """
        super().__init__(sift_client)
        self._low_level_client = UserDefinedFunctionsLowLevelClient(
            grpc_client=self.client.grpc_client
        )
        self.versions = UserDefinedFunctionVersionsAPIAsync(sift_client)

    async def get(self, *, user_defined_function_id: str) -> UserDefinedFunction:
        """Get a UserDefinedFunction.

        Args:
            user_defined_function_id: The ID of the function.

        Returns:
            The UserDefinedFunction.
        """
        function = await self._low_level_client.get_function(function_id=user_defined_function_id)
        return self._apply_client_to_instance(function)

    async def list_(
        self,
        *,
        name: str | None = None,
        name_contains: str | None = None,
        name_regex: str | re.Pattern | None = None,
        # self ids
        user_defined_function_ids: list[str] | list[UserDefinedFunction] | None = None,
        # common filters
        include_archived: bool = False,
        filter_query: str | None = None,
        order_by: str | None = None,
        limit: int | None = None,
        page_size: int | None = None,
    ) -> list[UserDefinedFunction]:
        """List user-defined functions.

        Args:
            name: Exact name of the function.
            name_contains: Partial name of the function.
            name_regex: Regular expression to filter functions by name.
            user_defined_function_ids: Filter to these UserDefinedFunctions or function IDs.
            include_archived: If True, include archived functions in results.
            filter_query: Explicit CEL query to filter functions.
            order_by: Field and direction to order results by.
            limit: Maximum number of functions to return. If None, returns all matches.
            page_size: Number of results to fetch per request.

        Returns:
            A list of UserDefinedFunction objects that match the filter criteria.
        """
        filter_parts = [
            *self._build_name_cel_filters(
                name=name, name_contains=name_contains, name_regex=name_regex
            ),
            *self._build_common_cel_filters(
                include_archived=include_archived, filter_query=filter_query
            ),
        ]
        if user_defined_function_ids:
            ids = [
                f._id_or_error if isinstance(f, UserDefinedFunction) else f
                for f in user_defined_function_ids
            ]
            filter_parts.append(cel.in_("user_defined_function_id", ids))
        query_filter = cel.and_(*filter_parts)

        functions = await self._low_level_client.list_all_functions(
            query_filter=query_filter or None,
            order_by=order_by,
            max_results=limit,
            **({"page_size": page_size} if page_size is not None else {}),
        )
        return self._apply_client_to_instances(functions)

    async def find(self, **kwargs) -> UserDefinedFunction | None:
        """Find one function. Takes the same arguments as `list_`.

        Raises if more than one matches.

        Args:
            **kwargs: Keyword arguments to pass to `list_`.

        Returns:
            The UserDefinedFunction found or None.
        """
        functions = await self.list_(**kwargs)
        if len(functions) > 1:
            raise ValueError(f"Multiple ({len(functions)}) functions found for query")
        elif len(functions) == 1:
            return functions[0]
        return None

    async def create(self, create: UserDefinedFunctionCreate | dict) -> UserDefinedFunction:
        """Create a new function.

        Args:
            create: The function definition.

        Returns:
            The created UserDefinedFunction.
        """
        if isinstance(create, dict):
            create = UserDefinedFunctionCreate.model_validate(create)
        created = await self._low_level_client.create_function(create=create)
        return self._apply_client_to_instance(created)

    async def update(
        self,
        user_defined_function: str | UserDefinedFunction,
        update: UserDefinedFunctionUpdate | dict,
        change_notes: str | None = None,
    ) -> UserDefinedFunction:
        """Update a function.

        Changes to the expression, inputs, description, or metadata create a new version.

        Args:
            user_defined_function: The UserDefinedFunction or function ID to update.
            update: Updates to apply to the function.
            change_notes: A note to attach to the new version. The server treats an empty
                note as a change, so the current version's note carries over when omitted.

        Returns:
            The updated UserDefinedFunction.
        """
        current = (
            user_defined_function
            if isinstance(user_defined_function, UserDefinedFunction)
            else await self.get(user_defined_function_id=user_defined_function)
        )
        if isinstance(update, dict):
            update = UserDefinedFunctionUpdate.model_validate(update)
        update.resource_id = current._id_or_error
        updated = await self._low_level_client.update_function(
            update, change_notes=change_notes or current.latest_version.change_notes
        )
        return self._apply_client_to_instance(updated)

    async def validate_expression(
        self, expression: str, function_inputs: list[FunctionInput]
    ) -> UserDefinedFunctionValidation:
        """Check an expression without saving it.

        Args:
            expression: The expression to check.
            function_inputs: The inputs the expression refers to. The server rejects an
                empty list.

        Returns:
            Whether the expression compiles, and its output type or error.
        """
        return await self._low_level_client.validate_function(
            expression=expression, function_inputs=function_inputs
        )

    async def get_where_used(
        self,
        user_defined_function: str | UserDefinedFunction | None = None,
        *,
        version: str | UserDefinedFunctionVersion | None = None,
    ) -> FunctionUsage:
        """Get what uses a function.

        Check this before changing inputs or the output type. The server refuses those
        changes once a function is in use.

        Args:
            user_defined_function: The UserDefinedFunction or function ID.
            version: A specific UserDefinedFunctionVersion or version ID, instead of the
                function as a whole.

        Returns:
            The functions, calculated channels, and rules that use it.

        Raises:
            ValueError: If neither or both are provided.
        """
        if (user_defined_function is None) == (version is None):
            raise ValueError("Exactly one of user_defined_function or version must be provided")
        if version is not None:
            version_id = (
                version._id_or_error if isinstance(version, UserDefinedFunctionVersion) else version
            )
            return self._apply_client_to_usage(
                await self._low_level_client.get_dependents(version_id=version_id)
            )
        function_id = (
            user_defined_function._id_or_error
            if isinstance(user_defined_function, UserDefinedFunction)
            else user_defined_function
        )
        return self._apply_client_to_usage(
            await self._low_level_client.get_dependents(function_id=function_id)
        )

    def _apply_client_to_usage(self, usage: FunctionUsage) -> FunctionUsage:
        """Hand the client to every object in the usage, so their properties resolve."""
        self._apply_client_to_instances(usage.functions)
        self._apply_client_to_instances(usage.calculated_channels)
        self._apply_client_to_instances(usage.rules)
        return usage

    async def archive(
        self, user_defined_function: str | UserDefinedFunction
    ) -> UserDefinedFunction:
        """Archive a function.

        Args:
            user_defined_function: The UserDefinedFunction or function ID to archive.

        Returns:
            The archived UserDefinedFunction.
        """
        return await self.update(user_defined_function, UserDefinedFunctionUpdate(is_archived=True))

    async def unarchive(
        self, user_defined_function: str | UserDefinedFunction
    ) -> UserDefinedFunction:
        """Unarchive a function.

        Args:
            user_defined_function: The UserDefinedFunction or function ID to unarchive.

        Returns:
            The unarchived UserDefinedFunction.
        """
        return await self.update(
            user_defined_function, UserDefinedFunctionUpdate(is_archived=False)
        )
