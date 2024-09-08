pub mod errors;
pub mod metrics;
pub mod relevance;

use ordered_float::OrderedFloat;

pub use relevance::Relevance;
pub use relevance::RelevanceMap;

pub type GoldScore = i32;
pub type PredScore = OrderedFloat<f64>;

pub type Qrels<K> = relevance::RelevanceStore<K, GoldScore>;
pub type QrelsBuilder<K> = relevance::RelevanceStoreBuilder<K, GoldScore>;

pub type Run<K> = relevance::RelevanceStore<K, PredScore>;
pub type RunBuilder<K> = relevance::RelevanceStoreBuilder<K, PredScore>;
