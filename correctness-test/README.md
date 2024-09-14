# correctness-test

```shell
wget https://github.com/usnistgov/trec_eval/archive/refs/tags/v9.0.8.tar.gz
tar -xf v9.0.8.tar.gz
make -C trec_eval-9.0.8
./trec_eval-9.0.8/trec_eval -c -m all_trec trec_eval-9.0.8/test/qrels.test trec_eval-9.0.8/test/results.test
```
