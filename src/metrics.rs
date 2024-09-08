pub(crate) mod average_precision;
pub(crate) mod f1;
pub(crate) mod hits;
pub(crate) mod precision;
pub(crate) mod recall;
pub(crate) mod reciprocal_rank;

use std::collections::HashMap;

use crate::errors::EmirError;
use crate::qrels::Qrels;
use crate::run::Run;

pub enum Metric {
    /// Number of relevant documents retrieved.
    Hits(usize),

    /// Precision at k.
    Precision(usize),

    /// Recall at k.
    Recall(usize),

    /// F1 score at k.
    F1(usize),

    /// Average precision at k.
    AveragePrecision(usize),

    /// Reciprocal rank at k.
    ReciprocalRank(usize),
}

pub fn evaluate(
    qrels: &Qrels,
    run: &Run,
    metric: Metric,
    rel_lvl: i32,
) -> Result<HashMap<String, f64>, EmirError> {
    for query_id in run.query_ids() {
        if qrels.get_rels(query_id).is_none() {
            return Err(EmirError::MissingQueryId(query_id.clone()));
        }
    }
    let mut scores = HashMap::new();
    for (query_id, preds) in run.iter() {
        let rels = qrels.get_rels(query_id).unwrap();
        let score = match metric {
            Metric::Hits(k) => hits::compute_hits(rels, preds, k, rel_lvl),
            Metric::Precision(k) => precision::compute_precision(rels, preds, k, rel_lvl),
            Metric::Recall(k) => recall::compute_recall(rels, preds, k, rel_lvl),
            Metric::F1(k) => f1::compute_f1(rels, preds, k, rel_lvl),
            Metric::AveragePrecision(k) => {
                average_precision::compute_average_precision(rels, preds, k, rel_lvl)
            }
            Metric::ReciprocalRank(k) => {
                reciprocal_rank::compute_reciprocal_rank(rels, preds, k, rel_lvl)
            }
        };
        scores.insert(query_id.clone(), score);
    }
    Ok(scores)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use big_s::S;
    use maplit::hashmap;
    use rstest::*;

    fn compare_hashmaps(a: &HashMap<String, f64>, b: &HashMap<String, f64>) {
        assert_eq!(a.len(), b.len());
        for (k, v) in a.iter() {
            assert_relative_eq!(v, b.get(k).unwrap());
        }
    }

    #[rstest]
    // Hits (relevance >= 1)
    #[case::hits_k_0_rel_lvl_1(Metric::Hits(0), 1, hashmap! { S("q1") => 2.0 })]
    #[case::hits_k_1_rel_lvl_1(Metric::Hits(1), 1, hashmap! { S("q1") => 1.0 })]
    #[case::hits_k_2_rel_lvl_1(Metric::Hits(2), 1, hashmap! { S("q1") => 1.0 })]
    #[case::hits_k_3_rel_lvl_1(Metric::Hits(3), 1, hashmap! { S("q1") => 2.0 })]
    #[case::hits_k_4_rel_lvl_1(Metric::Hits(4), 1, hashmap! { S("q1") => 2.0 })]
    #[case::hits_k_5_rel_lvl_1(Metric::Hits(5), 1, hashmap! { S("q1") => 2.0 })]
    // Hits (relevance >= 2)
    #[case::hits_k_0_rel_lvl_2(Metric::Hits(0), 2, hashmap! { S("q1") => 1.0 })]
    #[case::hits_k_1_rel_lvl_2(Metric::Hits(1), 2, hashmap! { S("q1") => 0.0 })]
    #[case::hits_k_2_rel_lvl_2(Metric::Hits(2), 2, hashmap! { S("q1") => 0.0 })]
    #[case::hits_k_3_rel_lvl_2(Metric::Hits(3), 2, hashmap! { S("q1") => 1.0 })]
    #[case::hits_k_4_rel_lvl_2(Metric::Hits(4), 2, hashmap! { S("q1") => 1.0 })]
    #[case::hits_k_5_rel_lvl_2(Metric::Hits(5), 2, hashmap! { S("q1") => 1.0 })]
    // Precision (relevance >= 1)
    #[case::precision_k_0_rel_lvl_1(Metric::Precision(0), 1, hashmap! { S("q1") => 2.0 / 4.0 })]
    #[case::precision_k_1_rel_lvl_1(Metric::Precision(1), 1, hashmap! { S("q1") => 1.0 / 1.0 })]
    #[case::precision_k_2_rel_lvl_1(Metric::Precision(2), 1, hashmap! { S("q1") => 1.0 / 2.0 })]
    #[case::precision_k_3_rel_lvl_1(Metric::Precision(3), 1, hashmap! { S("q1") => 2.0 / 3.0 })]
    #[case::precision_k_4_rel_lvl_1(Metric::Precision(4), 1, hashmap! { S("q1") => 2.0 / 4.0 })]
    #[case::precision_k_5_rel_lvl_1(Metric::Precision(5), 1, hashmap! { S("q1") => 2.0 / 5.0 })]
    // Precision (relevance >= 2)
    #[case::precision_k_0_rel_lvl_2(Metric::Precision(0), 2, hashmap! { S("q1") => 1.0 / 4.0 })]
    #[case::precision_k_1_rel_lvl_2(Metric::Precision(1), 2, hashmap! { S("q1") => 0.0 / 1.0 })]
    #[case::precision_k_2_rel_lvl_2(Metric::Precision(2), 2, hashmap! { S("q1") => 0.0 / 2.0 })]
    #[case::precision_k_3_rel_lvl_2(Metric::Precision(3), 2, hashmap! { S("q1") => 1.0 / 3.0 })]
    #[case::precision_k_4_rel_lvl_2(Metric::Precision(4), 2, hashmap! { S("q1") => 1.0 / 4.0 })]
    #[case::precision_k_5_rel_lvl_2(Metric::Precision(5), 2, hashmap! { S("q1") => 1.0 / 5.0 })]
    // Recall (relevance >= 1)
    #[case::recall_k_0_rel_lvl_1(Metric::Recall(0), 1, hashmap! { S("q1") => 2.0 / 2.0 })]
    #[case::recall_k_1_rel_lvl_1(Metric::Recall(1), 1, hashmap! { S("q1") => 1.0 / 2.0 })]
    #[case::recall_k_2_rel_lvl_1(Metric::Recall(2), 1, hashmap! { S("q1") => 1.0 / 2.0 })]
    #[case::recall_k_3_rel_lvl_1(Metric::Recall(3), 1, hashmap! { S("q1") => 2.0 / 2.0 })]
    #[case::recall_k_4_rel_lvl_1(Metric::Recall(4), 1, hashmap! { S("q1") => 2.0 / 2.0 })]
    #[case::recall_k_5_rel_lvl_1(Metric::Recall(5), 1, hashmap! { S("q1") => 2.0 / 2.0 })]
    // Recall (relevance >= 2)
    #[case::recall_k_0_rel_lvl_2(Metric::Recall(0), 2, hashmap! { S("q1") => 1.0 / 1.0 })]
    #[case::recall_k_1_rel_lvl_2(Metric::Recall(1), 2, hashmap! { S("q1") => 0.0 / 1.0 })]
    #[case::recall_k_2_rel_lvl_2(Metric::Recall(2), 2, hashmap! { S("q1") => 0.0 / 1.0 })]
    #[case::recall_k_3_rel_lvl_2(Metric::Recall(3), 2, hashmap! { S("q1") => 1.0 / 1.0 })]
    #[case::recall_k_4_rel_lvl_2(Metric::Recall(4), 2, hashmap! { S("q1") => 1.0 / 1.0 })]
    #[case::recall_k_5_rel_lvl_2(Metric::Recall(5), 2, hashmap! { S("q1") => 1.0 / 1.0 })]
    // F1 (relevance >= 1)
    #[case::f1_k_0_rel_lvl_1(Metric::F1(0), 1, hashmap! { S("q1") => 2.0 * (2.0 / 4.0) * (2.0 / 2.0) / ((2.0 / 4.0) + (2.0 / 2.0)) })]
    #[case::f1_k_1_rel_lvl_1(Metric::F1(1), 1, hashmap! { S("q1") => 2.0 * (1.0 / 1.0) * (1.0 / 2.0) / ((1.0 / 1.0) + (1.0 / 2.0)) })]
    #[case::f1_k_2_rel_lvl_1(Metric::F1(2), 1, hashmap! { S("q1") => 2.0 * (1.0 / 2.0) * (1.0 / 2.0) / ((1.0 / 2.0) + (1.0 / 2.0)) })]
    #[case::f1_k_3_rel_lvl_1(Metric::F1(3), 1, hashmap! { S("q1") => 2.0 * (2.0 / 3.0) * (2.0 / 2.0) / ((2.0 / 3.0) + (2.0 / 2.0)) })]
    #[case::f1_k_4_rel_lvl_1(Metric::F1(4), 1, hashmap! { S("q1") => 2.0 * (2.0 / 4.0) * (2.0 / 2.0) / ((2.0 / 4.0) + (2.0 / 2.0)) })]
    #[case::f1_k_5_rel_lvl_1(Metric::F1(5), 1, hashmap! { S("q1") => 2.0 * (2.0 / 5.0) * (2.0 / 2.0) / ((2.0 / 5.0) + (2.0 / 2.0)) })]
    // F1 (relevance >= 2)
    #[case::f1_k_0_rel_lvl_2(Metric::F1(0), 2, hashmap! { S("q1") => 2.0 * (1.0 / 4.0) * (1.0 / 1.0) / ((1.0 / 4.0) + (1.0 / 1.0)) })]
    #[case::f1_k_1_rel_lvl_2(Metric::F1(1), 2, hashmap! { S("q1") => 0.0 })]
    #[case::f1_k_2_rel_lvl_2(Metric::F1(2), 2, hashmap! { S("q1") => 0.0 })]
    #[case::f1_k_3_rel_lvl_2(Metric::F1(3), 2, hashmap! { S("q1") => 2.0 * (1.0 / 3.0) * (1.0 / 1.0) / ((1.0 / 3.0) + (1.0 / 1.0)) })]
    #[case::f1_k_4_rel_lvl_2(Metric::F1(4), 2, hashmap! { S("q1") => 2.0 * (1.0 / 4.0) * (1.0 / 1.0) / ((1.0 / 4.0) + (1.0 / 1.0)) })]
    #[case::f1_k_5_rel_lvl_2(Metric::F1(5), 2, hashmap! { S("q1") => 2.0 * (1.0 / 5.0) * (1.0 / 1.0) / ((1.0 / 5.0) + (1.0 / 1.0)) })]
    // Average precision (relevance >= 1)
    #[case::average_precision_k_0_rel_lvl_1(Metric::AveragePrecision(0), 1, hashmap! { S("q1") => ((1.0 / 1.0) + (2.0 / 3.0)) / 2.0 })]
    #[case::average_precision_k_1_rel_lvl_1(Metric::AveragePrecision(1), 1, hashmap! { S("q1") => (1.0 / 1.0) / 1.0 })]
    #[case::average_precision_k_2_rel_lvl_1(Metric::AveragePrecision(2), 1, hashmap! { S("q1") => (1.0 / 1.0) / 1.0 })]
    #[case::average_precision_k_3_rel_lvl_1(Metric::AveragePrecision(3), 1, hashmap! { S("q1") => ((1.0 / 1.0) + (2.0 / 3.0)) / 2.0 })]
    #[case::average_precision_k_4_rel_lvl_1(Metric::AveragePrecision(4), 1, hashmap! { S("q1") => ((1.0 / 1.0) + (2.0 / 3.0)) / 2.0 })]
    #[case::average_precision_k_5_rel_lvl_1(Metric::AveragePrecision(5), 1, hashmap! { S("q1") => ((1.0 / 1.0) + (2.0 / 3.0)) / 2.0 })]
    // Average precision (relevance >= 2)
    #[case::average_precision_k_0_rel_lvl_2(Metric::AveragePrecision(0), 2, hashmap! { S("q1") => (1.0 / 3.0) / 1.0 })]
    #[case::average_precision_k_1_rel_lvl_2(Metric::AveragePrecision(1), 2, hashmap! { S("q1") => 0.0 })]
    #[case::average_precision_k_2_rel_lvl_2(Metric::AveragePrecision(2), 2, hashmap! { S("q1") => 0.0 })]
    #[case::average_precision_k_3_rel_lvl_2(Metric::AveragePrecision(3), 2, hashmap! { S("q1") => (1.0 / 3.0) / 1.0 })]
    #[case::average_precision_k_4_rel_lvl_2(Metric::AveragePrecision(4), 2, hashmap! { S("q1") => (1.0 / 3.0) / 1.0 })]
    #[case::average_precision_k_5_rel_lvl_2(Metric::AveragePrecision(5), 2, hashmap! { S("q1") => (1.0 / 3.0) / 1.0 })]
    fn test_evaluate(
        #[case] metric: Metric,
        #[case] rel_lvl: i32,
        #[case] expected: HashMap<String, f64>,
    ) {
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
                    S("d1") => 0.5,
                    S("d2") => 0.4,
                    S("d3") => 0.3,
                    S("d4") => 0.2,
                },
            },
        );
        let scores = evaluate(&qrels, &run, metric, rel_lvl).unwrap();
        compare_hashmaps(&scores, &expected);
    }
}
