"""Report-tree parent steps: an identity-keyed registry built without reordering.

Each test's package/module/class ancestors ("hierarchy" parents) and each
``@pytest.mark.parametrize`` axis ("parametrize" parents) become parent steps the
leaf nests under. Parents are kept in identity-keyed registries — created once and
reused by every descendant regardless of execution order — so the plugin never
reorders test items. A parent is closed as soon as the last leaf in its subtree
finishes (``release_finished_leaf``), with ``finalize_parents`` as the session-end
backstop for anything still open.
"""

from __future__ import annotations

import logging
import warnings
from typing import TYPE_CHECKING, Any, List, Optional, Tuple

import pytest

from sift_client._internal.pytest_plugin.modes import gate_enabled
from sift_client._internal.pytest_plugin.options import (
    CLASS_STEP_OPTION,
    MODULE_STEP_OPTION,
    PACKAGE_STEP_OPTION,
    PARAMETRIZE_NESTING_OPTION,
)

logger = logging.getLogger(__name__)

if TYPE_CHECKING:
    from typing import Callable

    from sift_client.util.test_results import ReportContext
    from sift_client.util.test_results.context_manager import NewStep

# --- Report-tree type aliases ---------------------------------------------
# The plugin juggles a few small tuple/dict shapes for the parent step tree;
# naming them keeps the signatures below readable. Defined with ``typing``
# generics (not ``list``/``tuple``) because some are used in runtime
# ``StashKey[...]`` subscriptions, which must stay importable on Python 3.8.
#
# A hierarchy parent's identity is just a ``str`` (the ancestor node's
# ``nodeid``); a parametrize parent's identity is a ``ParametrizeKey``: the
# test's param-stripped node id followed by its outer-to-inner axis frames
# (e.g. ``("pkg/test_m.py::TestC::test_a", "v=1")``).
ParametrizeKey = Tuple[str, ...]
# Outer-to-inner display-name axis path stashed per parametrized item
# (``(originalname, "v=1", ...)``); the leaf is its last frame.
ParametrizePath = Tuple[str, ...]
# One collection-tree ancestor: ``(identity, display name, docstring, rendered)``.
# ``rendered`` is True iff that layer's ``sift_*_step`` ini flag opens a step.
HierarchyFrame = Tuple[str, str, Optional[str], bool]
# Outer-to-inner ancestor frames stashed per item.
HierarchyChain = Tuple[HierarchyFrame, ...]
# A rendered parent to open, as returned by ``resolved_parents``.
HierarchyParent = Tuple[str, str, Optional[str]]  # (identity, name, docstring)
ParametrizeParent = Tuple[ParametrizeKey, str]  # (registry key, frame name)
# A gated-in leaf's parents: its rendered hierarchy identities and parametrize keys.
LeafParents = Tuple[List[str], List[ParametrizeKey]]

parametrize_path_key = pytest.StashKey[ParametrizePath]()

hierarchy_key = pytest.StashKey[HierarchyChain]()
# See ``HierarchyFrame`` above for the chain entry shape. ``identity`` is the
# node's ``nodeid``: two ancestors at the same depth with the same display name
# but reached via different paths (e.g., ``proj_a/utils`` and ``proj_b/utils`` in
# a monorepo) get distinct identities, so they never silently merge. Non-rendered
# frames open no step; the next rendered descendant attaches to the nearest
# rendered ancestor instead.

# Open report-tree parent steps, keyed by identity so they are created once and
# reused by every descendant regardless of test execution order. The leaf step
# for each test is created under its resolved parent (see ``report.step_impl``),
# so no global ordering of test items is required. Parents live OUTSIDE
# ``ReportContext.step_stack`` (created with ``push=False``) and are closed early
# by ``release_finished_leaf``, or at session end by ``finalize_parents``.
#
# Hierarchy parents (packages / modules / classes) keyed by the ancestor node's
# ``nodeid``:
hierarchy_parents: dict[str, NewStep] = {}
# Parametrize parents keyed by ``ParametrizeKey``, so sibling parametrizations of
# one test share a parent while parametrizations under different
# tests/classes/modules never collide:
parametrize_parents: dict[ParametrizeKey, NewStep] = {}

