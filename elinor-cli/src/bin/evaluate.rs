use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use elinor::Metric;
use elinor::PredRecord;
use elinor::PredRelStore;
use elinor::TrueRecord;
use elinor::TrueRelStore;
use polars::prelude::*;

#[derive(Parser, Debug)]
#[command(version, about = "Evaluate the performance of a ranking model.")]
struct Args {
    /// Path to the input JSONL file for true relevance.
    #[arg(short, long)]
    true_jsonl: PathBuf,

    /// Path to the input JSONL file for predicted relevance.
    #[arg(short, long)]
    pred_jsonl: PathBuf,

    /// Path to the output CSV file.
    #[arg(short, long)]
    output_csv: Option<PathBuf>,

    /// Use tab separator instead of comma in the output CSV.
    #[arg(long)]
    tab_separator: bool,

    /// Metric to evaluate.
    #[arg(short, long, num_args = 1..)]
    metrics: Vec<Metric>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let true_lines = elinor_cli::load_lines(&args.true_jsonl)?;
    let true_records = true_lines
        .into_iter()
        .map(|line| serde_json::from_str::<TrueRecord<String>>(&line).unwrap());
    let true_rels = TrueRelStore::from_records(true_records)?;

    let pred_lines = elinor_cli::load_lines(&args.pred_jsonl)?;
    let pred_records = pred_lines
        .into_iter()
        .map(|line| serde_json::from_str::<PredRecord<String>>(&line).unwrap());
    let pred_rels = PredRelStore::from_records(pred_records)?;

    let metrics = if args.metrics.is_empty() {
        default_metrics()
    } else {
        args.metrics
    };

    println!("n_queries_in_true\t{}", true_rels.n_queries());
    println!("n_queries_in_pred\t{}", pred_rels.n_queries());
    println!("n_docs_in_true\t{}", true_rels.n_docs());
    println!("n_docs_in_pred\t{}", pred_rels.n_docs());
    println!("n_true_relevant_docs\t{}", n_relevant_docs(&true_rels));

    let mut columns = vec![];
    for metric in metrics {
        let result = elinor::evaluate(&true_rels, &pred_rels, metric)?;
        println!("{:#}\t{:.4}", metric, result.mean());
        let scores = result.scores();
        if columns.is_empty() {
            let query_ids = scores.keys().map(|k| k.as_str()).collect::<Vec<_>>();
            columns.push(Series::new("query_id".into(), query_ids));
        }
        let values = scores.values().copied().collect::<Vec<_>>();
        columns.push(Series::new(format!("{metric:#}").into(), values));
    }

    if let Some(output_csv) = args.output_csv {
        let mut df = DataFrame::new(columns)?;
        let mut file = std::fs::File::create(output_csv)?;
        let separator = if args.tab_separator { b'\t' } else { b',' };
        CsvWriter::new(&mut file)
            .with_separator(separator)
            .finish(&mut df)?;
    }

    Ok(())
}

fn n_relevant_docs(true_rels: &TrueRelStore<String>) -> usize {
    let records = true_rels.records();
    records.into_iter().filter(|r| r.score > 0).count()
}

fn default_metrics() -> Vec<Metric> {
    let mut metrics = Vec::new();
    for k in [1, 5, 10] {
        metrics.push(Metric::Success { k });
    }
    for k in [5, 10, 15, 20] {
        metrics.push(Metric::Recall { k });
    }
    for k in [5, 10, 15, 20] {
        metrics.push(Metric::Precision { k });
    }
    for k in [5, 10, 15, 20] {
        metrics.push(Metric::AP { k });
    }
    for k in [5, 10, 15, 20] {
        metrics.push(Metric::NDCG { k });
    }
    metrics.push(Metric::RR { k: 0 });
    metrics
}
