use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use elinor::Metric;
use elinor::{GoldRelStore, PredRelStore};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    gold_json: PathBuf,

    #[arg(short, long)]
    pred_json: PathBuf,

    #[arg(short, long, default_values_t = &[0, 1, 5, 10, 15, 20, 30, 100, 200, 500, 1000])]
    ks: Vec<usize>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let reader = BufReader::new(File::open(&args.gold_json)?);
    let gold_map = serde_json::from_reader(reader)?;
    let gold_rels = GoldRelStore::<String>::from_map(gold_map);

    let reader = BufReader::new(File::open(&args.pred_json)?);
    let pred_map = serde_json::from_reader(reader)?;
    let pred_rels = PredRelStore::<String>::from_map(pred_map);

    let metrics = all_metrics(&args.ks);
    for metric in metrics {
        let evaluated = elinor::evaluate(&gold_rels, &pred_rels, metric)?;
        let score = evaluated.mean_score();
        println!("{metric}\t{score:.4}");
    }

    Ok(())
}

fn all_metrics(ks: &[usize]) -> Vec<Metric> {
    let mut metrics = Vec::new();
    metrics.extend(ks.iter().map(|&k| Metric::Hits { k }));
    metrics.extend(ks.iter().map(|&k| Metric::Success { k }));
    metrics.extend(ks.iter().map(|&k| Metric::Precision { k }));
    metrics.extend(ks.iter().map(|&k| Metric::Recall { k }));
    metrics.extend(ks.iter().map(|&k| Metric::F1 { k }));
    metrics.push(Metric::RPrecision);
    metrics.extend(ks.iter().map(|&k| Metric::AP { k }));
    metrics.extend(ks.iter().map(|&k| Metric::RR { k }));
    metrics.extend(ks.iter().map(|&k| Metric::NDCG { k }));
    metrics.extend(ks.iter().map(|&k| Metric::NDCGBurges { k }));
    metrics.push(Metric::Bpref);
    metrics
}
