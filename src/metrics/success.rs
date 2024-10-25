use std::collections::BTreeMap;

use crate::PredScore;
use crate::Relevance;
use crate::TrueScore;

/// Returns 1 if at least one relevant document is found, 0 otherwise.
pub fn compute_success<K>(
    trues: &BTreeMap<K, TrueScore>,
    sorted_preds: &[Relevance<K, PredScore>],
    k: usize,
    rel_lvl: TrueScore,
) -> f64
where
    K: Eq + Ord,
{
    let k = if k == 0 { sorted_preds.len() } else { k };
    for pred in sorted_preds.iter().take(k) {
        if let Some(&rel) = trues.get(&pred.doc_id) {
            if rel >= rel_lvl {
                return 1.0;
            }
        }
    }
    0.0
}
