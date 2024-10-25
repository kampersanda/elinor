use std::collections::BTreeMap;

use crate::PredScore;
use crate::Relevance;
use crate::TrueScore;

/// Computes the number of hits.
pub fn compute_hits<K>(
    trues: &BTreeMap<K, TrueScore>,
    sorted_preds: &[Relevance<K, PredScore>],
    k: usize,
    rel_lvl: TrueScore,
) -> f64
where
    K: Eq + Ord,
{
    let k = if k == 0 { sorted_preds.len() } else { k };
    let mut hits = 0;
    for pred in sorted_preds.iter().take(k) {
        if let Some(&rel) = trues.get(&pred.doc_id) {
            if rel >= rel_lvl {
                hits += 1;
            }
        }
    }
    hits as f64
}
