use std::collections::BTreeMap;

use crate::metrics::hits::compute_hits;
use crate::PredScore;
use crate::Relevance;
use crate::TrueScore;

/// Computes the recall at k.
pub fn compute_recall<K>(
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
    let n_rels = trues.values().filter(|&&rel| rel >= rel_lvl).count();
    if n_rels == 0 {
        0.0
    } else {
        compute_hits(trues, sorted_preds, k, rel_lvl) / n_rels as f64
    }
}
