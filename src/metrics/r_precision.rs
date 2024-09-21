use std::collections::HashMap;

use crate::metrics::precision::compute_precision;
use crate::GoldScore;
use crate::PredScore;
use crate::Relevance;

/// Computes the R-Precision.
pub fn compute_r_precision<K>(
    golds: &HashMap<K, GoldScore>,
    sorted_preds: &[Relevance<K, PredScore>],
    rel_lvl: GoldScore,
) -> f64
where
    K: Eq + std::hash::Hash,
{
    let n_rels = golds.values().filter(|&&rel| rel >= rel_lvl).count();
    if n_rels == 0 {
        0.0
    } else {
        compute_precision(golds, sorted_preds, n_rels, rel_lvl)
    }
}
