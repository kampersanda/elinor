//! Elinor (**E**valuation **l**ibrary in **in**f**o**rmation **r**etrieval) is a library
//! for evaluating information retrieval (IR) systems.
//! It provides a comprehensive set of metrics and statistical tests for evaluating and comparing IR systems.
//!
//! # Key features
//!
//! * **IR-specific design:**
//!     Elinor is tailored specifically for evaluating IR systems, with an intuitive interface designed for IR engineers.
//!     It offers a streamlined workflow that simplifies common IR evaluation tasks.
//! * **Comprehensive evaluation metrics:**
//!     Elinor supports a wide range of key evaluation metrics, such as Precision, MAP, MRR, and nDCG.
//!     The supported metrics are available in [`Metric`].
//!     The evaluation results are validated against trec_eval to ensure accuracy and reliability.
//! * **In-depth statistical testing:**
//!     Elinor includes several statistical tests, such as Student's t-test, Bootstrap test, and Randomized Tukey HSD test.
//!     Not only p-values but also other important statistics, such as effect sizes and confidence intervals, are provided for thorough reporting.
//!     See the [`statistical_tests`] module for more details.
//!
//! # Ubiquitous language
//!
//! Elinor uses the following terms for convenience:
//!
//! * *True relevance score* means the relevance judgment provided by human assessors.
//! * *Predicted relevance score* means the similarity score predicted by the system.
//!
//! # Basic usage in evaluating several metrics
//!
//! You first need to prepare true and predicted relevance scores for evaluation.
//! These scores are stored in instances of [`TrueRelStore`] and [`PredRelStore`], respectively.
//! You can build these instances using [`TrueRelStoreBuilder`] and [`PredRelStoreBuilder`].
//!
//! Then, you can evaluate the predicted relevance scores using the [`evaluate`] function and
//! the specified metric. The available metrics are defined in the [`Metric`] enum.
//!
//! An example to evaluate Precision@3, MAP, MRR, and nDCG@3 is shown below:
//!
//! ```
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use approx::assert_abs_diff_eq;
//! use elinor::{TrueRelStoreBuilder, PredRelStoreBuilder, Metric};
//!
//! // Prepare true relevance scores.
//! // In binary-relevance metrics, 0 means non-relevant and the others mean relevant.
//! let mut b = TrueRelStoreBuilder::new();
//! b.add_record("q_1", "d_1", 1)?;
//! b.add_record("q_1", "d_2", 0)?;
//! b.add_record("q_1", "d_3", 2)?;
//! b.add_record("q_2", "d_2", 2)?;
//! b.add_record("q_2", "d_4", 1)?;
//! let true_rels = b.build();
//!
//! // Prepare predicted relevance scores.
//! let mut b = PredRelStoreBuilder::new();
//! b.add_record("q_1", "d_1", 0.5.into())?;
//! b.add_record("q_1", "d_2", 0.4.into())?;
//! b.add_record("q_1", "d_3", 0.3.into())?;
//! b.add_record("q_2", "d_4", 0.1.into())?;
//! b.add_record("q_2", "d_1", 0.2.into())?;
//! b.add_record("q_2", "d_3", 0.3.into())?;
//! let pred_rels = b.build();
//!
//! // Evaluate Precision@3.
//! let result = elinor::evaluate(&true_rels, &pred_rels, Metric::Precision { k: 3 })?;
//! assert_abs_diff_eq!(result.mean(), 0.5000, epsilon = 1e-4);
//!
//! // Evaluate MAP, where all documents are considered via k=0.
//! let result = elinor::evaluate(&true_rels, &pred_rels, Metric::AP { k: 0 })?;
//! assert_abs_diff_eq!(result.mean(), 0.5000, epsilon = 1e-4);
//!
//! // Evaluate MRR, where the metric is specified via a string representation.
//! let result = elinor::evaluate(&true_rels, &pred_rels, "rr".parse()?)?;
//! assert_abs_diff_eq!(result.mean(), 0.6667, epsilon = 1e-4);
//!
//! // Evaluate nDCG@3, where the metric is specified via a string representation.
//! let result = elinor::evaluate(&true_rels, &pred_rels, "ndcg@3".parse()?)?;
//! assert_abs_diff_eq!(result.mean(), 0.4751, epsilon = 1e-4);
//! # Ok(())
//! # }
//! ```
//!
//! # Statistical tests for comparing two systems
//!
//! The [`statistical_tests`] module provides various statistical tests for comparing systems.
//!
//! The example shows how to perform [Student's t-test](statistical_tests::StudentTTest) for Precision scores between two systems.
//! Not only the p-value but also various statistics, such as variance and effect size, are provided for thorough reporting.
//!
//! ```
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use approx::assert_relative_eq;
//! use elinor::{TrueRelStoreBuilder, PredRelStoreBuilder, Metric};
//! use elinor::statistical_tests::{StudentTTest, pairs_from_maps};
//!
//! // Prepare true relevance scores.
//! let mut b = TrueRelStoreBuilder::new();
//! b.add_record("q_1", "d_1", 1)?;
//! b.add_record("q_1", "d_2", 1)?;
//! b.add_record("q_2", "d_1", 1)?;
//! b.add_record("q_2", "d_2", 1)?;
//! let true_rels = b.build();
//!
//! // Prepare predicted relevance scores for system A.
//! let mut b = PredRelStoreBuilder::new();
//! b.add_record("q_1", "d_1", 0.2.into())?;
//! b.add_record("q_1", "d_2", 0.1.into())?;
//! b.add_record("q_2", "d_1", 0.2.into())?;
//! b.add_record("q_2", "d_2", 0.1.into())?;
//! let pred_rels_a = b.build();
//!
//! // Prepare predicted relevance scores for system B.
//! let mut b = PredRelStoreBuilder::new();
//! b.add_record("q_1", "d_3", 0.2.into())?;
//! b.add_record("q_1", "d_2", 0.1.into())?;
//! b.add_record("q_2", "d_3", 0.2.into())?;
//! let pred_rels_b = b.build();
//!
//! // Evaluate Precision for both systems.
//! let metric = Metric::Precision { k: 0 };
//! let result_a = elinor::evaluate(&true_rels, &pred_rels_a, metric)?;
//! let result_b = elinor::evaluate(&true_rels, &pred_rels_b, metric)?;
//!
//! // Perform two-sided paired Student's t-test.
//! let pairs = pairs_from_maps(result_a.scores(), result_b.scores())?;
//! let stat = StudentTTest::from_paired_samples(pairs)?;
//!
//! // Various statistics can be obtained from the t-test result.
//! assert!(stat.mean() > 0.0);
//! assert!(stat.variance() > 0.0);
//! assert!(stat.effect_size() > 0.0);
//! assert!(stat.t_stat() > 0.0);
//! assert!((0.0..=1.0).contains(&stat.p_value()));
//!
//! // Margin of error at a 95% confidence level.
//! let moe95 = stat.margin_of_error(0.05)?;
//! assert!(moe95 > 0.0);
//!
//! // Confidence interval at a 95% confidence level.
//! let (ci95_btm, ci95_top) = stat.confidence_interval(0.05)?;
//! assert_relative_eq!(ci95_btm, stat.mean() - moe95);
//! assert_relative_eq!(ci95_top, stat.mean() + moe95);
//! # Ok(())
//! # }
//! ```
//!
//! # Statistical tests for comparing three or more systems
//!
//! When comparing three or more systems,
//! you can use [Tukey HSD test](statistical_tests::TukeyHsdTest) and
//! [Randomized Tukey HSD test](statistical_tests::RandomizedTukeyHsdTest).
//!
//! An example to compare Precision scores among three systems is shown below:
//!
//! ```
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use elinor::{TrueRelStoreBuilder, PredRelStoreBuilder, Metric};
//! use elinor::statistical_tests::{RandomizedTukeyHsdTest, TukeyHsdTest, tuples_from_maps};
//!
//! // Prepare true relevance scores.
//! let mut b = TrueRelStoreBuilder::new();
//! b.add_record("q_1", "d_1", 1)?;
//! b.add_record("q_1", "d_2", 1)?;
//! b.add_record("q_2", "d_1", 1)?;
//! b.add_record("q_2", "d_2", 1)?;
//! let true_rels = b.build();
//!
//! // Prepare predicted relevance scores for system A.
//! let mut b = PredRelStoreBuilder::new();
//! b.add_record("q_1", "d_1", 0.2.into())?;
//! b.add_record("q_1", "d_2", 0.1.into())?;
//! b.add_record("q_2", "d_1", 0.2.into())?;
//! b.add_record("q_2", "d_2", 0.1.into())?;
//! let pred_rels_a = b.build();
//!
//! // Prepare predicted relevance scores for system B.
//! let mut b = PredRelStoreBuilder::new();
//! b.add_record("q_1", "d_3", 0.2.into())?;
//! b.add_record("q_1", "d_2", 0.1.into())?;
//! b.add_record("q_2", "d_3", 0.2.into())?;
//! let pred_rels_b = b.build();
//!
//! // Prepare predicted relevance scores for system C.
//! let mut b = PredRelStoreBuilder::new();
//! b.add_record("q_1", "d_1", 0.2.into())?;
//! b.add_record("q_2", "d_2", 0.1.into())?;
//! b.add_record("q_2", "d_4", 0.2.into())?;
//! let pred_rels_c = b.build();
//!
//! // Evaluate Precision for all systems.
//! let metric = Metric::Precision { k: 0 };
//! let result_a = elinor::evaluate(&true_rels, &pred_rels_a, metric)?;
//! let result_b = elinor::evaluate(&true_rels, &pred_rels_b, metric)?;
//! let result_c = elinor::evaluate(&true_rels, &pred_rels_c, metric)?;
//!
//! // Prepare tupled scores for tests.
//! let tupled_scores = tuples_from_maps([result_a.scores(), result_b.scores(), result_c.scores()])?;
//!
//! // Perform Tukey HSD test with paired observations.
//! let hsd_stat = TukeyHsdTest::from_tupled_samples(tupled_scores.iter(), 3)?;
//! let effect_sizes = hsd_stat.effect_sizes();
//!
//! // Perform randomized Tukey HSD test.
//! let hsd_stat = RandomizedTukeyHsdTest::from_tupled_samples(tupled_scores.iter(), 3)?;
//! let p_values = hsd_stat.p_values();
//! # Ok(())
//! # }
//! ```
//!
//! # Instantiating relevance stores with [Serde](https://serde.rs/)
//!
//! [`TrueRelStore`] and [`PredRelStore`] can be instantiated from
//! [`TrueRecord`] and [`PredRecord`] instances, respectively,
//! where each record consists of three fields: `query_id`, `document_id`, and `score`.
//!
//! Both [`TrueRecord`] and [`PredRecord`] support serialization and deserialization via Serde,
//! allowing you to easily instantiate relevance stores from JSON or other formats.
//!
//! If you use Serde, enable the `serde` feature in the `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! elinor = { version = "*", features = ["serde"] }
//! ```
//!
//! An example to instantiate relevance stores from JSONL strings is shown below:
//!
//! ```
//! # #[cfg(not(feature = "serde"))]
//! # fn main() {}
//! #
//! # #[cfg(feature = "serde")]
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use elinor::{TrueRelStore, TrueRecord, PredRelStore, PredRecord};
//!
//! let true_data = r#"
//! {"query_id": "q_1", "doc_id": "d_1", "score": 1}
//! {"query_id": "q_1", "doc_id": "d_2", "score": 0}
//! {"query_id": "q_1", "doc_id": "d_3", "score": 2}
//! {"query_id": "q_2", "doc_id": "d_2", "score": 2}
//! {"query_id": "q_2", "doc_id": "d_4", "score": 1}
//! "#.trim();
//!
//! let pred_data = r#"
//! {"query_id": "q_1", "doc_id": "d_1", "score": 0.5}
//! {"query_id": "q_1", "doc_id": "d_2", "score": 0.4}
//! {"query_id": "q_1", "doc_id": "d_3", "score": 0.3}
//! {"query_id": "q_2", "doc_id": "d_4", "score": 0.1}
//! {"query_id": "q_2", "doc_id": "d_1", "score": 0.2}
//! {"query_id": "q_2", "doc_id": "d_3", "score": 0.3}
//! "#.trim();
//!
//! let true_records = true_data
//!     .lines()
//!     .map(|line| serde_json::from_str::<TrueRecord<String>>(line).unwrap());
//! let pred_records = pred_data
//!     .lines()
//!     .map(|line| serde_json::from_str::<PredRecord<String>>(line).unwrap());
//!
//! let true_rels = TrueRelStore::from_records(true_records)?;
//! let pred_rels = PredRelStore::from_records(pred_records)?;
//!
//! assert_eq!(true_rels.n_queries(), 2);
//! assert_eq!(true_rels.n_docs(), 5);
//! assert_eq!(pred_rels.n_queries(), 2);
//! assert_eq!(pred_rels.n_docs(), 6);
//! # Ok(())
//! # }
//! ```
//!
//! # Crate features
//!
//! * `serde` - Enables Serde for [`TrueRecord`] and [`PredRecord`].
//!
//! # Acknowledgments
//!
//! This library is inspired by Sakai's books on IR evaluation and statistical testing:
//!
//! - 酒井 哲也.
//!   [情報アクセス評価方法論](https://www.coronasha.co.jp/np/isbn/9784339024968/).
//!   コロナ社, 2015.
//! - Tetsuya Sakai.
//!   [Laboratory Experiments in Information Retrieval: Sample Sizes, Effect Sizes, and Statistical Power](https://doi.org/10.1007/978-981-13-1199-4).
//!   Springer, 2018.
//!
//! I recommend reading these books before using this library.
#![deny(missing_docs)]

