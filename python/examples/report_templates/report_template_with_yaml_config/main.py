import os
from datetime import datetime
from pathlib import Path

from dotenv import load_dotenv
from sift_py.grpc.transport import SiftChannelConfig, use_sift_channel
from sift_py.report_templates.service import ReportTemplateService
from sift_py.rule.service import RuleService

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

    with use_sift_channel(sift_channel_config) as channel:
        # First create rules
        rule_service = RuleService(channel)
        rules = rule_service.load_rules_from_yaml(
            paths=[rule_modules],
        )

        # Now create report templates
        report_template_service = ReportTemplateService(channel)
        report_template_service.load_report_templates_from_yaml([report_templates])

        # Archive one template, for the sake of example
        report_template_to_update = report_template_service.get_report_template(
            client_key="nostromo-report-template-1"
        )
        if report_template_to_update:
            report_template_to_update.archived_date = datetime.now()
            report_template_service.create_or_update_report_template(report_template_to_update)
