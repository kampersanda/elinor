use anyhow::Result;
use elinor::Metric;

fn main() -> Result<()> {
    // <QueryID> <Dummy> <DocID> <Relevance>
    let gold_rels_data = "
q_1 0 d_1 1
q_1 0 d_2 0
q_1 0 d_3 2
q_2 0 d_2 2
q_2 0 d_4 1
    "
    .trim();

    // <QueryID> <Dummy> <DocID> <Rank> <Score> <RunName>
    let pred_rels_data = "
q_1 0 d_1 1 0.5 SAMPLE
q_1 0 d_2 2 0.4 SAMPLE
q_1 0 d_3 3 0.3 SAMPLE
q_2 0 d_3 1 0.3 SAMPLE
q_2 0 d_1 2 0.2 SAMPLE
q_2 0 d_4 3 0.1 SAMPLE
    "
    .trim();

    let gold_rels = elinor::trec::parse_gold_rels_in_trec(gold_rels_data.lines())?;
    let pred_rels = elinor::trec::parse_pred_rels_in_trec(pred_rels_data.lines())?;

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
        println!("{:?}: {:.4}", metric, evaluated.mean());
    }

    Ok(())
}
