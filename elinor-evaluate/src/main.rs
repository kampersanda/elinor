use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use elinor::trec;
use elinor::Metric;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    gold_file: PathBuf,

    #[arg(short, long)]
    pred_file: PathBuf,

    #[arg(short, long, default_values_t = &[0, 1, 5, 10, 15, 20, 30, 100, 200, 500, 1000])]
    ks: Vec<usize>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let gold_rels = trec::parse_gold_rels_in_trec(load_lines(&args.gold_file)?.into_iter())?;
    let pred_rels = trec::parse_pred_rels_in_trec(load_lines(&args.pred_file)?.into_iter())?;

    let metrics = all_metrics(&args.ks);
    for metric in metrics {
        let evaluated = elinor::evaluate(&gold_rels, &pred_rels, metric)?;
        let score = evaluated.mean_score();
        println!("{metric}\t{score:.4}");
    }

    Ok(())
}

fn load_lines<P: AsRef<Path>>(file: P) -> Result<Vec<String>> {
    let file = File::open(file)?;
    let reader = BufReader::new(file);
    let lines = reader.lines().collect::<Result<Vec<_>, _>>()?;
    Ok(lines)
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
