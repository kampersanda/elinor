use std::collections::HashMap;

use crate::GoldScore;
use crate::PredScore;
use crate::Relevance;

#[derive(Clone, Copy, Debug)]
pub enum DcgWeighting {
    Jarvelin,
    Burges,
}

fn weighted_score(rel: GoldScore, weighting: DcgWeighting) -> f64 {
    match weighting {
        DcgWeighting::Jarvelin => rel as f64,
        DcgWeighting::Burges => 2.0_f64.powi(rel as i32) - 1.0,
    }
}

/// Computes the DCG at k for a given relevance level.
///
/// # Arguments
///
/// * `rels` - Map of relevance levels for each document.
/// * `preds` - Slice of predicted documents with their scores.
/// * `k` - Number of documents to consider.
/// * `weighting` - Weighting scheme to use.
pub fn compute_dcg<K>(
    rels: &HashMap<K, GoldScore>,
    preds: &[Relevance<K, PredScore>],
    k: usize,
    weighting: DcgWeighting,
) -> f64
where
    K: Eq + std::hash::Hash,
{
    let k = if k == 0 { preds.len() } else { k };
    let mut dcg = 0.0;
    for (i, pred) in preds.iter().take(k).enumerate() {
        if let Some(&rel) = rels.get(&pred.doc_id) {
            dcg += weighted_score(rel, weighting) / (i as f64 + 2.0).log2();
        }
    }
    dcg
}

/// Computes the NDCG at k for a given relevance level.
///
/// # Arguments
///
/// * `rels` - Map of relevance levels for each document.
/// * `golds` - Slice of gold documents with their scores.
/// * `preds` - Slice of predicted documents with their scores.
/// * `k` - Number of documents to consider.
/// * `weighting` - Weighting scheme to use.
pub fn compute_ndcg<K>(
    rels: &HashMap<K, GoldScore>,
    golds: &[Relevance<K, GoldScore>],
    preds: &[Relevance<K, PredScore>],
    k: usize,
    weighting: DcgWeighting,
) -> f64
where
    K: Eq + std::hash::Hash + Clone,
{
    let golds = golds
        .iter()
        .map(|r| Relevance {
            doc_id: r.doc_id.clone(),
            score: PredScore::from(r.score),
        })
        .collect::<Vec<_>>();
    let dcg = compute_dcg(rels, preds, k, weighting);
    let idcg = compute_dcg(rels, &golds, k, weighting);
    if idcg == 0.0 {
        1.0
    } else {
        dcg / idcg
    }
}
