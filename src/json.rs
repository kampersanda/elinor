//! JSON parsers for Qrels and Run data.
use crate::errors::EmirError;
use crate::GoldScore;
use crate::PredScore;
use crate::Qrels;
use crate::QrelsBuilder;
use crate::Run;
use crate::RunBuilder;

/// Parses the given JSON data into a Qrels data structure.
pub fn parse_qrels_from_json(data: &str) -> Result<Qrels<String>, EmirError> {
    let qrels_map: serde_json::Value = serde_json::from_str(data)
        .map_err(|e| EmirError::InvalidFormat(format!("Invalid JSON format: {}", e)))?;
    let qrels_map = qrels_map.as_object().map_or_else(
        || Err(EmirError::InvalidFormat("Invalid JSON format".to_string())),
        Ok,
    )?;
    let mut b = QrelsBuilder::new();
    for (query_id, doc_scores) in qrels_map {
        let query_id = query_id.as_str();
        let doc_scores = doc_scores.as_object().map_or_else(
            || Err(EmirError::InvalidFormat("Invalid JSON format".to_string())),
            Ok,
        )?;
        for (doc_id, score) in doc_scores {
            let doc_id = doc_id.as_str();
            let score = score.as_u64().map_or_else(
                || Err(EmirError::InvalidFormat("Invalid JSON format".to_string())),
                Ok,
            )?;
            let score = GoldScore::try_from(score)
                .map_err(|_| EmirError::InvalidFormat("Invalid score".to_string()))?;
            b.add_score(query_id.to_string(), doc_id.to_string(), score)?;
        }
    }
    Ok(b.build())
}

/// Parses the given JSON data into a Run data structure.
pub fn parse_run_from_json(data: &str) -> Result<Run<String>, EmirError> {
    let run_map: serde_json::Value = serde_json::from_str(data)
        .map_err(|e| EmirError::InvalidFormat(format!("Invalid JSON format: {}", e)))?;
    let mut b = RunBuilder::new();
    for (query_id, doc_scores) in run_map.as_object().unwrap() {
        let query_id = query_id.as_str();
        for (doc_id, score) in doc_scores.as_object().unwrap() {
            let doc_id = doc_id.as_str();
            let score = PredScore::from(score.as_f64().unwrap());
            b.add_score(query_id.to_string(), doc_id.to_string(), score)?;
        }
    }
    Ok(b.build())
}
