use std::collections::HashMap;

use crate::metrics::precision::compute_precision;
use crate::GoldScore;
use crate::PredScore;
use crate::Relevance;

/// Computes the average precision at k.
pub fn compute_average_precision<K>(
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
    let n_golds = golds.values().filter(|&&rel| rel >= rel_lvl).count();
    if n_golds == 0 {
        return 0.0;
    }
    let mut sum = 0.0;
    for (i, pred) in sorted_preds.iter().enumerate().take(k) {
        if let Some(&rel) = golds.get(&pred.doc_id) {
            if rel >= rel_lvl {
                sum += compute_precision(golds, sorted_preds, i + 1, rel_lvl);
            }
        }
    }
    sum / n_golds as f64
}
