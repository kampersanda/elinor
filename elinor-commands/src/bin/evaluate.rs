use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use elinor::{GoldRecord, GoldRelStore, Metric, PredRecord, PredRelStore};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long, help = "Path to the input JSONL file")]
    gold_jsonl: PathBuf,

    #[arg(short, long, help = "Path to the input JSONL file")]
    pred_jsonl: PathBuf,

    #[arg(short, long, help = "Metric to evaluate")]
    metrics: Vec<Metric>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let gold_lines = elinor_commands::load_lines(&args.gold_jsonl)?;
    let gold_records = gold_lines
        .into_iter()
        .map(|line| serde_json::from_str::<GoldRecord<String>>(&line).unwrap());
    let gold_rels = GoldRelStore::from_records(gold_records)?;

    let pred_lines = elinor_commands::load_lines(&args.pred_jsonl)?;
    let pred_records = pred_lines
        .into_iter()
        .map(|line| serde_json::from_str::<PredRecord<String>>(&line).unwrap());
    let pred_rels = PredRelStore::from_records(pred_records)?;

    let metrics = if args.metrics.is_empty() {
        default_metrics()
    } else {
        args.metrics
    };

    let results: Vec<_> = metrics
        .into_iter()
        .map(|metric| elinor::evaluate(&gold_rels, &pred_rels, metric))
        .collect::<Result<_, _>>()?;

    for (metric, result) in results.iter().zip(results.iter()) {}

    Ok(())
}

fn default_metrics() -> Vec<Metric> {
    vec![
        Metric::Precision { k: 10 },
        Metric::Recall { k: 10 },
        Metric::F1 { k: 10 },
        Metric::AP { k: 0 },
        Metric::RR { k: 0 },
        Metric::NDCG { k: 10 },
    ]
}
