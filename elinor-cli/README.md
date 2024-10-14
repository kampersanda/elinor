# elinor-cli

elinor-cli is a set of command-line tools for evaluating IR systems:

- [elinor-evaluate](#elinor-evaluate): Evaluate the ranking metrics of the system.
- [elinor-compare](#elinor-compare): Compare the metrics of multiple systems with statistical tests.
- [elinor-convert](#elinor-convert): Convert the TREC format into the JSONL format for elinor-evaluate.

## elinor-evaluate

This tool evaluates the ranking metrics of the system.

### Input format

- `query_id`: The ID of the query.
- `doc_id`: The ID of the document.
- `score`: The relevance score of the query-document pair.

```jsonl
{"query_id":"q_1","doc_id":"d_1","score":2}
{"query_id":"q_1","doc_id":"d_7","score":0}
{"query_id":"q_2","doc_id":"d_3","score":2}
```

```jsonl
{"query_id":"q_1","doc_id":"d_1","score":0.65}
{"query_id":"q_1","doc_id":"d_4","score":0.23}
{"query_id":"q_2","doc_id":"d_3","score":0.48}
```

### Example usage

```sh
cargo run --release -p elinor-cli --bin elinor-evaluate -- \
  --gold-jsonl test-data/sample/gold.jsonl \
  --pred-jsonl test-data/sample/pred_1.jsonl \
  --metrics precision@3 ap rr ndcg@3
```

```
precision@3     0.5833
ap      0.8229
rr      0.8125
ndcg@3  0.8286
```

```sh
cargo run --release -p elinor-cli --bin elinor-evaluate -- \
  --gold-jsonl test-data/sample/gold.jsonl \
  --pred-jsonl test-data/sample/pred_1.jsonl \
  --output-csv test-data/sample/pred_1.csv \  # Specify output CSV
  --metrics precision@3 ap rr ndcg@3
```

```csv
query_id,precision@3,ap,rr,ndcg@3
q_1,0.6666666666666666,0.5833333333333333,0.5,0.66967181649423
q_2,0.6666666666666666,1.0,1.0,0.8597186998521972
q_3,0.6666666666666666,0.5833333333333333,0.5,0.6199062332840657
q_4,0.6666666666666666,0.5833333333333333,0.5,0.66967181649423
q_5,0.3333333333333333,1.0,1.0,1.0
q_6,0.6666666666666666,0.8333333333333333,1.0,0.9502344167898356
q_7,0.3333333333333333,1.0,1.0,1.0
q_8,0.6666666666666666,1.0,1.0,0.8597186998521972
```

## elinor-compare

```sh
cargo run --release -p elinor-cli --bin elinor-compare -- \
  --input-csvs test-data/sample/pred_1.csv \
  --input-csvs test-data/sample/pred_2.csv
```

```
# Alias
+----------+-----------------------------+
| Alias    | Path                        |
+----------+-----------------------------+
| System_1 | test-data/sample/pred_1.csv |
| System_2 | test-data/sample/pred_2.csv |
+----------+-----------------------------+

# Means
+-------------+----------+----------+
| Metric      | System_1 | System_2 |
+-------------+----------+----------+
| precision@3 | 0.5833   | 0.2917   |
| ap          | 0.8229   | 0.4479   |
| rr          | 0.8125   | 0.5625   |
| ndcg@3      | 0.8286   | 0.4649   |
+-------------+----------+----------+

# Paired Student's t-test for (System_1 - System_2)
+-------------+--------+--------+--------+--------+---------+---------+
| Metric      | Mean   | Var    | ES     | T Stat | P Value | 95% MOE |
+-------------+--------+--------+--------+--------+---------+---------+
| precision@3 | 0.2917 | 0.0774 | 1.0485 | 2.9656 | 0.0209  | 0.2326  |
| ap          | 0.3750 | 0.1012 | 1.1789 | 3.3343 | 0.0125  | 0.2659  |
| rr          | 0.2500 | 0.0714 | 0.9354 | 2.6458 | 0.0331  | 0.2234  |
| ndcg@3      | 0.3637 | 0.1026 | 1.1356 | 3.2119 | 0.0148  | 0.2677  |
+-------------+--------+--------+--------+--------+---------+---------+

# Bootstrap test (n_resamples=10000)
+-------------+---------+
| Metric      | P Value |
+-------------+---------+
| precision@3 | 0.0255  |
| ap          | 0.0298  |
| rr          | 0.0658  |
| ndcg@3      | 0.0291  |
+-------------+---------+

# Fisher's randomized test (n_iters=10000)
+-------------+---------+
| Metric      | P Value |
+-------------+---------+
| precision@3 | 0.0649  |
| ap          | 0.0645  |
| rr          | 0.1226  |
| ndcg@3      | 0.0627  |
+-------------+---------+
```

```sh
cargo run --release -p elinor-cli --bin elinor-compare -- \
  --input-csvs test-data/sample/pred_1.csv \
  --input-csvs test-data/sample/pred_2.csv \
  --input-csvs test-data/sample/pred_3.csv
```

```
# Alias
+----------+-----------------------------+
| Alias    | Path                        |
+----------+-----------------------------+
| System_1 | test-data/sample/pred_1.csv |
| System_2 | test-data/sample/pred_2.csv |
| System_3 | test-data/sample/pred_3.csv |
+----------+-----------------------------+

# precision@3
+----------+--------+---------+
| System   | Mean   | 95% MOE |
+----------+--------+---------+
| System 1 | 0.5833 | 0.1498  |
| System 2 | 0.2917 | 0.1498  |
| System 3 | 0.4167 | 0.1498  |
+----------+--------+---------+
## Two-way ANOVA without replication
+-----------------+------------+----+----------+--------+---------+
| Factor          | Variation  | DF | Variance | F Stat | P Value |
+-----------------+------------+----+----------+--------+---------+
| Between-systems | 0.3426     | 2  | 0.1713   | 4.3898 | 0.0331  |
| Between-topics  | 0.3287     | 7  | 0.0470   | 1.2034 | 0.3623  |
| Residual        | 0.5463     | 14 | 0.0390   |        |         |
+-----------------+------------+----+----------+--------+---------+
## Between-system effect sizes from Tukey Hsd test
+----------+----------+----------+----------+
| ES       | System_1 | System_2 | System_3 |
+----------+----------+----------+----------+
| System_1 | 0.0000   | 1.4765   | 0.8437   |
| System_2 | -1.4765  | 0.0000   | -0.6328  |
| System_3 | -0.8437  | 0.6328   | 0.0000   |
+----------+----------+----------+----------+
## P-values from randomized Tukey Hsd test (n_iters=10000)
+----------+----------+----------+----------+
| P Value  | System_1 | System_2 | System_3 |
+----------+----------+----------+----------+
| System_1 | 1.0000   | 0.0244   | 0.2589   |
| System_2 | 0.0244   | 1.0000   | 0.6500   |
| System_3 | 0.2589   | 0.6500   | 1.0000   |
+----------+----------+----------+----------+

# ap
+----------+--------+---------+
| System   | Mean   | 95% MOE |
+----------+--------+---------+
| System 1 | 0.8229 | 0.2785  |
| System 2 | 0.4479 | 0.2785  |
| System 3 | 0.4479 | 0.2785  |
+----------+--------+---------+
## Two-way ANOVA without replication
+-----------------+------------+----+----------+--------+---------+
| Factor          | Variation  | DF | Variance | F Stat | P Value |
+-----------------+------------+----+----------+--------+---------+
| Between-systems | 0.7500     | 2  | 0.3750   | 2.7794 | 0.0963  |
| Between-topics  | 0.5182     | 7  | 0.0740   | 0.5487 | 0.7843  |
| Residual        | 1.8889     | 14 | 0.1349   |        |         |
+-----------------+------------+----+----------+--------+---------+
## Between-system effect sizes from Tukey Hsd test
+----------+----------+----------+----------+
| ES       | System_1 | System_2 | System_3 |
+----------+----------+----------+----------+
| System_1 | 0.0000   | 1.0209   | 1.0209   |
| System_2 | -1.0209  | 0.0000   | 0.0000   |
| System_3 | -1.0209  | -0.0000  | 0.0000   |
+----------+----------+----------+----------+
## P-values from randomized Tukey Hsd test (n_iters=10000)
+----------+----------+----------+----------+
| P Value  | System_1 | System_2 | System_3 |
+----------+----------+----------+----------+
| System_1 | 1.0000   | 0.1631   | 0.1631   |
| System_2 | 0.1631   | 1.0000   | 1.0000   |
| System_3 | 0.1631   | 1.0000   | 1.0000   |
+----------+----------+----------+----------+

# rr
+----------+--------+---------+
| System   | Mean   | 95% MOE |
+----------+--------+---------+
| System 1 | 0.8125 | 0.2681  |
| System 2 | 0.5625 | 0.2681  |
| System 3 | 0.5208 | 0.2681  |
+----------+--------+---------+
## Two-way ANOVA without replication
+-----------------+------------+----+----------+--------+---------+
| Factor          | Variation  | DF | Variance | F Stat | P Value |
+-----------------+------------+----+----------+--------+---------+
| Between-systems | 0.3981     | 2  | 0.1991   | 1.5926 | 0.2381  |
| Between-topics  | 0.7396     | 7  | 0.1057   | 0.8452 | 0.5692  |
| Residual        | 1.7500     | 14 | 0.1250   |        |         |
+-----------------+------------+----+----------+--------+---------+
## Between-system effect sizes from Tukey Hsd test
+----------+----------+----------+----------+
| ES       | System_1 | System_2 | System_3 |
+----------+----------+----------+----------+
| System_1 | 0.0000   | 0.7071   | 0.8250   |
| System_2 | -0.7071  | 0.0000   | 0.1179   |
| System_3 | -0.8250  | -0.1179  | 0.0000   |
+----------+----------+----------+----------+
## P-values from randomized Tukey Hsd test (n_iters=10000)
+----------+----------+----------+----------+
| P Value  | System_1 | System_2 | System_3 |
+----------+----------+----------+----------+
| System_1 | 1.0000   | 0.4018   | 0.2543   |
| System_2 | 0.4018   | 1.0000   | 0.9821   |
| System_3 | 0.2543   | 0.9821   | 1.0000   |
+----------+----------+----------+----------+

# ndcg@3
+----------+--------+---------+
| System   | Mean   | 95% MOE |
+----------+--------+---------+
| System 1 | 0.8286 | 0.2519  |
| System 2 | 0.4649 | 0.2519  |
| System 3 | 0.5461 | 0.2519  |
+----------+--------+---------+
## Two-way ANOVA without replication
+-----------------+------------+----+----------+--------+---------+
| Factor          | Variation  | DF | Variance | F Stat | P Value |
+-----------------+------------+----+----------+--------+---------+
| Between-systems | 0.5831     | 2  | 0.2916   | 2.6414 | 0.1063  |
| Between-topics  | 0.3676     | 7  | 0.0525   | 0.4758 | 0.8366  |
| Residual        | 1.5454     | 14 | 0.1104   |        |         |
+-----------------+------------+----+----------+--------+---------+
## Between-system effect sizes from Tukey Hsd test
+----------+----------+----------+----------+
| ES       | System_1 | System_2 | System_3 |
+----------+----------+----------+----------+
| System_1 | 0.0000   | 1.0946   | 0.8504   |
| System_2 | -1.0946  | 0.0000   | -0.2443  |
| System_3 | -0.8504  | 0.2443   | 0.0000   |
+----------+----------+----------+----------+
## P-values from randomized Tukey Hsd test (n_iters=10000)
+----------+----------+----------+----------+
| P Value  | System_1 | System_2 | System_3 |
+----------+----------+----------+----------+
| System_1 | 1.0000   | 0.1065   | 0.2874   |
| System_2 | 0.1065   | 1.0000   | 0.9107   |
| System_3 | 0.2874   | 0.9107   | 1.0000   |
+----------+----------+----------+----------+
```

## elinor-convert

```sh
cargo run --release -p elinor-cli --bin elinor-convert -- \
  --input-trec qrels.trec \
  --output-jsonl qrels.jsonl \
  --rel-type gold
```

```sh
cargo run --release -p elinor-cli --bin elinor-convert -- \
  --input-trec run.trec \
  --output-jsonl run.jsonl \
  --rel-type pred
```

## Licensing

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
