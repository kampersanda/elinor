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
    let n_rels = rels.values().filter(|&&rel| rel >= rel_lvl).count();
    let n_non_rels = rels.len() - n_rels;
    let norm = n_rels.min(n_non_rels) as f64;

    let mut bpref = 0.0;
    let mut n_prefix_non_rels = 0.0;
    for pred in preds {
        if let Some(&rel) = rels.get(&pred.doc_id) {
            if rel >= rel_lvl {
                bpref += 1.0 - n_prefix_non_rels / norm;
            } else {
                n_prefix_non_rels += 1.0;
            }
        }
    }
    bpref / n_rels as f64
}
