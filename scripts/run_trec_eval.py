#!/usr/bin/env python3

import argparse
import json
import subprocess
import sys


def run_trec_eval(
    trec_eval_dir: str, qrels_file: str, results_file: str
) -> dict[str, str]:
    """Run trec_eval and parse the output."""
    command = f"./{trec_eval_dir}/trec_eval -c -m all_trec {qrels_file} {results_file}"
    result = subprocess.run(command, capture_output=True, shell=True)
    if result.returncode != 0:
        print(result.stderr.decode("utf-8"), file=sys.stderr)
        sys.exit(1)
    parsed: dict[str, str] = {}
    for line in result.stdout.decode("utf-8").split("\n"):
        if not line:
            continue
        metric, _, value = line.split()
        parsed[metric] = value
    return parsed


if __name__ == "__main__":
    p = argparse.ArgumentParser()
    p.add_argument("trec_eval_dir")
    p.add_argument("qrels_file")
    p.add_argument("results_file")
    args = p.parse_args()

    trec_eval_dir: str = args.trec_eval_dir
    qrels_file: str = args.qrels_file
    results_file: str = args.results_file

    trec_results = run_trec_eval(trec_eval_dir, qrels_file, results_file)

    print(
        json.dumps(
            {
                "qrels_file": qrels_file,
                "results_file": results_file,
                "trec_eval_output": trec_results,
            },
            indent=2,
        )
    )
