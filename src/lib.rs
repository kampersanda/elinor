pub mod errors;
pub mod metrics;
pub mod qrels;
pub mod run;

use std::collections::HashMap;

pub use errors::EmirError;
pub use qrels::Qrels;
pub use run::Run;

pub struct Relevance<T> {
    pub doc_id: String,
    pub score: T,
}

pub type RelevanceMap<T> = HashMap<String, T>;
