use anyhow::Result;
use ireval::Metric;

fn main() -> Result<()> {
    let qrels_data = r#"
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
        }
    "#;

    let run_data = r#"
        {
            "q_1": {
                "d_1": 0.5,
                "d_2": 0.4,
                "d_3": 0.3
            },
            "q_2": {
                "d_4": 0.1,
                "d_1": 0.2,
                "d_3": 0.3
            }
        }
    "#;

    let qrels = ireval::json::parse_qrels_from_json(qrels_data)?;
    let run = ireval::json::parse_run_from_json(run_data)?;

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
    let evaluated = ireval::evaluate(&qrels, &run, metrics.iter().cloned())?;

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
