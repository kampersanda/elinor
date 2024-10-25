use std::collections::BTreeMap;

use crate::metrics::hits::compute_hits;
use crate::PredScore;
use crate::Relevance;
use crate::TrueScore;

/// Computes the precision at k.
pub fn compute_precision<K>(
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
        0.0
    } else {
        compute_hits::<K>(trues, sorted_preds, k, rel_lvl) / k as f64
    }
}
