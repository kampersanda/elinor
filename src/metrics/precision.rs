use crate::Predicted;
use crate::RelevanceMap;

use crate::metrics::hits::compute_hits;

/// Computes the precision at k for a given relevance level.
///
/// # Arguments
///
/// * `rels` - Map of relevance levels for each document.
/// * `preds` - Slice of predicted documents with their scores.
/// * `k` - Number of documents to consider.
/// * `rel_lvl` - Relevance level to consider.
pub fn compute_precision(
    rels: &RelevanceMap,
    preds: &[Predicted],
    k: usize,
    rel_lvl: usize,
) -> f64 {
    let k = if k == 0 { preds.len() } else { k };
    if k == 0 {
        0.0
    } else {
        compute_hits(rels, preds, k, rel_lvl) / k as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
