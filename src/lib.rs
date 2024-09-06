pub mod metrics;
pub mod qrels;
pub mod run;
use hashbrown::HashMap;

pub struct Predicted {
    pub id: String,
    pub score: f64,
}

pub struct Relevance {
    pub id: String,
    pub score: usize,
}

pub type RelevanceMap = HashMap<String, usize>;

// pub type Qrels = HashMap<String, Vec<Relevance>>;
// pub type Run = HashMap<String, Vec<Relevance>>;
