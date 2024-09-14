//! Metrics for evaluating information retrieval systems.
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

pub const RELEVANT_LEVEL: GoldScore = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DcgWeighting {
    /// <https://dl.acm.org/doi/10.1145/582415.582418>
    Jarvelin,

    /// <https://dl.acm.org/doi/10.1145/1102351.1102363>
    Burges,
}

/// Metrics for evaluating information retrieval systems.
///
/// # Arguments
///
/// * `k` - Number of top documents to consider.
/// * `w` - Weighting scheme to use (only for DCG and nDCG).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Metric {
    /// Number of relevant documents retrieved:
    ///
    /// ```math
    /// \text{Hits} = | \text{Res} \cap \text{Rel} |
    /// ```
    ///
    /// where:
    ///
    /// * $`\text{Res}`$ is the set of retrieved documents.
    /// * $`\text{Rel}`$ is the set of relevant documents.
    Hits { k: usize },

    /// Binary metric indicating whether at least one relevant document is retrieved:
    ///
    /// ```math
    /// \text{Success} = \left\{ \begin{array}{ll}
    ///     1 & \text{if Hits} > 0 \\
    ///     0 & \text{otherwise}
    /// \end{array} \right.
    /// ```
    Success { k: usize },

    /// Proportion of the retrieved documents that are relevant:
    ///
    /// ```math
    /// \text{Precision} = \frac{\text{Hits}}{|\text{Res}|}
    /// ```
    ///
    /// Note that, when `k` is set, the denominator is fixed to `k`
    /// even if the number of retrieved documents is fewer than `k`:
    ///
    /// ```math
    /// \text{Precision}@k = \frac{\text{Hits}}{k}
    /// ```
    Precision { k: usize },

    /// Ratio between the retrieved documents that are relevant and
    /// the total number of relevant documents:
    ///
    /// ```math
    /// \text{Recall} = \frac{\text{Hits}}{| \text{Rel} |}
    /// ```
    Recall { k: usize },

    /// Harmonic mean of precision and recall:
    ///
    /// ```math
    /// \text{F1} = 2 \times \frac{\text{Precision} \times \text{Recall}}{\text{Precision} + \text{Recall}}
    /// ```
    F1 { k: usize },

    /// Average of the Precision scores computed after each relevant document is retrieved:
    ///
    /// ```math
    /// \text{AP} = \frac{1}{| \text{Rel} |} \sum_{i=1}^{| \text{Res} |} \text{Precision}@i \times
    /// \left\{ \begin{array}{ll} 1 & \text{if the } i \text{-th document is relevant} \\ 0 & \text{otherwise} \end{array} \right.
    /// ```
    AveragePrecision { k: usize },

    /// Multiplicative inverse of the rank of the first retrieved relevant document:
    ///
    /// ```math
    /// \text{RR} = \frac{1}{\text{the rank of the first retrieved relevant document}}
    /// ```
    ReciprocalRank { k: usize },

    /// Discounted cumulative gain at k.
    ///
    /// ```math
    /// \text{DCG}@k = \sum_{i=1}^k \frac{G(\text{rel}_i)}{\log_2(i + 1)}
    /// ```
    Dcg { k: usize, w: DcgWeighting },

    /// Normalized discounted cumulative gain at k.
    ///
    /// ```math
    /// \text{nDCG}@k = \frac{\text{DCG}@k}{\text{IDCG}@k}
    /// ```
    Ndcg { k: usize, w: DcgWeighting },
}

impl std::fmt::Display for Metric {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Metric::Hits { k } => {
                write!(f, "{}", format_binary_metric("Hits", *k))
            }
            Metric::Success { k } => {
                write!(f, "{}", format_binary_metric("Success", *k))
            }
            Metric::Precision { k } => {
                write!(f, "{}", format_binary_metric("Precision", *k))
            }
            Metric::Recall { k } => {
                write!(f, "{}", format_binary_metric("Recall", *k))
            }
            Metric::F1 { k } => {
                write!(f, "{}", format_binary_metric("F1", *k))
            }
            Metric::AveragePrecision { k } => {
                write!(f, "{}", format_binary_metric("MAP", *k))
            }
            Metric::ReciprocalRank { k } => {
                write!(f, "{}", format_binary_metric("MRR", *k))
            }
            Metric::Dcg { k, w } => {
                write!(f, "{}", format_dcg_metric("DCG", *k, *w))
            }
            Metric::Ndcg { k, w } => {
                write!(f, "{}", format_dcg_metric("nDCG", *k, *w))
            }
        }
    }
}

