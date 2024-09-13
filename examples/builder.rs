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
