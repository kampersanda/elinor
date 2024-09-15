use std::collections::HashMap;

use crate::metrics::precision::compute_precision;
use crate::GoldScore;
use crate::PredScore;
use crate::Relevance;

/// Computes the R-Precision.
///
/// # Arguments
///
/// * `rels` - Map of relevance levels for each document.
/// * `preds` - Slice of predicted documents with their scores.
/// * `rel_lvl` - Relevance level to consider.
pub fn compute_r_precision<K>(
    rels: &HashMap<K, GoldScore>,
    preds: &[Relevance<K, PredScore>],
    rel_lvl: GoldScore,
) -> f64
where
    K: Eq + std::hash::Hash,
{
    let n_rels = rels.values().filter(|&&rel| rel >= rel_lvl).count();
    if n_rels == 0 {
        0.0
    } else {
        compute_precision(rels, preds, n_rels, rel_lvl)
    }
}
