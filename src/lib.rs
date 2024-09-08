//! # emir: Evaluation Measures in Information Retrieval
//!
//! `emir` is a library for evaluating information retrieval systems.
//!
//! ## Examples
//!
//! ```
//! use std::collections::HashMap;
//! use emir::{Qrels, Run, Metric};
//!
//! let qrels_map = HashMap::from([
//!     ("q_1".to_string(), HashMap::from([
//!         ("d_1".to_string(), 1),
//!         ("d_2".to_string(), 0),
//!         ("d_3".to_string(), 2),
//!     ])),
//! ]);
//!
//! let run_map = HashMap::from([
//!     ("q_1".to_string(), HashMap::from([
//!         ("d_1".to_string(), 0.5.into()),
//!         ("d_2".to_string(), 0.4.into()),
//!         ("d_3".to_string(), 0.3.into()),
//!         ("d_4".to_string(), 0.2.into()),
//!     ])),
//! ]);
//!
//! let qrels = Qrels::from_map(qrels_map);
//! let run = Run::from_map(run_map);
//! let metrics = vec![Metric::Precision(1)];
//!
//! let evaluated = emir::evaluate(&qrels, &run, metrics).unwrap();
//! ```
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

pub type GoldScore = u32;
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
