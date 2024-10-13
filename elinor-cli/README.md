# elinor-cli

## elinor-evaluate

```sh
cargo run --release -p elinor-cli --bin elinor-evaluate -- \
  --gold-jsonl test-data/toy/gold.jsonl \
  --pred-jsonl test-data/toy/pred_1.jsonl \
  --metrics precision@3 ap rr ndcg@3
```

```
precision@3     0.2667
ap      0.3500
rr      0.5000
ndcg@3  0.4480
```

```sh
cargo run --release -p elinor-cli --bin elinor-evaluate -- \
  --gold-jsonl test-data/toy/gold.jsonl \
  --pred-jsonl test-data/toy/pred_1.jsonl \
  --output-csv test-data/toy/pred_1.csv \  # Added
  --metrics precision@3 ap rr ndcg@3
```

## elinor-compare

```sh
cargo run --release -p elinor-cli --bin elinor-compare -- \
  --input-csvs test-data/toy/pred_1.csv \
  --input-csvs test-data/toy/pred_2.csv
```

```
# Alias
+----------+--------------------------+
| Alias    | Path                     |
+----------+--------------------------+
| System_1 | test-data/toy/pred_1.csv |
| System_2 | test-data/toy/pred_2.csv |
+----------+--------------------------+

# Means
+-------------+----------+----------+
| Metric      | System_1 | System_2 |
+-------------+----------+----------+
| precision@3 | 0.2667   | 0.4667   |
| ap          | 0.3500   | 0.8167   |
| rr          | 0.5000   | 0.9000   |
| ndcg@3      | 0.4480   | 0.8480   |
+-------------+----------+----------+

# Paired Student's t-test for (System_1 - System_2)
+-------------+---------+--------+---------+---------+---------+---------+
| Metric      | Mean    | Var    | ES      | T Stat  | P Value | 95% MOE |
+-------------+---------+--------+---------+---------+---------+---------+
| precision@3 | -0.2000 | 0.0889 | -0.6708 | -1.5000 | 0.2080  | 0.3702  |
| ap          | -0.4667 | 0.5368 | -0.6369 | -1.4242 | 0.2275  | 0.9097  |
| rr          | -0.4000 | 0.4250 | -0.6136 | -1.3720 | 0.2420  | 0.8095  |
| ndcg@3      | -0.4000 | 0.4354 | -0.6062 | -1.3555 | 0.2467  | 0.8193  |
+-------------+---------+--------+---------+---------+---------+---------+

# Bootstrap test (n_resamples=10000)
+-------------+---------+
| Metric      | P Value |
+-------------+---------+
| precision@3 | 0.0583  |
| ap          | 0.3108  |
| rr          | 0.2406  |
| ndcg@3      | 0.2486  |
+-------------+---------+

# Fisher's randomized test (n_iters=10000)
+-------------+---------+
| Metric      | P Value |
+-------------+---------+
| precision@3 | 0.3871  |
| ap          | 0.2521  |
| rr          | 0.3792  |
| ndcg@3      | 0.3836  |
+-------------+---------+
```

```sh
cargo run --release -p elinor-cli --bin elinor-compare -- \
  --input-csvs test-data/toy/pred_1.csv \
  --input-csvs test-data/toy/pred_2.csv \
  --input-csvs test-data/toy/pred_3.csv
```

