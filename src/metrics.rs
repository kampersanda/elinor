pub(crate) mod average_precision;
pub(crate) mod hits;
pub(crate) mod precision;
pub(crate) mod recall;
pub(crate) mod reciprocal_rank;

use std::collections::HashMap;

use crate::EmirError;
use crate::Qrels;
use crate::Run;

pub enum Metric {
    /// Number of relevant documents retrieved.
    Hits(usize),

    /// Precision at k.
    Precision(usize),

    /// Recall at k.
    Recall(usize),

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
    #[case::hits_k_0_rel_lvl_1(Metric::Hits(0), 1, hashmap! { S("q1") => 0.0 })]
    #[case::hits_k_1_rel_lvl_1(Metric::Hits(1), 1, hashmap! { S("q1") => 1.0 })]
    #[case::hits_k_2_rel_lvl_1(Metric::Hits(2), 1, hashmap! { S("q1") => 2.0 })]
    #[case::hits_k_3_rel_lvl_1(Metric::Hits(3), 1, hashmap! { S("q1") => 2.0 })]
    #[case::hits_k_4_rel_lvl_1(Metric::Hits(4), 1, hashmap! { S("q1") => 3.0 })]
    #[case::hits_k_5_rel_lvl_1(Metric::Hits(5), 1, hashmap! { S("q1") => 3.0 })]
    #[case::hits_k_0_rel_lvl_2(Metric::Hits(0), 2, hashmap! { S("q1") => 0.0 })]
    #[case::hits_k_1_rel_lvl_2(Metric::Hits(1), 2, hashmap! { S("q1") => 0.0 })]
    #[case::hits_k_2_rel_lvl_2(Metric::Hits(2), 2, hashmap! { S("q1") => 1.0 })]
    #[case::hits_k_3_rel_lvl_2(Metric::Hits(3), 2, hashmap! { S("q1") => 1.0 })]
    #[case::hits_k_4_rel_lvl_2(Metric::Hits(4), 2, hashmap! { S("q1") => 1.0 })]
    #[case::hits_k_5_rel_lvl_2(Metric::Hits(5), 2, hashmap! { S("q1") => 1.0 })]
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
                    S("d2") => 2,
                    S("d3") => 0,
                    S("d4") => 1,
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
                    S("d5") => 0.1,
                },
            },
        );
        let scores = evaluate(&qrels, &run, metric, rel_lvl).unwrap();
        compare_hashmaps(&scores, &expected);
    }
}
