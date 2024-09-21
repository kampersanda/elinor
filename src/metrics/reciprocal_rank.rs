use std::collections::HashMap;

use crate::GoldScore;
use crate::PredScore;
use crate::Relevance;

/// Computes the reciprocal rank at k.
pub fn compute_reciprocal_rank<K>(
    golds: &HashMap<K, GoldScore>,
    sorted_preds: &[Relevance<K, PredScore>],
    k: usize,
    rel_lvl: GoldScore,
) -> f64
where
    K: Eq + std::hash::Hash,
{
    let k = if k == 0 { sorted_preds.len() } else { k };
    if k == 0 {
        return 0.0;
    }
    for (i, pred) in sorted_preds.iter().enumerate().take(k) {
        if let Some(&rel) = golds.get(&pred.doc_id) {
            if rel >= rel_lvl {
                return 1.0 / (i as f64 + 1.0);
            }
        }
    }
    0.0
}