```
# Alias
+----------+--------------------------+
| Alias    | Path                     |
+----------+--------------------------+
| System_1 | test-data/toy/pred_1.csv |
| System_2 | test-data/toy/pred_2.csv |
| System_3 | test-data/toy/pred_3.csv |
+----------+--------------------------+

# precision@3
+----------+--------+---------+
| System   | Mean   | 95% MOE |
+----------+--------+---------+
| System 1 | 0.2667 | 0.1883  |
| System 2 | 0.4667 | 0.1883  |
| System 3 | 0.3333 | 0.1883  |
+----------+--------+---------+
## Two-way ANOVA without replication
+-----------------+------------+----+----------+--------+---------+
| Factor          | Variation  | DF | Variance | F Stat | P Value |
+-----------------+------------+----+----------+--------+---------+
| Between-systems | 0.1037     | 2  | 0.0519   | 1.5556 | 0.2687  |
| Between-queries | 0.4000     | 4  | 0.1000   | 3.0000 | 0.0870  |
| Residual        | 0.2667     | 8  | 0.0333   |        |         |
+-----------------+------------+----+----------+--------+---------+
## Between-system effect sizes from Tukey Hsd test
+----------+----------+----------+----------+
| ES       | System_1 | System_2 | System_3 |
+----------+----------+----------+----------+
| System_1 | 0.0000   | -1.0954  | -0.3651  |
| System_2 | 1.0954   | 0.0000   | 0.7303   |
| System_3 | 0.3651   | -0.7303  | 0.0000   |
+----------+----------+----------+----------+
## P-values from randomized Tukey Hsd test (n_iters=10000)
+----------+----------+----------+----------+
| P Value  | System_1 | System_2 | System_3 |
+----------+----------+----------+----------+
| System_1 | 1.0000   | 0.3976   | 1.0000   |
| System_2 | 0.3976   | 1.0000   | 0.6190   |
| System_3 | 1.0000   | 0.6190   | 1.0000   |
+----------+----------+----------+----------+

# ap
+----------+--------+---------+
| System   | Mean   | 95% MOE |
+----------+--------+---------+
| System 1 | 0.3500 | 0.3986  |
| System 2 | 0.8167 | 0.3986  |
| System 3 | 0.3167 | 0.3986  |
+----------+--------+---------+
## Two-way ANOVA without replication
+-----------------+------------+----+----------+--------+---------+
| Factor          | Variation  | DF | Variance | F Stat | P Value |
+-----------------+------------+----+----------+--------+---------+
| Between-systems | 0.7815     | 2  | 0.3907   | 2.6150 | 0.1337  |
| Between-queries | 0.1407     | 4  | 0.0352   | 0.2355 | 0.9106  |
| Residual        | 1.1954     | 8  | 0.1494   |        |         |
+-----------------+------------+----+----------+--------+---------+
## Between-system effect sizes from Tukey Hsd test
+----------+----------+----------+----------+
| ES       | System_1 | System_2 | System_3 |
+----------+----------+----------+----------+
| System_1 | 0.0000   | -1.2073  | 0.0862   |
| System_2 | 1.2073   | 0.0000   | 1.2935   |
| System_3 | -0.0862  | -1.2935  | 0.0000   |
+----------+----------+----------+----------+
## P-values from randomized Tukey Hsd test (n_iters=10000)
+----------+----------+----------+----------+
| P Value  | System_1 | System_2 | System_3 |
+----------+----------+----------+----------+
| System_1 | 1.0000   | 0.2168   | 1.0000   |
| System_2 | 0.2168   | 1.0000   | 0.1744   |
| System_3 | 1.0000   | 0.1744   | 1.0000   |
+----------+----------+----------+----------+

# rr
+----------+--------+---------+
| System   | Mean   | 95% MOE |
+----------+--------+---------+
| System 1 | 0.5000 | 0.3585  |
| System 2 | 0.9000 | 0.3585  |
| System 3 | 0.4000 | 0.3585  |
+----------+--------+---------+
## Two-way ANOVA without replication
+-----------------+------------+----+----------+--------+---------+
| Factor          | Variation  | DF | Variance | F Stat | P Value |
+-----------------+------------+----+----------+--------+---------+
| Between-systems | 0.7000     | 2  | 0.3500   | 2.8966 | 0.1132  |
| Between-queries | 0.4333     | 4  | 0.1083   | 0.8966 | 0.5087  |
| Residual        | 0.9667     | 8  | 0.1208   |        |         |
+-----------------+------------+----+----------+--------+---------+
## Between-system effect sizes from Tukey Hsd test
+----------+----------+----------+----------+
| ES       | System_1 | System_2 | System_3 |
+----------+----------+----------+----------+
| System_1 | 0.0000   | -1.1507  | 0.2877   |
| System_2 | 1.1507   | 0.0000   | 1.4384   |
| System_3 | -0.2877  | -1.4384  | 0.0000   |
+----------+----------+----------+----------+
## P-values from randomized Tukey Hsd test (n_iters=10000)
+----------+----------+----------+----------+
| P Value  | System_1 | System_2 | System_3 |
+----------+----------+----------+----------+
| System_1 | 1.0000   | 0.3826   | 0.9242   |
| System_2 | 0.3826   | 1.0000   | 0.2140   |
| System_3 | 0.9242   | 0.2140   | 1.0000   |
+----------+----------+----------+----------+

# ndcg@3
+----------+--------+---------+
| System   | Mean   | 95% MOE |
+----------+--------+---------+
| System 1 | 0.4480 | 0.3752  |
| System 2 | 0.8480 | 0.3752  |
| System 3 | 0.3941 | 0.3752  |
+----------+--------+---------+
## Two-way ANOVA without replication
+-----------------+------------+----+----------+--------+---------+
| Factor          | Variation  | DF | Variance | F Stat | P Value |
+-----------------+------------+----+----------+--------+---------+
| Between-systems | 0.6149     | 2  | 0.3074   | 2.3228 | 0.1602  |
| Between-queries | 0.2519     | 4  | 0.0630   | 0.4758 | 0.7532  |
| Residual        | 1.0589     | 8  | 0.1324   |        |         |
+-----------------+------------+----+----------+--------+---------+
## Between-system effect sizes from Tukey Hsd test
+----------+----------+----------+----------+
| ES       | System_1 | System_2 | System_3 |
+----------+----------+----------+----------+
| System_1 | 0.0000   | -1.0995  | 0.1482   |
| System_2 | 1.0995   | 0.0000   | 1.2476   |
| System_3 | -0.1482  | -1.2476  | 0.0000   |
+----------+----------+----------+----------+
## P-values from randomized Tukey Hsd test (n_iters=10000)
+----------+----------+----------+----------+
| P Value  | System_1 | System_2 | System_3 |
+----------+----------+----------+----------+
| System_1 | 1.0000   | 0.3086   | 0.9797   |
| System_2 | 0.3086   | 1.0000   | 0.2070   |
| System_3 | 0.9797   | 0.2070   | 1.0000   |
+----------+----------+----------+----------+
```

## Licensing

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
