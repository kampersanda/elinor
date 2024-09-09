use anyhow::Result;
use emir::DcgWeighting;
use emir::Metric;
use emir::QrelsBuilder;
use emir::RunBuilder;

fn main() -> Result<()> {
    let mut qb = QrelsBuilder::new();
    qb.add_score("q_1", "d_1", 1)?;
    qb.add_score("q_1", "d_2", 0)?;
    qb.add_score("q_1", "d_3", 2)?;
    qb.add_score("q_2", "d_2", 2)?;
    qb.add_score("q_2", "d_4", 1)?;
    let qrels = qb.build();

    let mut rb = RunBuilder::new();
    rb.add_score("q_1", "d_1", 0.5.into())?;
    rb.add_score("q_1", "d_2", 0.4.into())?;
    rb.add_score("q_1", "d_3", 0.3.into())?;
    rb.add_score("q_2", "d_4", 0.1.into())?;
    rb.add_score("q_2", "d_1", 0.2.into())?;
    rb.add_score("q_2", "d_3", 0.3.into())?;
    let run = rb.build();

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
