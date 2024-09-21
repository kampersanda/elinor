# Elinor: Evaluation Library in Information Retrieval

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

// The metrics to evaluate can be specified via Metric instances.
let metrics = vec![
    Metric::Precision { k: 3 },
    Metric::AP { k: 0 }, // k=0 means all documents.
    // The instances can also be specified via strings.
    "rr".parse()?,
    "ndcg@3".parse()?,
];

// Evaluate.
let evaluated = elinor::evaluate(&gold_rels, &pred_rels, metrics.iter().cloned())?;

// Macro-averaged scores.
for metric in &metrics {
    let score = evaluated.mean_scores[metric];
    println!("{metric}: {score:.4}");
}
// => precision@3: 0.5000
// => ap: 0.5000
// => rr: 0.6667
// => ndcg@3: 0.4751
```

Other examples are available in the [`examples`](https://github.com/kampersanda/elinor/tree/main/examples) directory.

## Licensing

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
