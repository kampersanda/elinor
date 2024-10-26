#!/usr/bin/env python3

import argparse
import json
import subprocess
import sys


def run_elinor_evaluate(
    target_dir: str, qrels_jsonl: str, results_jsonl: str, metrics: list[str]
) -> dict[str, str]:
    metric_args = " ".join([f"-m {metric}" for metric in metrics])
    command = f"./{target_dir}/elinor-evaluate -t {qrels_jsonl} -p {results_jsonl} {metric_args}"
    result = subprocess.run(command, capture_output=True, shell=True)
    if result.returncode != 0:
        print(result.stderr.decode("utf-8"), file=sys.stderr)
        sys.exit(1)
    parsed: dict[str, str] = {}
    for line in result.stdout.decode("utf-8").split("\n"):
        if not line:
            continue
        metric, value = line.split()
        parsed[metric] = value
    return parsed


def compare_decimal_places(a: str, b: str, decimal_places: int) -> bool:
    return round(float(a), decimal_places) == round(float(b), decimal_places)


if __name__ == "__main__":
    p = argparse.ArgumentParser()
    p.add_argument("target_dir", help="e.g., target/release")
    p.add_argument("qrels_jsonl")
    p.add_argument("results_jsonl")
    p.add_argument("trec_output_json")
    p.add_argument("--decimal-places", type=int, default=3)
    args = p.parse_args()

    target_dir: str = args.target_dir
    qrels_jsonl: str = args.qrels_jsonl
    results_jsonl: str = args.results_jsonl
    trec_output_json: str = args.trec_output_json
    decimal_places: int = args.decimal_places

    with open(trec_output_json) as f:
        trec_results = json.load(f)

    # (trec_eval, elinor)
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
    ks = [5, 10, 15, 20, 30, 100, 200, 500, 1000]
    metric_pairs.extend([(f"P_{k}", f"precision@{k}") for k in ks])
    metric_pairs.extend([(f"recall_{k}", f"recall@{k}") for k in ks])
    metric_pairs.extend([(f"map_cut_{k}", f"ap@{k}") for k in ks])
    metric_pairs.extend([(f"ndcg_cut_{k}", f"ndcg@{k}") for k in ks])

    elinor_results = run_elinor_evaluate(
        target_dir,
        qrels_jsonl,
        results_jsonl,
        [metric for _, metric in metric_pairs],
    )

    # Add some additional basic metrics
    metric_pairs.extend(
        [
            ("num_q", "n_queries_in_true"),
            ("num_q", "n_queries_in_pred"),
            ("num_ret", "n_docs_in_pred"),
            ("num_rel", "n_relevant_docs"),
        ]
    )

    failed_rows: list[str] = []
    for trec_metric, elinor_metric in metric_pairs:
        trec_score = trec_results["trec_eval_output"][trec_metric]
        elinor_score = elinor_results[elinor_metric]
        match = compare_decimal_places(trec_score, elinor_score, decimal_places)
        row = f"{trec_metric}\t{elinor_metric}\t{trec_score}\t{elinor_score}\t{match}"
        print(f"{trec_metric}\t{elinor_metric}\t{trec_score}\t{elinor_score}\t{match}")
        if not match:
            failed_rows.append(row)

    if failed_rows:
        print("Mismatched cases:", file=sys.stderr)
        for row in failed_rows:
            print(row, file=sys.stderr)
        sys.exit(1)
    else:
        print(f"All metrics match 🎉 with {decimal_places=}", file=sys.stderr)
