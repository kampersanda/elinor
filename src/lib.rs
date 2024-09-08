pub mod errors;
pub mod metrics;
pub mod qrels;
pub mod run;

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Relevance<T> {
    pub doc_id: String,
    pub score: T,
}

pub type RelevanceMap<T> = HashMap<String, T>;
