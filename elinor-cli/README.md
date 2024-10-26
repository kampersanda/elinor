# elinor-cli

[![Crates.io](https://img.shields.io/crates/v/elinor-cli)](https://crates.io/crates/elinor-cli)
[![Build Status](https://github.com/kampersanda/elinor/actions/workflows/ci.yml/badge.svg)](https://github.com/kampersanda/elinor/actions)

elinor-cli is a set of command-line tools for evaluating IR systems:

- [elinor-evaluate](#elinor-evaluate) evaluates the ranking metrics of the system.
- [elinor-compare](#elinor-compare) compares the metrics of multiple systems with statistical tests.
- [elinor-convert](#elinor-convert) converts the TREC format into the JSONL format for elinor-evaluate.

## Installation

Simply use cargo to install from crates.io.

```sh
cargo install elinor-cli
```

## Ubiquitous language

Elinor uses the following terms for convenience:

- *True relevance score* means the relevance judgment provided by human assessors.
- *Predicted relevance score* means the similarity score predicted by the system.

## elinor-evaluate

elinor-evaluate evaluates the ranking metrics of the system.

### Input format

elinor-evaluate requires two JSONL files of true and predicted relevance scores.
Each line in the JSONL file should be a JSON object with the following fields:

- `query_id`: The ID of the query.
- `doc_id`: The ID of the document.
- `score`: The relevance score of the query-document pair.
  - If it is a true one, the score should be a non-negative integer (e.g., 0, 1, 2).
  - If it is a predicted one, the score can be a float (e.g., 0.1, 0.5, 1.0).

An example of the JSONL file for the true relevance scores is:

```jsonl
{"query_id":"q_1","doc_id":"d_1","score":2}
{"query_id":"q_1","doc_id":"d_7","score":0}
{"query_id":"q_2","doc_id":"d_3","score":2}
```

An example of the JSONL file for the predicted relevance scores is:

```jsonl
{"query_id":"q_1","doc_id":"d_1","score":0.65}
{"query_id":"q_1","doc_id":"d_4","score":0.23}
{"query_id":"q_2","doc_id":"d_3","score":0.48}
```

The specifications are:

- There is no need to sort the lines in the JSONL files.
- The query-document pairs should be unique in each file.
- The query IDs in the true and predicted files should be the same.
- In binary metrics (e.g., Precision, Recall, F1),
  true relevance scores more than 0 are considered relevant.

Sample JSONL files are available in the [`test-data/sample`](../test-data/sample/) directory.

### Example usage

Here is example usage with sample JSONL files in the [`test-data/sample`](../test-data/sample/) directory.

If you want to evaluate the Precision@3, Average Precision (AP), Reciprocal Rank (RR), and nDCG@3 metrics, run:

```sh
elinor-evaluate \
  --true-jsonl test-data/sample/true.jsonl \
  --pred-jsonl test-data/sample/pred_1.jsonl \
  --metrics precision@3 ap rr ndcg@3
```

The available metrics are shown in [Metric](https://docs.rs/elinor/latest/elinor/metrics/enum.Metric.html).

The output will show several basic statistics and the macro-averaged scores for each metric:

```
n_queries_in_true       8
n_queries_in_pred       8
n_docs_in_true  20
n_docs_in_pred  24
n_relevant_docs    14
precision@3     0.5833
ap      0.8229
rr      0.8125
ndcg@3  0.8286
```

The detailed results can be saved to a CSV file by specifying the `--output-csv` option:

```sh
elinor-evaluate \
  --true-jsonl test-data/sample/true.jsonl \
  --pred-jsonl test-data/sample/pred_1.jsonl \
  --output-csv test-data/sample/pred_1.csv \  # Specify output CSV path
  --metrics precision@3 ap rr ndcg@3
```

The CSV file will contain the scores for each query:

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

The CSV files can be input to elinor-compare to compare the metrics of multiple systems.

## elinor-compare

elinor-compare compares the metrics of multiple systems with statistical tests.

This tool supports several statistical tests and reports various statistics for in-depth analysis.
This tool is designed not only for IR systems but also for any systems that can be evaluated with metrics.

### Input format

elinor-compare requires multiple CSV files that contain the scores of the metrics for each query,
such as the output of elinor-evaluate.

Precisely, the CSV files should have the following columns:

- `topic_id`: The ID of the topic (e.g., query).
  - The colum name is arbitrary.
  - The column names must be the same across the CSV files.
  - The topic IDs should be the same across the CSV files.
- `metric_1`, `metric_2`, ...: The scores of the metrics for the query.
  - The column names are the metric names.
  - The column names should be the same across the CSV files.
  - The metric scores should be floats.

Sample CSV files are available in the [`test-data/sample`](../test-data/sample/) directory.

### Example usage: Comparing two systems

Here is example usage with sample CSV files in the [`test-data/sample`](../test-data/sample/) directory.

If you want to compare the metrics of two systems, run:

```sh
elinor-compare \
  --input-csvs test-data/sample/pred_1.csv \
  --input-csvs test-data/sample/pred_2.csv
```

The output will be:

```
# Basic statistics
+-----------+-------+
| Key       | Value |
+-----------+-------+
| n_systems | 2     |
| n_topics  | 8     |
| n_metrics | 4     |
+-----------+-------+

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

# Two-sided paired Student's t-test for (System_1 - System_2)
+-------------+--------+--------+--------+--------+---------+---------+
| Metric      | Mean   | Var    | ES     | t-stat | p-value | 95% MOE |
+-------------+--------+--------+--------+--------+---------+---------+
| precision@3 | 0.2917 | 0.0774 | 1.0485 | 2.9656 | 0.0209  | 0.2326  |
| ap          | 0.3750 | 0.1012 | 1.1789 | 3.3343 | 0.0125  | 0.2659  |
| rr          | 0.2500 | 0.0714 | 0.9354 | 2.6458 | 0.0331  | 0.2234  |
| ndcg@3      | 0.3637 | 0.1026 | 1.1356 | 3.2119 | 0.0148  | 0.2677  |
+-------------+--------+--------+--------+--------+---------+---------+

# Two-sided paired Bootstrap test (n_resamples = 10000)
+-------------+---------+
| Metric      | p-value |
+-------------+---------+
| precision@3 | 0.0240  |
| ap          | 0.0292  |
| rr          | 0.0602  |
| ndcg@3      | 0.0283  |
+-------------+---------+

# Fisher's randomized test (n_iters = 10000)
+-------------+---------+
| Metric      | p-value |
+-------------+---------+
| precision@3 | 0.0596  |
| ap          | 0.0657  |
| rr          | 0.1248  |
| ndcg@3      | 0.0612  |
+-------------+---------+
```

See the following documentation for more details about the statistical tests:

- [Student's t-test](https://docs.rs/elinor/latest/elinor/statistical_tests/student_t_test/struct.StudentTTest.html)
- [Bootstrap test](https://docs.rs/elinor/latest/elinor/statistical_tests/bootstrap_test/struct.BootstrapTest.html)
- [Fisher's randomized test](https://docs.rs/elinor/latest/elinor/statistical_tests/randomized_tukey_hsd_test/struct.RandomizedTukeyHsdTest.html)

### Example usage: Comparing three systems

If you want to compare the metrics of three (or more) systems, run:

```sh
elinor-compare \
  --input-csvs test-data/sample/pred_1.csv \
  --input-csvs test-data/sample/pred_2.csv \
  --input-csvs test-data/sample/pred_3.csv
```

The output will be:

```
# Basic statistics
+-----------+-------+
| Key       | Value |
+-----------+-------+
| n_systems | 3     |
| n_topics  | 8     |
| n_metrics | 4     |
+-----------+-------+

# Alias
+----------+-----------------------------+
| Alias    | Path                        |
+----------+-----------------------------+
| System_1 | test-data/sample/pred_1.csv |
| System_2 | test-data/sample/pred_2.csv |
| System_3 | test-data/sample/pred_3.csv |
+----------+-----------------------------+

# precision@3
## System means
+----------+--------+---------+
| System   | Mean   | 95% MOE |
+----------+--------+---------+
| System_1 | 0.5833 | 0.1498  |
| System_2 | 0.2917 | 0.1498  |
| System_3 | 0.4167 | 0.1498  |
+----------+--------+---------+
## Two-way ANOVA without replication
+-----------------+------------+----+----------+--------+---------+
| Factor          | Variation  | DF | Variance | F-stat | p-value |
+-----------------+------------+----+----------+--------+---------+
| Between-systems | 0.3426     | 2  | 0.1713   | 4.3898 | 0.0331  |
| Between-topics  | 0.3287     | 7  | 0.0470   | 1.2034 | 0.3623  |
| Residual        | 0.5463     | 14 | 0.0390   |        |         |
+-----------------+------------+----+----------+--------+---------+
## Effect sizes for Tukey HSD test
+----------+----------+----------+----------+
| ES       | System_1 | System_2 | System_3 |
+----------+----------+----------+----------+
| System_1 | 0.0000   | 1.4765   | 0.8437   |
| System_2 | -1.4765  | 0.0000   | -0.6328  |
| System_3 | -0.8437  | 0.6328   | 0.0000   |
+----------+----------+----------+----------+
## p-values for randomized Tukey HSD test (n_iters = 10000)
+----------+----------+----------+----------+
| p-value  | System_1 | System_2 | System_3 |
+----------+----------+----------+----------+
| System_1 | 1.0000   | 0.0248   | 0.2511   |
| System_2 | 0.0248   | 1.0000   | 0.6557   |
| System_3 | 0.2511   | 0.6557   | 1.0000   |
+----------+----------+----------+----------+

(The statistics for the other metrics will be shown as well.)
```

See the following documentation for more details about the statistical tests:

- [Two-way ANOVA without replication](https://docs.rs/elinor/latest/elinor/statistical_tests/two_way_anova_without_replication/struct.TwoWayAnovaWithoutReplication.html)
- [Tukey HSD test](https://docs.rs/elinor/latest/elinor/statistical_tests/tukey_hsd_test/struct.TukeyHsdTest.html)
- [Randomized Tukey HSD test](https://docs.rs/elinor/latest/elinor/statistical_tests/randomized_tukey_hsd_test/struct.RandomizedTukeyHsdTest.html)

### Example usage: Printing the tables in a tab-separated format

If you set `--print-mode raw`, the tables will be printed in a tab-separated format,
enabling you to copy and paste them into a spreadsheet:

```sh
elinor-compare \
  --input-csvs test-data/sample/pred_1.csv \
  --input-csvs test-data/sample/pred_2.csv \
  --print-mode raw
```

## elinor-convert

elinor-convert converts the TREC format into the JSONL format for elinor-evaluate.

For [Qrels](https://trec.nist.gov/data/qrels_eng/) files:

```sh
elinor-convert \
  --input-trec qrels.trec \
  --output-jsonl qrels.jsonl \
  --rel-type true
```

For [Run](https://faculty.washington.edu/levow/courses/ling573_SPR2011/hw/trec_eval_desc.htm) files:

```sh
elinor-convert \
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
