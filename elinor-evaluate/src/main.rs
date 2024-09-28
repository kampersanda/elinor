use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufReader;
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
            metrics,
        } => main_measure(gold_json, pred_json, metrics)?,
    }

    // let metrics = args
    //     .metrics
    //     .iter()
    //     .map(|s| s.parse::<Metric>())
    //     .collect::<Result<Vec<_>, _>>()?;

    // if metrics.is_empty() {
    //     return Err(anyhow::anyhow!("No metrics specified"));
    // }

    // let reader = BufReader::new(File::open(&args.gold_json)?);
    // let gold_map = serde_json::from_reader(reader)?;
    // let gold_store = GoldRelStore::<String>::from_map(gold_map);

    // let mut pred_stores = Vec::new();
    // for pred_json in &args.pred_jsons {
    //     let reader = BufReader::new(File::open(pred_json)?);
    //     let pred_map = serde_json::from_reader(reader)?;
    //     let pred_store = PredRelStore::<String>::from_map(pred_map);
    //     pred_stores.push(pred_store);
    // }

    // let mut evaluated_results = Vec::new();
    // for pred_store in &pred_stores {
    //     let mut evaluated_result = HashMap::new();
    //     for &metric in &metrics {
    //         let evaluated = elinor::evaluate(&gold_store, pred_store, metric)?;
    //         evaluated_result.insert(metric.clone(), evaluated);
    //     }
    //     evaluated_results.push(evaluated_result);
    // }

    // let n_systems = pred_stores.len();
    // let system_aliases: Vec<_> = (1..=n_systems).map(|i| format!("System {i}")).collect();

    // // Show alias.
    // let mut table = Table::new();
    // table.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    // table.set_titles(row!["Alias", "File"]);
    // for (alias, pred_json) in system_aliases.iter().zip(args.pred_jsons.iter()) {
    //     table.add_row(row![alias, format!("{}", pred_json.display())]);
    // }
    // table.printstd();

    // let mut table = Table::new();
    // table.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    // table.set_titles(Row::new(
    //     vec![Cell::new("Metric")]
    //         .into_iter()
    //         .chain(system_aliases.iter().map(|alias| Cell::new(alias)))
    //         .collect(),
    // ));
    // for metric in metrics {
    //     let mut mean_scores = Vec::new();
    //     for evaluated_result in &evaluated_results {
    //         let evaluated = evaluated_result.get(&metric).unwrap();
    //         mean_scores.push(evaluated.mean_score());
    //     }
    //     table.add_row(Row::new(
    //         vec![Cell::new(&format!("{metric}"))]
    //             .into_iter()
    //             .chain(
    //                 mean_scores
    //                     .iter()
    //                     .map(|score| Cell::new(&format!("{score:.4}"))),
    //             )
    //             .collect(),
    //     ));
    // }
    // table.printstd();

    Ok(())
}

fn main_measure(gold_json: PathBuf, pred_json: PathBuf, metrics: Vec<String>) -> Result<()> {
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
    for (metric, result) in results {
        let mean_score = result.mean_score();
        rows.push(vec![format!("{metric}"), format!("{mean_score:.4}")]);
    }
    create_table(rows).printstd();

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

fn create_table(rows: Vec<Vec<String>>) -> Table {
    let mut table = Table::new();
    table.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.set_titles(rows[0].iter().map(|s| Cell::new(s)).collect());
    for row in rows.iter().skip(1) {
        table.add_row(row.iter().map(|s| Cell::new(s)).collect());
    }
    table
}