fn format_binary_metric(name: &str, k: usize) -> String {
    if k == 0 {
        format!("{name}")
    } else {
        format!("{name}@{k}")
    }
}

fn format_dcg_metric(name: &str, k: usize, weighting: DcgWeighting) -> String {
    if k == 0 {
        format!("{name}_{weighting:?}")
    } else {
        format!("{name}_{weighting:?}@{k}")
    }
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
    for query_id in run.query_ids() {
        let preds = run.get_sorted(query_id).unwrap();
        let rels = qrels.get_map(query_id).unwrap();
        let score = match metric {
            Metric::Hits { k } => hits::compute_hits(rels, preds, k, RELEVANT_LEVEL),
            Metric::Success { k } => hits::compute_success(rels, preds, k, RELEVANT_LEVEL),
            Metric::Precision { k } => precision::compute_precision(rels, preds, k, RELEVANT_LEVEL),
            Metric::Recall { k } => recall::compute_recall(rels, preds, k, RELEVANT_LEVEL),
            Metric::F1 { k } => f1::compute_f1(rels, preds, k, RELEVANT_LEVEL),
            Metric::AveragePrecision { k } => {
                average_precision::compute_average_precision(rels, preds, k, RELEVANT_LEVEL)
            }
            Metric::ReciprocalRank { k } => {
                reciprocal_rank::compute_reciprocal_rank(rels, preds, k, RELEVANT_LEVEL)
            }
            Metric::Dcg { k, w } => ndcg::compute_dcg(rels, preds, k, w),
            Metric::Ndcg { k, w } => {
                let golds = qrels.get_sorted(query_id).unwrap();
                ndcg::compute_ndcg(rels, golds, preds, k, w)
            }
        };
        results.insert(query_id.clone(), score);
    }
    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use maplit::hashmap;
    use rstest::*;

    const LOG_2_2: f64 = 1.0;
    const LOG_2_3: f64 = 1.584962500721156;
    const LOG_2_4: f64 = 2.0;

    fn compare_hashmaps(a: &HashMap<char, f64>, b: &HashMap<char, f64>) {
        assert_eq!(a.len(), b.len());
        for (k, v) in a.iter() {
            assert_relative_eq!(v, b.get(k).unwrap());
        }
    }

    #[rstest]
    // Hits
    #[case::hits_k_0_rel_lvl_1(Metric::Hits { k: 0 }, hashmap! { 'A' => 2.0 })]
    #[case::hits_k_1_rel_lvl_1(Metric::Hits { k: 1 }, hashmap! { 'A' => 1.0 })]
    #[case::hits_k_2_rel_lvl_1(Metric::Hits { k: 2 }, hashmap! { 'A' => 1.0 })]
    #[case::hits_k_3_rel_lvl_1(Metric::Hits { k: 3 }, hashmap! { 'A' => 2.0 })]
    #[case::hits_k_4_rel_lvl_1(Metric::Hits { k: 4 }, hashmap! { 'A' => 2.0 })]
    #[case::hits_k_5_rel_lvl_1(Metric::Hits { k: 5 }, hashmap! { 'A' => 2.0 })]
    // Hit rate
    #[case::hit_rate_k_0_rel_lvl_1(Metric::Success { k: 0 }, hashmap! { 'A' => 1.0 })]
    #[case::hit_rate_k_1_rel_lvl_1(Metric::Success { k: 1 }, hashmap! { 'A' => 1.0 })]
    #[case::hit_rate_k_2_rel_lvl_1(Metric::Success { k: 2 }, hashmap! { 'A' => 1.0 })]
    #[case::hit_rate_k_3_rel_lvl_1(Metric::Success { k: 3 }, hashmap! { 'A' => 1.0 })]
    #[case::hit_rate_k_4_rel_lvl_1(Metric::Success { k: 4 }, hashmap! { 'A' => 1.0 })]
    #[case::hit_rate_k_5_rel_lvl_1(Metric::Success { k: 5 }, hashmap! { 'A' => 1.0 })]
    // Precision
    #[case::precision_k_0_rel_lvl_1(Metric::Precision { k: 0 }, hashmap! { 'A' => 2.0 / 4.0 })]
    #[case::precision_k_1_rel_lvl_1(Metric::Precision { k: 1 }, hashmap! { 'A' => 1.0 / 1.0 })]
    #[case::precision_k_2_rel_lvl_1(Metric::Precision { k: 2 }, hashmap! { 'A' => 1.0 / 2.0 })]
    #[case::precision_k_3_rel_lvl_1(Metric::Precision { k: 3 }, hashmap! { 'A' => 2.0 / 3.0 })]
    #[case::precision_k_4_rel_lvl_1(Metric::Precision { k: 4 }, hashmap! { 'A' => 2.0 / 4.0 })]
    #[case::precision_k_5_rel_lvl_1(Metric::Precision { k: 5 }, hashmap! { 'A' => 2.0 / 5.0 })]
    // Recall
    #[case::recall_k_0_rel_lvl_1(Metric::Recall { k: 0 }, hashmap! { 'A' => 2.0 / 2.0 })]
    #[case::recall_k_1_rel_lvl_1(Metric::Recall { k: 1 }, hashmap! { 'A' => 1.0 / 2.0 })]
    #[case::recall_k_2_rel_lvl_1(Metric::Recall { k: 2 }, hashmap! { 'A' => 1.0 / 2.0 })]
    #[case::recall_k_3_rel_lvl_1(Metric::Recall { k: 3 }, hashmap! { 'A' => 2.0 / 2.0 })]
    #[case::recall_k_4_rel_lvl_1(Metric::Recall { k: 4 }, hashmap! { 'A' => 2.0 / 2.0 })]
    #[case::recall_k_5_rel_lvl_1(Metric::Recall { k: 5 }, hashmap! { 'A' => 2.0 / 2.0 })]
    // F1
    #[case::f1_k_0_rel_lvl_1(Metric::F1 { k: 0 }, hashmap! { 'A' => 2.0 * (2.0 / 4.0) * (2.0 / 2.0) / ((2.0 / 4.0) + (2.0 / 2.0)) })]
    #[case::f1_k_1_rel_lvl_1(Metric::F1 { k: 1 }, hashmap! { 'A' => 2.0 * (1.0 / 1.0) * (1.0 / 2.0) / ((1.0 / 1.0) + (1.0 / 2.0)) })]
    #[case::f1_k_2_rel_lvl_1(Metric::F1 { k: 2 }, hashmap! { 'A' => 2.0 * (1.0 / 2.0) * (1.0 / 2.0) / ((1.0 / 2.0) + (1.0 / 2.0)) })]
    #[case::f1_k_3_rel_lvl_1(Metric::F1 { k: 3 }, hashmap! { 'A' => 2.0 * (2.0 / 3.0) * (2.0 / 2.0) / ((2.0 / 3.0) + (2.0 / 2.0)) })]
    #[case::f1_k_4_rel_lvl_1(Metric::F1 { k: 4 }, hashmap! { 'A' => 2.0 * (2.0 / 4.0) * (2.0 / 2.0) / ((2.0 / 4.0) + (2.0 / 2.0)) })]
    #[case::f1_k_5_rel_lvl_1(Metric::F1 { k: 5 }, hashmap! { 'A' => 2.0 * (2.0 / 5.0) * (2.0 / 2.0) / ((2.0 / 5.0) + (2.0 / 2.0)) })]
    // Average precision
    #[case::average_precision_k_0_rel_lvl_1(Metric::AveragePrecision { k: 0 }, hashmap! { 'A' => ((1.0 / 1.0) + (2.0 / 3.0)) / 2.0 })]
    #[case::average_precision_k_1_rel_lvl_1(Metric::AveragePrecision { k: 1 }, hashmap! { 'A' => (1.0 / 1.0) / 2.0 })]
    #[case::average_precision_k_2_rel_lvl_1(Metric::AveragePrecision { k: 2 }, hashmap! { 'A' => (1.0 / 1.0) / 2.0 })]
    #[case::average_precision_k_3_rel_lvl_1(Metric::AveragePrecision { k: 3 }, hashmap! { 'A' => ((1.0 / 1.0) + (2.0 / 3.0)) / 2.0 })]
    #[case::average_precision_k_4_rel_lvl_1(Metric::AveragePrecision { k: 4 }, hashmap! { 'A' => ((1.0 / 1.0) + (2.0 / 3.0)) / 2.0 })]
    #[case::average_precision_k_5_rel_lvl_1(Metric::AveragePrecision { k: 5 }, hashmap! { 'A' => ((1.0 / 1.0) + (2.0 / 3.0)) / 2.0 })]
    // Reciprocal rank
    #[case::reciprocal_rank_k_0_rel_lvl_1(Metric::ReciprocalRank { k: 0 }, hashmap! { 'A' => 1.0 / 1.0 })]
    #[case::reciprocal_rank_k_1_rel_lvl_1(Metric::ReciprocalRank { k: 1 }, hashmap! { 'A' => 1.0 / 1.0 })]
    #[case::reciprocal_rank_k_2_rel_lvl_1(Metric::ReciprocalRank { k: 2 }, hashmap! { 'A' => 1.0 / 1.0 })]
    #[case::reciprocal_rank_k_3_rel_lvl_1(Metric::ReciprocalRank { k: 3 }, hashmap! { 'A' => 1.0 / 1.0 })]
    #[case::reciprocal_rank_k_4_rel_lvl_1(Metric::ReciprocalRank { k: 4 }, hashmap! { 'A' => 1.0 / 1.0 })]
    #[case::reciprocal_rank_k_5_rel_lvl_1(Metric::ReciprocalRank { k: 5 }, hashmap! { 'A' => 1.0 / 1.0 })]
    // DCG (Jarvelin)
    #[case::dcg_k_0_jarvelin(Metric::Dcg { k: 0, w: DcgWeighting::Jarvelin }, hashmap! { 'A' => 1.0 / LOG_2_2 + 2.0 / LOG_2_4 })]
    #[case::dcg_k_1_jarvelin(Metric::Dcg { k: 1, w: DcgWeighting::Jarvelin }, hashmap! { 'A' => 1.0 / LOG_2_2 })]
    #[case::dcg_k_2_jarvelin(Metric::Dcg { k: 2, w: DcgWeighting::Jarvelin }, hashmap! { 'A' => 1.0 / LOG_2_2 })]
    #[case::dcg_k_3_jarvelin(Metric::Dcg { k: 3, w: DcgWeighting::Jarvelin }, hashmap! { 'A' => 1.0 / LOG_2_2 + 2.0 / LOG_2_4 })]
    #[case::dcg_k_4_jarvelin(Metric::Dcg { k: 4, w: DcgWeighting::Jarvelin }, hashmap! { 'A' => 1.0 / LOG_2_2 + 2.0 / LOG_2_4 })]
    #[case::dcg_k_5_jarvelin(Metric::Dcg { k: 5, w: DcgWeighting::Jarvelin }, hashmap! { 'A' => 1.0 / LOG_2_2 + 2.0 / LOG_2_4 })]
    // DCG (Burges)
    #[case::dcg_k_0_burges(Metric::Dcg { k: 0, w: DcgWeighting::Burges }, hashmap! { 'A' => 1.0 / LOG_2_2 + 3.0 / LOG_2_4 })]
    #[case::dcg_k_1_burges(Metric::Dcg { k: 1, w: DcgWeighting::Burges }, hashmap! { 'A' => 1.0 / LOG_2_2 })]
    #[case::dcg_k_2_burges(Metric::Dcg { k: 2, w: DcgWeighting::Burges }, hashmap! { 'A' => 1.0 / LOG_2_2 })]
    #[case::dcg_k_3_burges(Metric::Dcg { k: 3, w: DcgWeighting::Burges }, hashmap! { 'A' => 1.0 / LOG_2_2 + 3.0 / LOG_2_4 })]
    #[case::dcg_k_4_burges(Metric::Dcg { k: 4, w: DcgWeighting::Burges }, hashmap! { 'A' => 1.0 / LOG_2_2 + 3.0 / LOG_2_4 })]
    #[case::dcg_k_5_burges(Metric::Dcg { k: 5, w: DcgWeighting::Burges }, hashmap! { 'A' => 1.0 / LOG_2_2 + 3.0 / LOG_2_4 })]
    // NDCG (Jarvelin)
    #[case::ndcg_k_0_jarvelin(Metric::Ndcg { k: 0, w: DcgWeighting::Jarvelin }, hashmap! { 'A' => (1.0 / LOG_2_2 + 2.0 / LOG_2_4) / (2.0 / LOG_2_2 + 1.0 / LOG_2_3) })]
    #[case::ndcg_k_1_jarvelin(Metric::Ndcg { k: 1, w: DcgWeighting::Jarvelin }, hashmap! { 'A' => (1.0 / LOG_2_2) / (2.0 / LOG_2_2) })]
    #[case::ndcg_k_2_jarvelin(Metric::Ndcg { k: 2, w: DcgWeighting::Jarvelin }, hashmap! { 'A' => (1.0 / LOG_2_2) / (2.0 / LOG_2_2 + 1.0 / LOG_2_3) })]
    #[case::ndcg_k_3_jarvelin(Metric::Ndcg { k: 3, w: DcgWeighting::Jarvelin }, hashmap! { 'A' => (1.0 / LOG_2_2 + 2.0 / LOG_2_4) / (2.0 / LOG_2_2 + 1.0 / LOG_2_3) })]
    #[case::ndcg_k_4_jarvelin(Metric::Ndcg { k: 4, w: DcgWeighting::Jarvelin }, hashmap! { 'A' => (1.0 / LOG_2_2 + 2.0 / LOG_2_4) / (2.0 / LOG_2_2 + 1.0 / LOG_2_3) })]
    #[case::ndcg_k_5_jarvelin(Metric::Ndcg { k: 5, w: DcgWeighting::Jarvelin }, hashmap! { 'A' => (1.0 / LOG_2_2 + 2.0 / LOG_2_4) / (2.0 / LOG_2_2 + 1.0 / LOG_2_3) })]
    // NDCG (Burges)
    #[case::ndcg_k_0_burges(Metric::Ndcg { k: 0, w: DcgWeighting::Burges }, hashmap! { 'A' => (1.0 / LOG_2_2 + 3.0 / LOG_2_4) / (3.0 / LOG_2_2 + 1.0 / LOG_2_3) })]
    #[case::ndcg_k_1_burges(Metric::Ndcg { k: 1, w: DcgWeighting::Burges }, hashmap! { 'A' => (1.0 / LOG_2_2) / (3.0 / LOG_2_2) })]
    #[case::ndcg_k_2_burges(Metric::Ndcg { k: 2, w: DcgWeighting::Burges }, hashmap! { 'A' => (1.0 / LOG_2_2) / (3.0 / LOG_2_2 + 1.0 / LOG_2_3) })]
    #[case::ndcg_k_3_burges(Metric::Ndcg { k: 3, w: DcgWeighting::Burges }, hashmap! { 'A' => (1.0 / LOG_2_2 + 3.0 / LOG_2_4) / (3.0 / LOG_2_2 + 1.0 / LOG_2_3) })]
    #[case::ndcg_k_4_burges(Metric::Ndcg { k: 4, w: DcgWeighting::Burges }, hashmap! { 'A' => (1.0 / LOG_2_2 + 3.0 / LOG_2_4) / (3.0 / LOG_2_2 + 1.0 / LOG_2_3) })]
    #[case::ndcg_k_5_burges(Metric::Ndcg { k: 5, w: DcgWeighting::Burges }, hashmap! { 'A' => (1.0 / LOG_2_2 + 3.0 / LOG_2_4) / (3.0 / LOG_2_2 + 1.0 / LOG_2_3) })]
    fn test_compute_metric(#[case] metric: Metric, #[case] expected: HashMap<char, f64>) {
        let qrels = Qrels::from_map(hashmap! {
            'A' => hashmap! {
                'X' => 1,
                'Y' => 0,
                'Z' => 2,
            },
        });
        let run = Run::from_map(hashmap! {
            'A' => hashmap! {
                'X' => 0.5.into(),
                'Y' => 0.4.into(),
                'Z' => 0.3.into(),
                'W' => 0.2.into(),
            },
        });
        let results = compute_metric(&qrels, &run, metric).unwrap();
        compare_hashmaps(&results, &expected);
    }
}
