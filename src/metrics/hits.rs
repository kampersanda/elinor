use std::collections::BTreeMap;

use crate::GoldScore;
use crate::PredScore;
use crate::Relevance;

/// Computes the number of hits.
pub fn compute_hits<K>(
    golds: &BTreeMap<K, GoldScore>,
    sorted_preds: &[Relevance<K, PredScore>],
    k: usize,
    rel_lvl: GoldScore,
) -> f64
where
    K: Eq + Ord,
{
    let k = if k == 0 { sorted_preds.len() } else { k };
    let mut hits = 0;
    for pred in sorted_preds.iter().take(k) {
        if let Some(&rel) = golds.get(&pred.doc_id) {
            if rel >= rel_lvl {
                hits += 1;
            }
        }
    }
    hits as f64
}
