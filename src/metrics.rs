pub(crate) mod average_precision;
pub(crate) mod f1;
pub(crate) mod hits;
pub(crate) mod ndcg;
pub(crate) mod precision;
pub(crate) mod recall;
pub(crate) mod reciprocal_rank;

use std::collections::HashMap;

use crate::errors::EmirError;
use crate::GoldScore;
use crate::Qrels;
use crate::Run;

#[derive(Debug, Clone, Copy)]
pub enum DcgWeighting {
    /// https://dl.acm.org/doi/10.1145/582415.582418
    Jarvelin,

    /// https://dl.acm.org/doi/10.1145/1102351.1102363
    Burges,
}

#[derive(Debug, Clone, Copy)]
pub enum Metric {
    /// Number of relevant documents retrieved.
    Hits(usize, GoldScore),

    /// Fraction of queries for which at least one relevant document is retrieved.
    HitRate(usize, GoldScore),

    /// Precision at k.
    Precision(usize, GoldScore),

    /// Recall at k.
    Recall(usize, GoldScore),

    /// F1 score at k.
    F1(usize, GoldScore),

    /// Average precision at k.
    AveragePrecision(usize, GoldScore),

    /// Reciprocal rank at k.
    ReciprocalRank(usize, GoldScore),

    /// Discounted cumulative gain at k.
    Dcg(usize, DcgWeighting),

    /// Normalized discounted cumulative gain at k.
    Ndcg(usize, DcgWeighting),
}

