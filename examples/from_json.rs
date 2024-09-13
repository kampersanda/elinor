use anyhow::Result;
use emir::DcgWeighting;
use emir::Metric;

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

    let qrels = emir::json::parse_qrels_from_json(qrels_data)?;
    let run = emir::json::parse_run_from_json(run_data)?;

    let metrics = vec![
        Metric::Hits { k: 3 },
        Metric::Success { k: 3 },
        Metric::Precision { k: 3 },
        Metric::Recall { k: 3 },
        Metric::F1 { k: 3 },
        Metric::AveragePrecision { k: 3 },
        Metric::ReciprocalRank { k: 3 },
        Metric::Ndcg {
            k: 3,
            w: DcgWeighting::Jarvelin,
        },
        Metric::Ndcg {
            k: 3,
            w: DcgWeighting::Burges,
        },
    ];
    let evaluated = emir::evaluate(&qrels, &run, metrics.iter().cloned())?;

    println!("=== Mean scores ===");
    for metric in &metrics {
        let score = evaluated.mean_scores[metric];
        println!("{metric}: {score:.4}");
    }

    println!("\n=== Scores for each query ===");
    for metric in &metrics {
        println!("{metric}");
        let qid_to_score = &evaluated.scores[metric];
        for qid in ["q_1", "q_2"] {
            let score = qid_to_score[qid];
            println!("- {qid}: {score:.4}");
        }
    }

    Ok(())
}
