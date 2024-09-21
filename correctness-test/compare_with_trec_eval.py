#!/usr/bin/env python3
"""
Script to check the correctness of elinor by comparing its output with trec_eval.
"""

import argparse
import subprocess
import sys


def run_trec_eval(
    trec_eval_dir: str, qrels_file: str, results_file: str
) -> dict[str, str]:
    command = f"./{trec_eval_dir}/trec_eval -c -m all_trec {qrels_file} {results_file}"
    print(f"Running: {command}")
    result = subprocess.run(command, capture_output=True, shell=True)
    parsed: dict[str, str] = {}
    for line in result.stdout.decode("utf-8").split("\n"):
        if not line:
            continue
        metric, _, value = line.split()
        parsed[metric] = value
    return parsed


def run_elinor_evaluate(
    elinor_dir: str, qrels_file: str, results_file: str
) -> dict[str, str]:
    ks = [0, 1, 5, 10, 15, 20, 30, 100, 200, 500, 1000]
    ks_args = " ".join([f"-k {k}" for k in ks])
    command = (
        f"./{elinor_dir}/elinor-evaluate -g {qrels_file} -p {results_file} {ks_args}"
    )
    print(f"Running: {command}")
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
    p.add_argument("trec_eval_dir")
    p.add_argument("elinor_dir")
    p.add_argument("--decimal-places", type=int, default=3)
    args = p.parse_args()

    trec_eval_dir: str = args.trec_eval_dir
    elinor_dir: str = args.elinor_dir
    decimal_places: int = args.decimal_places

    failed_ids = []
    test_data = [
        (f"{trec_eval_dir}/test/qrels.test", f"{trec_eval_dir}/test/results.test"),
        (f"{trec_eval_dir}/test/qrels.rel_level", f"{trec_eval_dir}/test/results.test"),
    ]

    for data_id, (qrels_file, results_file) in enumerate(test_data, 1):
        trec_results = run_trec_eval(trec_eval_dir, qrels_file, results_file)
        elinor_results = run_elinor_evaluate(elinor_dir, qrels_file, results_file)

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

        print("case_id\ttrec_metric\telinor_metric\ttrec_score\telinor_score\tmatch")
        for metric_id, (trec_metric, elinor_metric) in enumerate(metric_pairs, 1):
            case_id = f"{data_id}.{metric_id}"
            trec_score = trec_results[trec_metric]
            elinor_score = elinor_results[elinor_metric]
            match = compare_decimal_places(trec_score, elinor_score, decimal_places)
            row = f"{case_id}\t{trec_metric}\t{elinor_metric}\t{trec_score}\t{elinor_score}\t{match}"
            print(row)

            if not match:
                failed_ids.append(case_id)

    if failed_ids:
        print("Mismatched cases:", failed_ids, file=sys.stderr)
        sys.exit(1)
    else:
        print(f"All metrics match 🎉 with {decimal_places=}")
