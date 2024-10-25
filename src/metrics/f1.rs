use std::collections::BTreeMap;

use crate::metrics::hits::compute_hits;
use crate::PredScore;
use crate::Relevance;
use crate::TrueScore;

/// Computes the F1 score at k.
pub fn compute_f1<K>(
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
    let hits = compute_hits(trues, sorted_preds, k, rel_lvl);
    let precision = hits / k as f64;
    let recall = hits / trues.values().filter(|&&rel| rel >= rel_lvl).count() as f64;
    if precision + recall == 0.0 {
        0.0
    } else {
        2.0 * (precision * recall) / (precision + recall)
    }
}
