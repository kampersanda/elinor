use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;

use clap::Parser;
use emir::trec;
use emir::Metric;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    qrels_file: PathBuf,

    #[arg(short, long)]
    run_file: PathBuf,

    #[arg(short, long, default_values_t = &[0, 1, 5, 10, 15, 20, 30, 100, 200, 500, 1000])]
    ks: Vec<usize>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let qrels = trec::parse_qrels_from_trec(load_lines(&args.qrels_file)?.into_iter())?;
    let run = trec::parse_run_from_trec(load_lines(&args.run_file)?.into_iter())?;

    let metrics = all_metrics(&args.ks);
    let evaluated = emir::evaluate(&qrels, &run, metrics.iter().cloned())?;

    for metric in &metrics {
        let score = evaluated.mean_scores[metric];
        println!("{metric}\t{score:.4}");
    }

    Ok(())
}

fn load_lines<P: AsRef<Path>>(file: P) -> Result<Vec<String>, Box<dyn std::error::Error>> {
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
    metrics.extend(ks.iter().map(|&k| Metric::AveragePrecision { k }));
    metrics.extend(ks.iter().map(|&k| Metric::ReciprocalRank { k }));
    metrics.extend(ks.iter().map(|&k| Metric::Ndcg { k }));
    metrics.extend(ks.iter().map(|&k| Metric::NdcgBurges { k }));
    metrics
}
