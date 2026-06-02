"""Parent-step stacks: the parametrize and hierarchy frames shared across items.

Holds the collection-phase stash keys and the two module-level frame stacks
(``parametrize_stack`` / ``hierarchy_stack``), the helpers that build a chain
for an item and drain the stacks, and the per-item reconcilers the autouse
fixtures delegate to. Frames are shared across sibling test items and drained
innermost-first at session end.
"""

from __future__ import annotations

import warnings
from typing import Any, Tuple

import pytest

from sift_client._internal.pytest_plugin.options import (
    CLASS_STEP_OPTION,
    MODULE_STEP_OPTION,
    PACKAGE_STEP_OPTION,
    PARAMETRIZE_NESTING_OPTION,
)

STASH_MISSING = object()

parametrize_path_key = pytest.StashKey[Tuple[str, ...]]()
# Each frame: (path_key, open step). Frames are shared across sibling test items
# and drained at session end.
parametrize_stack: list[tuple[str, Any]] = []

hierarchy_key = pytest.StashKey[Tuple[Tuple[str, str, "str | None", bool], ...]]()
# Outer-to-inner frames for the item's collection-tree ancestors. Each chain
# entry is ``(identity, name, doc, rendered)``:
#   - ``identity``: a globally-unique key (``node.nodeid``) used for diff
#     comparison. Two ancestors at the same depth with the same display name
#     but reached via different paths (e.g., ``proj_a/utils`` and
#     ``proj_b/utils`` in a monorepo) get distinct identities, so they never
#     silently merge in the diff.
#   - ``name``: the human-readable step name used when ``rendered`` opens the
#     Sift step.
#   - ``doc``: docstring used for the step description if rendered.
#   - ``rendered``: True iff the corresponding ``sift_*_step`` ini flag is on.
#     Non-rendered frames participate in the diff but do not call
#     ``rc.new_step(...)``; they appear with ``ns=None`` in the stack.
#
# Stack entries: ``(identity, name, open_step_or_None)``. Frames are shared
# across sibling test items and drained at session end. Drained AFTER
# parametrize_stack since parametrize parents nest inside hierarchy parents.
hierarchy_stack: list[tuple[str, str, Any]] = []


def drain_step_stack(stack: list, *, swallow_errors: bool = True) -> None:
    """Pop and close every frame.

    With ``swallow_errors=True`` (default, used at teardown / session end),
    per-frame failures are surfaced as ``SiftPytestStepDrainWarning`` so a
    single misbehaving ``__exit__`` can't block the rest of the stack from
    cleaning up or cascade out of pytest's finalizer chain.

    With ``swallow_errors=False`` (mid-session, when a class transition forces
    parametrize parents to close), the stack is still fully drained but the
    first per-frame exception is re-raised at the end as a
    ``SiftPytestStepDrainError`` so a real upstream invariant violation
    surfaces as a test error instead of a silenceable warning.
    """
    from sift_client.pytest_plugin import SiftPytestStepDrainError, SiftPytestStepDrainWarning

    errors: list[tuple[str, BaseException]] = []
    while stack:
        entry = stack.pop()
        # Tolerate either ``(name, ns)`` (parametrize stack) or
        # ``(identity, name, ns)`` (hierarchy stack) entries.
        name, ns = entry[-2], entry[-1]
        if ns is None:
            # Non-rendered diff-only frame (e.g. a Package frame when
            # ``sift_package_step=false``); nothing to close.
            continue
        try:
            ns.__exit__(None, None, None)
        except Exception as exc:
            if swallow_errors:
                warnings.warn(
                    f"Sift plugin: closing step {name!r} during drain raised "
                    f"{type(exc).__name__}: {exc}",
                    SiftPytestStepDrainWarning,
                    stacklevel=2,
                )
            else:
                errors.append((name, exc))
    if errors:
        first_name, first_exc = errors[0]
        raise SiftPytestStepDrainError(
            f"Sift plugin: {len(errors)} step(s) raised while draining mid-session; "
            f"first failure on {first_name!r}: {type(first_exc).__name__}: {first_exc}"
        ) from first_exc


def drain_parametrize_stack(*, swallow_errors: bool = True) -> None:
    drain_step_stack(parametrize_stack, swallow_errors=swallow_errors)


def drain_hierarchy_stack(*, swallow_errors: bool = True) -> None:
    drain_step_stack(hierarchy_stack, swallow_errors=swallow_errors)


