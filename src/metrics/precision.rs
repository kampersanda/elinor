use std::collections::HashMap;

use crate::metrics::hits::compute_hits;
use crate::GoldScore;
use crate::PredScore;
use crate::Relevance;

/// Computes the precision at k for a given relevance level.
pub fn compute_precision<K>(
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
        0.0
    } else {
        compute_hits::<K>(golds, sorted_preds, k, rel_lvl) / k as f64
    }
}
