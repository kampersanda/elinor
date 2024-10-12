//! TREC format parser.
use crate::errors::ElinorError;
use crate::GoldRelStore;
use crate::GoldRelStoreBuilder;
use crate::GoldScore;
use crate::PredRelStore;
use crate::PredRelStoreBuilder;
use crate::PredScore;

/// Parses the Qrels data in the TREC format into a [`GoldRelStore`].
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
/// use elinor::trec::parse_gold_rels_in_trec;
///
/// let data = "
/// q_1 0 d_1 1
/// q_1 0 d_2 0
/// q_1 0 d_3 2
/// q_2 0 d_2 2
/// q_2 0 d_4 1
/// ".trim();
///
/// let gold_rels = parse_gold_rels_in_trec(data.lines())?;
/// assert_eq!(gold_rels.n_queries(), 2);
/// assert_eq!(gold_rels.n_docs(), 5);
/// assert_eq!(gold_rels.get_score("q_1", "d_3"), Some(&2));
/// # Ok(())
/// # }
/// ```
pub fn parse_gold_rels_in_trec<I, S>(lines: I) -> Result<GoldRelStore<String>, ElinorError>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut b = GoldRelStoreBuilder::new();
    for line in lines {
        let line = line.as_ref();
        let rows = line.split_whitespace().collect::<Vec<_>>();
        if rows.len() < 4 {
            return Err(ElinorError::InvalidFormat(line.to_string()));
        }
        let query_id = rows[0].to_string();
        let doc_id = rows[2].to_string();
        let score = rows[3]
            .parse::<i32>()
            .map_err(|_| ElinorError::InvalidFormat(format!("Invalid score: {}", rows[3])))?;
        let score = GoldScore::try_from(score.max(0)).unwrap();
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
            return Err(ElinorError::InvalidFormat(line.to_string()));
        }
        let query_id = rows[0].to_string();
        let doc_id = rows[2].to_string();
        let score = rows[4]
            .parse::<PredScore>()
            .map_err(|_| ElinorError::InvalidFormat(format!("Invalid score: {}", rows[4])))?;
        b.add_record(query_id, doc_id, score)?;
    }
    Ok(b.build())
}
