use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::Result;
use clap::Parser;
use elinor::trec;

#[derive(Clone, Debug)]
enum RelevanceType {
    True,
    Pred,
}

impl FromStr for RelevanceType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "true" => Ok(Self::True),
            "pred" => Ok(Self::Pred),
            _ => Err(format!("Invalid relevance type: {}", s)),
        }
    }
}

#[derive(Parser, Debug)]
#[command(version, about = "Convert TREC format to JSONL format.")]
struct Args {
    /// Path to the input TREC file.
    #[arg(short, long)]
    input_trec: PathBuf,

    /// Path to the output JSONL file.
    #[arg(short, long)]
    output_jsonl: PathBuf,

    /// Relevance type from 'true' or 'pred'.
    #[arg(short, long)]
    rel_type: RelevanceType,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let lines = elinor_cli::load_lines(&args.input_trec)?;
    let mut writer = BufWriter::new(File::create(&args.output_jsonl)?);

    match args.rel_type {
        RelevanceType::True => {
            let true_rels = trec::parse_true_rels_in_trec(lines)?;
            let true_records = true_rels.into_records();
            for record in true_records {
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
