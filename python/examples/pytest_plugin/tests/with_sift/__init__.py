"""Subpackage of tests that use the Sift plugin APIs explicitly.

Demonstrates ``step.measure`` (numeric / string / bool bounds), nested
``step.substep``, gate markers, class and nested-class step nesting, stacked
parametrize, and ``step.report_outcome``. The package directory itself
becomes a parent step in the report tree (via ``sift_package_step``, on by
default).
"""
