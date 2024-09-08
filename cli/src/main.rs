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

    #[arg(short, long, default_value_t = 0)]
    k: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let qrels = trec::parse_qrels_from_trec(load_lines(&args.qrels_file)?.into_iter())?;
    let run = trec::parse_run_from_trec(load_lines(&args.run_file)?.into_iter())?;
    let metrics = [
        Metric::Hits(args.k),
        Metric::HitRate(args.k),
        Metric::Precision(args.k),
        Metric::Recall(args.k),
        Metric::F1(args.k),
        Metric::AveragePrecision(args.k),
        Metric::ReciprocalRank(args.k),
        Metric::Ndcg(args.k, DcgWeighting::Jarvelin),
        Metric::Ndcg(args.k, DcgWeighting::Burges),
    ];

    let evaluated = emir::evaluate(&qrels, &run, metrics)?;
    for (metric, score) in evaluated.mean_scores.iter() {
        println!("{metric}: {score:.4}");
    }

    Ok(())
}

fn load_lines<P: AsRef<Path>>(file: P) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file = File::open(file)?;
    let reader = BufReader::new(file);
    let lines = reader.lines().collect::<Result<Vec<_>, _>>()?;
    Ok(lines)
}