pub mod errors;
pub mod metrics;
pub mod relevance;
pub mod statistical_tests;
pub mod trec;

use std::collections::BTreeMap;

use ordered_float::OrderedFloat;

pub use errors::ElinorError;
pub use errors::Result;
pub use metrics::Metric;
pub use relevance::Record;
pub use relevance::Relevance;

/// Data type to store a true relevance score.
/// In binary relevance, 0 means non-relevant and the others mean relevant.
pub type TrueScore = u32;

/// Data type to store a predicted relevance score.
/// A higher score means more relevant.
pub type PredScore = OrderedFloat<f64>;

/// Record type to store a true relevance score.
pub type TrueRecord<K> = Record<K, TrueScore>;

/// Record type to store a predicted relevance score.
pub type PredRecord<K> = Record<K, PredScore>;

/// Data structure to store true relevance scores.
pub type TrueRelStore<K> = relevance::RelevanceStore<K, TrueScore>;

/// Builder for [`TrueRelStore`].
pub type TrueRelStoreBuilder<K> = relevance::RelevanceStoreBuilder<K, TrueScore>;

/// Data structure to store predicted relevance scores.
pub type PredRelStore<K> = relevance::RelevanceStore<K, PredScore>;

/// Builder for [`PredRelStore`].
pub type PredRelStoreBuilder<K> = relevance::RelevanceStoreBuilder<K, PredScore>;

