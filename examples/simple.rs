use anyhow::Result;
use elinor::GoldRelStoreBuilder;
use elinor::Metric;
use elinor::PredRelStoreBuilder;

fn main() -> Result<()> {
    // Prepare gold relevance scores.
    let mut b = GoldRelStoreBuilder::new();
    b.add_score("q_1", "d_1", 1)?;
    b.add_score("q_1", "d_2", 0)?;
    b.add_score("q_1", "d_3", 2)?;
    b.add_score("q_2", "d_2", 2)?;
    b.add_score("q_2", "d_4", 1)?;
    let gold_rels = b.build();

    // Prepare predicted relevance scores.
    let mut b = PredRelStoreBuilder::new();
    b.add_score("q_1", "d_1", 0.5.into())?;
    b.add_score("q_1", "d_2", 0.4.into())?;
    b.add_score("q_1", "d_3", 0.3.into())?;
    b.add_score("q_2", "d_4", 0.1.into())?;
    b.add_score("q_2", "d_1", 0.2.into())?;
    b.add_score("q_2", "d_3", 0.3.into())?;
    let pred_rels = b.build();

    // The metrics to evaluate can be specified via Metric instances.
    let metrics = vec![
        Metric::Precision { k: 3 },
        Metric::AP { k: 0 }, // k=0 means all documents.
        // The instances can also be specified via strings.
        "rr".parse()?,
        "ndcg@3".parse()?,
    ];

    // Evaluate.
    let evaluated = elinor::evaluate(&gold_rels, &pred_rels, metrics.iter().cloned())?;

    // Macro-averaged scores.
    for metric in &metrics {
        let score = evaluated.mean_scores[metric];
        println!("{metric}: {score:.4}");
    }

    Ok(())
}
