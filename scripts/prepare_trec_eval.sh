#! /bin/bash

set -eux

# TREC EVAL
TREC_VERSION="9.0.8"
if [ -d "trec_eval-$TREC_VERSION" ]; then
    echo "Directory trec_eval-$TREC_VERSION exists."
else
    echo "Directory trec_eval-$TREC_VERSION does not exist."
    rm -f v$TREC_VERSION.tar.gz
    wget https://github.com/usnistgov/trec_eval/archive/refs/tags/v$TREC_VERSION.tar.gz
    tar -xf v$TREC_VERSION.tar.gz
    make -C trec_eval-$TREC_VERSION
fi
