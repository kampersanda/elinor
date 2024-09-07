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
    let mut evaluated = HashMap::new();
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
        evaluated.insert(query_id.clone(), score);
    }
    Ok(evaluated)
}

#[cfg(test)]
mod tests {
    use super::*;
}
