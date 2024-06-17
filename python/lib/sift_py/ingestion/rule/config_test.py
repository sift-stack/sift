from .config import RuleConfig, RuleActionCreatePhaseAnnotation


def test_rule_config_basic_expression():
    expression = "voltage > 10"
    config = RuleConfig(
        name="High Voltage",
        description="Rock & Roll",
        expression=expression,
        action=RuleActionCreatePhaseAnnotation(),
    )
    assert config.expression == expression


def test_rule_config_test_single_channel_placeholder():
    expression = "$1 > 10"
    config = RuleConfig(
        name="voltage",
        description="high voltage",
        expression=expression,
        action=RuleActionCreatePhaseAnnotation(),
        ident_map={"$1": "voltage"},
    )
    assert config.expression == "voltage > 10"


def test_rule_config_test_multi_channel_placeholder():
    expression = '$1 == "Accelerating" && $2 > $3'
    config = RuleConfig(
        name="overheating",
        description="checks if vehicle overheats while accelerating",
        expression=expression,
        action=RuleActionCreatePhaseAnnotation(),
        ident_map={
            "$1": "vehicle_state",
            "$2": "motor.temperature",
            "$3": 80,
        },
    )
    assert config.expression == 'vehicle_state == "Accelerating" && motor.temperature > 80'
