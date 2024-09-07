pub mod errors;
pub mod metrics;
pub mod qrels;
pub mod run;

use hashbrown::HashMap;

use errors::EmirError;

pub struct Relevance<T> {
    pub id: String,
    pub score: T,
}

pub type RelevanceMap<T> = HashMap<String, T>;
