use std::collections::HashMap;

use emir::DcgWeighting;
use emir::Metric;
use emir::Qrels;
use emir::Run;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let qrels_map = HashMap::from([
        (
            "q_1".to_string(),
            HashMap::from([
                ("d_1".to_string(), 1),
                ("d_2".to_string(), 0),
                ("d_3".to_string(), 2),
            ]),
        ),
        (
            "q_2".to_string(),
            HashMap::from([("d_2".to_string(), 2), ("d_4".to_string(), 1)]),
        ),
    ]);

    let run_map = HashMap::from([
        (
            "q_1".to_string(),
            HashMap::from([
                ("d_1".to_string(), 0.5.into()),
                ("d_2".to_string(), 0.4.into()),
                ("d_3".to_string(), 0.3.into()),
                ("d_4".to_string(), 0.2.into()),
            ]),
        ),
        (
            "q_2".to_string(),
            HashMap::from([
                ("d_4".to_string(), 0.1.into()),
                ("d_1".to_string(), 0.2.into()),
            ]),
        ),
    ]);

    let qrels = Qrels::from_map(qrels_map);
    let run = Run::from_map(run_map);

    let metrics = vec![
        Metric::Hits(3),
        Metric::HitRate(3),
        Metric::Precision(3),
        Metric::Recall(3),
        Metric::F1(3),
        Metric::AveragePrecision(3),
        Metric::ReciprocalRank(3),
        Metric::Ndcg(3, DcgWeighting::Jarvelin),
        Metric::Ndcg(3, DcgWeighting::Burges),
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
