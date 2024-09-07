pub mod errors;
pub mod metrics;
pub mod qrels;
pub mod run;

use std::collections::HashMap;

use errors::EmirError;
use qrels::Qrels;
use qrels::QrelsBuilder;
use run::Run;
use run::RunBuilder;

pub struct Relevance<T> {
    pub doc_id: String,
    pub score: T,
}

pub type RelevanceMap<T> = HashMap<String, T>;