def close_frame(name: str, ns: Any) -> None:
    """Close a single frame, warning on per-frame failure.

    Used by the mid-session hierarchy-stack pop and the rollback paths so a
    misbehaving ``__exit__`` neither shadows the original exception nor leaks
    sibling frames. ``ns=None`` indicates a non-rendered diff-only frame; skip.
    """
    from sift_client.pytest_plugin import SiftPytestStepDrainWarning

    if ns is None:
        return
    try:
        ns.__exit__(None, None, None)
    except Exception as exc:
        warnings.warn(
            f"Sift plugin: closing step {name!r} raised {type(exc).__name__}: {exc}",
            SiftPytestStepDrainWarning,
            stacklevel=2,
        )


def build_parametrize_path(item: pytest.Item) -> tuple[str, ...]:
    """Outer-to-inner step display names for a parametrized item.

    Pytest stores ``callspec.params`` with the BOTTOM decorator's axis first;
    the Sift step tree treats the TOP decorator as outermost, so we reverse.
    """
    callspec = getattr(item, "callspec", None)
    if callspec is None or not callspec.params:
        return ()
    originalname = getattr(item, "originalname", item.name)
    frames: list[str] = [originalname]
    for name, value in reversed(callspec.params.items()):
        frames.append(f"{name}={value!r}")
    return tuple(frames)


def build_hierarchy_chain(
    item: pytest.Item | pytest.Collector,
    config: pytest.Config,
) -> tuple[tuple[str, str, str | None, bool], ...]:
    """Outer-to-inner ``(identity, name, docstring, rendered)`` for collection ancestors.

    Walks ``item.parent`` upward and ALWAYS collects every ``pytest.Package``,
    ``pytest.Module``, and ``pytest.Class`` ancestor; they all participate in
    the diff that keeps the report tree coherent across tests, so two
    same-named ancestors reached via different paths (e.g., ``proj_a/utils``
    and ``proj_b/utils`` in a monorepo where the ``proj_*`` dirs are
    ``pytest.Dir`` nodes the walker skips) cannot silently merge.

    The ``identity`` field is ``node.nodeid``, globally unique per collected
    node. The diff compares on identity, not the display ``name``.

    The ``rendered`` flag is True iff the layer's ini flag is on
    (``sift_package_step`` / ``sift_module_step`` / ``sift_class_step``).
    Non-rendered frames participate in the diff for identity but don't open a
    Sift step.

    The ``node.obj`` access is a pytest property that imports the underlying
    Python object and can raise *any* exception (ImportError, custom
    metaclass errors, descriptor ``__doc__`` properties that throw). Guard
    broadly so a misbehaving collector doesn't abort the whole collection
    phase; that frame's docstring just becomes ``None``.
    """
    include_package = bool(PACKAGE_STEP_OPTION.resolve(config))
    include_module = bool(MODULE_STEP_OPTION.resolve(config))
    include_class = bool(CLASS_STEP_OPTION.resolve(config))

    chain: list[tuple[str, str, str | None, bool]] = []
    # ``node.parent`` is typed as the internal ``_pytest.nodes.Node`` which
    # isn't part of pytest's public API; widen to ``Any`` for the walk.
    node: Any = item
    while node is not None:
        if isinstance(node, pytest.Class):
            rendered = include_class
        elif isinstance(node, pytest.Module):
            rendered = include_module
        elif isinstance(node, pytest.Package):
            rendered = include_package
        else:
            node = node.parent
            continue
        try:
            doc = (
                (getattr(node, "obj", None) and getattr(node.obj, "__doc__", None)) or ""
            ).strip() or None
        except Exception:
            doc = None
        chain.append((node.nodeid, node.name, doc, rendered))
        node = node.parent
    return tuple(reversed(chain))


