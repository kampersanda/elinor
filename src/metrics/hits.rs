use crate::Relevance;
use crate::RelevanceMap;

/// Computes the number of hits at a given relevance level.
///
/// # Arguments
///
/// * `rels` - Map of relevance levels for each document.
/// * `preds` - Slice of predicted documents with their scores.
/// * `k` - Number of documents to consider.
/// * `rel_lvl` - Relevance level to consider.
pub fn compute_hits(
    rels: &RelevanceMap<i32>,
    preds: &[Relevance<f64>],
    k: usize,
    rel_lvl: i32,
) -> f64 {
    let mut hits = 0;
    for pred in &preds[..k] {
        if let Some(&rel) = rels.get(&pred.id) {
            if rel >= rel_lvl {
                hits += 1;
            }
        }
    }
    hits as f64
}