# Remaining descendant leaves per open-able parent, keyed exactly like the
# registries above. Populated from the collected (and selected) items in
# ``tally_expected_parents`` and decremented as each test finishes; when a count
# reaches zero the parent's whole subtree is done and it is closed early (see
# ``release_finished_leaf``) instead of waiting for session end.
expected_hierarchy: dict[str, int] = {}
expected_parametrize: dict[ParametrizeKey, int] = {}
# Each gated-in leaf's parent identities, so ``release_finished_leaf`` — which
# only receives a nodeid — knows which counters to decrement.
leaf_parents: dict[str, LeafParents] = {}


def build_parametrize_path(item: pytest.Item) -> ParametrizePath:
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
) -> HierarchyChain:
    """Outer-to-inner ``(identity, name, docstring, rendered)`` for collection ancestors.

    Walks ``item.parent`` upward and ALWAYS collects every ``pytest.Package``,
    ``pytest.Module``, and ``pytest.Class`` ancestor; they all carry the identity
    that keeps the report tree coherent across tests, so two same-named ancestors
    reached via different paths (e.g., ``proj_a/utils`` and ``proj_b/utils`` in a
    monorepo where the ``proj_*`` dirs are ``pytest.Dir`` nodes the walker skips)
    cannot silently merge.

    The ``identity`` field is ``node.nodeid``, globally unique per collected node.

    The ``rendered`` flag is True iff the layer's ini flag is on
    (``sift_package_step`` / ``sift_module_step`` / ``sift_class_step``).
    Non-rendered frames carry identity but don't open a Sift step.

    The ``node.obj`` access is a pytest property that imports the underlying
    Python object and can raise *any* exception (ImportError, custom
    metaclass errors, descriptor ``__doc__`` properties that throw). Guard
    broadly so a misbehaving collector doesn't abort the whole collection
    phase; that frame's docstring just becomes ``None``.
    """
    include_package = bool(PACKAGE_STEP_OPTION.resolve(config))
    include_module = bool(MODULE_STEP_OPTION.resolve(config))
    include_class = bool(CLASS_STEP_OPTION.resolve(config))

    chain: list[HierarchyFrame] = []
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


def resolved_parents(
    node: pytest.Item,
    config: pytest.Config,
) -> tuple[list[HierarchyParent], list[ParametrizeParent]]:
    """The rendered report-tree parents for ``node`` — the single source of truth.

    Shared by ``get_or_create_parent_chain`` (which opens these parents) and the
    early-close counters in ``tally_expected_parents`` (which count them), so the
    two can never key on different identities. Returns ``(hierarchy, parametrize)``
    outer-to-inner:

    * hierarchy: ``(identity, name, doc)`` for each rendered package/module/class
      ancestor. ``identity`` is the node's ``nodeid`` (the registry key).
    * parametrize: ``(registry key, frame name)`` for each parametrize axis except
      the innermost (the leaf is the ``step`` fixture's job). Empty when
      ``sift_parametrize_nesting`` is off or the item isn't parametrized.

    Reads the per-item stash written in ``pytest_itemcollected``; recomputes for
    items a later hook injected without going through it.
    """
    if hierarchy_key in node.stash:
        chain = node.stash[hierarchy_key]
    else:
        chain = build_hierarchy_chain(node, config)
    # Non-rendered frames open no step; the next rendered descendant attaches to
    # the nearest rendered ancestor, so they are simply dropped here.
    hierarchy = [(identity, name, doc) for identity, name, doc, rendered in chain if rendered]

    parametrize: list[ParametrizeParent] = []
    if PARAMETRIZE_NESTING_OPTION.resolve(config):
        if parametrize_path_key in node.stash:
            path = node.stash[parametrize_path_key]
        else:
            path = build_parametrize_path(node)
        if path:
            # Key parametrize parents by the test's param-stripped identity plus
            # the outer frame prefix, so sibling params share a parent but params
            # under different tests never merge.
            key: ParametrizeKey = (strip_param(node.nodeid),)
            for frame in path[:-1]:
                key = (*key, frame)
                parametrize.append((key, frame))
    return hierarchy, parametrize


def strip_param(nodeid: str) -> str:
    """Drop the trailing ``[param]`` from a nodeid, keeping ``file::Class::func``.

    The parametrize id is a variation of the test, not its identity — leaving it
    in would make a re-parametrization silently shift the grouping key. Splits on
    the last ``::`` segment and cuts at its first ``[``; class/function names
    never contain ``[``, so nested brackets in a param value can't confuse it.
    """
    head, sep, leaf = nodeid.rpartition("::")
    leaf = leaf.split("[", 1)[0]
    return f"{head}{sep}{leaf}"


