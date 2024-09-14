use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;

use clap::Parser;
use emir::trec;
use emir::DcgWeighting;
use emir::Metric;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    qrels_file: PathBuf,

    #[arg(short, long)]
    run_file: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let qrels = trec::parse_qrels_from_trec(load_lines(&args.qrels_file)?.into_iter())?;
    let run = trec::parse_run_from_trec(load_lines(&args.run_file)?.into_iter())?;

    let ks = vec![0, 5, 10, 15, 20, 30, 100, 200, 500, 1000];
    let metrics = all_metrics(&ks);

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
    metrics.extend(ks.iter().map(|&k| Metric::Ndcg {
        k,
        w: DcgWeighting::Jarvelin,
    }));
    metrics.extend(ks.iter().map(|&k| Metric::Ndcg {
        k,
        w: DcgWeighting::Burges,
    }));
    metrics
}
