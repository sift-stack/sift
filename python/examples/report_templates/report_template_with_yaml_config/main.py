import os
from pathlib import Path

from dotenv import load_dotenv
from sift_py.grpc.transport import SiftChannelConfig, use_sift_channel
from sift_py.ingestion.config.yaml.load import load_named_expression_modules
from sift_py.report_templates.service import ReportTemplateService
from sift_py.rule.service import RuleService, SubExpression

REPORT_TEMPLATES_DIR = Path().joinpath("report_templates")
RULE_MODULES_DIR = Path().joinpath("rule_modules")
EXPRESSION_MODULES_DIR = Path().joinpath("expression_modules")

if __name__ == "__main__":
    load_dotenv()

    apikey = os.getenv("SIFT_API_KEY")
    assert apikey, "Missing 'SIFT_API_KEY' environment variable."

    base_uri = os.getenv("BASE_URI")
    assert base_uri, "Missing 'BASE_URI' environment variable."

    # Create a gRPC transport channel configured specifically for the Sift API
    sift_channel_config = SiftChannelConfig(uri=base_uri, apikey=apikey)

    # Paths to your rules, named expressions, and report template
    report_templates = REPORT_TEMPLATES_DIR.joinpath("nostromo_report_template.yml")
    rule_modules = RULE_MODULES_DIR.joinpath("rules.yml")
    named_expressions = load_named_expression_modules(
        [
            EXPRESSION_MODULES_DIR.joinpath("kinematics.yml"),
            EXPRESSION_MODULES_DIR.joinpath("string.yml"),
        ]
    )

    with use_sift_channel(sift_channel_config) as channel:
        # First create rules
        rule_service = RuleService(channel)
        rules = rule_service.load_rules_from_yaml(
            paths=[rule_modules],
            sub_expressions=[
                SubExpression("kinetic_energy", named_expressions),
                SubExpression("failure", named_expressions),
            ],
        )

        # Now create report templates
        report_template_service = ReportTemplateService(channel)
        report_template_service.load_report_templates_from_yaml([report_templates])
