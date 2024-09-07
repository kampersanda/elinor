use crate::Relevance;
use crate::RelevanceMap;

/// Computes the reciprocal rank at k for a given relevance level.
///
/// # Arguments
///
/// * `rels` - Map of relevance levels for each document.
/// * `preds` - Slice of predicted documents with their scores.
/// * `k` - Number of documents to consider.
/// * `rel_lvl` - Relevance level to consider.
pub fn compute_reciprocal_rank(
    rels: &RelevanceMap<i32>,
    preds: &[Relevance<f64>],
    k: usize,
    rel_lvl: i32,
) -> f64 {
    let k = if k == 0 { preds.len() } else { k };
    if k == 0 {
        return 0.0;
    }
    for (i, pred) in preds.iter().enumerate().take(k) {
        if let Some(&rel) = rels.get(&pred.id) {
            if rel >= rel_lvl {
                return 1.0 / (i as f64 + 1.0);
            }
        }
    }
    0.0
}
