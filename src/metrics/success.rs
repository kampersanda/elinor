use std::collections::HashMap;

use crate::GoldScore;
use crate::PredScore;
use crate::Relevance;

/// Returns 1 if at least one relevant document is found, 0 otherwise.
pub fn compute_success<K>(
    golds: &HashMap<K, GoldScore>,
    sorted_preds: &[Relevance<K, PredScore>],
    k: usize,
    rel_lvl: GoldScore,
) -> f64
where
    K: Eq + std::hash::Hash,
{
    let k = if k == 0 { sorted_preds.len() } else { k };
    for pred in sorted_preds.iter().take(k) {
        if let Some(&rel) = golds.get(&pred.doc_id) {
            if rel >= rel_lvl {
                return 1.0;
            }
        }
    }
    0.0
}
