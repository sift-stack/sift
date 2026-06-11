"""Report-tree parent steps: an identity-keyed registry built without reordering.

Each test's package/module/class ancestors ("hierarchy" parents), its
scope-promoted parametrized params (session/package/module/class-scoped fixtures,
placed at their scope's level on the ladder), and each function-scoped
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

from sift_client._internal.pytest_plugin.audit_log import log_event
from sift_client._internal.pytest_plugin.modes import gate_enabled
from sift_client._internal.pytest_plugin.options import (
    CLASS_STEP_OPTION,
    MODULE_STEP_OPTION,
    PACKAGE_STEP_OPTION,
    PARAMETRIZE_NESTING_OPTION,
)

logger = logging.getLogger(__name__)

# Scope-aware parametrize placement and ``ids=`` label resolution read a few
# pytest internals (the fixture manager, ``callspec``). If a pytest version
# moves or reshapes those, we degrade to function-scoped nesting with
# ``name=value`` labels instead of failing the user's collection — and surface
# the loss once per session via ``_signal_introspection_degraded``. This latch
# keeps that to a single warning; ``reset_introspection_state`` clears it at the
# start of each session (see ``pytest_configure``).
_introspection_degraded = False


def reset_introspection_state() -> None:
    """Clear the one-shot introspection-failure latch. Called at session start."""
    global _introspection_degraded
    _introspection_degraded = False


def _signal_introspection_degraded(detail: str) -> None:
    """Warn + audit-log once that parametrize scope/label introspection failed.

    Fired only when reaching a pytest internal actually fails — not for the
    ordinary "no fixturedef, it's a mark-based param" case. The report still
    renders; scope-promoted params just fall back to function-scoped nesting.
    """
    global _introspection_degraded
    if _introspection_degraded:
        return
    _introspection_degraded = True
    # Local import avoids a circular import (pytest_plugin imports this module).
    from sift_client.pytest_plugin import SiftPytestPluginWarning

    warnings.warn(
        "Sift pytest plugin could not read pytest internals for scope-aware "
        f"parametrize placement ({detail}); parametrized fixtures will render "
        "flat (function-scoped) with name=value labels. The rest of the report "
        "is unaffected. This usually means an unsupported pytest version.",
        SiftPytestPluginWarning,
        stacklevel=2,
    )
    log_event(logger, logging.WARNING, "parametrize.introspection_degraded", detail=detail)


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
# A hierarchy parent's identity is a ``HierarchyKey``: the literal path from the
# report root to that parent, one segment per ancestor, each tagged by kind —
# ``("n", node_nodeid)`` for a package/module/class step, ``("p", label)`` for a
# scope-promoted param frame (e.g. a session/module/class-scoped parametrized
# fixture). Building identities as kind-tagged tuples (rather than a delimited
# string) keeps them collision-free even when a param label contains ``::``.
# A parametrize parent's identity is a ``ParametrizeKey``: the enclosing
# ``HierarchyKey`` path, then the test's param-stripped node id, then its
# outer-to-inner function-axis frames (e.g. ``(<path>, "…::test_a", "v=1")``).
HierarchySegment = Tuple[str, str]  # ("n", nodeid) | ("p", label)
HierarchyKey = Tuple[HierarchySegment, ...]
ParametrizeKey = Tuple[Any, ...]
# Outer-to-inner display-name axis path stashed per parametrized item
# (``(originalname, "v=1", ...)``); the leaf is its last frame. Function-scoped
# axes only — higher-scoped params are promoted into the hierarchy.
ParametrizePath = Tuple[str, ...]
# One collection-tree ancestor: ``(identity, display name, docstring, rendered,
# scope)``. ``rendered`` is True iff that layer's ``sift_*_step`` ini flag opens a
# step; ``scope`` is the pytest scope the node anchors ("package"/"module"/"class").
HierarchyFrame = Tuple[str, str, Optional[str], bool, str]
# Outer-to-inner ancestor frames stashed per item.
HierarchyChain = Tuple[HierarchyFrame, ...]
# A rendered parent to open, as returned by ``resolved_parents``.
HierarchyParent = Tuple[HierarchyKey, str, Optional[str]]  # (identity, name, docstring)
ParametrizeParent = Tuple[ParametrizeKey, str]  # (registry key, frame name)
# Scope-promoted params (anything broader than function scope) stashed per item.
# Each entry is ``(scope, param_name, label)``; in callspec application order.
ScopedParams = Tuple[Tuple[str, str, str], ...]
# A gated-in leaf's parents: its rendered hierarchy identities and parametrize keys.
LeafParents = Tuple[List[HierarchyKey], List[ParametrizeKey]]

parametrize_path_key = pytest.StashKey[ParametrizePath]()

hierarchy_key = pytest.StashKey[HierarchyChain]()
scoped_params_key = pytest.StashKey[ScopedParams]()
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
# Hierarchy parents (packages / modules / classes / scope-promoted params) keyed
# by their ``HierarchyKey`` root-to-node path:
hierarchy_parents: dict[HierarchyKey, NewStep] = {}
# Parametrize parents keyed by ``ParametrizeKey``, so sibling parametrizations of
# one test share a parent while parametrizations under different
# tests/classes/modules never collide:
parametrize_parents: dict[ParametrizeKey, NewStep] = {}

# Remaining descendant leaves per open-able parent, keyed exactly like the
# registries above. Populated from the collected (and selected) items in
# ``tally_expected_parents`` and decremented as each test finishes; when a count
# reaches zero the parent's whole subtree is done and it is closed early (see
# ``release_finished_leaf``) instead of waiting for session end.
expected_hierarchy: dict[HierarchyKey, int] = {}
expected_parametrize: dict[ParametrizeKey, int] = {}
# Each gated-in leaf's parent identities, so ``release_finished_leaf`` — which
# only receives a nodeid — knows which counters to decrement.
leaf_parents: dict[str, LeafParents] = {}


# --- Private-pytest-API surface -------------------------------------------
# Scope-aware parametrize placement reads a few pytest internals that have no
# public equivalent. They are confined to the helpers below and each is guarded
# so a pytest change degrades to flat (function-scoped) rendering with a single
# warning, never a crash. The inventory, so it stays auditable in one place:
#
#   * ``session._fixturemanager.getfixturedefs`` — the only genuinely private
#     access; obtaining a parametrized fixture's ``FixtureDef``. Wrapped in
#     ``_fixturedefs`` (degrade + warn). The attributes read off the result
#     (``.scope``, ``.baseid``, ``.ids``) are documented-stable public API.
#   * ``item.callspec.params`` / ``.indices`` — semi-private but de-facto stable
#     (every parametrize-aware plugin relies on it); read under try/except.
#   * ``mark.args``/``mark.kwargs`` and ``node.parent``/``node.obj`` — public.
#
# pytest's own private scope dict (``callspec._arg2scope``) is deliberately NOT
# read; ``_param_scope`` reconstructs the same result from public data.
# CI exercises this surface against pytest 7/8/9 (see test-pytest-compat).
def _fixturedefs(item: pytest.Item, name: str) -> Any:
    """The ``FixtureDef`` tuple for ``name``, or None when it is mark-based.

    The signature of ``getfixturedefs`` differs by pytest version: 8.x/9.x take
    the node object, pytest 7.x takes the nodeid string. So we try the node
    first and retry with the nodeid on failure. The retry trigger spans both
    error shapes the wrong-type call raises: pytest 8.x/9.x raise ``TypeError``
    when handed a nodeid where a node is expected, while pytest 7.x raises
    ``AttributeError`` (it calls ``nodeid.find(...)`` on what is actually a
    node, which has no ``.find``). Catching only ``TypeError`` would leave the
    7.x path dead and silently degrade every parametrized test on pytest 7.

    A clean ``None`` means "no such fixture" (a mark-based param), which is
    normal and not a failure. Reaching the fixture manager at all is the part
    that depends on a pytest internal: if that attribute is gone or the call
    raises something unexpected, degrade to ``None`` and latch the one-shot
    warning rather than letting it escape into the user's collection.
    """
    manager = getattr(getattr(item, "session", None), "_fixturemanager", None)
    if manager is None:
        _signal_introspection_degraded("fixture manager unavailable")
        return None
    try:
        return manager.getfixturedefs(name, item)
    except (TypeError, AttributeError):
        try:
            return manager.getfixturedefs(name, item.nodeid)  # type: ignore[arg-type]
        except Exception as exc:
            _signal_introspection_degraded(f"getfixturedefs failed ({exc!r})")
            return None
    except Exception as exc:
        _signal_introspection_degraded(f"getfixturedefs failed ({exc!r})")
        return None


def _fixture_scope(item: pytest.Item, name: str) -> str | None:
    """Return the scope of a fixture param, or None if it is mark-based."""
    defs = _fixturedefs(item, name)
    if defs:
        return defs[-1].scope
    return None


def _mark_argnames(mark: pytest.Mark) -> list[str]:
    """The argnames a ``parametrize`` mark covers (``"a,b"`` → ``["a", "b"]``)."""
    argnames = mark.args[0]
    if isinstance(argnames, str):
        return [a.strip() for a in argnames.split(",")]
    return list(argnames)


def _param_scope(item: pytest.Item, name: str) -> str:
    """The pytest scope governing param ``name`` on ``item``.

    Resolved from public data, mirroring pytest's own rule (see
    ``_find_parametrized_scope`` in pytest's ``python.py``): an explicit mark
    ``scope=`` wins, else a parametrized/indirect fixture contributes its
    declared ``fixturedef.scope``, else a plain ``@pytest.mark.parametrize`` axis
    is ``"function"``. We deliberately do NOT read pytest's private
    ``callspec._arg2scope``; the only internal still consulted is obtaining the
    fixturedef (see ``_fixturedefs``), whose ``.scope`` attribute is public.

    One intentional divergence from ``_arg2scope``: a combined ``indirect`` axis
    (``"a,b"``) whose fixtures have different scopes gets each name its own
    scope here, where pytest collapses both to the narrowest. Per-name placement
    is what the report wants, and ``build_scoped_params`` already buckets by it.
    ``callspec.params`` order is application order, NOT scope order, so callers
    must bucket by this rather than trust position.
    """
    # An explicit mark ``scope=`` takes precedence (pytest's ``Scope.from_user``
    # before the fixture-derived scope).
    for mark in item.iter_markers("parametrize"):
        if name in _mark_argnames(mark):
            mark_scope = mark.kwargs.get("scope")
            if mark_scope:
                return mark_scope
    # Otherwise a parametrized or indirect fixture lends its declared scope.
    fixture_scope = _fixture_scope(item, name)
    if fixture_scope is not None:
        return fixture_scope
    return "function"


def _id_from_spec(ids: Any, index: int, value: Any) -> str | None:
    """Resolve one axis's author-supplied ``ids`` spec to a string, or None.

    A list spec is indexed by the param's position; a callable spec (an ID
    factory) is invoked with the param value, mirroring how pytest builds the
    node ID. A callable that raises or returns ``None`` yields None, so the
    caller falls back to the structured ``name=value`` label. ``None`` here also
    covers "the author supplied no ``ids``" — those auto-generated IDs are noisier
    than ``name=value`` for non-trivial values, so we never adopt them.
    """
    if ids is None:
        return None
    if callable(ids):
        try:
            result = ids(value)
        except Exception:
            return None
        return str(result) if result is not None else None
    if index < len(ids):
        return str(ids[index])
    return None


def _explicit_param_id(item: pytest.Item, name: str, value: Any) -> str | None:
    """The author-supplied pytest ID for param ``name`` on ``item``, else None.

    Honours an explicit ``ids=`` — list or callable factory — declared on the
    fixture (``@pytest.fixture(params=..., ids=...)``) or on the
    ``@pytest.mark.parametrize`` axis, matching the friendly labels pytest puts
    in the node ID. Combined axes (``"a,b"``) are skipped: their single shared
    ID can't be attributed to one of the two frames the report renders, so those
    fall back to ``name=value``.
    """
    callspec = getattr(item, "callspec", None)
    if callspec is None:
        return None
    index = callspec.indices.get(name)
    if index is None:
        return None
    # Fixture params: the ``ids`` spec lives on the active FixtureDef.
    defs = _fixturedefs(item, name)
    if defs:
        resolved = _id_from_spec(getattr(defs[-1], "ids", None), index, value)
        if resolved is not None:
            return resolved
    # mark.parametrize: the ``ids`` spec lives in the marker kwargs.
    for mark in item.iter_markers("parametrize"):
        names = _mark_argnames(mark)
        if len(names) == 1 and names[0] == name:
            resolved = _id_from_spec(mark.kwargs.get("ids"), index, value)
            if resolved is not None:
                return resolved
    return None


def _param_label(item: pytest.Item, name: str, value: Any) -> str:
    """Display label for one param axis: its explicit pytest ID, else ``name=value``."""
    return _explicit_param_id(item, name, value) or f"{name}={value!r}"


def build_scoped_params(item: pytest.Item) -> ScopedParams:
    """Scope-promoted params for ``item`` (anything broader than function scope).

    Each entry is ``(scope, name, label)`` in ``callspec.params`` application
    order. ``resolved_parents`` buckets these by scope and places them at their
    scope's hierarchy level. Function-scoped axes are excluded — they stay inner,
    handled by ``build_parametrize_path``.
    """
    callspec = getattr(item, "callspec", None)
    if callspec is None or not callspec.params:
        return ()
    try:
        out: list[tuple[str, str, str]] = []
        for name, value in callspec.params.items():
            scope = _param_scope(item, name)
            if scope == "function":
                continue
            out.append((scope, name, _param_label(item, name, value)))
        return tuple(out)
    except Exception as exc:
        # Degrade to "nothing promoted": every axis stays function-scoped via
        # build_parametrize_path, so the leaf still renders, just flat.
        _signal_introspection_degraded(f"scoped-param resolution failed ({exc!r})")
        return ()


def build_parametrize_path(item: pytest.Item) -> ParametrizePath:
    """Outer-to-inner function-axis display names for a parametrized item.

    Pytest stores ``callspec.params`` with the BOTTOM decorator's axis first;
    the Sift step tree treats the TOP decorator as outermost, so we reverse.
    Only function-scoped axes appear here — higher-scoped params are promoted
    into the hierarchy by ``resolved_parents``. Each axis is labelled by its
    explicit pytest ID when the author supplied one, otherwise by ``name=value``
    (see ``_param_label``). The first frame is always ``originalname`` so the
    leaf step (``path[-1]`` in ``report.step_impl``) is the bare function name
    when a test has no function-scoped params of its own.
    """
    callspec = getattr(item, "callspec", None)
    if callspec is None or not callspec.params:
        return ()
    originalname = getattr(item, "originalname", item.name)
    frames: list[str] = [originalname]
    try:
        for name, value in reversed(callspec.params.items()):
            if _param_scope(item, name) != "function":
                continue
            frames.append(_param_label(item, name, value))
    except Exception as exc:
        # Mirror the build_scoped_params degradation: with scope resolution
        # broken nothing is promoted, so render every axis here under the bare
        # leaf name (the pre-scope-aware behavior) rather than dropping frames.
        _signal_introspection_degraded(f"parametrize-path resolution failed ({exc!r})")
        frames = [originalname]
        try:
            for name, value in reversed(callspec.params.items()):
                frames.append(f"{name}={value!r}")
        except Exception:
            return ()
    return tuple(frames)


def build_hierarchy_chain(
    item: pytest.Item | pytest.Collector,
    config: pytest.Config,
) -> HierarchyChain:
    """Outer-to-inner ``(identity, name, docstring, rendered, scope)`` for collection ancestors.

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
        # Check Package before Module: on pytest 7.x ``Package`` subclasses
        # ``Module``, so the Module branch would otherwise swallow a package node
        # and render it (ignoring ``sift_package_step``). pytest 8's collection
        # refactor made them unrelated, so the order is harmless there.
        if isinstance(node, pytest.Class):
            rendered, scope = include_class, "class"
        elif isinstance(node, pytest.Package):
            rendered, scope = include_package, "package"
        elif isinstance(node, pytest.Module):
            rendered, scope = include_module, "module"
        else:
            node = node.parent
            continue
        try:
            doc = (
                (getattr(node, "obj", None) and getattr(node.obj, "__doc__", None)) or ""
            ).strip() or None
        except Exception:
            doc = None
        chain.append((node.nodeid, node.name, doc, rendered, scope))
        node = node.parent
    return tuple(reversed(chain))


