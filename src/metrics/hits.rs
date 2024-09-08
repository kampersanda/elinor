use crate::Relevance;
use crate::RelevanceMap;

/// Computes the number of hits at a given relevance level.
///
/// # Arguments
///
/// * `rels` - Map of relevance levels for each document.
/// * `preds` - Sorted slice of predicted documents with their scores.
/// * `k` - Number of documents to consider.
/// * `rel_lvl` - Relevance level to consider.
pub fn compute_hits(
    rels: &RelevanceMap<i32>,
    preds: &[Relevance<f64>],
    k: usize,
    rel_lvl: i32,
) -> f64 {
    let k = if k == 0 { preds.len() } else { k };
    let mut hits = 0;
    for pred in &preds[..k] {
        if let Some(&rel) = rels.get(&pred.doc_id) {
            if rel >= rel_lvl {
                hits += 1;
            }
        }
    }
    hits as f64
}
