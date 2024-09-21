# Elinor: Evaluation Library in INfOrmation Retrieval

<p align="left">
    <a href="https://github.com/kampersanda/elinor/actions/workflows/ci.yml?query=branch%3Amain"><img src="https://img.shields.io/github/actions/workflow/status/kampersanda/elinor/ci.yml?branch=main&style=flat-square" alt="actions status" /></a>
    &nbsp;
    <a href="https://crates.io/crates/elinor"><img src="https://img.shields.io/crates/v/elinor.svg?style=flat-square" alt="Crates.io version" /></a>
    &nbsp;
    <a href="https://docs.rs/elinor"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>
</p>

Elinor is a Rust library for evaluating information retrieval systems,
inspired by [ranx](https://github.com/AmenRa/ranx) and [Sakai's book](https://www.coronasha.co.jp/np/isbn/9784339024968/).

## Features

- **IRer-friendly**:
  The library is designed to be easy to use for developers in information retrieval.
- **Flexible**:
  The library supports various evaluation metrics, such as Precision, MAP, MRR, and nDCG.
  The supported metrics are available in [Metric](https://docs.rs/elinor/latest/elinor/metrics/enum.Metric.html).

## Documentation

See https://docs.rs/elinor/.

Or, you can build and open the documentation locally
by running the following command:

```sh
RUSTDOCFLAGS="--html-in-header katex.html" cargo doc --no-deps --open
```

## Getting Started

A simple routine to prepare gold and predicted relevance scores
and evaluate them using Precision@3, MAP, MRR, and nDCG@3:

```rust
use elinor::{GoldRelStoreBuilder, PredRelStoreBuilder, Metric};
use approx::assert_abs_diff_eq;

// Prepare gold relevance scores.
let mut b = GoldRelStoreBuilder::new();
b.add_score("q_1", "d_1", 1)?;
b.add_score("q_1", "d_2", 0)?;
b.add_score("q_1", "d_3", 2)?;
b.add_score("q_2", "d_2", 2)?;
b.add_score("q_2", "d_4", 1)?;
let gold_rels = b.build();

// Prepare predicted relevance scores.
let mut b = PredRelStoreBuilder::new();
b.add_score("q_1", "d_1", 0.5.into())?;
b.add_score("q_1", "d_2", 0.4.into())?;
b.add_score("q_1", "d_3", 0.3.into())?;
b.add_score("q_2", "d_4", 0.1.into())?;
b.add_score("q_2", "d_1", 0.2.into())?;
b.add_score("q_2", "d_3", 0.3.into())?;
let pred_rels = b.build();

// Evaluate Precision@3.
let evaluated = elinor::evaluate(&gold_rels, &pred_rels, Metric::Precision { k: 3 })?;
assert_abs_diff_eq!(evaluated.mean_score(), 0.5000, epsilon = 1e-4);

// Evaluate MAP, where all documents are considered via k=0.
let evaluated = elinor::evaluate(&gold_rels, &pred_rels, Metric::AP { k: 0 })?;
assert_abs_diff_eq!(evaluated.mean_score(), 0.5000, epsilon = 1e-4);

// Evaluate MRR, where the metric is specified via a string representation.
let evaluated = elinor::evaluate(&gold_rels, &pred_rels, "rr".parse()?)?;
assert_abs_diff_eq!(evaluated.mean_score(), 0.6667, epsilon = 1e-4);

// Evaluate nDCG@3, where the metric is specified via a string representation.
let evaluated = elinor::evaluate(&gold_rels, &pred_rels, "ndcg@3".parse()?)?;
assert_abs_diff_eq!(evaluated.mean_score(), 0.4751, epsilon = 1e-4);
```

Other examples are available in the [`examples`](https://github.com/kampersanda/elinor/tree/main/examples) directory.

## Licensing

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
