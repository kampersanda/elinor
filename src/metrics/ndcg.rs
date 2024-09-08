use crate::Relevance;
use crate::RelevanceMap;

use crate::metrics::DcgWeighting;

fn weighted_score(rel: i32, weighting: DcgWeighting) -> f64 {
    match weighting {
        DcgWeighting::Jarvelin => rel as f64,
        DcgWeighting::Burges => 2.0_f64.powi(rel) - 1.0,
    }
}

/// Computes the DCG at k for a given relevance level.
///
/// # Arguments
///
/// * `rels` - Map of relevance levels for each document.
/// * `preds` - Slice of predicted documents with their scores.
/// * `k` - Number of documents to consider.
/// * `weighting` - Weighting scheme to use.
pub fn compute_dcg(
    rels: &RelevanceMap<i32>,
    preds: &[Relevance<f64>],
    k: usize,
    weighting: DcgWeighting,
) -> f64 {
    let k = if k == 0 { preds.len() } else { k };
    let mut dcg = 0.0;
    for (i, pred) in preds.iter().take(k).enumerate() {
        if let Some(&rel) = rels.get(&pred.doc_id) {
            dcg += weighted_score(rel, weighting) / (i as f64 + 2.0).log2();
        }
    }
    dcg
}
