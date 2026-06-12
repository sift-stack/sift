"""Subpackage of plain pytest tests with no Sift awareness.

Demonstrates that the plugin captures any test's pass/fail with no opt-in
needed — no ``step`` fixture, no markers, no imports from ``sift_client``.
The package directory itself becomes a parent step in the report tree (via
``sift_package_step``, on by default).
"""
