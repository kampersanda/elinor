use std::collections::HashMap;

use anyhow::Result;
use emir::DcgWeighting;
use emir::Metric;
use emir::Qrels;
use emir::Run;

fn main() -> Result<()> {
    let qrels_map = HashMap::from([
        ("q_1", HashMap::from([("d_1", 1), ("d_2", 0), ("d_3", 2)])),
        ("q_2", HashMap::from([("d_2", 2), ("d_4", 1)])),
    ]);

    let run_map = HashMap::from([
        (
            "q_1",
            HashMap::from([
                ("d_1", 0.5.into()),
                ("d_2", 0.4.into()),
                ("d_3", 0.3.into()),
            ]),
        ),
        (
            "q_2",
            HashMap::from([
                ("d_4", 0.1.into()),
                ("d_1", 0.2.into()),
                ("d_3", 0.3.into()),
            ]),
        ),
    ]);

    let qrels = Qrels::from_map(qrels_map);
    let run = Run::from_map(run_map);

    let metrics = vec![
        Metric::Hits { k: 3 },
        Metric::HitRate { k: 3 },
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
    let evaluated = emir::evaluate(&qrels, &run, metrics)?;

    println!("=== Mean scores ===");
    for (metric, score) in evaluated.mean_scores.iter() {
        println!("{metric}: {score:.4}");
    }

    println!("\n=== Scores by query ===");
    for (metric, scores) in evaluated.scores.iter() {
        println!("{metric}");
        for (query_id, score) in scores.iter() {
            println!("- {query_id}: {score:.4}");
        }
    }

    Ok(())
}
