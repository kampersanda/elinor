mod tables;

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};

use anyhow::Result;
use big_s::S;
use clap::{Parser, Subcommand};
use elinor::{Evaluated, GoldRelStore, Metric, PredRelStore};

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
        result_jsons: Vec<PathBuf>,
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
        SubCommand::Compare { result_jsons } => main_compare(result_jsons)?,
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
    score_table.to_csv(&mut csv::Writer::from_path(&result_csv)?)?;

    let mut metric_table = MetricTable::new();
    for (metric, result) in &results {
        metric_table.insert(metric.clone(), "Score", result.clone());
    }
    metric_table.printstd();

    Ok(())
}

fn main_compare(result_jsons: Vec<PathBuf>) -> Result<()> {
    let mut metric_table = MetricTable::new();
    for result_json in &result_jsons {
        let result: HashMap<String, HashMap<String, f64>> = load_json(&result_json)?;
        for (metric, scores) in result {
            let metric = metric.parse::<Metric>()?;
            let evaluated = Evaluated::from_scores(scores);
            metric_table.insert(metric, get_file_name(result_json), evaluated);
        }
    }
    metric_table.printstd();

    if result_jsons.len() == 2 {
        let mut pc_table = PairedComparisonTable::new();
        let system_a = get_file_name(&result_jsons[0]);
        let system_b = get_file_name(&result_jsons[1]);
        for metric in metric_table.metrics() {
            let result_a = metric_table.get(&metric, &system_a).unwrap().clone();
            let result_b = metric_table.get(&metric, &system_b).unwrap().clone();
            pc_table.insert(metric, result_a, result_b);
        }
        pc_table.printstd();
    } else if result_jsons.len() > 2 {
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
    let file = File::open(file)?;
    let reader = BufReader::new(file);
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

fn results_to_json(results: &[(Metric, elinor::Evaluated<String>)]) -> serde_json::Value {
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

fn get_file_name(path: &Path) -> String {
    path.file_name().unwrap().to_str().unwrap().to_string()
}
