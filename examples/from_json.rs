use anyhow::Result;

#[cfg(not(feature = "serde"))]
fn main() -> Result<()> {
    println!("This example requires the 'serde' feature, such as `cargo pred_rels --example from_json --features serde`.");
    Ok(())
}

#[cfg(feature = "serde")]
fn main() -> Result<()> {
    use elinor::GoldRelStore;
    use elinor::GoldScore;
    use elinor::Metric;
    use elinor::PredRelStore;
    use elinor::PredScore;
    use std::collections::HashMap;

    let gold_rels_data = r#"
{
    "q_1": {
        "d_1": 1,
        "d_2": 0,
        "d_3": 2
    },
    "q_2": {
        "d_2": 2,
        "d_4": 1
    }
}"#;

    let pred_rels_data = r#"
{
    "q_1": {
        "d_1": 0.5,
        "d_2": 0.4,
        "d_3": 0.3
    },
    "q_2": {
        "d_3": 0.3,
        "d_1": 0.2,
        "d_4": 0.1
    }
}"#;

    let gold_rels_map: HashMap<String, HashMap<String, GoldScore>> =
        serde_json::from_str(gold_rels_data)?;
    let pred_rels_map: HashMap<String, HashMap<String, PredScore>> =
        serde_json::from_str(pred_rels_data)?;

    let gold_rels = GoldRelStore::from_map(gold_rels_map);
    let pred_rels = PredRelStore::from_map(pred_rels_map);

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
    let evaluated = elinor::evaluate(&gold_rels, &pred_rels, metrics.iter().cloned())?;

    println!("=== Mean scores ===");
    for metric in &metrics {
        let score = evaluated.mean_scores[metric];
        println!("{metric}: {score:.4}");
    }

    println!("\n=== Scores for each query ===");
    for metric in &metrics {
        println!("{metric}");
        let qid_to_score = &evaluated.all_scores[metric];
        for qid in ["q_1", "q_2"] {
            let score = qid_to_score[qid];
            println!("- {qid}: {score:.4}");
        }
    }

    Ok(())
}
