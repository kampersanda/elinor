mod tables;

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};

use anyhow::Result;
use big_s::S;
use clap::{Parser, Subcommand};
use elinor::{Evaluated, GoldRelStore, Metric, PredRelStore};
use prettytable::{Cell, Table};

use crate::tables::MetricTable;

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
        result_json: PathBuf,

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
            result_json,
            metrics,
        } => main_measure(gold_json, pred_json, result_json, metrics)?,
        SubCommand::Compare { result_jsons } => main_compare(result_jsons)?,
    }

    Ok(())
}

fn main_measure(
    gold_json: PathBuf,
    pred_json: PathBuf,
    result_json: PathBuf,
    metrics: Vec<String>,
) -> Result<()> {
    if metrics.is_empty() {
        return Err(anyhow::anyhow!("No metrics specified"));
    }
    let metrics = parse_metrics(metrics)?;
    let gold_rels = GoldRelStore::<String>::from_map(load_json(&gold_json)?);
    let pred_rels = PredRelStore::<String>::from_map(load_json(&pred_json)?);

    // Metric, Evaluated
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

    let json = results_to_json(&results);
    let writer = BufWriter::new(File::create(&result_json)?);
    serde_json::to_writer_pretty(writer, &json)?;

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

    // let system_names_vec = system_names.iter().cloned().collect::<Vec<String>>();
    // for (metric, system_to_result) in &metric_to_results {
    //     let system_a = system_names_vec[0].clone();
    //     let system_b = system_names_vec[1].clone();
    //     let result_a = system_to_result.get(&system_a).unwrap();
    //     let result_b = system_to_result.get(&system_b).unwrap();
    //     compare_two_systems(metric, result_a, result_b)?;
    // }

    Ok(())
}

fn compare_two_systems(
    metric: &Metric,
    result_a: &Evaluated<String>,
    result_b: &Evaluated<String>,
) -> Result<()> {
    let paired_scores = elinor::paired_scores_from_evaluated(&result_a, &result_b)?;
    let stat = elinor::statistical_tests::StudentTTest::from_paired_samples(paired_scores)?;

    let mut rows: Vec<Vec<String>> = Vec::new();
    rows.push(vec![format!("{metric}"), S("Statistic")]);
    rows.push(vec![S("Mean"), format!("{:.4}", stat.mean())]);
    rows.push(vec![S("Variance"), format!("{:.4}", stat.var())]);
    rows.push(vec![S("Effect Size"), format!("{:.4}", stat.effect_size())]);
    rows.push(vec![S("T Stat"), format!("{:.4}", stat.t_stat())]);
    rows.push(vec![S("P Value"), format!("{:.4}", stat.p_value())]);

    create_table(rows).printstd();

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

fn create_table(rows: Vec<Vec<String>>) -> Table {
    let mut table = Table::new();
    table.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.set_titles(rows[0].iter().map(|s| Cell::new(s)).collect());
    for row in rows.iter().skip(1) {
        table.add_row(row.iter().map(|s| Cell::new(s)).collect());
    }
    table
}

fn get_file_name(path: &Path) -> String {
    path.file_name().unwrap().to_str().unwrap().to_string()
}
