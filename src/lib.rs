pub mod errors;
pub mod metrics;
pub mod relevance;

use ordered_float::OrderedFloat;
use std::collections::HashMap;
use std::collections::HashSet;

pub use metrics::DcgWeighting;
pub use metrics::Metric;
pub use relevance::Relevance;
pub use relevance::RelevanceMap;

pub type GoldScore = i32;
pub type PredScore = OrderedFloat<f64>;

pub type Qrels<K> = relevance::RelevanceStore<K, GoldScore>;
pub type QrelsBuilder<K> = relevance::RelevanceStoreBuilder<K, GoldScore>;

pub type Run<K> = relevance::RelevanceStore<K, PredScore>;
pub type RunBuilder<K> = relevance::RelevanceStoreBuilder<K, PredScore>;

pub struct Evaluated<K> {
    pub mean_scores: HashMap<Metric, f64>,
    pub scores: HashMap<Metric, HashMap<K, f64>>,
}

pub fn evaluate<K>(
    qrels: &Qrels<K>,
    run: &Run<K>,
    metrics: HashSet<Metric>,
) -> Result<Evaluated<K>, errors::EmirError<K>>
where
    K: Clone + Eq + std::hash::Hash + std::fmt::Display,
{
    let mut mean_scores = HashMap::new();
    let mut scores = HashMap::new();
    for &metric in metrics.iter() {
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
