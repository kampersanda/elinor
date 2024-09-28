use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::Result;
use clap::Parser;
use elinor::trec;

#[derive(Clone, Debug)]
enum RelevanceType {
    Gold,
    Pred,
}

impl FromStr for RelevanceType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "gold" => Ok(RelevanceType::Gold),
            "pred" => Ok(RelevanceType::Pred),
            _ => Err(format!("Invalid relevance type: {}", s)),
        }
    }
}

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    input_trec: PathBuf,

    #[arg(short, long)]
    output_json: PathBuf,

    #[arg(short, long)]
    rel_type: RelevanceType,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let lines = load_lines(&args.input_trec)?;
    let writer = BufWriter::new(File::create(&args.output_json)?);

    match args.rel_type {
        RelevanceType::Gold => {
            let gold_rels = trec::parse_gold_rels_in_trec(lines)?;
            let map = gold_rels.into_map();
            serde_json::to_writer(writer, &map)?;
        }
        RelevanceType::Pred => {
            let pred_rels = trec::parse_pred_rels_in_trec(lines)?;
            let map = pred_rels.into_map();
            serde_json::to_writer(writer, &map)?;
        }
    }
    Ok(())
}

fn load_lines<P: AsRef<Path>>(file: P) -> Result<Vec<String>> {
    let file = File::open(file)?;
    let reader = BufReader::new(file);
    let lines = reader.lines().collect::<Result<Vec<_>, _>>()?;
    Ok(lines)
}
