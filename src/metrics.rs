pub(crate) mod average_precision;
pub(crate) mod hits;
pub(crate) mod precision;
pub(crate) mod recall;
pub(crate) mod reciprocal_rank;

use crate::EmirError;
use crate::Qrels;
use crate::Run;

pub enum Metric {
    /// Number of relevant documents retrieved.
    Hits(usize),
}

pub fn evaluate(qrels: &Qrels, run: &Run, metric: Metric, rel_lvl: i32) -> Result<f64, EmirError> {
    for query_id in run.query_ids() {
        if qrels.get_rels(query_id).is_none() {
            return Err(EmirError::MissingQueryId(query_id.clone()));
        }
    }
    let mut scores = Vec::new();
    for (query_id, preds) in run.iter() {
        let rels = qrels.get_rels(query_id).unwrap();
        let score = match metric {
            Metric::Hits(k) => hits::compute_hits(rels, preds, k, rel_lvl),
        };
        scores.push(score);
    }
    Ok(scores.iter().sum::<f64>() / scores.len() as f64)
}
