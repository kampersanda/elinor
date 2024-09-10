use crate::GoldScore;
use crate::PredScore;
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
pub fn compute_hits<K>(
    rels: &RelevanceMap<K, GoldScore>,
    preds: &[Relevance<K, PredScore>],
    k: usize,
    rel_lvl: GoldScore,
) -> f64
where
    K: Eq + std::hash::Hash,
{
    let k = if k == 0 { preds.len() } else { k };
    let mut hits = 0;
    for pred in preds.iter().take(k) {
        if let Some(&rel) = rels.get(&pred.doc_id) {
            if rel >= rel_lvl {
                hits += 1;
            }
        }
    }
    hits as f64
}

/// Returns 1 if at least one relevant document is found, 0 otherwise.
///
/// # Arguments
///
/// * `rels` - Map of relevance levels for each document.
/// * `preds` - Sorted slice of predicted documents with their scores.
/// * `k` - Number of documents to consider.
/// * `rel_lvl` - Relevance level to consider.
pub fn compute_success<K>(
    rels: &RelevanceMap<K, GoldScore>,
    preds: &[Relevance<K, PredScore>],
    k: usize,
    rel_lvl: GoldScore,
) -> f64
where
    K: Eq + std::hash::Hash,
{
    let k = if k == 0 { preds.len() } else { k };
    for pred in preds.iter().take(k) {
        if let Some(&rel) = rels.get(&pred.doc_id) {
            if rel >= rel_lvl {
                return 1.0;
            }
        }
    }
    0.0
}