/// Struct to store evaluated results.
pub struct Evaluation<K> {
    metric: Metric,
    scores: BTreeMap<K, f64>,
    mean: f64,
    variance: f64,
}

impl<K> Evaluation<K> {
    /// Returns the metric used for evaluation.
    pub const fn metric(&self) -> Metric {
        self.metric
    }

    /// Returns the reference to the mappping from query ids to scores.
    pub const fn scores(&self) -> &BTreeMap<K, f64> {
        &self.scores
    }

    /// Returns the macro-averaged score.
    pub const fn mean(&self) -> f64 {
        self.mean
    }

    /// Returns the variance of the scores.
    pub const fn variance(&self) -> f64 {
        self.variance
    }

    /// Returns the standard deviation of the scores.
    pub fn std_dev(&self) -> f64 {
        self.variance.sqrt()
    }
}

/// Evaluates the given predicted relevance scores against the true relevance scores.
///
/// # Errors
///
/// See [`metrics::compute_metric`] for the list of possible errors.
pub fn evaluate<K>(
    true_rels: &TrueRelStore<K>,
    pred_rels: &PredRelStore<K>,
    metric: Metric,
) -> Result<Evaluation<K>>
where
    K: Clone + Eq + Ord + std::fmt::Display,
{
    let scores = metrics::compute_metric(true_rels, pred_rels, metric)?;
    let mean = scores.values().sum::<f64>() / scores.len() as f64;
    let variance = scores
        .values()
        .map(|&score| (score - mean).powi(2))
        .sum::<f64>()
        / scores.len() as f64;
    Ok(Evaluation {
        metric,
        scores,
        mean,
        variance,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_evaluate() {
        let mut b = TrueRelStoreBuilder::new();
        b.add_record("q_1", "d_1", 1).unwrap();
        b.add_record("q_1", "d_2", 0).unwrap();
        b.add_record("q_1", "d_3", 2).unwrap();
        b.add_record("q_2", "d_2", 2).unwrap();
        b.add_record("q_2", "d_4", 1).unwrap();
        let true_rels = b.build();

        let mut b = PredRelStoreBuilder::new();
        b.add_record("q_1", "d_1", 0.5.into()).unwrap();
        b.add_record("q_1", "d_2", 0.4.into()).unwrap();
        b.add_record("q_1", "d_3", 0.3.into()).unwrap();
        b.add_record("q_2", "d_4", 0.1.into()).unwrap();
        b.add_record("q_2", "d_1", 0.2.into()).unwrap();
        b.add_record("q_2", "d_3", 0.3.into()).unwrap();
        let pred_rels = b.build();

        let evaluated = evaluate(&true_rels, &pred_rels, Metric::Precision { k: 3 }).unwrap();
        assert_eq!(evaluated.metric(), Metric::Precision { k: 3 });

        let mean: f64 = (2. / 3. + 1. / 3.) / 2.;
        let variance = ((2. / 3. - mean).powi(2) + (1. / 3. - mean).powi(2)) / 2.;
        assert_relative_eq!(evaluated.mean(), mean);
        assert_relative_eq!(evaluated.variance(), variance);
        assert_relative_eq!(evaluated.std_dev(), variance.sqrt());

        let scores = evaluated.scores();
        assert_eq!(scores.len(), 2);
        assert_relative_eq!(scores["q_1"], 2. / 3.);
        assert_relative_eq!(scores["q_2"], 1. / 3.);
    }
}
