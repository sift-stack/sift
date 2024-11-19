import os
from pathlib import Path

from dotenv import load_dotenv
from sift_py.report_templates.service import ReportTemplateService
from report_template_config import load_rules, nostromos_report_template
from sift_py.grpc.transport import SiftChannelConfig, use_sift_channel
from sift_py.rule.service import RuleService, SubExpression

TELEMETRY_CONFIGS_DIR = Path().joinpath("telemetry_configs")
RULE_MODULES_DIR = Path().joinpath("rule_modules")


if __name__ == "__main__":
    load_dotenv()

    apikey = os.getenv("SIFT_API_KEY")
    assert apikey, "Missing 'SIFT_API_KEY' environment variable."

    base_uri = os.getenv("BASE_URI")
    assert base_uri, "Missing 'BASE_URI' environment variable."

    # Create a gRPC transport channel configured specifically for the Sift API
    sift_channel_config = SiftChannelConfig(uri=base_uri, apikey=apikey, use_ssl=False)

    with use_sift_channel(sift_channel_config) as channel:
        # First create rules
        rule_service = RuleService(channel)
        rules = load_rules()  # Load rules from python
        [rule_service.create_or_update_rule(rule) for rule in rules]

        # Now create report template
        report_template_service = ReportTemplateService(channel)
        report_template_to_create = nostromos_report_template()
        report_template_to_create.rules = rules  # Add the rules we just created
        report_template_service.create_or_update_report_template(report_template_to_create)

        # Then make some updates to the template we created (for the sake of the example)
        report_template_to_update = report_template_service.get_report_template(
            client_key="nostromo-report-template"
        )
        if report_template_to_update:
            rules = [
                rule for rule in report_template_to_update.rules if rule.name != "overheating"
            ]  # Remove some rules
            report_template_to_update.rules = rules
            report_template_to_update.description = (
                "A report template for the Nostromo without overheating rule"
            )
            report_template_service.create_or_update_report_template(report_template_to_update)
