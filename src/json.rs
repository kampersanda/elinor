use crate::errors::EmirError;
use crate::GoldScore;
use crate::PredScore;
use crate::Qrels;
use crate::QrelsBuilder;
use crate::Run;
use crate::RunBuilder;

pub fn parse_qrels_from_json(data: &str) -> Result<Qrels<String>, EmirError<String>> {
    let qrels_map: serde_json::Value = serde_json::from_str(data).unwrap();
    let mut b = QrelsBuilder::new();
    for (query_id, doc_scores) in qrels_map.as_object().unwrap() {
        let query_id = query_id.as_str();
        for (doc_id, score) in doc_scores.as_object().unwrap() {
            let doc_id = doc_id.as_str();
            let score = GoldScore::try_from(score.as_u64().unwrap()).unwrap();
            b.add_score(query_id.to_string(), doc_id.to_string(), score)?;
        }
    }
    Ok(b.build())
}

pub fn parse_run_from_json(data: &str) -> Result<Run<String>, EmirError<String>> {
    let run_map: serde_json::Value = serde_json::from_str(data).unwrap();
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
