//! # Emir: Evaluation Measures in Information Retrieval
//!
//! Emir is a Rust library for evaluating information retrieval systems,
//! which is inspired by [ranx](https://github.com/AmenRa/ranx).
//!
//! ## Features
//!
//! ## Glossary
//!
//! * **Qrels** - Collection of relevance judgments for a set of queries and documents.
//! * **Run** - Collection of predicted scores for a set of queries and documents.
//!
//! ## Getting started
//!
//! A simple example to prepare qrels and run data and evaluate them using MAP and nDCG.
//!
//! ```
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use emir::{QrelsBuilder, RunBuilder, Metric, DcgWeighting};
//!
//! let mut qb = QrelsBuilder::new();
//! qb.add_score("q_1", "d_1", 1)?;
//! qb.add_score("q_1", "d_2", 0)?;
//! qb.add_score("q_1", "d_3", 2)?;
//! qb.add_score("q_2", "d_2", 2)?;
//! qb.add_score("q_2", "d_4", 1)?;
//! let qrels = qb.build();
//!
//! let mut rb = RunBuilder::new();
//! rb.add_score("q_1", "d_1", 0.5.into())?;
//! rb.add_score("q_1", "d_2", 0.4.into())?;
//! rb.add_score("q_1", "d_3", 0.3.into())?;
//! rb.add_score("q_2", "d_4", 0.1.into())?;
//! rb.add_score("q_2", "d_1", 0.2.into())?;
//! rb.add_score("q_2", "d_3", 0.3.into())?;
//! let run = rb.build();
//!
//! let metrics = vec![
//!     Metric::AveragePrecision { k: 3 },
//!     Metric::Ndcg { k: 3, w: DcgWeighting::Jarvelin },
//! ];
//! let evaluated = emir::evaluate(&qrels, &run, metrics)?;
//!
//! // Macro-averaged scores.
//! for (metric, score) in evaluated.mean_scores.iter() {
//!    println!("{metric}: {score:.4}");
//! }
//! // => MAP@3: 0.5833
//! // => nDCG_Jarvelin@3: 0.4751
//!
//! // Scores per query.
//! for (metric, scores) in evaluated.scores.iter() {
//!     println!("{metric}");
//!     for (query_id, score) in scores.iter() {
//!         println!("- {query_id}: {score:.4}");
//!     }
//! }
//! // => MAP@3
//! // => - q_1: 0.8333
//! // => - q_2: 0.3333
//! // => nDCG_Jarvelin@3
//! // => - q_1: 0.7602
//! // => - q_2: 0.1900
//! # Ok(())
//! # }
//! ```
// #![deny(missing_docs)]

pub mod errors;
pub mod metrics;
pub mod relevance;
pub mod trec;

use ordered_float::OrderedFloat;
use std::collections::HashMap;
use std::collections::HashSet;

pub use metrics::DcgWeighting;
pub use metrics::Metric;
pub use relevance::Relevance;
pub use relevance::RelevanceMap;

/// Data type to store a relevance score.
pub type GoldScore = u32;

/// Data type to store a predicted score.
pub type PredScore = OrderedFloat<f64>;

pub type Qrels<K> = relevance::RelevanceStore<K, GoldScore>;

pub type QrelsBuilder<K> = relevance::RelevanceStoreBuilder<K, GoldScore>;

pub type Run<K> = relevance::RelevanceStore<K, PredScore>;

pub type RunBuilder<K> = relevance::RelevanceStoreBuilder<K, PredScore>;

pub const RELEVANT_LEVEL: GoldScore = 1;

pub struct Evaluated<K> {
    pub mean_scores: HashMap<Metric, f64>,
    pub scores: HashMap<Metric, HashMap<K, f64>>,
}

pub fn evaluate<K, M>(
    qrels: &Qrels<K>,
    run: &Run<K>,
    metrics: M,
) -> Result<Evaluated<K>, errors::EmirError<K>>
where
    K: Clone + Eq + std::hash::Hash + std::fmt::Display,
    M: IntoIterator<Item = Metric>,
{
    let metrics: HashSet<Metric> = metrics.into_iter().collect();
    let mut mean_scores = HashMap::new();
    let mut scores = HashMap::new();
    for metric in metrics {
        let result = metrics::compute_metric(qrels, run, metric)?;
        let mean_score = result.iter().map(|(_, x)| x).sum::<f64>() / result.len() as f64;
        mean_scores.insert(metric, mean_score);
        scores.insert(metric, result);
    }
    Ok(Evaluated {
        mean_scores,
        scores,
    })
}