def reconcile_hierarchy(request: pytest.FixtureRequest, config: pytest.Config) -> None:
    """Open/close hierarchy parents so the open stack matches the item's chain.

    Diffs the item's desired ``(package, module, class)`` chain against
    ``hierarchy_stack`` on identity (nodeid), pops the stale tail, and pushes
    new rendered frames. Which node types render is decided at build time by
    ``sift_package_step`` / ``sift_module_step`` / ``sift_class_step``; when the
    chain changes, the parametrize stack is drained first since parametrize
    parents nest INSIDE these.
    """
    # Fall back to computing the chain on-demand for items that bypassed
    # ``pytest_collection_modifyitems`` (e.g., dynamically inserted by another
    # plugin's later hook). Defaulting to ``()`` would incorrectly drain the
    # entire open hierarchy stack for those items.
    desired = request.node.stash.get(hierarchy_key, STASH_MISSING)
    if desired is STASH_MISSING:
        desired = build_hierarchy_chain(request.node, config)
    common = 0
    # Compare on identity (nodeid); same-named ancestors at different paths
    # MUST stay distinct.
    while (
        common < len(hierarchy_stack)
        and common < len(desired)
        and hierarchy_stack[common][0] == desired[common][0]
    ):
        common += 1
    # Any change to the hierarchy chain orphans parametrize parents from the
    # previous test. Drain them before mutating the hierarchy stack so
    # ReportContext's top-of-stack invariant holds. Strict mode: a per-frame
    # ``__exit__`` failure here signals a real upstream drift between the
    # plugin stacks and ReportContext; raise it as a test error instead of a
    # silenceable warning.
    if common < len(hierarchy_stack) or common < len(desired):
        drain_parametrize_stack(swallow_errors=False)
    # Symmetric per-frame guard for the hierarchy pop so one bad ``__exit__``
    # doesn't leave hierarchy_stack partially drained for every subsequent test.
    while len(hierarchy_stack) > common:
        _identity, name, ns = hierarchy_stack.pop()
        close_frame(name, ns)
    if not desired[common:]:
        return
    # Fetch ``report_context`` lazily, but only when there's at least one
    # rendered frame to push. Pure diff-only frames (e.g. a Package frame when
    # ``sift_package_step=false``) just update hierarchy_stack with ns=None.
    rc = None
    # Roll back any partial push so a mid-loop exception doesn't leave half
    # the chain orphaned on the stack. Per-frame guard inside the rollback so
    # a failing ``__exit__`` doesn't shadow the original exception or leak
    # the remaining opened frames.
    opened: list[tuple[str, str, Any]] = []
    try:
        for identity, name, doc, rendered in desired[common:]:
            if rendered:
                if rc is None:
                    rc = request.getfixturevalue("report_context")
                ns = rc.new_step(name=name, description=doc, assertion_as_fail_not_error=False)
                ns.__enter__()
                opened.append((identity, name, ns))
            else:
                opened.append((identity, name, None))
    except BaseException:
        while opened:
            _identity, name, ns = opened.pop()
            close_frame(name, ns)
        raise
    hierarchy_stack.extend(opened)


def reconcile_parametrize(request: pytest.FixtureRequest, config: pytest.Config) -> None:
    """Open/close shared parametrize parents so the open stack matches the item.

    Diffs the item's desired parametrize path against ``parametrize_stack``:
    pops the stale tail, then opens new parents (everything except the innermost
    frame, which the ``step`` fixture creates as the leaf). Parents persist
    across sibling items so a tree like ``test_x[a=1]`` / ``test_x[a=2]`` shares
    one ``test_x`` container. No-op when ``sift_parametrize_nesting=false``.
    """
    if not PARAMETRIZE_NESTING_OPTION.resolve(config):
        return
    # Fall back to on-demand computation for dynamically-inserted items;
    # see reconcile_hierarchy for the same rationale.
    desired = request.node.stash.get(parametrize_path_key, STASH_MISSING)
    if desired is STASH_MISSING:
        desired = build_parametrize_path(request.node)
    parents = desired[:-1]
    common = 0
    while (
        common < len(parametrize_stack)
        and common < len(parents)
        and parametrize_stack[common][0] == parents[common]
    ):
        common += 1
    # Per-frame guard so one bad ``__exit__`` doesn't leave parametrize_stack
    # partially drained for every subsequent test.
    while len(parametrize_stack) > common:
        name, ns = parametrize_stack.pop()
        close_frame(name, ns)
    if not parents[common:]:
        return
    rc = request.getfixturevalue("report_context")
    opened: list[tuple[str, Any]] = []
    try:
        for display in parents[common:]:
            ns = rc.new_step(name=display, assertion_as_fail_not_error=False)
            ns.__enter__()
            opened.append((display, ns))
    except BaseException:
        while opened:
            name, ns = opened.pop()
            close_frame(name, ns)
        raise
    parametrize_stack.extend(opened)
