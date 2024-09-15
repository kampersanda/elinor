use anyhow::Result;
use ireval::Metric;

fn main() -> Result<()> {
    // <QueryID> <Dummy> <DocID> <Relevance>
    let qrels_data = "
q_1 0 d_1 1
q_1 0 d_2 0
q_1 0 d_3 2
q_2 0 d_2 2
q_2 0 d_4 1
    "
    .trim();

    // <QueryID> <Dummy> <DocID> <Rank> <Score> <RunName>
    let run_data = "
q_1 0 d_1 1 0.5 SAMPLE
q_1 0 d_2 2 0.4 SAMPLE
q_1 0 d_3 3 0.3 SAMPLE
q_2 0 d_3 1 0.3 SAMPLE
q_2 0 d_1 2 0.2 SAMPLE
q_2 0 d_4 3 0.1 SAMPLE
    "
    .trim();

    let qrels = ireval::trec::parse_qrels_from_trec(qrels_data.lines())?;
    let run = ireval::trec::parse_run_from_trec(run_data.lines())?;

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