def _pick_class_index(
    node: pytest.Item, name: str, class_idxs: list[int], chain: HierarchyChain
) -> int | None:
    """Which Class chain frame a class-scoped param ``name`` anchors to.

    For a class-scoped fixture, the owning class is the deepest Class frame whose
    nodeid prefixes the fixture's ``baseid`` — so a fixture defined on an outer
    class anchors there, not under an inner nested class. Mark-based class params
    (no fixturedef) and unmatched cases fall back to the innermost class.
    """
    if not class_idxs:
        return None
    defs = _fixturedefs(node, name)
    baseid = getattr(defs[-1], "baseid", "") if defs else ""
    if baseid:
        best, best_len = None, -1
        for i in class_idxs:
            nid = chain[i][0]
            if (baseid == nid or baseid.startswith(nid + "::")) and len(nid) > best_len:
                best, best_len = i, len(nid)
        if best is not None:
            return best
    return class_idxs[-1]


def _scoped_param_anchors(
    node: pytest.Item, scoped: ScopedParams, chain: HierarchyChain
) -> dict[int | None, list[str]]:
    """Map each scope-promoted param to the chain-frame index it nests at.

    Returns an ``anchors`` map of chain-frame index → param labels at that frame.
    The ``None`` key holds params with no collector node — session-scoped params
    (emitted above the chain) and the rare fall-through when a param's scope has
    no frame at all. Scopes are processed package→module→class (so broader nests
    outside narrower at a shared anchor), and each scope's params are reversed
    (top-decorator-outermost). A param whose scope has no matching frame falls
    back to the nearest broader frame (module, else the ``None`` root bucket).
    """
    pkg_idx = next((i for i, f in enumerate(chain) if f[4] == "package"), None)
    mod_idx = next((i for i, f in enumerate(chain) if f[4] == "module"), None)
    class_idxs = [i for i, f in enumerate(chain) if f[4] == "class"]

    by_scope: dict[str, list[tuple[str, str]]] = {}
    for scope, name, label in scoped:
        by_scope.setdefault(scope, []).append((name, label))

    anchors: dict[int | None, list[str]] = {}

    def place(idx: int | None, label: str) -> None:
        anchors.setdefault(idx, []).append(label)

    # Reverse within each scope so the top decorator / outermost axis nests first.
    for _name, label in reversed(by_scope.get("session", [])):
        place(None, label)
    for _name, label in reversed(by_scope.get("package", [])):
        place(pkg_idx if pkg_idx is not None else mod_idx, label)
    for _name, label in reversed(by_scope.get("module", [])):
        place(mod_idx, label)
    for name, label in reversed(by_scope.get("class", [])):
        idx = _pick_class_index(node, name, class_idxs, chain)
        place(idx if idx is not None else mod_idx, label)
    return anchors


