# Correctness test compared with the reference implementation

## With trec_eval

```shell
./correctness-test/prepare_trec_eval.sh
./correctness-test/compare_with_trec_eval.py trec_eval-9.0.8/trec_eval target/release/elinor-evaluate
```