pub fn compute_metric<K>(
    qrels: &Qrels<K>,
    run: &Run<K>,
    metric: Metric,
) -> Result<HashMap<K, f64>, EmirError<K>>
where
    K: Clone + Eq + std::hash::Hash + std::fmt::Display,
{
    for query_id in run.query_ids() {
        if qrels.get_map(query_id).is_none() {
            return Err(EmirError::MissingQueryId(query_id.clone()));
        }
    }
    let mut results = HashMap::new();
    for (query_id, preds) in run.query_ids_and_sorted() {
        let rels = qrels.get_map(query_id).unwrap();
        let score = match metric {
            Metric::Hits(k, rel_lvl) => hits::compute_hits(rels, preds, k, rel_lvl),
            Metric::HitRate(k, rel_lvl) => hits::compute_if_hit(rels, preds, k, rel_lvl),
            Metric::Precision(k, rel_lvl) => precision::compute_precision(rels, preds, k, rel_lvl),
            Metric::Recall(k, rel_lvl) => recall::compute_recall(rels, preds, k, rel_lvl),
            Metric::F1(k, rel_lvl) => f1::compute_f1(rels, preds, k, rel_lvl),
            Metric::AveragePrecision(k, rel_lvl) => {
                average_precision::compute_average_precision(rels, preds, k, rel_lvl)
            }
            Metric::ReciprocalRank(k, rel_lvl) => {
                reciprocal_rank::compute_reciprocal_rank(rels, preds, k, rel_lvl)
            }
            Metric::Dcg(k, weighting) => ndcg::compute_dcg(rels, preds, k, weighting),
            Metric::Ndcg(k, weighting) => {
                let golds = qrels.get_sorted(query_id).unwrap();
                ndcg::compute_ndcg(rels, golds, preds, k, weighting)
            }
        };
        results.insert(query_id.clone(), score);
    }
    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PredScore;
    use approx::assert_relative_eq;
    use big_s::S;
    use maplit::hashmap;
    use rstest::*;

    const LOG_2_2: f64 = 1.0;
    const LOG_2_3: f64 = 1.584962500721156;
    const LOG_2_4: f64 = 2.0;

    fn compare_hashmaps(a: &HashMap<String, f64>, b: &HashMap<String, f64>) {
        assert_eq!(a.len(), b.len());
        for (k, v) in a.iter() {
            assert_relative_eq!(v, b.get(k).unwrap());
        }
    }

    #[rstest]
    // Hits (relevance >= 1)
    #[case::hits_k_0_rel_lvl_1(Metric::Hits(0, 1), hashmap! { S("q1") => 2.0 })]
    #[case::hits_k_1_rel_lvl_1(Metric::Hits(1, 1), hashmap! { S("q1") => 1.0 })]
    #[case::hits_k_2_rel_lvl_1(Metric::Hits(2, 1), hashmap! { S("q1") => 1.0 })]
    #[case::hits_k_3_rel_lvl_1(Metric::Hits(3, 1), hashmap! { S("q1") => 2.0 })]
    #[case::hits_k_4_rel_lvl_1(Metric::Hits(4, 1), hashmap! { S("q1") => 2.0 })]
    #[case::hits_k_5_rel_lvl_1(Metric::Hits(5, 1), hashmap! { S("q1") => 2.0 })]
    // Hits (relevance >= 2)
    #[case::hits_k_0_rel_lvl_2(Metric::Hits(0, 2), hashmap! { S("q1") => 1.0 })]
    #[case::hits_k_1_rel_lvl_2(Metric::Hits(1, 2), hashmap! { S("q1") => 0.0 })]
    #[case::hits_k_2_rel_lvl_2(Metric::Hits(2, 2), hashmap! { S("q1") => 0.0 })]
    #[case::hits_k_3_rel_lvl_2(Metric::Hits(3, 2), hashmap! { S("q1") => 1.0 })]
    #[case::hits_k_4_rel_lvl_2(Metric::Hits(4, 2), hashmap! { S("q1") => 1.0 })]
    #[case::hits_k_5_rel_lvl_2(Metric::Hits(5, 2), hashmap! { S("q1") => 1.0 })]
    // Hit rate (relevance >= 1)
    #[case::hit_rate_k_0_rel_lvl_1(Metric::HitRate(0, 1), hashmap! { S("q1") => 1.0 })]
    #[case::hit_rate_k_1_rel_lvl_1(Metric::HitRate(1, 1), hashmap! { S("q1") => 1.0 })]
    #[case::hit_rate_k_2_rel_lvl_1(Metric::HitRate(2, 1), hashmap! { S("q1") => 1.0 })]
    #[case::hit_rate_k_3_rel_lvl_1(Metric::HitRate(3, 1), hashmap! { S("q1") => 1.0 })]
    #[case::hit_rate_k_4_rel_lvl_1(Metric::HitRate(4, 1), hashmap! { S("q1") => 1.0 })]
    #[case::hit_rate_k_5_rel_lvl_1(Metric::HitRate(5, 1), hashmap! { S("q1") => 1.0 })]
    // Hit rate (relevance >= 2)
    #[case::hit_rate_k_0_rel_lvl_2(Metric::HitRate(0, 2), hashmap! { S("q1") => 1.0 })]
    #[case::hit_rate_k_1_rel_lvl_2(Metric::HitRate(1, 2), hashmap! { S("q1") => 0.0 })]
    #[case::hit_rate_k_2_rel_lvl_2(Metric::HitRate(2, 2), hashmap! { S("q1") => 0.0 })]
    #[case::hit_rate_k_3_rel_lvl_2(Metric::HitRate(3, 2), hashmap! { S("q1") => 1.0 })]
    #[case::hit_rate_k_4_rel_lvl_2(Metric::HitRate(4, 2), hashmap! { S("q1") => 1.0 })]
    #[case::hit_rate_k_5_rel_lvl_2(Metric::HitRate(5, 2), hashmap! { S("q1") => 1.0 })]
    // Precision (relevance >= 1)
    #[case::precision_k_0_rel_lvl_1(Metric::Precision(0, 1), hashmap! { S("q1") => 2.0 / 4.0 })]
    #[case::precision_k_1_rel_lvl_1(Metric::Precision(1, 1), hashmap! { S("q1") => 1.0 / 1.0 })]
    #[case::precision_k_2_rel_lvl_1(Metric::Precision(2, 1), hashmap! { S("q1") => 1.0 / 2.0 })]
    #[case::precision_k_3_rel_lvl_1(Metric::Precision(3, 1), hashmap! { S("q1") => 2.0 / 3.0 })]
    #[case::precision_k_4_rel_lvl_1(Metric::Precision(4, 1), hashmap! { S("q1") => 2.0 / 4.0 })]
    #[case::precision_k_5_rel_lvl_1(Metric::Precision(5, 1), hashmap! { S("q1") => 2.0 / 5.0 })]
    // Precision (relevance >= 2)
    #[case::precision_k_0_rel_lvl_2(Metric::Precision(0, 2), hashmap! { S("q1") => 1.0 / 4.0 })]
    #[case::precision_k_1_rel_lvl_2(Metric::Precision(1, 2), hashmap! { S("q1") => 0.0 / 1.0 })]
    #[case::precision_k_2_rel_lvl_2(Metric::Precision(2, 2), hashmap! { S("q1") => 0.0 / 2.0 })]
    #[case::precision_k_3_rel_lvl_2(Metric::Precision(3, 2), hashmap! { S("q1") => 1.0 / 3.0 })]
    #[case::precision_k_4_rel_lvl_2(Metric::Precision(4, 2), hashmap! { S("q1") => 1.0 / 4.0 })]
    #[case::precision_k_5_rel_lvl_2(Metric::Precision(5, 2), hashmap! { S("q1") => 1.0 / 5.0 })]
    // Recall (relevance >= 1)
    #[case::recall_k_0_rel_lvl_1(Metric::Recall(0, 1), hashmap! { S("q1") => 2.0 / 2.0 })]
    #[case::recall_k_1_rel_lvl_1(Metric::Recall(1, 1), hashmap! { S("q1") => 1.0 / 2.0 })]
    #[case::recall_k_2_rel_lvl_1(Metric::Recall(2, 1), hashmap! { S("q1") => 1.0 / 2.0 })]
    #[case::recall_k_3_rel_lvl_1(Metric::Recall(3, 1), hashmap! { S("q1") => 2.0 / 2.0 })]
    #[case::recall_k_4_rel_lvl_1(Metric::Recall(4, 1), hashmap! { S("q1") => 2.0 / 2.0 })]
    #[case::recall_k_5_rel_lvl_1(Metric::Recall(5, 1), hashmap! { S("q1") => 2.0 / 2.0 })]
    // Recall (relevance >= 2)
    #[case::recall_k_0_rel_lvl_2(Metric::Recall(0, 2), hashmap! { S("q1") => 1.0 / 1.0 })]
    #[case::recall_k_1_rel_lvl_2(Metric::Recall(1, 2), hashmap! { S("q1") => 0.0 / 1.0 })]
    #[case::recall_k_2_rel_lvl_2(Metric::Recall(2, 2), hashmap! { S("q1") => 0.0 / 1.0 })]
    #[case::recall_k_3_rel_lvl_2(Metric::Recall(3, 2), hashmap! { S("q1") => 1.0 / 1.0 })]
    #[case::recall_k_4_rel_lvl_2(Metric::Recall(4, 2), hashmap! { S("q1") => 1.0 / 1.0 })]
    #[case::recall_k_5_rel_lvl_2(Metric::Recall(5, 2), hashmap! { S("q1") => 1.0 / 1.0 })]
    // F1 (relevance >= 1)
    #[case::f1_k_0_rel_lvl_1(Metric::F1(0, 1), hashmap! { S("q1") => 2.0 * (2.0 / 4.0) * (2.0 / 2.0) / ((2.0 / 4.0) + (2.0 / 2.0)) })]
    #[case::f1_k_1_rel_lvl_1(Metric::F1(1, 1), hashmap! { S("q1") => 2.0 * (1.0 / 1.0) * (1.0 / 2.0) / ((1.0 / 1.0) + (1.0 / 2.0)) })]
    #[case::f1_k_2_rel_lvl_1(Metric::F1(2, 1), hashmap! { S("q1") => 2.0 * (1.0 / 2.0) * (1.0 / 2.0) / ((1.0 / 2.0) + (1.0 / 2.0)) })]
    #[case::f1_k_3_rel_lvl_1(Metric::F1(3, 1), hashmap! { S("q1") => 2.0 * (2.0 / 3.0) * (2.0 / 2.0) / ((2.0 / 3.0) + (2.0 / 2.0)) })]
    #[case::f1_k_4_rel_lvl_1(Metric::F1(4, 1), hashmap! { S("q1") => 2.0 * (2.0 / 4.0) * (2.0 / 2.0) / ((2.0 / 4.0) + (2.0 / 2.0)) })]
    #[case::f1_k_5_rel_lvl_1(Metric::F1(5, 1), hashmap! { S("q1") => 2.0 * (2.0 / 5.0) * (2.0 / 2.0) / ((2.0 / 5.0) + (2.0 / 2.0)) })]
    // F1 (relevance >= 2)
    #[case::f1_k_0_rel_lvl_2(Metric::F1(0, 2), hashmap! { S("q1") => 2.0 * (1.0 / 4.0) * (1.0 / 1.0) / ((1.0 / 4.0) + (1.0 / 1.0)) })]
    #[case::f1_k_1_rel_lvl_2(Metric::F1(1, 2), hashmap! { S("q1") => 0.0 })]
    #[case::f1_k_2_rel_lvl_2(Metric::F1(2, 2), hashmap! { S("q1") => 0.0 })]
    #[case::f1_k_3_rel_lvl_2(Metric::F1(3, 2), hashmap! { S("q1") => 2.0 * (1.0 / 3.0) * (1.0 / 1.0) / ((1.0 / 3.0) + (1.0 / 1.0)) })]
    #[case::f1_k_4_rel_lvl_2(Metric::F1(4, 2), hashmap! { S("q1") => 2.0 * (1.0 / 4.0) * (1.0 / 1.0) / ((1.0 / 4.0) + (1.0 / 1.0)) })]
    #[case::f1_k_5_rel_lvl_2(Metric::F1(5, 2), hashmap! { S("q1") => 2.0 * (1.0 / 5.0) * (1.0 / 1.0) / ((1.0 / 5.0) + (1.0 / 1.0)) })]
    // Average precision (relevance >= 1)
    #[case::average_precision_k_0_rel_lvl_1(Metric::AveragePrecision(0, 1), hashmap! { S("q1") => ((1.0 / 1.0) + (2.0 / 3.0)) / 2.0 })]
    #[case::average_precision_k_1_rel_lvl_1(Metric::AveragePrecision(1, 1), hashmap! { S("q1") => (1.0 / 1.0) / 1.0 })]
    #[case::average_precision_k_2_rel_lvl_1(Metric::AveragePrecision(2, 1), hashmap! { S("q1") => (1.0 / 1.0) / 1.0 })]
    #[case::average_precision_k_3_rel_lvl_1(Metric::AveragePrecision(3, 1), hashmap! { S("q1") => ((1.0 / 1.0) + (2.0 / 3.0)) / 2.0 })]
    #[case::average_precision_k_4_rel_lvl_1(Metric::AveragePrecision(4, 1), hashmap! { S("q1") => ((1.0 / 1.0) + (2.0 / 3.0)) / 2.0 })]
    #[case::average_precision_k_5_rel_lvl_1(Metric::AveragePrecision(5, 1), hashmap! { S("q1") => ((1.0 / 1.0) + (2.0 / 3.0)) / 2.0 })]
    // Average precision (relevance >= 2)
    #[case::average_precision_k_0_rel_lvl_2(Metric::AveragePrecision(0, 2), hashmap! { S("q1") => (1.0 / 3.0) / 1.0 })]
    #[case::average_precision_k_1_rel_lvl_2(Metric::AveragePrecision(1, 2), hashmap! { S("q1") => 0.0 })]
    #[case::average_precision_k_2_rel_lvl_2(Metric::AveragePrecision(2, 2), hashmap! { S("q1") => 0.0 })]
    #[case::average_precision_k_3_rel_lvl_2(Metric::AveragePrecision(3, 2), hashmap! { S("q1") => (1.0 / 3.0) / 1.0 })]
    #[case::average_precision_k_4_rel_lvl_2(Metric::AveragePrecision(4, 2), hashmap! { S("q1") => (1.0 / 3.0) / 1.0 })]
    #[case::average_precision_k_5_rel_lvl_2(Metric::AveragePrecision(5, 2), hashmap! { S("q1") => (1.0 / 3.0) / 1.0 })]
    // Reciprocal rank (relevance >= 1)
    #[case::reciprocal_rank_k_0_rel_lvl_1(Metric::ReciprocalRank(0, 1), hashmap! { S("q1") => 1.0 / 1.0 })]
    #[case::reciprocal_rank_k_1_rel_lvl_1(Metric::ReciprocalRank(1, 1), hashmap! { S("q1") => 1.0 / 1.0 })]
    #[case::reciprocal_rank_k_2_rel_lvl_1(Metric::ReciprocalRank(2, 1), hashmap! { S("q1") => 1.0 / 1.0 })]
    #[case::reciprocal_rank_k_3_rel_lvl_1(Metric::ReciprocalRank(3, 1), hashmap! { S("q1") => 1.0 / 1.0 })]
    #[case::reciprocal_rank_k_4_rel_lvl_1(Metric::ReciprocalRank(4, 1), hashmap! { S("q1") => 1.0 / 1.0 })]
    #[case::reciprocal_rank_k_5_rel_lvl_1(Metric::ReciprocalRank(5, 1), hashmap! { S("q1") => 1.0 / 1.0 })]
    // Reciprocal rank (relevance >= 2)
    #[case::reciprocal_rank_k_0_rel_lvl_2(Metric::ReciprocalRank(0, 2), hashmap! { S("q1") => 1.0 / 3.0 })]
    #[case::reciprocal_rank_k_1_rel_lvl_2(Metric::ReciprocalRank(1, 2), hashmap! { S("q1") => 0.0 })]
    #[case::reciprocal_rank_k_2_rel_lvl_2(Metric::ReciprocalRank(2, 2), hashmap! { S("q1") => 0.0 })]
    #[case::reciprocal_rank_k_3_rel_lvl_2(Metric::ReciprocalRank(3, 2), hashmap! { S("q1") => 1.0 / 3.0 })]
    #[case::reciprocal_rank_k_4_rel_lvl_2(Metric::ReciprocalRank(4, 2), hashmap! { S("q1") => 1.0 / 3.0 })]
    #[case::reciprocal_rank_k_5_rel_lvl_2(Metric::ReciprocalRank(5, 2), hashmap! { S("q1") => 1.0 / 3.0 })]
    // DCG (Jarvelin)
    #[case::dcg_k_0_jarvelin(Metric::Dcg(0, DcgWeighting::Jarvelin), hashmap! { S("q1") => 1.0 / LOG_2_2 + 2.0 / LOG_2_4 })]
    #[case::dcg_k_1_jarvelin(Metric::Dcg(1, DcgWeighting::Jarvelin), hashmap! { S("q1") => 1.0 / LOG_2_2 })]
    #[case::dcg_k_2_jarvelin(Metric::Dcg(2, DcgWeighting::Jarvelin), hashmap! { S("q1") => 1.0 / LOG_2_2 })]
    #[case::dcg_k_3_jarvelin(Metric::Dcg(3, DcgWeighting::Jarvelin), hashmap! { S("q1") => 1.0 / LOG_2_2 + 2.0 / LOG_2_4 })]
    #[case::dcg_k_4_jarvelin(Metric::Dcg(4, DcgWeighting::Jarvelin), hashmap! { S("q1") => 1.0 / LOG_2_2 + 2.0 / LOG_2_4 })]
    #[case::dcg_k_5_jarvelin(Metric::Dcg(5, DcgWeighting::Jarvelin), hashmap! { S("q1") => 1.0 / LOG_2_2 + 2.0 / LOG_2_4 })]
    // DCG (Burges)
    #[case::dcg_k_0_burges(Metric::Dcg(0, DcgWeighting::Burges), hashmap! { S("q1") => 1.0 / LOG_2_2 + 3.0 / LOG_2_4 })]
    #[case::dcg_k_1_burges(Metric::Dcg(1, DcgWeighting::Burges), hashmap! { S("q1") => 1.0 / LOG_2_2 })]
    #[case::dcg_k_2_burges(Metric::Dcg(2, DcgWeighting::Burges), hashmap! { S("q1") => 1.0 / LOG_2_2 })]
    #[case::dcg_k_3_burges(Metric::Dcg(3, DcgWeighting::Burges), hashmap! { S("q1") => 1.0 / LOG_2_2 + 3.0 / LOG_2_4 })]
    #[case::dcg_k_4_burges(Metric::Dcg(4, DcgWeighting::Burges), hashmap! { S("q1") => 1.0 / LOG_2_2 + 3.0 / LOG_2_4 })]
    #[case::dcg_k_5_burges(Metric::Dcg(5, DcgWeighting::Burges), hashmap! { S("q1") => 1.0 / LOG_2_2 + 3.0 / LOG_2_4 })]
    // NDCG (Jarvelin)
    #[case::ndcg_k_0_jarvelin(Metric::Ndcg(0, DcgWeighting::Jarvelin), hashmap! { S("q1") => (1.0 / LOG_2_2 + 2.0 / LOG_2_4) / (2.0 / LOG_2_2 + 1.0 / LOG_2_3) })]
    #[case::ndcg_k_1_jarvelin(Metric::Ndcg(1, DcgWeighting::Jarvelin), hashmap! { S("q1") => (1.0 / LOG_2_2) / (2.0 / LOG_2_2) })]
    #[case::ndcg_k_2_jarvelin(Metric::Ndcg(2, DcgWeighting::Jarvelin), hashmap! { S("q1") => (1.0 / LOG_2_2) / (2.0 / LOG_2_2 + 1.0 / LOG_2_3) })]
    #[case::ndcg_k_3_jarvelin(Metric::Ndcg(3, DcgWeighting::Jarvelin), hashmap! { S("q1") => (1.0 / LOG_2_2 + 2.0 / LOG_2_4) / (2.0 / LOG_2_2 + 1.0 / LOG_2_3) })]
    #[case::ndcg_k_4_jarvelin(Metric::Ndcg(4, DcgWeighting::Jarvelin), hashmap! { S("q1") => (1.0 / LOG_2_2 + 2.0 / LOG_2_4) / (2.0 / LOG_2_2 + 1.0 / LOG_2_3) })]
    #[case::ndcg_k_5_jarvelin(Metric::Ndcg(5, DcgWeighting::Jarvelin), hashmap! { S("q1") => (1.0 / LOG_2_2 + 2.0 / LOG_2_4) / (2.0 / LOG_2_2 + 1.0 / LOG_2_3) })]
    // NDCG (Burges)
    #[case::ndcg_k_0_burges(Metric::Ndcg(0, DcgWeighting::Burges), hashmap! { S("q1") => (1.0 / LOG_2_2 + 3.0 / LOG_2_4) / (3.0 / LOG_2_2 + 1.0 / LOG_2_3) })]
    #[case::ndcg_k_1_burges(Metric::Ndcg(1, DcgWeighting::Burges), hashmap! { S("q1") => (1.0 / LOG_2_2) / (3.0 / LOG_2_2) })]
    #[case::ndcg_k_2_burges(Metric::Ndcg(2, DcgWeighting::Burges), hashmap! { S("q1") => (1.0 / LOG_2_2) / (3.0 / LOG_2_2 + 1.0 / LOG_2_3) })]
    #[case::ndcg_k_3_burges(Metric::Ndcg(3, DcgWeighting::Burges), hashmap! { S("q1") => (1.0 / LOG_2_2 + 3.0 / LOG_2_4) / (3.0 / LOG_2_2 + 1.0 / LOG_2_3) })]
    #[case::ndcg_k_4_burges(Metric::Ndcg(4, DcgWeighting::Burges), hashmap! { S("q1") => (1.0 / LOG_2_2 + 3.0 / LOG_2_4) / (3.0 / LOG_2_2 + 1.0 / LOG_2_3) })]
    #[case::ndcg_k_5_burges(Metric::Ndcg(5, DcgWeighting::Burges), hashmap! { S("q1") => (1.0 / LOG_2_2 + 3.0 / LOG_2_4) / (3.0 / LOG_2_2 + 1.0 / LOG_2_3) })]
    fn test_compute_metric(#[case] metric: Metric, #[case] expected: HashMap<String, f64>) {
        let qrels = Qrels::from_map(
            None,
            hashmap! {
                S("q1") => hashmap! {
                    S("d1") => 1,
                    S("d2") => 0,
                    S("d3") => 2,
                },
            },
        );
        let run = Run::from_map(
            None,
            hashmap! {
                S("q1") => hashmap! {
                    S("d1") => PredScore::from(0.5),
                    S("d2") => PredScore::from(0.4),
                    S("d3") => PredScore::from(0.3),
                    S("d4") => PredScore::from(0.2),
                },
            },
        );
        let results = compute_metric(&qrels, &run, metric).unwrap();
        compare_hashmaps(&results, &expected);
    }
}
