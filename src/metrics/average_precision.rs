use crate::GoldScore;
use crate::PredScore;
use crate::Relevance;
use crate::RelevanceMap;

use crate::metrics::precision::compute_precision;

/// Computes the average precision at k for a given relevance level.
///
/// # Arguments
///
/// * `rels` - Map of relevance levels for each document.
/// * `preds` - Slice of predicted documents with their scores.
/// * `k` - Number of documents to consider.
/// * `rel_lvl` - Relevance level to consider.
pub fn compute_average_precision<K>(
    rels: &RelevanceMap<K, GoldScore>,
    preds: &[Relevance<K, PredScore>],
    k: usize,
    rel_lvl: GoldScore,
) -> f64
where
    K: Eq + std::hash::Hash,
{
    let k = if k == 0 { preds.len() } else { k };
    if k == 0 {
        return 0.0;
    }
    let mut sum = 0.0;
    let mut n_rels = 0;
    for (i, pred) in preds.iter().enumerate().take(k) {
        if let Some(&rel) = rels.get(&pred.doc_id) {
            if rel >= rel_lvl {
                n_rels += 1;
                sum += compute_precision(rels, preds, i + 1, rel_lvl);
            }
        }
    }
    if n_rels == 0 {
        0.0
    } else {
        sum / n_rels as f64
    }
}
