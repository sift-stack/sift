from __future__ import annotations

from typing import TYPE_CHECKING

from sift_client._internal.low_level_wrappers.user_defined_functions import (
    UserDefinedFunctionsLowLevelClient,
)
from sift_client.resources._base import ResourceBase
from sift_client.sift_types.user_defined_function import (
    FunctionDependents,
    FunctionInput,
    UserDefinedFunction,
    UserDefinedFunctionCreate,
    UserDefinedFunctionUpdate,
    UserDefinedFunctionValidation,
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

    async def get(self, version_id: str) -> UserDefinedFunction:
        """Get one version.

        Args:
            version_id: The ID of the version.

        Returns:
            The function at that version.
        """
        version = await self._low_level_client.get_version(version_id=version_id)
        return self._apply_client_to_instance(version)

    async def list_(
        self,
        *,
        function: str | UserDefinedFunction | None = None,
        name: str | None = None,
        # version specific
        version: int | None = None,
        # common filters
        include_archived: bool = False,
        filter_query: str | None = None,
        order_by: str | None = None,
        limit: int | None = None,
        page_size: int | None = None,
    ) -> list[UserDefinedFunction]:
        """List a function's versions.

        Args:
            function: The UserDefinedFunction or function ID whose versions to list.
            name: The function name, as an alternative to `function`.
            version: Filter to a single version number.
            include_archived: If True, include archived versions in results.
            filter_query: Explicit CEL query to filter versions.
            order_by: Field and direction to order results by.
            limit: Maximum number of versions to return. If None, returns all matches.
            page_size: Number of results to fetch per request.

        Returns:
            A list of UserDefinedFunction objects, one per version.
        """
        filter_parts = self._build_common_cel_filters(
            include_archived=include_archived, filter_query=filter_query
        )
        if version is not None:
            filter_parts.append(cel.equals("version", version))
        query_filter = cel.and_(*filter_parts)

        versions = await self._low_level_client.list_all_versions(
            function_id=(
                function._id_or_error if isinstance(function, UserDefinedFunction) else function
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

    async def get(self, function_id: str) -> UserDefinedFunction:
        """Get a UserDefinedFunction.

        Args:
            function_id: The ID of the function.

        Returns:
            The UserDefinedFunction.
        """
        function = await self._low_level_client.get_function(function_id=function_id)
        return self._apply_client_to_instance(function)

    async def list_(
        self,
        *,
        name: str | None = None,
        name_contains: str | None = None,
        name_regex: str | re.Pattern | None = None,
        # self ids
        function_ids: list[str] | None = None,
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
            function_ids: Filter to functions with any of these IDs.
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
        if function_ids:
            filter_parts.append(cel.in_("user_defined_function_id", function_ids))
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
        function: str | UserDefinedFunction,
        update: UserDefinedFunctionUpdate | dict,
    ) -> UserDefinedFunction:
        """Update a function. This creates a new version.

        Args:
            function: The UserDefinedFunction or function ID to update.
            update: Updates to apply to the function.

        Returns:
            The updated UserDefinedFunction.
        """
        function_id = (
            function._id_or_error if isinstance(function, UserDefinedFunction) else function
        )
        if isinstance(update, dict):
            update = UserDefinedFunctionUpdate.model_validate(update)
        update.resource_id = function_id
        updated = await self._low_level_client.update_function(update)
        return self._apply_client_to_instance(updated)

    async def sync(
        self, functions: list[UserDefinedFunctionCreate | dict]
    ) -> list[UserDefinedFunction]:
        """Create or update each function so Sift matches the definitions given.

        Functions match by name. A new name is created. An existing one is updated,
        which produces a new version. Nothing is archived.

        Args:
            functions: The function definitions to apply.

        Returns:
            The created or updated functions, in the order given.
        """
        definitions = [
            UserDefinedFunctionCreate.model_validate(f) if isinstance(f, dict) else f
            for f in functions
        ]
        wanted = {d.name for d in definitions}
        existing = {f.name: f for f in await self.list_(include_archived=True) if f.name in wanted}

        results = []
        for definition in definitions:
            current = existing.get(definition.name)
            if current is None:
                results.append(await self.create(definition))
                continue
            changes = definition.model_dump(exclude_unset=True, exclude={"name"})
            results.append(await self.update(current, changes))
        return results

    async def validate(
        self, expression: str, function_inputs: list[FunctionInput] | None = None
    ) -> UserDefinedFunctionValidation:
        """Check an expression without saving it.

        Args:
            expression: The expression to check.
            function_inputs: The inputs the expression refers to.

        Returns:
            Whether the expression compiles, and its output type or error.
        """
        return await self._low_level_client.validate_function(
            expression=expression, function_inputs=function_inputs
        )

    async def dependents(
        self, function: str | UserDefinedFunction, *, version_id: str | None = None
    ) -> FunctionDependents:
        """Get what depends on a function.

        Check this before changing inputs or the output type. The server refuses those
        changes once a function has dependents.

        Args:
            function: The UserDefinedFunction or function ID.
            version_id: A specific version, instead of the function as a whole.

        Returns:
            The IDs of dependent functions, calculated channels, and rules.
        """
        if version_id is not None:
            return await self._low_level_client.get_dependents(version_id=version_id)
        function_id = (
            function._id_or_error if isinstance(function, UserDefinedFunction) else function
        )
        return await self._low_level_client.get_dependents(function_id=function_id)

    async def archive(self, function: str | UserDefinedFunction) -> UserDefinedFunction:
        """Archive a function.

        Args:
            function: The UserDefinedFunction or function ID to archive.

        Returns:
            The archived UserDefinedFunction.
        """
        return await self.update(function, UserDefinedFunctionUpdate(is_archived=True))

    async def unarchive(self, function: str | UserDefinedFunction) -> UserDefinedFunction:
        """Unarchive a function.

        Args:
            function: The UserDefinedFunction or function ID to unarchive.

        Returns:
            The unarchived UserDefinedFunction.
        """
        return await self.update(function, UserDefinedFunctionUpdate(is_archived=False))
