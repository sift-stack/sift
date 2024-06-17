from __future__ import annotations
from .config import (
    RuleActionCreateDataReviewAnnotation,
    RuleActionCreatePhaseAnnotation,
    RuleActionKind,
)
from .yaml import (
    NamedExpressionYamlSpec,
    NamedExpressionsYamlSpec,
    RulesYamlSpec,
    rule_config_from_yaml,
)
from typing import cast

import yaml


def test_rule_config():
    rules_yaml = cast(RulesYamlSpec, yaml.safe_load(TEST_RULE_YAML_STR))
    raw_rules = rules_yaml["rules"]

    assert raw_rules is not None
    assert len(raw_rules) == 3

    overheating, speeding, failures = raw_rules
    assert overheating.get("name") == "overheating"
    assert overheating.get("description") == "Checks for vehicle overheating"
    assert overheating.get("expression") == 'vehicle_state == "Accelerating" && temperature > 80'
    assert overheating.get("type") == "phase"

    assert speeding.get("name") == "speeding"
    assert speeding.get("description") == "Checks high vehicle speed"
    assert speeding.get("expression") == "mainmotor.velocity > 20"
    assert speeding.get("type") == "phase"

    assert failures.get("name") == "failures"
    assert failures.get("description") == "Checks for failure logs"
    assert failures.get("type") == "review"

    named_expression = cast(NamedExpressionYamlSpec, failures.get("expression"))
    assert named_expression is not None
    assert named_expression.get("name") == "log_substring_contains"

    identifiers = named_expression.get("identifiers")
    assert len(identifiers.items()) == 2

    overheating_rule = rule_config_from_yaml(overheating)
    assert overheating_rule.name == "overheating"
    assert overheating_rule.description == "Checks for vehicle overheating"
    assert overheating_rule.expression == 'vehicle_state == "Accelerating" && temperature > 80'
    assert overheating_rule.action.kind() == RuleActionKind.ANNOTATION
    assert isinstance(overheating_rule.action, RuleActionCreatePhaseAnnotation)

    speeding_rule = rule_config_from_yaml(speeding)
    assert speeding_rule.name == "speeding"
    assert speeding_rule.description == "Checks high vehicle speed"
    assert speeding_rule.expression == "mainmotor.velocity > 20"
    assert speeding_rule.action.kind() == RuleActionKind.ANNOTATION
    assert isinstance(speeding_rule.action, RuleActionCreatePhaseAnnotation)


def test_named_expressions():
    named_expressions = cast(
        NamedExpressionsYamlSpec, yaml.safe_load(TEST_NAMED_EXPRESSIONS_YAML_STR)
    )

    log_substring_contains = named_expressions.get("log_substring_contains")
    assert log_substring_contains is not None
    assert log_substring_contains == "contains($1, $2)"

    is_even = named_expressions.get("is_even")
    assert is_even is not None
    assert is_even == "mod($1, 2) == 0"


def test_rule_config_with_named_expression():
    rules_yaml = cast(RulesYamlSpec, yaml.safe_load(TEST_RULE_YAML_STR))
    raw_rules = rules_yaml["rules"]
    assert raw_rules is not None

    failures = raw_rules[2]
    assert failures.get("name") == "failures"

    expression = failures.get("expression")
    assert isinstance(failures.get("expression"), dict)
    expression = cast(dict, expression)
    assert expression.get("name") == "log_substring_contains"

    ident_map = expression.get("identifiers")
    assert ident_map is not None
    assert ident_map.get("$1") == "log"
    assert ident_map.get("$2") == '"ERROR"'

    named_expressions = cast(
        NamedExpressionsYamlSpec, yaml.safe_load(TEST_NAMED_EXPRESSIONS_YAML_STR)
    )
    failures_rule = rule_config_from_yaml(failures, named_expressions)
    assert failures_rule.name == "failures"
    assert failures_rule.description == "Checks for failure logs"
    assert failures_rule.action.kind() == RuleActionKind.ANNOTATION
    assert isinstance(failures_rule.action, RuleActionCreateDataReviewAnnotation)
    assert failures_rule.expression == 'contains(log, "ERROR")'


TEST_RULE_YAML_STR = """
rules:
  - name: overheating
    description: Checks for vehicle overheating
    expression: vehicle_state == "Accelerating" && temperature > 80
    type: phase

  - name: speeding
    description: Checks high vehicle speed
    expression: mainmotor.velocity > 20
    type: phase

  - name: failures
    description: Checks for failure logs
    type: review
    assignee: homer@simpsons.com
    expression:
      name: log_substring_contains
      identifiers:
          $1: log
          $2: '\"ERROR\"'
    tags:
        - foo
        - bar
        - baz
"""

TEST_NAMED_EXPRESSIONS_YAML_STR = """
log_substring_contains:
  contains($1, $2)
is_even:
  mod($1, 2) == 0
"""
