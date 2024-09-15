# Information Retrieval Evaluation Library in Rust

<p align="left">
    <a href="https://github.com/kampersanda/ireval/actions/workflows/rust.yml?query=branch%3Amain"><img src="https://img.shields.io/github/actions/workflow/status/kampersanda/ireval/rust.yml?branch=main&style=flat-square" alt="actions status" /></a>
    &nbsp;
    <a href="https://crates.io/crates/ireval"><img src="https://img.shields.io/crates/v/ireval.svg?style=flat-square" alt="Crates.io version" /></a>
    &nbsp;
    <a href="https://docs.rs/ireval"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>
</p>

This is a Rust library for evaluating information retrieval systems,
which is inspired by [ranx](https://github.com/AmenRa/ranx).

## Features

* **IRer-friendly**:
    The library is designed to be easy to use for developers in information retrieval
    by providing TREC-like data structures, such as Qrels and Run.
* **Flexible**:
    The library supports various evaluation metrics, such as Precision, MAP, MRR, and nDCG.
    The supported metrics are available in `Metric`.

## Documentation

See https://docs.rs/ireval/.

Or, you can build and open the documentation locally
by running the following command:

```sh
RUSTDOCFLAGS="--html-in-header katex.html" cargo doc --no-deps --open
```

## Examples

A simple routine to prepare Qrels and Run data
and evaluate them using Precision@3, MAP, MRR, and nDCG@3:

```rust
use ireval::{QrelsBuilder, RunBuilder, Metric};

// Construct Qrels data.
let mut qb = QrelsBuilder::new();
qb.add_score("q_1", "d_1", 1)?;
qb.add_score("q_1", "d_2", 0)?;
qb.add_score("q_1", "d_3", 2)?;
qb.add_score("q_2", "d_2", 2)?;
qb.add_score("q_2", "d_4", 1)?;
let qrels = qb.build();

// Construct Run data.
let mut rb = RunBuilder::new();
rb.add_score("q_1", "d_1", 0.5.into())?;
rb.add_score("q_1", "d_2", 0.4.into())?;
rb.add_score("q_1", "d_3", 0.3.into())?;
rb.add_score("q_2", "d_4", 0.1.into())?;
rb.add_score("q_2", "d_1", 0.2.into())?;
rb.add_score("q_2", "d_3", 0.3.into())?;
let run = rb.build();

// The metrics to evaluate can be specified via Metric instances.
let metrics = vec![
    Metric::Precision { k: 3 },
    Metric::AP { k: 0 }, // k=0 means all documents.
    // The instances can also be specified via strings.
    "rr".parse()?,
    "ndcg@3".parse()?,
];

// Evaluate the qrels and run data.
let evaluated = ireval::evaluate(&qrels, &run, metrics.iter().cloned())?;

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

Other examples are available in the [`examples`](https://github.com/kampersanda/ireval/tree/main/examples) directory.

## Licensing

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
