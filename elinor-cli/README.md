# elinor-cli

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
  --input-csvs test-data/sakai-book/X.csv \
  --input-csvs test-data/sakai-book/Y.csv \
  --input-csvs test-data/sakai-book/Z.csv
```

````
# Alias
+----------+----------------------------+
| Alias    | Path                       |
+----------+----------------------------+
| System_1 | test-data/sakai-book/X.csv |
| System_2 | test-data/sakai-book/Y.csv |
| System_3 | test-data/sakai-book/Z.csv |
+----------+----------------------------+

# score
+----------+--------+---------+
| System   | Mean   | 95% MOE |
+----------+--------+---------+
| System 1 | 0.3450 | 0.0670  |
| System 2 | 0.2700 | 0.0670  |
| System 3 | 0.2450 | 0.0670  |
+----------+--------+---------+
## Two-way ANOVA without replication
+-----------------+------------+----+----------+--------+---------+
| Factor          | Variation  | DF | Variance | F Stat | P Value |
+-----------------+------------+----+----------+--------+---------+
| Between-systems | 0.1083     | 2  | 0.0542   | 2.4749 | 0.0976  |
| Between-queries | 1.0293     | 19 | 0.0542   | 2.4754 | 0.0086  |
| Residual        | 0.8317     | 38 | 0.0219   |        |         |
+-----------------+------------+----+----------+--------+---------+
## Between-system effect sizes from Tukey Hsd test
+----------+----------+----------+----------+
| ES       | System_1 | System_2 | System_3 |
+----------+----------+----------+----------+
| System_1 | 0.0000   | 0.5070   | 0.6760   |
| System_2 | -0.5070  | 0.0000   | 0.1690   |
| System_3 | -0.6760  | -0.1690  | 0.0000   |
+----------+----------+----------+----------+
## P-values from randomized Tukey Hsd test (n_iters=10000)
+----------+----------+----------+----------+
| P Value  | System_1 | System_2 | System_3 |
+----------+----------+----------+----------+
| System_1 | 1.0000   | 0.2608   | 0.1006   |
| System_2 | 0.2608   | 1.0000   | 0.8915   |
| System_3 | 0.1006   | 0.8915   | 1.0000   |
+----------+----------+----------+----------+```

## Licensing

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
````
