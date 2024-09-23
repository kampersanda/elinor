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

## API documentation

See https://docs.rs/elinor/.

Or, you can build and open the documentation locally
by running the following command:

```sh
RUSTDOCFLAGS="--html-in-header katex.html" cargo doc --no-deps --open
```

## Licensing

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
