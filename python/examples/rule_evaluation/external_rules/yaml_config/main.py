import argparse
import os
from datetime import datetime
from pathlib import Path

from dotenv import load_dotenv
from sift_py.grpc.transport import SiftChannelConfig, use_sift_channel
from sift_py.rule_evaluation.service import RuleEvaluationService

RULE_MODULES_DIR = Path().joinpath("rule_modules")


if __name__ == "__main__":
    """
    Example of evaluating external rules against a run on the 'NostromoLV426' asset.
    You must already have a run created with the NostromoLV426 asset.
    """

    def parse_args():
        parser = argparse.ArgumentParser(
            description="Evaluate external rules against a specific run."
        )
        parser.add_argument(
            "--run_id", required=True, help="The ID of the run to evaluate rules against."
        )
        return parser.parse_args()

    args = parse_args()
    run_id = args.run_id

    load_dotenv()

    apikey = os.getenv("SIFT_API_KEY")
    if apikey is None:
        raise Exception("Missing 'SIFT_API_KEY' environment variable.")

    base_uri = os.getenv("BASE_URI")
    if base_uri is None:
        raise Exception("Missing 'BASE_URI' environment variable.")

    sift_channel_config = SiftChannelConfig(uri=base_uri, apikey=apikey)

    paths = [
        RULE_MODULES_DIR.joinpath("voltage.yml"),
        RULE_MODULES_DIR.joinpath("velocity.yml"),
        RULE_MODULES_DIR.joinpath("nostromo.yml"),
    ]

    with use_sift_channel(sift_channel_config) as channel:
        # Evaluate the rules from a yaml config as external rules.
        rule_eval_service = RuleEvaluationService(channel)
        report = rule_eval_service.evaluate_external_rules_from_yaml(
            run_id,
            paths,
            report_name=f"Rule Evaluation Example ({datetime.now().strftime('%Y-%m-%d %H:%M:%S')})",
        )

        # Wait for the report to finish then print the results.
        print("Waiting up to 60s for report to finish ...")
        print(f"Report ID: {report.report_id}")
        finished = report.wait_until_done(timeout=60)
        if not finished:
            print("Report did not finish in 60s")
        else:
            total_open = 0
            total_failed = 0
            total_passed = 0

            results = report.get_results()
            for rule_summary in results.summaries:
                total_open += rule_summary.num_open
                total_failed += rule_summary.num_failed
                total_passed += rule_summary.num_passed

            print("Report Summary:")
            print(f"Total Open: {total_open}")
            print(f"Total Failed: {total_failed}")
            print(f"Total Passed: {total_passed}")
