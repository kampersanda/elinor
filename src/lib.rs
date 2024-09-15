//! # Information Retrieval Evaluation Library
//!
//! This is a Rust library for evaluating information retrieval systems,
//! which is inspired by [ranx](https://github.com/AmenRa/ranx).
//!
//! ## Features
//!
//! * **IRer-friendly**:
//!     The library is designed to be easy to use for developers in information retrieval
//!     by providing TREC-like data structures, such as Qrels and Run.
//! * **Flexible**:
//!     The library supports various evaluation metrics, such as Precision, MAP, MRR, and nDCG.
//!     The supported metrics are available in [`Metric`].
//!
//! ## Glossary from TREC
//!
//! * **Qrels** - Collection of relevance judgments for a set of queries and documents.
//! * **Run** - Collection of predicted scores for a set of queries and documents.
//!
//! ## Getting Started
//!
//! A simple routine to prepare Qrels and Run data
//! and evaluate them using Precision@3, MAP, MRR, and nDCG@3:
//!
//! ```
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use elinor::{QrelsBuilder, RunBuilder, Metric};
//!
//! // Construct Qrels data.
//! let mut qb = QrelsBuilder::new();
//! qb.add_score("q_1", "d_1", 1)?;
//! qb.add_score("q_1", "d_2", 0)?;
//! qb.add_score("q_1", "d_3", 2)?;
//! qb.add_score("q_2", "d_2", 2)?;
//! qb.add_score("q_2", "d_4", 1)?;
//! let qrels = qb.build();
//!
//! // Construct Run data.
//! let mut rb = RunBuilder::new();
//! rb.add_score("q_1", "d_1", 0.5.into())?;
//! rb.add_score("q_1", "d_2", 0.4.into())?;
//! rb.add_score("q_1", "d_3", 0.3.into())?;
//! rb.add_score("q_2", "d_4", 0.1.into())?;
//! rb.add_score("q_2", "d_1", 0.2.into())?;
//! rb.add_score("q_2", "d_3", 0.3.into())?;
//! let run = rb.build();
//!
//! // The metrics to evaluate can be specified via Metric instances.
//! let metrics = vec![
//!     Metric::Precision { k: 3 },
//!     Metric::AP { k: 0 }, // k=0 means all documents.
//!     // The instances can also be specified via strings.
//!     "rr".parse()?,
//!     "ndcg@3".parse()?,
//! ];
//!
//! // Evaluate the qrels and run data.
//! let evaluated = elinor::evaluate(&qrels, &run, metrics.iter().cloned())?;
//!
//! // Macro-averaged scores.
//! for metric in &metrics {
//!     let score = evaluated.mean_scores[metric];
//!     println!("{metric}: {score:.4}");
//! }
//! // => precision@3: 0.5000
//! // => ap: 0.5000
//! // => rr: 0.6667
//! // => ndcg@3: 0.4751
//! # Ok(())
//! # }
//! ```
//!
//! Other examples are available in the [`examples`](https://github.com/kampersanda/elinor/tree/main/examples) directory.
#![deny(missing_docs)]

pub mod errors;
pub mod metrics;
pub mod relevance;
pub mod trec;

use ordered_float::OrderedFloat;
use std::collections::HashMap;
use std::collections::HashSet;

pub use metrics::Metric;
pub use relevance::Relevance;

/// Data type to store a relevance score.
/// In binary relevance, 0 means non-relevant and the others mean relevant.
pub type GoldScore = u32;

/// Data type to store a predicted score.
pub type PredScore = OrderedFloat<f64>;

/// Data structure to store Qrels.
pub type Qrels<K> = relevance::RelevanceStore<K, GoldScore>;

/// Builder for [`Qrels`].
pub type QrelsBuilder<K> = relevance::RelevanceStoreBuilder<K, GoldScore>;

/// Data structure to store a Run.
pub type Run<K> = relevance::RelevanceStore<K, PredScore>;

/// Builder for [`Run`].
pub type RunBuilder<K> = relevance::RelevanceStoreBuilder<K, PredScore>;

/// Data type to store evaluated scores.
pub struct Evaluated<K> {
    /// Metric to macro-averaged score.
    pub mean_scores: HashMap<Metric, f64>,

    /// Metric to mapping from query ID to the score.
    pub all_scores: HashMap<Metric, HashMap<K, f64>>,
}

/// Evaluates the given qrels and run data using the specified metrics.
pub fn evaluate<K, M>(
    qrels: &Qrels<K>,
    run: &Run<K>,
    metrics: M,
) -> Result<Evaluated<K>, errors::ElinorError>
where
    K: Clone + Eq + std::hash::Hash + std::fmt::Display,
    M: IntoIterator<Item = Metric>,
{
    let metrics: HashSet<Metric> = metrics.into_iter().collect();
    let mut mean_scores = HashMap::new();
    let mut all_scores = HashMap::new();
    for metric in metrics {
        let result = metrics::compute_metric(qrels, run, metric)?;
        let mean_score = result.values().sum::<f64>() / result.len() as f64;
        mean_scores.insert(metric, mean_score);
        all_scores.insert(metric, result);
    }
    Ok(Evaluated {
        mean_scores,
        all_scores,
    })
}
