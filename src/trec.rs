//! TREC format parser for Qrels and Run data.
use crate::errors::EmirError;
use crate::GoldScore;
use crate::PredScore;
use crate::Qrels;
use crate::QrelsBuilder;
use crate::Run;
use crate::RunBuilder;

/// Parses the given TREC data into a Qrels data structure.
///
/// # Format
///
/// Each line should be `<QueryID> <Dummy> <DocID> <Score>`,
/// where `<Dummy>` is ignored.
pub fn parse_qrels_from_trec<I, S>(lines: I) -> Result<Qrels<String>, EmirError<String>>
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    let mut b = QrelsBuilder::new();
    for line in lines {
        let line = line.as_ref();
        let rows = line.split_whitespace().collect::<Vec<_>>();
        if rows.len() != 4 {
            return Err(EmirError::InvalidFormat);
        }
        let query_id = rows[0].to_string();
        let doc_id = rows[2].to_string();
        let score = rows[3].parse::<GoldScore>().unwrap();
        b.add_score(query_id, doc_id, score)?;
    }
    Ok(b.build())
}

/// Parses the given TREC data into a Run data structure.
///
/// # Format
///
/// Each line should be `<QueryID> <Dummy> <DocID> <Rank> <Score> <RunName>`,
/// where `<Dummy>` and `<Rank>` are ignored.
pub fn parse_run_from_trec<I, S>(lines: I) -> Result<Run<String>, EmirError<String>>
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    let mut name = None;
    let mut b = RunBuilder::new();
    for line in lines {
        let line = line.as_ref();
        if line.is_empty() {
            continue;
        }
        let rows = line.split_whitespace().collect::<Vec<_>>();
        let query_id = rows[0].to_string();
        let doc_id = rows[2].to_string();
        let score = rows[4].parse::<PredScore>().unwrap();
        b.add_score(query_id, doc_id, score)?;
        if name.is_none() {
            name = Some(rows[5].to_string());
        }
    }
    if let Some(name) = name {
        Ok(b.build().with_name(name.as_str()))
    } else {
        Err(EmirError::EmptyLines)
    }
}
