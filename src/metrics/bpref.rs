use std::collections::HashMap;

use crate::GoldScore;
use crate::PredScore;
use crate::Relevance;

/// Computes the Bpref.
///
/// # Arguments
///
/// * `rels` - Map of relevance levels for each document.
/// * `preds` - Slice of predicted documents with their scores.
/// * `rel_lvl` - Relevance level to consider.
pub fn compute_bpref<K>(
    rels: &HashMap<K, GoldScore>,
    preds: &[Relevance<K, PredScore>],
    rel_lvl: GoldScore,
) -> f64
where
    K: Eq + std::hash::Hash,
{
    let n_rels = rels.values().filter(|&&rel| rel >= rel_lvl).count() as f64;
    let n_non_rels = rels.len() as f64 - n_rels;

    let mut bpref = 0.0;
    let mut n_non_rels_so_far = 0.0_f64;

    for pred in preds {
        if let Some(&rel) = rels.get(&pred.doc_id) {
            if rel >= rel_lvl {
                bpref += 1.0 - n_non_rels_so_far.min(n_rels) / n_non_rels.min(n_rels);
            } else {
                n_non_rels_so_far += 1.0;
            }
        }
    }
    if n_rels != 0.0 {
        bpref /= n_rels
    }
    bpref
}
