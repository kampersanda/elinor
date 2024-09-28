use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

use anyhow::Result;
use big_s::S;
use clap::{Parser, Subcommand};
use elinor::{GoldRelStore, Metric, PredRelStore};
use prettytable::{Cell, Table};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    command: SubCommand,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    Measure {
        #[arg(short, long)]
        gold_json: PathBuf,

        #[arg(short, long)]
        pred_json: PathBuf,

        #[arg(short, long)]
        output_json: PathBuf,

        #[arg(
            short,
            long,
            default_values_t = &["precision@10".to_string(), "ap".to_string(), "rr".to_string(), "ndcg@10".to_string()],
        )]
        metrics: Vec<String>,
    },
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        SubCommand::Measure {
            gold_json,
            pred_json,
            output_json,
            metrics,
        } => main_measure(gold_json, pred_json, output_json, metrics)?,
    }

    Ok(())
}

fn main_measure(
    gold_json: PathBuf,
    pred_json: PathBuf,
    output_json: PathBuf,
    metrics: Vec<String>,
) -> Result<()> {
    if metrics.is_empty() {
        return Err(anyhow::anyhow!("No metrics specified"));
    }
    let metrics = parse_metrics(metrics)?;

    let reader = BufReader::new(File::open(&gold_json)?);
    let gold_map = serde_json::from_reader(reader)?;
    let gold_rels = GoldRelStore::<String>::from_map(gold_map);

    let reader = BufReader::new(File::open(pred_json)?);
    let pred_map = serde_json::from_reader(reader)?;
    let pred_rels = PredRelStore::<String>::from_map(pred_map);

    let mut results = Vec::new();
    for &metric in &metrics {
        let result = elinor::evaluate(&gold_rels, &pred_rels, metric)?;
        results.push((metric, result));
    }

    let mut rows = Vec::new();
    rows.push(vec![S("Metric"), S("Score")]);
    for (metric, result) in &results {
        let mean_score = result.mean_score();
        rows.push(vec![format!("{metric}"), format!("{mean_score:.4}")]);
    }
    create_table(rows).printstd();

    let json = results_to_json(results);
    let writer = BufWriter::new(File::create(&output_json)?);
    serde_json::to_writer_pretty(writer, &json)?;

    Ok(())
}

fn parse_metrics(metrics: Vec<String>) -> Result<Vec<Metric>> {
    let mut parsed = Vec::new();
    let mut checked = HashSet::new();
    for metric in metrics {
        let metric = metric.parse::<Metric>()?;
        if checked.contains(&metric) {
            continue;
        }
        checked.insert(metric.clone());
        parsed.push(metric);
    }
    Ok(parsed)
}

fn results_to_json(results: Vec<(Metric, elinor::Evaluated<String>)>) -> serde_json::Value {
    let mut metric_to_scores = serde_json::Map::new();
    for (metric, result) in results {
        let mut qid_to_score = serde_json::Map::new();
        for (k, v) in result.scores() {
            qid_to_score.insert(k.clone(), serde_json::json!(*v));
        }
        metric_to_scores.insert(format!("{metric}"), serde_json::json!(qid_to_score));
    }
    serde_json::Value::Object(metric_to_scores)
}

fn create_table(rows: Vec<Vec<String>>) -> Table {
    let mut table = Table::new();
    table.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.set_titles(rows[0].iter().map(|s| Cell::new(s)).collect());
    for row in rows.iter().skip(1) {
        table.add_row(row.iter().map(|s| Cell::new(s)).collect());
    }
    table
}
