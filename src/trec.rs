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
///
/// # Example
///
/// ```rust
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use ireval::trec::parse_qrels_from_trec;
///
/// let data = "
/// q_1 0 d_1 1
/// q_1 0 d_2 0
/// q_1 0 d_3 2
/// q_2 0 d_2 2
/// q_2 0 d_4 1
/// ".trim();
///
/// let qrels = parse_qrels_from_trec(data.lines())?;
/// assert_eq!(qrels.n_queries(), 2);
/// assert_eq!(qrels.n_docs(), 5);
/// assert_eq!(qrels.get_score("q_1", "d_3"), Some(&2));
/// assert_eq!(qrels.name(), None);
/// # Ok(())
/// # }
/// ```
pub fn parse_qrels_from_trec<I, S>(lines: I) -> Result<Qrels<String>, EmirError>
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    let mut b = QrelsBuilder::new();
    for line in lines {
        let line = line.as_ref();
        let rows = line.split_whitespace().collect::<Vec<_>>();
        if rows.len() != 4 {
            return Err(EmirError::InvalidFormat(line.to_string()));
        }
        let query_id = rows[0].to_string();
        let doc_id = rows[2].to_string();
        let score = rows[3]
            .parse::<GoldScore>()
            .map_err(|_| EmirError::InvalidFormat(format!("Invalid score: {}", rows[3])))?;
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
///
/// # Example
///
/// ```rust
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use ireval::trec::parse_run_from_trec;
///
/// let data = "
/// q_1 0 d_1 1 0.5 SAMPLE
/// q_1 0 d_2 2 0.4 SAMPLE
/// q_1 0 d_3 3 0.3 SAMPLE
/// q_2 0 d_3 1 0.3 SAMPLE
/// q_2 0 d_1 2 0.2 SAMPLE
/// q_2 0 d_4 3 0.1 SAMPLE
/// ".trim();
///
/// let run = parse_run_from_trec(data.lines())?;
/// assert_eq!(run.n_queries(), 2);
/// assert_eq!(run.n_docs(), 6);
/// assert_eq!(run.get_score("q_1", "d_3"), Some(&0.3.into()));
/// assert_eq!(run.name(), Some("SAMPLE"));
/// # Ok(())
/// # }
/// ```
pub fn parse_run_from_trec<I, S>(lines: I) -> Result<Run<String>, EmirError>
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    let mut name = None;
    let mut b = RunBuilder::new();
    for line in lines {
        let line = line.as_ref();
        let rows = line.split_whitespace().collect::<Vec<_>>();
        if rows.len() != 6 {
            return Err(EmirError::InvalidFormat(line.to_string()));
        }
        let query_id = rows[0].to_string();
        let doc_id = rows[2].to_string();
        let score = rows[4]
            .parse::<PredScore>()
            .map_err(|_| EmirError::InvalidFormat(format!("Invalid score: {}", rows[4])))?;
        b.add_score(query_id, doc_id, score)?;
        if name.is_none() {
            name = Some(rows[5].to_string());
        }
    }
    name.map_or_else(
        || Err(EmirError::MissingEntry("No line is found".to_string())),
        |name| Ok(b.build().with_name(name.as_str())),
    )
}
