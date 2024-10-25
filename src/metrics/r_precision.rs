use std::collections::BTreeMap;

use crate::metrics::precision::compute_precision;
use crate::PredScore;
use crate::Relevance;
use crate::TrueScore;

/// Computes the R-Precision.
pub fn compute_r_precision<K>(
    trues: &BTreeMap<K, TrueScore>,
    sorted_preds: &[Relevance<K, PredScore>],
    rel_lvl: TrueScore,
) -> f64
where
    K: Eq + Ord,
{
    let n_rels = trues.values().filter(|&&rel| rel >= rel_lvl).count();
    if n_rels == 0 {
        0.0
    } else {
        compute_precision(trues, sorted_preds, n_rels, rel_lvl)
    }
}
