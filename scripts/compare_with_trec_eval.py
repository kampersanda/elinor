"""
Script to check the correctness of elinor by comparing its output with trec_eval.

Usage:
    $ python3 ./scripts/compare_with_trec_eval.py ./trec_eval-9.0.8/trec_eval ./target/release/elinor-evaluate
"""

import argparse
import subprocess
import sys


def run_trec_eval(trec_eval: str, qrels_file: str, results_file: str) -> dict[str, str]:
    command = f"./{trec_eval} -c -m all_trec {qrels_file} {results_file}"
    result = subprocess.run(command, capture_output=True, shell=True)
    parsed: dict[str, str] = {}
    for line in result.stdout.decode("utf-8").split("\n"):
        if not line:
            continue
        metric, _, value = line.split()
        parsed[metric] = value
    return parsed


def run_elinor_evaluate(
    elinor_evaluate: str, qrels_file: str, results_file: str
) -> dict[str, str]:
    ks = [0, 1, 5, 10, 15, 20, 30, 100, 200, 500, 1000]
    ks_args = " ".join([f"-k {k}" for k in ks])
    command = f"./{elinor_evaluate} -q {qrels_file} -r {results_file} {ks_args}"
    result = subprocess.run(command, capture_output=True, shell=True)
    parsed: dict[str, str] = {}
    for line in result.stdout.decode("utf-8").split("\n"):
        if not line:
            continue
        metric, value = line.split()
        parsed[metric] = value
    return parsed


if __name__ == "__main__":
    p = argparse.ArgumentParser()
    p.add_argument("trec_eval")
    p.add_argument("elinor_evaluate")
    args = p.parse_args()

    trec_eval: str = args.trec_eval
    elinor_evaluate: str = args.elinor_evaluate

    qrels_file = "trec_eval-9.0.8/test/qrels.test"
    results_file = "trec_eval-9.0.8/test/results.test"

    trec_results = run_trec_eval(trec_eval, qrels_file, results_file)
    elinor_results = run_elinor_evaluate(elinor_evaluate, qrels_file, results_file)

    ks = [5, 10, 15, 20, 30, 100, 200, 500, 1000]

    metric_pairs = []
    metric_pairs.extend([(f"success_{k}", f"success@{k}") for k in [1, 5, 10]])
    metric_pairs.extend(
        [
            ("set_P", "precision"),
            ("set_recall", "recall"),
            ("set_F", "f1"),
            ("Rprec", "r_precision"),
            ("map", "ap"),
            ("recip_rank", "rr"),
            ("ndcg", "ndcg"),
            ("bpref", "bpref"),
        ]
    )
    metric_pairs.extend([(f"P_{k}", f"precision@{k}") for k in ks])
    metric_pairs.extend([(f"recall_{k}", f"recall@{k}") for k in ks])
    metric_pairs.extend([(f"map_cut_{k}", f"ap@{k}") for k in ks])
    metric_pairs.extend([(f"ndcg_cut_{k}", f"ndcg@{k}") for k in ks])

    failed_rows = []

    print("trec_metric\telinor_metric\ttrec_score\telinor_score\tmatch")
    for trec_metric, elinor_metric in metric_pairs:
        trec_score = trec_results[trec_metric]
        elinor_score = elinor_results[elinor_metric]
        match = trec_score == elinor_score
        row = f"{trec_metric}\t{elinor_metric}\t{trec_score}\t{elinor_score}\t{match}"
        if not match:
            failed_rows.append(row)
        print(row)

    if failed_rows:
        print("\nFailed rows:", file=sys.stderr)
        for row in failed_rows:
            print(row, file=sys.stderr)
        sys.exit(1)
