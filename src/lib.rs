//! # Elinor: Evaluation Library in Information Retrieval
//!
//! Elinor is a Rust library for evaluating information retrieval systems,
//! inspired by [ranx](https://github.com/AmenRa/ranx) and [Sakai's book](https://www.coronasha.co.jp/np/isbn/9784339024968/).
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
//! ## Getting Started
//!
//! A simple routine to prepare gold and predicted relevance scores
//! and evaluate them using Precision@3, MAP, MRR, and nDCG@3:
//!
//! ```
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use elinor::{GoldRelStoreBuilder, PredRelStoreBuilder, Metric};
//!
//! // Prepare gold relevance scores.
//! let mut b = GoldRelStoreBuilder::new();
//! b.add_score("q_1", "d_1", 1)?;
//! b.add_score("q_1", "d_2", 0)?;
//! b.add_score("q_1", "d_3", 2)?;
//! b.add_score("q_2", "d_2", 2)?;
//! b.add_score("q_2", "d_4", 1)?;
//! let gold_rels = b.build();
//!
//! // Prepare predicted relevance scores.
//! let mut b = PredRelStoreBuilder::new();
//! b.add_score("q_1", "d_1", 0.5.into())?;
//! b.add_score("q_1", "d_2", 0.4.into())?;
//! b.add_score("q_1", "d_3", 0.3.into())?;
//! b.add_score("q_2", "d_4", 0.1.into())?;
//! b.add_score("q_2", "d_1", 0.2.into())?;
//! b.add_score("q_2", "d_3", 0.3.into())?;
//! let pred_rels = b.build();
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
//! // Evaluate.
//! let evaluated = elinor::evaluate(&gold_rels, &pred_rels, metrics.iter().cloned())?;
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
pub mod statistical_tests;
pub mod trec;

use ordered_float::OrderedFloat;
use std::collections::HashMap;
use std::collections::HashSet;

pub use metrics::Metric;
pub use relevance::Relevance;

/// Data type to store a gold relevance score.
/// In binary relevance, 0 means non-relevant and the others mean relevant.
pub type GoldScore = u32;

/// Data type to store a predicted relevance score.
/// A higher score means more relevant.
pub type PredScore = OrderedFloat<f64>;

/// Data structure to store gold relevance scores.
pub type GoldRelStore<K> = relevance::RelevanceStore<K, GoldScore>;

/// Builder for [`GoldRelStore`].
pub type GoldRelStoreBuilder<K> = relevance::RelevanceStoreBuilder<K, GoldScore>;

/// Data structure to store predicted relevance scores.
pub type PredRelStore<K> = relevance::RelevanceStore<K, PredScore>;

/// Builder for [`PredRelStore`].
pub type PredRelStoreBuilder<K> = relevance::RelevanceStoreBuilder<K, PredScore>;

/// Data type to store evaluated scores.
pub struct Evaluated<K> {
    /// Metric to macro-averaged score.
    pub mean_scores: HashMap<Metric, f64>,

    /// Metric to mapping from query ID to the score.
    pub all_scores: HashMap<Metric, HashMap<K, f64>>,
}

/// Evaluates the given gold_rels and pred_rels data using the specified metrics.
pub fn evaluate<K, M>(
    gold_rels: &GoldRelStore<K>,
    pred_rels: &PredRelStore<K>,
    metrics: M,
) -> Result<Evaluated<K>, errors::ElinorError>
where
    K: Clone + Eq + Ord + std::hash::Hash + std::fmt::Display,
    M: IntoIterator<Item = Metric>,
{
    let metrics: HashSet<Metric> = metrics.into_iter().collect();
    let mut mean_scores = HashMap::new();
    let mut all_scores = HashMap::new();
    for metric in metrics {
        let result = metrics::compute_metric(gold_rels, pred_rels, metric)?;
        let mean_score = result.values().sum::<f64>() / result.len() as f64;
        mean_scores.insert(metric, mean_score);
        all_scores.insert(metric, result);
    }
    Ok(Evaluated {
        mean_scores,
        all_scores,
    })
}
