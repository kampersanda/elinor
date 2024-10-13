use std::fs::File;
use std::io::{BufWriter, Write};
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
    #[arg(short, long, help = "Path to the input TREC file")]
    input_trec: PathBuf,

    #[arg(short, long, help = "Path to the output JSONL file")]
    output_jsonl: PathBuf,

    #[arg(short, long, help = "Relevance type from 'gold' or 'pred'")]
    rel_type: RelevanceType,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let lines = elinor_cli::load_lines(&args.input_trec)?;
    let mut writer = BufWriter::new(File::create(&args.output_jsonl)?);

    match args.rel_type {
        RelevanceType::Gold => {
            let gold_rels = trec::parse_gold_rels_in_trec(lines)?;
            let gold_records = gold_rels.into_records();
            for record in gold_records {
                serde_json::to_writer(&mut writer, &record)?;
                writer.write_all(b"\n")?;
            }
        }
        RelevanceType::Pred => {
            let pred_rels = trec::parse_pred_rels_in_trec(lines)?;
            let pred_records = pred_rels.into_records();
            for record in pred_records {
                serde_json::to_writer(&mut writer, &record)?;
                writer.write_all(b"\n")?;
            }
        }
    }
    Ok(())
}