def get_or_create_parent_chain(
    node: pytest.Item,
    config: pytest.Config,
    request: pytest.FixtureRequest,
) -> NewStep | None:
    """Resolve the innermost report-tree parent for ``node``, creating any missing ancestors.

    Walks the item's rendered hierarchy ancestors (outer-to-inner) and then its
    parametrize axes (see ``resolved_parents``), get-or-creating one parent step
    per identity in the registries. Each new parent is opened under the running
    parent (``push=False``, so it stays off ``ReportContext.step_stack``) and
    reused by every later descendant — no contiguity of sibling items is required,
    so test execution order is irrelevant.

    Returns the innermost parent the leaf should attach to, or ``None`` when no
    rendered parent applies (the leaf becomes a report-root step). ``report_context``
    is fetched lazily, only when a parent actually needs creating, so excluded
    items never trigger eager context setup.
    """
    rc_cache: list[ReportContext] = []

    def rc() -> ReportContext:
        if not rc_cache:
            rc_cache.append(request.getfixturevalue("report_context"))
        return rc_cache[0]

    return _resolve_parent_chain(node, config, rc)


def resolve_parent_chain_in_context(
    node: pytest.Item,
    config: pytest.Config,
    context: ReportContext,
) -> NewStep | None:
    """``get_or_create_parent_chain`` for callers holding a ``ReportContext`` directly.

    The collection-skip path runs from ``pytest_runtest_makereport`` (the autouse
    fixtures never ran for a marker-skipped item), so it has no ``FixtureRequest``
    to resolve ``report_context`` from, only the session ``ReportContext``. It
    must still nest the skipped item's step under the same registry parents a
    running sibling uses, so it shares the create-once logic here.
    """
    return _resolve_parent_chain(node, config, lambda: context)


def _resolve_parent_chain(
    node: pytest.Item,
    config: pytest.Config,
    rc: Callable[[], ReportContext],
) -> NewStep | None:
    """Shared body of the two parent-chain resolvers; ``rc`` supplies the context.

    ``rc`` is called only when a parent actually needs creating, so a caller that
    passes a lazy getter keeps the "no eager context setup" guarantee.
    """
    hierarchy, parametrize = resolved_parents(node, config)
    parent_step: Any = None  # TestStep of the running innermost parent, or None (root).
    innermost: NewStep | None = None

    for identity, name, doc in hierarchy:
        ns = hierarchy_parents.get(identity)
        if ns is None:
            ns = rc().new_step(
                name=name,
                description=doc,
                assertion_as_fail_not_error=False,
                parent=parent_step,
                push=False,
            )
            ns.__enter__()
            hierarchy_parents[identity] = ns
        parent_step = ns.current_step
        innermost = ns

    for key, frame in parametrize:
        ns = parametrize_parents.get(key)
        if ns is None:
            ns = rc().new_step(
                name=frame,
                assertion_as_fail_not_error=False,
                parent=parent_step,
                push=False,
            )
            ns.__enter__()
            parametrize_parents[key] = ns
        parent_step = ns.current_step
        innermost = ns

    return innermost


def close_parent(ns: NewStep) -> None:
    """Close one open report-tree parent, stamping its last-descendant finish time.

    Shared by mid-session early close (``release_finished_leaf``) and the
    session-end drain (``finalize_parents``). The ``end_time`` override comes from
    ``ReportContext.parent_end_times`` so the parent's window ends at its latest
    descendant rather than wall-clock at close. A misbehaving ``__exit__`` is
    surfaced as a warning so it never blocks the remaining parents or cascades out
    of pytest's finalizer chain.
    """
    from sift_client.pytest_plugin import REPORT_CONTEXT, SiftPytestStepDrainWarning

    step = ns.current_step
    if step is None:
        return
    if REPORT_CONTEXT is not None:
        ns._sift_end_time_override = REPORT_CONTEXT.parent_end_times.get(step.step_path)
    try:
        ns.__exit__(None, None, None)
    except Exception as exc:
        warnings.warn(
            f"Sift plugin: closing parent step {step.name!r} raised {type(exc).__name__}: {exc}",
            SiftPytestStepDrainWarning,
            stacklevel=2,
        )


