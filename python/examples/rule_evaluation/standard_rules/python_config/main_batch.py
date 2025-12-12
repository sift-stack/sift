import argparse
import os
import time
from datetime import datetime

from dotenv import load_dotenv
from rule_configs_batch import load_nostromos_lv_426_rule_configs
from sift_py.grpc.transport import SiftChannelConfig, use_sift_channel
from sift_py.rule.service import RuleService
from sift_py.rule_evaluation.service import RuleEvaluationService

if __name__ == "__main__":
    """
    Example of evaluating external rules (created via batch API) against a run on the 'NostromoLV426' asset.
    You must already have a run created with the NostromoLV426 asset.
    
    This example uses the batch API (BatchUpdateRules) to create all 850 rules in a single RPC call,
    as opposed to the individual create_or_update_rules approach.
    """

    script_start_time = time.perf_counter()
    print("=" * 80)
    print("SCRIPT STARTING (BATCH MODE)")
    print("=" * 80)

    def parse_args():
        parser = argparse.ArgumentParser(description="Evaluate external rules (batch) against a specific run.")
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

    rule_configs = load_nostromos_lv_426_rule_configs()
    print(f"Loaded {len(rule_configs)} rule configs (external rules for batch API)")

    with use_sift_channel(sift_channel_config) as channel:
        # Create rules using batch API (single RPC call for all rules)
        rule_service = RuleService(channel)
        print(f"\n[MAIN] Calling create_external_rules (BATCH) with {len(rule_configs)} rules...")
        print(f"[MAIN] This will make a single BatchUpdateRules RPC call instead of {len(rule_configs)} individual calls")
        print(f"[MAIN] Start time: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
        
        create_rules_start = time.perf_counter()
        rule_identifiers = rule_service.create_external_rules(rule_configs)
        create_rules_end = time.perf_counter()
        create_rules_duration = create_rules_end - create_rules_start
        
        print(f"[MAIN] Finished create_external_rules (BATCH)")
        print(f"[MAIN] End time: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
        print(f"[MAIN] create_external_rules (BATCH) took {create_rules_duration:.2f} seconds ({create_rules_duration:.3f} seconds)")
        print(f"[MAIN] Created {len(rule_identifiers)} rules in batch")
        print(f"[MAIN] Average time per rule: {create_rules_duration / len(rule_configs):.4f} seconds")

        # Summary metrics after rule creation
        print("\n" + "=" * 80)
        print("BATCH RULE CREATION SUMMARY")
        print("=" * 80)
        print(f"Total rules requested: {len(rule_configs)}")
        print(f"Total rules created: {len(rule_identifiers)}")
        print(f"Success rate: {(len(rule_identifiers) / len(rule_configs) * 100):.1f}%")
        print(f"Total time: {create_rules_duration:.3f} seconds")
        print(f"Throughput: {len(rule_identifiers) / create_rules_duration:.2f} rules/second")
        print(f"Average time per rule: {create_rules_duration / len(rule_configs):.4f} seconds")
        print("=" * 80)

        # Evaluate the rules as external rules.
        print(f"\n[MAIN] Starting external rule evaluation...")
        eval_start = time.perf_counter()
        rule_eval_service = RuleEvaluationService(channel)
        report = rule_eval_service.evaluate_external_rules(
            run_id,
            rule_configs,
            report_name=f"Rule Evaluation Example (Batch) ({datetime.now().strftime('%Y-%m-%d %H:%M:%S')})",
        )
        eval_end = time.perf_counter()
        eval_duration = eval_end - eval_start
        print(f"[MAIN] External rule evaluation request took {eval_duration:.3f} seconds")

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

    script_end_time = time.perf_counter()
    script_total_duration = script_end_time - script_start_time
    
    print("=" * 80)
    print("SCRIPT ENDING (BATCH MODE)")
    print(f"Total script execution time: {script_total_duration:.2f} seconds ({script_total_duration:.3f} seconds)")
    print("=" * 80)

