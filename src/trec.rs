use crate::errors::EmirError;
use crate::GoldScore;
use crate::PredScore;
use crate::Qrels;
use crate::QrelsBuilder;
use crate::Run;
use crate::RunBuilder;

pub fn parse_qrels_from_trec<I, S>(lines: I) -> Result<Qrels<String>, EmirError<String>>
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    let mut b = QrelsBuilder::new();
    for line in lines {
        let line = line.as_ref();
        let rows = line.split_whitespace().collect::<Vec<_>>();
        let query_id = rows[0].to_string();
        let doc_id = rows[2].to_string();
        let score = rows[3].parse::<GoldScore>().unwrap();
        b.add_score(query_id, doc_id, score)?;
    }
    Ok(b.build())
}

pub fn parse_run_from_trec<I, S>(lines: I) -> Result<Run<String>, EmirError<String>>
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    let mut b = RunBuilder::new();
    for (i, line) in lines.enumerate() {
        let line = line.as_ref();
        let rows = line.split_whitespace().collect::<Vec<_>>();
        let query_id = rows[0].to_string();
        let doc_id = rows[2].to_string();
        let score = rows[4].parse::<PredScore>().unwrap();
        b.add_score(query_id, doc_id, score)?;
        if i == 0 {
            b = b.name(rows[5].to_string());
        }
    }
    Ok(b.build())
}
