use crate::Relevance;
use crate::RelevanceMap;

use crate::metrics::hits::compute_hits;

/// Computes the F1 score at k for a given relevance level.
///
/// # Arguments
///
/// * `rels` - Map of relevance levels for each document.
/// * `preds` - Slice of predicted documents with their scores.
/// * `k` - Number of documents to consider.
/// * `rel_lvl` - Relevance level to consider.
pub fn compute_f1(
    rels: &RelevanceMap<i32>,
    preds: &[Relevance<f64>],
    k: usize,
    rel_lvl: i32,
) -> f64 {
    let k = if k == 0 { preds.len() } else { k };
    if k == 0 {
        return 0.0;
    }
    let hits = compute_hits(rels, preds, k, rel_lvl);
    let precision = hits / k as f64;
    let recall = hits / rels.values().filter(|&&rel| rel >= rel_lvl).count() as f64;
    if precision + recall == 0.0 {
        0.0
    } else {
        2.0 * (precision * recall) / (precision + recall)
    }
}
