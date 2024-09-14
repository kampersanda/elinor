"""
Script to check the correctness of emir by comparing its output with trec_eval.

Usage:
    $ python3 ./scripts/compare_with_trec_eval.py ./target/release/evaluate
"""

import argparse
import os
import subprocess
import sys


def download_trec_eval():
    if os.path.exists("trec_eval-9.0.8"):
        print("trec_eval-9.0.8 already exists", file=sys.stderr)
        return
    subprocess.run(
        "wget https://github.com/usnistgov/trec_eval/archive/refs/tags/v9.0.8.tar.gz",
        shell=True,
    )
    subprocess.run("tar -xf v9.0.8.tar.gz", shell=True)
    subprocess.run("make -C trec_eval-9.0.8", shell=True)


def run_trec_eval() -> dict[str, str]:
    command = "./trec_eval-9.0.8/trec_eval -c -m all_trec trec_eval-9.0.8/test/qrels.test trec_eval-9.0.8/test/results.test"
    result = subprocess.run(command, capture_output=True, shell=True)
    parsed: dict[str, str] = {}
    for line in result.stdout.decode("utf-8").split("\n"):
        if not line:
            continue
        metric, _, value = line.split()
        parsed[metric] = value
    return parsed


def run_emir_eval(emir_exe: str) -> dict[str, str]:
    ks = [0, 1, 5, 10, 15, 20, 30, 100, 200, 500, 1000]
    command = (
        f"{emir_exe} -q trec_eval-9.0.8/test/qrels.test -r trec_eval-9.0.8/test/results.test"
        + "".join([f" -k {k}" for k in ks])
    )
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
    p.add_argument("emir_exe")
    args = p.parse_args()

    download_trec_eval()
    trec_results = run_trec_eval()
    emir_results = run_emir_eval(args.emir_exe)

    ks = [5, 10, 15, 20, 30, 100, 200, 500, 1000]

    metric_pairs = []
    metric_pairs.extend([(f"success_{k}", f"success@{k}") for k in [1, 5, 10]])
    metric_pairs.extend(
        [
            ("set_P", "precision"),
            ("set_recall", "recall"),
            ("set_F", "f1"),
            ("map", "map"),
            ("recip_rank", "mrr"),
            ("ndcg", "ndcg"),
        ]
    )
    metric_pairs.extend([(f"P_{k}", f"precision@{k}") for k in ks])
    metric_pairs.extend([(f"recall_{k}", f"recall@{k}") for k in ks])
    metric_pairs.extend([(f"map_cut_{k}", f"map@{k}") for k in ks])
    metric_pairs.extend([(f"ndcg_cut_{k}", f"ndcg@{k}") for k in ks])

    print("trec_metric\temir_metric\ttrec_score\temir_score\tmatch")
    for trec_metric, emir_metric in metric_pairs:
        trec_score = trec_results[trec_metric]
        emir_score = emir_results[emir_metric]
        match = trec_score == emir_score
        print(f"{trec_metric}\t{emir_metric}\t{trec_score}\t{emir_score}\t{match}")