def close_parents_innermost_first(parents: list[NewStep]) -> None:
    """Close the given open parents deepest-``step_path`` first.

    Innermost-first means a child parent's ``propagate_step_result`` (status) and
    ``note_close`` (finish time) reach its parent's bookkeeping before that parent
    resolves — so a failing/late subtree rolls up correctly whether parents close
    mid-session or at session end.
    """
    parents.sort(
        key=lambda ns: ns.current_step.step_path.count(".") if ns.current_step else -1,
        reverse=True,
    )
    for ns in parents:
        close_parent(ns)


def finalize_parents() -> None:
    """Close every still-open report-tree parent at session end, innermost-first.

    The backstop for anything ``release_finished_leaf`` did not already close
    early (e.g. a parent whose subtree never fully ran because the session was
    aborted). Idempotent: the registries and counters are cleared up front, so the
    second drain site (``pytest_sessionfinish`` after ``report_context_impl``) is
    a no-op.
    """
    parents = [*parametrize_parents.values(), *hierarchy_parents.values()]
    parametrize_parents.clear()
    hierarchy_parents.clear()
    expected_hierarchy.clear()
    expected_parametrize.clear()
    leaf_parents.clear()
    close_parents_innermost_first(parents)


def tally_expected_parents(session: pytest.Session) -> None:
    """Count each open-able parent's descendant leaves, for mid-session early close.

    Runs after all ``modifyitems`` and deselection (``pytest_collection_finish``),
    so ``session.items`` is the final, selected set. Only gated-in items are
    counted — that keeps ``sift_exclude``-d siblings (and an entirely gated-off
    session, e.g. the dev suite's own outer run) out of the tallies, so a
    partially-excluded class still closes when its included tests finish. The maps
    are rebuilt every session because pytester runs inner sessions in-process,
    sharing this module state.
    """
    expected_hierarchy.clear()
    expected_parametrize.clear()
    leaf_parents.clear()
    for item in session.items:
        if not gate_enabled(item, session.config):
            continue
        hierarchy, parametrize = resolved_parents(item, session.config)
        h_ids = [identity for identity, _, _ in hierarchy]
        p_keys = [key for key, _ in parametrize]
        if not h_ids and not p_keys:
            continue  # leaf is a report-root step; no parent to close
        leaf_parents[item.nodeid] = (h_ids, p_keys)
        for identity in h_ids:
            expected_hierarchy[identity] = expected_hierarchy.get(identity, 0) + 1
        for key in p_keys:
            expected_parametrize[key] = expected_parametrize.get(key, 0) + 1


def _decrement_parent_counts(
    keys: list[Any],
    expected: dict[Any, int],
    registry: dict[Any, NewStep],
    ready: list[NewStep],
) -> None:
    """Decrement each key's remaining-descendant count by one.

    When a count reaches zero the parent's subtree is complete: drop it from both
    the count map and the registry and queue its still-open step (if any) onto
    ``ready`` for closing. The hierarchy and parametrize branches of
    ``release_finished_leaf`` differ only in which (count, registry) pair they
    pass here.
    """
    for key in keys:
        remaining = expected.get(key)
        if remaining is None:
            continue
        if remaining <= 1:
            expected.pop(key, None)
            closing = registry.pop(key, None)
            if closing is not None:
                ready.append(closing)
        else:
            expected[key] = remaining - 1


def release_finished_leaf(nodeid: str) -> None:
    """Decrement the finished item's parents; close any whose subtree is now done.

    Called from ``pytest_runtest_logfinish``, which fires once per item for every
    outcome (pass / fail / skip / error). When a parent's remaining-leaf count
    reaches zero its whole subtree has finished, so it is closed now rather than
    at session end — giving incremental uploads a progressively-resolving report
    under any execution order. Closes innermost-first so a child parent rolls its
    result and finish time up before its own parent resolves; several levels can
    complete on the same leaf (e.g. the last param variant closes its parametrize
    parent, class, and module at once). Items not in ``leaf_parents`` (gated-off,
    or injected after collection) are ignored; anything left open is handled by
    ``finalize_parents``.
    """
    entry = leaf_parents.pop(nodeid, None)
    if entry is None:
        return
    h_ids, p_keys = entry
    ready: list[NewStep] = []
    _decrement_parent_counts(h_ids, expected_hierarchy, hierarchy_parents, ready)
    _decrement_parent_counts(p_keys, expected_parametrize, parametrize_parents, ready)
    if ready:
        close_parents_innermost_first(ready)
