use thiserror::Error;

#[derive(Error, Debug)]
pub enum EmirError<K>
where
    K: std::fmt::Display,
{
    #[error("Error: QueryID {0}, DocID {1}")]
    DuplicateDocId(K, K),

    #[error("Error: {0}")]
    MissingQueryId(K),
}
