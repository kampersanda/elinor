use anyhow::Result;

#[cfg(not(feature = "serde"))]
fn main() -> Result<()> {
    println!("This example requires the 'serde' feature, such as `cargo run --example from_jsonl --features serde`.");
    Ok(())
}

#[cfg(feature = "serde")]
fn main() -> Result<()> {
    use elinor::{GoldRecord, GoldRelStore, Metric, PredRecord, PredRelStore};

    let gold_data = r#"{"query_id": "q_1", "doc_id": "d_1", "score": 1}
{"query_id": "q_1", "doc_id": "d_2", "score": 0}
{"query_id": "q_1", "doc_id": "d_3", "score": 2}
{"query_id": "q_2", "doc_id": "d_2", "score": 2}
{"query_id": "q_2", "doc_id": "d_4", "score": 1}"#;

    let pred_data = r#"{"query_id": "q_1", "doc_id": "d_1", "score": 0.5}
{"query_id": "q_1", "doc_id": "d_2", "score": 0.4}
{"query_id": "q_1", "doc_id": "d_3", "score": 0.3}
{"query_id": "q_2", "doc_id": "d_3", "score": 0.3}
{"query_id": "q_2", "doc_id": "d_1", "score": 0.2}
{"query_id": "q_2", "doc_id": "d_4", "score": 0.1}"#;

    let gold_records = gold_data
        .lines()
        .map(|line| serde_json::from_str::<GoldRecord<String>>(line).unwrap());
    let pred_records = pred_data
        .lines()
        .map(|line| serde_json::from_str::<PredRecord<String>>(line).unwrap());

    let gold_rels = GoldRelStore::from_records(gold_records)?;
    let pred_rels = PredRelStore::from_records(pred_records)?;

    let metrics = vec![
        Metric::Hits { k: 3 },
        Metric::Success { k: 3 },
        Metric::Precision { k: 3 },
        Metric::Recall { k: 3 },
        Metric::F1 { k: 3 },
        Metric::AP { k: 3 },
        Metric::RR { k: 3 },
        Metric::NDCG { k: 3 },
        Metric::NDCGBurges { k: 3 },
    ];

    for metric in metrics {
        let evaluated = elinor::evaluate(&gold_rels, &pred_rels, metric)?;
        println!("{:?}: {:.4}", metric, evaluated.mean_score());
    }

    Ok(())
}
