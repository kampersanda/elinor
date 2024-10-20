# Elinor: Evaluation Library in INfOrmation Retrieval

<p align="left">
    <a href="https://github.com/kampersanda/elinor/actions/workflows/ci.yml?query=branch%3Amain"><img src="https://img.shields.io/github/actions/workflow/status/kampersanda/elinor/ci.yml?branch=main&style=flat-square" alt="actions status" /></a>
    &nbsp;
    <a href="https://crates.io/crates/elinor"><img src="https://img.shields.io/crates/v/elinor.svg?style=flat-square" alt="Crates.io version" /></a>
    &nbsp;
    <a href="https://docs.rs/elinor"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>
</p>

Elinor is a Rust library for evaluating information retrieval (IR) systems.
It provides a comprehensive set of tools and metrics tailored for IR engineers,
offering an intuitive and easy-to-use interface.

## Key features

- **IR-specific design:**
  Elinor is tailored specifically for evaluating IR systems, with an intuitive interface designed for IR engineers.
  It offers a streamlined workflow that simplifies common IR evaluation tasks.
- **Comprehensive evaluation metrics:**
  Elinor supports a wide range of key evaluation metrics, such as Precision, MAP, MRR, and nDCG.
  The supported metrics are available in [Metric](https://docs.rs/elinor/latest/elinor/metrics/enum.Metric.html).
  The evaluation results are validated against trec_eval to ensure accuracy and reliability.
- **In-depth statistical testing:**
  Elinor includes several statistical tests, such as Student's t-test or Randomized Tukey HSD test, to verify the generalizability of results.
  Not only p-values but also other statistics, such as effect sizes and confidence intervals, are provided for thorough reporting.
  See the [statistical_tests](https://docs.rs/elinor/latest/elinor/statistical_tests/index.html) module for more details.
- **Command-line tools:**
  [elinor-cli](./elinor-cli) provides command-line tools for evaluating and comparing IR systems.
  The tools support various metrics and statistical tests, facilitating comprehensive evaluations and in-depth analyses.

## API documentation

See https://docs.rs/elinor/.

Or, you can build and open the documentation locally
by running the following command:

```sh
RUSTDOCFLAGS="--html-in-header katex.html" cargo doc --no-deps --features serde --open
```

## Command-line tools

[elinor-cli](./elinor-cli) provides command-line tools for evaluating and comparing IR systems.

For example, you can obtain various statistics from several statistical tests, as shown below:

### Two-system comparison

```
# Means
+--------+----------+----------+
| Metric | System_1 | System_2 |
+--------+----------+----------+
| ndcg@5 | 0.3450   | 0.2700   |
+--------+----------+----------+

# Two-sided paired Student's t-test for (System_1 - System_2)
+--------+--------+--------+--------+--------+---------+---------+
| Metric | Mean   | Var    | ES     | t-stat | p-value | 95% MOE |
+--------+--------+--------+--------+--------+---------+---------+
| ndcg@5 | 0.0750 | 0.0251 | 0.4731 | 2.1158 | 0.0478  | 0.0742  |
+--------+--------+--------+--------+--------+---------+---------+

# Two-sided paired Bootstrap test (n_resamples = 10000)
+--------+---------+
| Metric | p-value |
+--------+---------+
| ndcg@5 | 0.0511  |
+--------+---------+

# Fisher's randomized test (n_iters = 10000)
+--------+---------+
| Metric | p-value |
+--------+---------+
| ndcg@5 | 0.0498  |
+--------+---------+
```

### Multi-system comparison

```
# ndcg@5
## System means
+----------+--------+---------+
| System   | Mean   | 95% MOE |
+----------+--------+---------+
| System_1 | 0.3450 | 0.0670  |
| System_2 | 0.2700 | 0.0670  |
| System_3 | 0.2450 | 0.0670  |
+----------+--------+---------+
## Two-way ANOVA without replication
+-----------------+------------+----+----------+--------+---------+
| Factor          | Variation  | DF | Variance | F-stat | p-value |
+-----------------+------------+----+----------+--------+---------+
| Between-systems | 0.1083     | 2  | 0.0542   | 2.4749 | 0.0976  |
| Between-topics  | 1.0293     | 19 | 0.0542   | 2.4754 | 0.0086  |
| Residual        | 0.8317     | 38 | 0.0219   |        |         |
+-----------------+------------+----+----------+--------+---------+
## Effect sizes for Tukey HSD test
+----------+----------+----------+----------+
| ES       | System_1 | System_2 | System_3 |
+----------+----------+----------+----------+
| System_1 | 0.0000   | 0.5070   | 0.6760   |
| System_2 | -0.5070  | 0.0000   | 0.1690   |
| System_3 | -0.6760  | -0.1690  | 0.0000   |
+----------+----------+----------+----------+
## p-values for randomized Tukey HSD test (n_iters = 10000)
+----------+----------+----------+----------+
| p-value  | System_1 | System_2 | System_3 |
+----------+----------+----------+----------+
| System_1 | 1.0000   | 0.2561   | 0.1040   |
| System_2 | 0.2561   | 1.0000   | 0.8926   |
| System_3 | 0.1040   | 0.8926   | 1.0000   |
+----------+----------+----------+----------+
```

## Correctness verification

In addition to the unit tests,
Elinor's evaluation results are validated to ensure accuracy and reliability:

- The metrics are validated against [trec_eval](https://github.com/usnistgov/trec_eval)
  using its test data.
- The statistical tests are validated against the results in
  [Sakai's book](https://www.coronasha.co.jp/np/isbn/9784339024968/)
  using its sample data.

## Licensing

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
