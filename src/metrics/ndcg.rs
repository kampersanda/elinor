use std::collections::BTreeMap;

use crate::PredScore;
use crate::Relevance;
use crate::TrueScore;

#[derive(Clone, Copy, Debug)]
pub enum DcgWeighting {
    Jarvelin,
    Burges,
}

fn weighted_score(rel: TrueScore, weighting: DcgWeighting) -> f64 {
    match weighting {
        DcgWeighting::Jarvelin => rel as f64,
        DcgWeighting::Burges => 2.0_f64.powi(rel as i32) - 1.0,
    }
}

/// Computes the DCG at k.
pub fn compute_dcg<K>(
    trues: &BTreeMap<K, TrueScore>,
    sorted_preds: &[Relevance<K, PredScore>],
    k: usize,
    weighting: DcgWeighting,
) -> f64
where
    K: Eq + Ord,
{
    let k = if k == 0 { sorted_preds.len() } else { k };
    let mut dcg = 0.0;
    for (i, pred) in sorted_preds.iter().take(k).enumerate() {
        if let Some(&rel) = trues.get(&pred.doc_id) {
            dcg += weighted_score(rel, weighting) / (i as f64 + 2.0).log2();
        }
    }
    dcg
}

/// Computes the NDCG at k.
pub fn compute_ndcg<K>(
    trues: &BTreeMap<K, TrueScore>,
    sorted_trues: &[Relevance<K, TrueScore>],
    sorted_preds: &[Relevance<K, PredScore>],
    k: usize,
    weighting: DcgWeighting,
) -> f64
where
    K: Eq + Ord + Clone,
{
    let sorted_trues = sorted_trues
        .iter()
        .map(|r| Relevance {
            doc_id: r.doc_id.clone(),
            score: PredScore::from(r.score),
        })
        .collect::<Vec<_>>();
    let dcg = compute_dcg(trues, sorted_preds, k, weighting);
    let idcg = compute_dcg(trues, &sorted_trues, k, weighting);
    if idcg == 0.0 {
        1.0
    } else {
        dcg / idcg
    }
}
