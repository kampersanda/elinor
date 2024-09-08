use crate::GoldScore;
use crate::PredScore;
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
pub fn compute_recall<K>(
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
    let n_rels = rels.values().filter(|&&rel| rel >= rel_lvl).count();
    if n_rels == 0 {
        0.0
    } else {
        compute_hits(rels, preds, k, rel_lvl) / n_rels as f64
    }
}
