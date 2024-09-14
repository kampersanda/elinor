#! /bin/bash

set -eux

if [ ! -d trec_eval-9.0.8 ]; then
    wget https://github.com/usnistgov/trec_eval/archive/refs/tags/v9.0.8.tar.gz
    tar -xf v9.0.8.tar.gz
    make -C trec_eval-9.0.8
fi

mkdir -p results
./trec_eval-9.0.8/trec_eval -c -m all_trec trec_eval-9.0.8/test/qrels.test trec_eval-9.0.8/test/results.test > results/trec_eval.txt
cargo run -p evaluate -- -q trec_eval-9.0.8/test/qrels.test -r trec_eval-9.0.8/test/results.test > results/emir-evaluate.txt
