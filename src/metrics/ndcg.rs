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
pub fn compute_dcg<K>(
    golds: &HashMap<K, GoldScore>,
    sorted_preds: &[Relevance<K, PredScore>],
    k: usize,
    weighting: DcgWeighting,
) -> f64
where
    K: Eq + std::hash::Hash,
{
    let k = if k == 0 { sorted_preds.len() } else { k };
    let mut dcg = 0.0;
    for (i, pred) in sorted_preds.iter().take(k).enumerate() {
        if let Some(&rel) = golds.get(&pred.doc_id) {
            dcg += weighted_score(rel, weighting) / (i as f64 + 2.0).log2();
        }
    }
    dcg
}

/// Computes the NDCG at k for a given relevance level.
pub fn compute_ndcg<K>(
    golds: &HashMap<K, GoldScore>,
    sorted_golds: &[Relevance<K, GoldScore>],
    sorted_preds: &[Relevance<K, PredScore>],
    k: usize,
    weighting: DcgWeighting,
) -> f64
where
    K: Eq + std::hash::Hash + Clone,
{
    let sorted_golds = sorted_golds
        .iter()
        .map(|r| Relevance {
            doc_id: r.doc_id.clone(),
            score: PredScore::from(r.score),
        })
        .collect::<Vec<_>>();
    let dcg = compute_dcg(golds, sorted_preds, k, weighting);
    let idcg = compute_dcg(golds, &sorted_golds, k, weighting);
    if idcg == 0.0 {
        1.0
    } else {
        dcg / idcg
    }
}
