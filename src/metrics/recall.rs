use crate::Relevance;
use crate::RelevanceMap;

use crate::metrics::hits::compute_hits;

/// Computes the recall at k for a given relevance level.
///
/// # Arguments
///
/// * `rels` - Map of relevance levels for each document.
/// * `preds` - Slice of predicted documents with their scores.
/// * `k` - Number of documents to consider.
/// * `rel_lvl` - Relevance level to consider.
pub fn compute_recall(
    rels: &RelevanceMap<i32>,
    preds: &[Relevance<f64>],
    k: usize,
    rel_lvl: i32,
) -> f64 {
    let k = if k == 0 { preds.len() } else { k };
    if k == 0 {
        return 0.0;
    }
    let n_rels = rels.values().filter(|&&rel| rel >= rel_lvl).count();
    if n_rels == 0 {
        0.0
    } else {
        compute_hits(rels, preds, k, rel_lvl) / n_rels as f64
    }
}