def resolved_parents(
    node: pytest.Item,
    config: pytest.Config,
) -> tuple[list[HierarchyParent], list[ParametrizeParent]]:
    """The rendered report-tree parents for ``node`` — the single source of truth.

    Shared by ``get_or_create_parent_chain`` (which opens these parents) and the
    early-close counters in ``tally_expected_parents`` (which count them), so the
    two can never key on different identities. Returns ``(hierarchy, parametrize)``
    outer-to-inner:

    * hierarchy: ``(identity, name, doc)`` for each rendered parent, built by
      walking the package/module/class chain and interleaving each scope-promoted
      param (session/package/module/class-scoped parametrized fixture, or a mark
      with ``scope=``) at its scope's level — broader scope nests outside narrower.
      ``identity`` is the ``HierarchyKey`` registry key (the root-to-node path).
    * parametrize: ``(registry key, frame name)`` for each function-scoped
      ``@pytest.mark.parametrize`` axis except the innermost (the leaf is the
      ``step`` fixture's job). Empty when ``sift_parametrize_nesting`` is off or
      the item has no function-scoped params.

    Reads the per-item stash written in ``pytest_itemcollected``; recomputes for
    items a later hook injected without going through it.
    """
    chain = (
        node.stash[hierarchy_key]
        if hierarchy_key in node.stash
        else build_hierarchy_chain(node, config)
    )
    scoped = (
        node.stash[scoped_params_key]
        if scoped_params_key in node.stash
        else build_scoped_params(node)
    )

    anchors = _scoped_param_anchors(node, scoped, chain)

    # Each parent's identity is the literal root-to-node path of kind-tagged
    # segments, so it is unique by construction even when a param label collides
    # with a node name or contains separators. Non-rendered frames still extend
    # the path (keeping descendant identities distinct across modules/classes) but
    # open no step; the next rendered parent attaches to the nearest rendered
    # ancestor via ``_resolve_parent_chain``'s carry-over.
    hierarchy: list[HierarchyParent] = []
    path: HierarchyKey = ()

    # Root-level params (session scope, plus any with no collector frame).
    for label in anchors.pop(None, []):
        path = (*path, ("p", label))
        hierarchy.append((path, label, None))

    for idx, (nodeid, name, _doc, rendered, _scope) in enumerate(chain):
        path = (*path, ("n", nodeid))
        if rendered:
            hierarchy.append((path, name, _doc))
        for label in anchors.get(idx, []):
            path = (*path, ("p", label))
            hierarchy.append((path, label, None))

    parametrize: list[ParametrizeParent] = []
    if PARAMETRIZE_NESTING_OPTION.resolve(config):
        ppath = (
            node.stash[parametrize_path_key]
            if parametrize_path_key in node.stash
            else build_parametrize_path(node)
        )
        if ppath:
            # Key function-axis parents by the full hierarchy path plus the test's
            # param-stripped nodeid, so sibling params share a parent but params
            # under different tests (or scope universes) never merge.
            base = strip_param(node.nodeid)
            key: ParametrizeKey = (path, base)
            for frame in ppath[:-1]:
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
