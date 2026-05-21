"""Mixin that exposes ``is_simulated`` on test-results entity types."""

from __future__ import annotations


class SimulatedMixin:
    """Mixin for sift_types whose response can be produced by the simulate path.

    The low-level wrapper stamps ``_simulated=True`` on entities it returns from
    a simulated branch (see ``TestResultsLowLevelClient._mark_simulated``). This
    mixin exposes that flag as a read-only ``is_simulated`` property so
    consumers and tests can detect when an instance was synthesized rather than
    round-tripped through Sift.

    Inheriting classes are expected to declare a private field
    ``_simulated: bool = False`` so pydantic tracks the default correctly.
    """

    _simulated: bool

    @property
    def is_simulated(self) -> bool:
        """True when this instance was returned from the simulate path.

        Set by the low-level wrapper when the call short-circuited to a
        synthesized response (either ``SiftClient._simulate`` mode or per-call
        ``log_file`` / ``simulate=True``). False for entities returned from a
        normal online call or constructed manually outside the SDK. Offline
        mode also reports True since responses are synthesized prior to
        replay.
        """
        return self._simulated
