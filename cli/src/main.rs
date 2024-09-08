use std::collections::HashSet;
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
    let metrics = HashSet::from_iter([
        Metric::Hits(0, 1),
        Metric::Precision(0, 1),
        Metric::Recall(0, 1),
        Metric::F1(0, 1),
        Metric::AveragePrecision(0, 1),
        Metric::ReciprocalRank(0, 1),
        Metric::Ndcg(0, DcgWeighting::Jarvelin),
        Metric::Ndcg(0, DcgWeighting::Burges),
    ]);
    let evaluated = emir::evaluate(&qrels, &run, metrics)?;
    println!("{:#?}", evaluated.mean_scores);
    Ok(())
}

fn load_lines<P: AsRef<Path>>(file: P) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file = File::open(file)?;
    let reader = BufReader::new(file);
    let lines = reader.lines().collect::<Result<Vec<_>, _>>()?;
    Ok(lines)
}
