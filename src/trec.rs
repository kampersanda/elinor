//! TREC format parser.
use crate::errors::ElinorError;
use crate::PredRelStore;
use crate::PredRelStoreBuilder;
use crate::PredScore;
use crate::TrueRelStore;
use crate::TrueRelStoreBuilder;
use crate::TrueScore;

/// Parses the Qrels data in the TREC format into a [`TrueRelStore`].
///
/// # Format
///
/// Each line should be `<QueryID> <Dummy> <DocID> <Score>`,
/// where `<Dummy>` is ignored.
///
/// # Caution
///
/// The score should be non-negative.
/// If the score is negative, it will be clamped to 0.
///
/// # Example
///
/// ```rust
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use elinor::trec::parse_true_rels_in_trec;
///
/// let data = "
/// q_1 0 d_1 1
/// q_1 0 d_2 0
/// q_1 0 d_3 2
/// q_2 0 d_2 2
/// q_2 0 d_4 1
/// ".trim();
///
/// let true_rels = parse_true_rels_in_trec(data.lines())?;
/// assert_eq!(true_rels.n_queries(), 2);
/// assert_eq!(true_rels.n_docs(), 5);
/// assert_eq!(true_rels.get_score("q_1", "d_3"), Some(&2));
/// # Ok(())
/// # }
/// ```
pub fn parse_true_rels_in_trec<I, S>(lines: I) -> Result<TrueRelStore<String>, ElinorError>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut b = TrueRelStoreBuilder::new();
    for line in lines {
        let line = line.as_ref();
        let rows = line.split_whitespace().collect::<Vec<_>>();
        if rows.len() < 4 {
            return Err(ElinorError::InvalidFormat(format!(
                "Qrels line must have four columns at least, but got {line}"
            )));
        }
        let query_id = rows[0].to_string();
        let doc_id = rows[2].to_string();
        let score = rows[3].parse::<i32>().map_err(|_| {
            ElinorError::InvalidFormat(format!(
                "The fourth column must be i32, but got {}",
                rows[3]
            ))
        })?;
        let score = TrueScore::try_from(score.max(0)).unwrap();
        b.add_record(query_id, doc_id, score)?;
    }
    Ok(b.build())
}

/// Parses the Run data in the TREC format into a [`PredRelStore`].
///
/// # Format
///
/// Each line should be `<QueryID> <Dummy> <DocID> <Rank> <Score> <RunName>`,
/// where `<Dummy>`, `<Rank>`, and `<RunName>` are ignored.
///
/// # Caution
///
/// Ties are arbitrarily broken because the rank is ignored.
///
/// # Example
///
/// ```rust
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use elinor::trec::parse_pred_rels_in_trec;
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
/// let pred_rels = parse_pred_rels_in_trec(data.lines())?;
/// assert_eq!(pred_rels.n_queries(), 2);
/// assert_eq!(pred_rels.n_docs(), 6);
/// assert_eq!(pred_rels.get_score("q_1", "d_3"), Some(&0.3.into()));
/// # Ok(())
/// # }
/// ```
pub fn parse_pred_rels_in_trec<I, S>(lines: I) -> Result<PredRelStore<String>, ElinorError>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut b = PredRelStoreBuilder::new();
    for line in lines {
        let line = line.as_ref();
        let rows = line.split_whitespace().collect::<Vec<_>>();
        if rows.len() < 5 {
            return Err(ElinorError::InvalidFormat(format!(
                "Run line must have five columns at least, but got {line}"
            )));
        }
        let query_id = rows[0].to_string();
        let doc_id = rows[2].to_string();
        let score = rows[4].parse::<PredScore>().map_err(|_| {
            ElinorError::InvalidFormat(format!("The fifth column must be f32, but got {}", rows[4]))
        })?;
        b.add_record(query_id, doc_id, score)?;
    }
    Ok(b.build())
}
