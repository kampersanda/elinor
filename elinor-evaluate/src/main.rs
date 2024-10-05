mod tables;

use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use anyhow::Result;
use clap::{Parser, Subcommand};
use elinor::{GoldRelStore, Metric, PredRelStore};

use crate::tables::{MetricTable, PairedComparisonTable, ScoreTable, TupledComparisonTable};

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
        result_csv: PathBuf,

        #[arg(
            short,
            long,
            default_values_t = &["precision@10".to_string(), "ap".to_string(), "rr".to_string(), "ndcg@10".to_string()],
        )]
        metrics: Vec<String>,
    },

    Compare {
        #[arg(short, long)]
        result_csvs: Vec<PathBuf>,
    },
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        SubCommand::Measure {
            gold_json,
            pred_json,
            result_csv,
            metrics,
        } => main_measure(gold_json, pred_json, result_csv, metrics)?,
        SubCommand::Compare { result_csvs } => main_compare(result_csvs)?,
    }

    Ok(())
}

fn main_measure(
    gold_json: PathBuf,
    pred_json: PathBuf,
    result_csv: PathBuf,
    metrics: Vec<String>,
) -> Result<()> {
    if metrics.is_empty() {
        return Err(anyhow::anyhow!("No metrics specified"));
    }
    let metrics = parse_metrics(metrics)?;
    let gold_rels = GoldRelStore::<String>::from_map(load_json(&gold_json)?);
    let pred_rels = PredRelStore::<String>::from_map(load_json(&pred_json)?);

    let mut results = vec![];
    for &metric in &metrics {
        let result = elinor::evaluate(&gold_rels, &pred_rels, metric)?;
        results.push((metric, result));
    }

    let mut score_table = ScoreTable::new();
    for (metric, result) in &results {
        score_table.insert(metric.clone(), result)?;
    }
    score_table.into_csv(&mut csv::Writer::from_path(&result_csv)?)?;

    let mut metric_table = MetricTable::new();
    for (metric, result) in &results {
        metric_table.insert(metric.clone(), "Score", result.clone());
    }
    metric_table.printstd();

    Ok(())
}

fn main_compare(result_csvs: Vec<PathBuf>) -> Result<()> {
    let mut metric_table = MetricTable::new();
    for result_csv in &result_csvs {
        let score_table = ScoreTable::from_csv(&mut csv::Reader::from_path(&result_csv)?)?;
        let result = score_table.to_results();
        for (metric, evaluated) in result {
            metric_table.insert(metric, get_file_name(result_csv), evaluated);
        }
    }
    metric_table.printstd();

    if result_csvs.len() == 2 {
        let mut pc_table = PairedComparisonTable::new();
        let system_a = get_file_name(&result_csvs[0]);
        let system_b = get_file_name(&result_csvs[1]);
        for metric in metric_table.metrics() {
            let result_a = metric_table.get(&metric, &system_a).unwrap().clone();
            let result_b = metric_table.get(&metric, &system_b).unwrap().clone();
            pc_table.insert(metric, result_a, result_b);
        }
        pc_table.summarize();
    } else if result_csvs.len() > 2 {
        let mut tc_table = TupledComparisonTable::new();
        for metric in metric_table.metrics() {
            let results = metric_table.get_all(&metric);
            tc_table.insert(metric, results);
        }
        tc_table.printstd();
    }

    Ok(())
}

fn load_json<P, T>(file: P) -> Result<T>
where
    P: AsRef<Path>,
    T: serde::de::DeserializeOwned,
{
    let reader = BufReader::new(File::open(file)?);
    Ok(serde_json::from_reader(reader)?)
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

fn get_file_name(path: &Path) -> String {
    path.file_name().unwrap().to_str().unwrap().to_string()
}
