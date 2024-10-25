use std::collections::BTreeMap;

use crate::PredScore;
use crate::Relevance;
use crate::TrueScore;

/// Computes the reciprocal rank at k.
pub fn compute_reciprocal_rank<K>(
    trues: &BTreeMap<K, TrueScore>,
    sorted_preds: &[Relevance<K, PredScore>],
    k: usize,
    rel_lvl: TrueScore,
) -> f64
where
    K: Eq + Ord,
{
    let k = if k == 0 { sorted_preds.len() } else { k };
    if k == 0 {
        return 0.0;
    }
    for (i, pred) in sorted_preds.iter().enumerate().take(k) {
        if let Some(&rel) = trues.get(&pred.doc_id) {
            if rel >= rel_lvl {
                return 1.0 / (i as f64 + 1.0);
            }
        }
    }
    0.0
}
