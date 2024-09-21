use std::collections::HashMap;

use crate::metrics::hits::compute_hits;
use crate::GoldScore;
use crate::PredScore;
use crate::Relevance;

/// Computes the F1 score at k for a given relevance level.
pub fn compute_f1<K>(
    golds: &HashMap<K, GoldScore>,
    sorted_preds: &[Relevance<K, PredScore>],
    k: usize,
    rel_lvl: GoldScore,
) -> f64
where
    K: Eq + std::hash::Hash,
{
    let k = if k == 0 { sorted_preds.len() } else { k };
    if k == 0 {
        return 0.0;
    }
    let hits = compute_hits(golds, sorted_preds, k, rel_lvl);
    let precision = hits / k as f64;
    let recall = hits / golds.values().filter(|&&rel| rel >= rel_lvl).count() as f64;
    if precision + recall == 0.0 {
        0.0
    } else {
        2.0 * (precision * recall) / (precision + recall)
    }
}
