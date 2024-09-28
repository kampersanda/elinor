use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use elinor::{GoldRelStore, Metric, PredRelStore};
use prettytable::{Cell, Row, Table};

#[macro_use]
extern crate prettytable;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    gold_json: PathBuf,

    #[arg(short, long)]
    pred_jsons: Vec<PathBuf>,

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
    let gold_store = GoldRelStore::<String>::from_map(gold_map);

    let mut pred_stores = Vec::new();
    for pred_json in &args.pred_jsons {
        let reader = BufReader::new(File::open(pred_json)?);
        let pred_map = serde_json::from_reader(reader)?;
        let pred_store = PredRelStore::<String>::from_map(pred_map);
        pred_stores.push(pred_store);
    }

    let mut evaluated_results = Vec::new();
    for pred_store in &pred_stores {
        let mut evaluated_result = HashMap::new();
        for &metric in &metrics {
            let evaluated = elinor::evaluate(&gold_store, pred_store, metric)?;
            evaluated_result.insert(metric.clone(), evaluated);
        }
        evaluated_results.push(evaluated_result);
    }

    let n_systems = pred_stores.len();
    let system_aliases: Vec<_> = (1..=n_systems).map(|i| format!("System {i}")).collect();

    // Show alias.
    let mut table = Table::new();
    table.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.set_titles(row!["Alias", "File"]);
    for (alias, pred_json) in system_aliases.iter().zip(args.pred_jsons.iter()) {
        table.add_row(row![alias, format!("{}", pred_json.display())]);
    }
    table.printstd();

    let mut table = Table::new();
    table.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.set_titles(Row::new(
        vec![Cell::new("Metric")]
            .into_iter()
            .chain(system_aliases.iter().map(|alias| Cell::new(alias)))
            .collect(),
    ));
    for metric in metrics {
        let mut mean_scores = Vec::new();
        for evaluated_result in &evaluated_results {
            let evaluated = evaluated_result.get(&metric).unwrap();
            mean_scores.push(evaluated.mean_score());
        }
        table.add_row(Row::new(
            vec![Cell::new(&format!("{metric}"))]
                .into_iter()
                .chain(
                    mean_scores
                        .iter()
                        .map(|score| Cell::new(&format!("{score:.4}"))),
                )
                .collect(),
        ));
    }
    table.printstd();

    Ok(())
}
