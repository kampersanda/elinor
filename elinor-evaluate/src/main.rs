use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use elinor::{GoldRelStore, Metric, PredRelStore};
use prettytable::Table;

#[macro_use]
extern crate prettytable;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    gold_json: PathBuf,

    #[arg(short, long)]
    pred_json: PathBuf,

    #[arg(
        short,
        long,
        default_values_t = &["precision@10".to_string(), "ap".to_string(), "rr".to_string(), "ndcg@10".to_string()],
    )]
    metrics: Vec<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let metrics = args
        .metrics
        .iter()
        .map(|s| s.parse::<Metric>())
        .collect::<Result<Vec<_>, _>>()?;

    if metrics.is_empty() {
        return Err(anyhow::anyhow!("No metrics specified"));
    }

    let reader = BufReader::new(File::open(&args.gold_json)?);
    let gold_map = serde_json::from_reader(reader)?;
    let gold_rels = GoldRelStore::<String>::from_map(gold_map);

    let reader = BufReader::new(File::open(&args.pred_json)?);
    let pred_map = serde_json::from_reader(reader)?;
    let pred_rels = PredRelStore::<String>::from_map(pred_map);

    let mut table = Table::new();
    table.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.set_titles(row!["Metric", "Score"]);

    for metric in metrics {
        let evaluated = elinor::evaluate(&gold_rels, &pred_rels, metric)?;
        let score = evaluated.mean_score();
        table.add_row(row![format!("{metric}"), format!("{score:.4}")]);
    }
    table.printstd();

    Ok(())
}
