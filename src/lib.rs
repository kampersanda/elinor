//! Elinor is a library for evaluating information retrieval systems,
//! inspired by [ranx](https://github.com/AmenRa/ranx) and [Sakai's book](https://www.coronasha.co.jp/np/isbn/9784339024968/).
//!
//! It provides a comprehensive set of tools and metrics tailored for information retrieval engineers,
//! offering an intuitive and easy-to-use interface.
//!
//! # Key Features
//!
//! * **IR-focused design:**
//!     Elinor is tailored specifically for evaluating information retrieval systems, with an intuitive interface designed for IR engineers.
//!     It offers a streamlined workflow that simplifies common IR evaluation tasks.
//!
//! * **Comprehensive evaluation metrics:**
//!     Elinor supports a wide range of key evaluation metrics, such as Precision, MAP, MRR, and nDCG.
//!     The supported metrics are available in [`Metric`].
//!     The evaluation results are validated against trec_eval to ensure accuracy and reliability.
//!
//! * **Robust statistical testing:**
//!     Elinor includes several statistical tests such as Student's t-test to verify the generalizability of results.
//!     It provides not only p-values but also effect sizes and confidence intervals for thorough reporting.
//!
//! # Basic usage in evaluating several metrics
//!
//! You first need to prepare gold relevance judgments and predicted relevance scores through
//! [`GoldRelStore`] and [`PredRelStore`], respectively.
//! You can build these instances using [`GoldRelStoreBuilder`] and [`PredRelStoreBuilder`].
//!
//! Then, you can evaluate the predicted relevance scores using the [`evaluate`] function and
//! the specified metric. The available metrics are defined in the [`Metric`] enum.
//!
//! An example is shown below:
//!
//! ```
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use approx::assert_abs_diff_eq;
//! use elinor::{GoldRelStoreBuilder, PredRelStoreBuilder, Metric};
//!
//! // Prepare gold relevance scores.
//! // In binary-relevance metrics, 0 means non-relevant and the others mean relevant.
//! let mut b = GoldRelStoreBuilder::new();
//! b.add_score("q_1", "d_1", 1)?;
//! b.add_score("q_1", "d_2", 0)?;
//! b.add_score("q_1", "d_3", 2)?;
//! b.add_score("q_2", "d_2", 2)?;
//! b.add_score("q_2", "d_4", 1)?;
//! let gold_rels = b.build();
//!
//! // Prepare predicted relevance scores.
//! let mut b = PredRelStoreBuilder::new();
//! b.add_score("q_1", "d_1", 0.5.into())?;
//! b.add_score("q_1", "d_2", 0.4.into())?;
//! b.add_score("q_1", "d_3", 0.3.into())?;
//! b.add_score("q_2", "d_4", 0.1.into())?;
//! b.add_score("q_2", "d_1", 0.2.into())?;
//! b.add_score("q_2", "d_3", 0.3.into())?;
//! let pred_rels = b.build();
//!
//! // Evaluate Precision@3.
//! let evaluated = elinor::evaluate(&gold_rels, &pred_rels, Metric::Precision { k: 3 })?;
//! assert_abs_diff_eq!(evaluated.mean_score(), 0.5000, epsilon = 1e-4);
//!
//! // Evaluate MAP, where all documents are considered via k=0.
//! let evaluated = elinor::evaluate(&gold_rels, &pred_rels, Metric::AP { k: 0 })?;
//! assert_abs_diff_eq!(evaluated.mean_score(), 0.5000, epsilon = 1e-4);
//!
//! // Evaluate MRR, where the metric is specified via a string representation.
//! let evaluated = elinor::evaluate(&gold_rels, &pred_rels, "rr".parse()?)?;
//! assert_abs_diff_eq!(evaluated.mean_score(), 0.6667, epsilon = 1e-4);
//!
//! // Evaluate nDCG@3, where the metric is specified via a string representation.
//! let evaluated = elinor::evaluate(&gold_rels, &pred_rels, "ndcg@3".parse()?)?;
//! assert_abs_diff_eq!(evaluated.mean_score(), 0.4751, epsilon = 1e-4);
//! # Ok(())
//! # }
//! ```
//!
//! # Statistical tests for comparing two systems
//!
//! The [`statistical_tests`] module provides statistical tests for comparing systems,
//! such as Student's t-test and bootstrap resampling.
//!
//! This example shows how to perform Student's t-test for Precision scores between two systems.
//! Not only the p-value but also various statistics, such as variance and effect size, are provided for thorough reporting.
//!
//! ```
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use approx::assert_relative_eq;
//! use elinor::{GoldRelStoreBuilder, PredRelStoreBuilder, Metric};
//! use elinor::paired_scores_from_evaluated;
//! use elinor::statistical_tests::StudentTTest;
//!
//! // Prepare gold relevance scores.
//! let mut b = GoldRelStoreBuilder::new();
//! b.add_score("q_1", "d_1", 1)?;
//! b.add_score("q_1", "d_2", 1)?;
//! b.add_score("q_2", "d_1", 1)?;
//! b.add_score("q_2", "d_2", 1)?;
//! let gold_rels = b.build();
//!
//! // Prepare predicted relevance scores for system A.
//! let mut b = PredRelStoreBuilder::new();
//! b.add_score("q_1", "d_1", 0.2.into())?;
//! b.add_score("q_1", "d_2", 0.1.into())?;
//! b.add_score("q_2", "d_1", 0.2.into())?;
//! b.add_score("q_2", "d_2", 0.1.into())?;
//! let pred_rels_a = b.build();
//!
//! // Prepare predicted relevance scores for system B.
//! let mut b = PredRelStoreBuilder::new();
//! b.add_score("q_1", "d_3", 0.2.into())?;
//! b.add_score("q_1", "d_2", 0.1.into())?;
//! b.add_score("q_2", "d_3", 0.2.into())?;
//! let pred_rels_b = b.build();
//!
//! // Evaluate Precision for both systems.
//! let evaluated_a = elinor::evaluate(&gold_rels, &pred_rels_a, Metric::Precision { k: 0 })?;
//! let evaluated_b = elinor::evaluate(&gold_rels, &pred_rels_b, Metric::Precision { k: 0 })?;
//!
//! // Perform Student's t-test.
//! let paired_scores = elinor::paired_scores_from_evaluated(&evaluated_a, &evaluated_b)?;
//! let result = StudentTTest::from_paired_samples(paired_scores)?;
//!
//! // Various statistics can be obtained from the t-test result.
//! assert!(result.mean() > 0.0);
//! assert!(result.var() > 0.0);
//! assert!(result.effect_size() > 0.0);
//! assert!(result.t_stat() > 0.0);
//! assert!(result.p_value() > 0.0);
//!
//! // Margin of error at a 95% confidence level.
//! let moe95 = result.margin_of_error(0.05)?;
//! assert!(moe95 > 0.0);
//!
//! // Confidence interval at a 95% confidence level.
//! let (ci95_btm, ci95_top) = result.confidence_interval(0.05)?;
//! assert_relative_eq!(ci95_btm, result.mean() - moe95);
//! assert_relative_eq!(ci95_top, result.mean() + moe95);
//!
//! // Check if the difference is significant at a 95% confidence level.
//! assert_eq!(result.is_significant(0.05), result.p_value() <= 0.05);
//! # Ok(())
//! # }
//! ```
//!
//! # Instantiating relevance stores from [`HashMap`]
//!
//! [`GoldRelStore`] and [`PredRelStore`] can also be instantiated from [`HashMap`]s.
//! The following mapping structure is expected:
//!
//! ```text
//! query_id => { doc_id => score }
//! ```
//!
//! It allows you to prepare data in JSON or other formats via [Serde](https://serde.rs/).
//! If you use Serde, enable the `serde` feature in the `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! elinor = { version = "*", features = ["serde"] }
//! ```
//!
//! An example to instantiate relevance stores from JSON is shown below:
//!
//! ```
//! # #[cfg(not(feature = "serde"))]
//! # fn main() {}
//! #
//! # #[cfg(feature = "serde")]
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use std::collections::HashMap;
//! use elinor::{GoldRelStore, GoldScore, PredRelStore, PredScore};
//!
//! let gold_rels_data = r#"
//! {
//!     "q_1": {
//!         "d_1": 1,
//!         "d_2": 0,
//!         "d_3": 2
//!     },
//!     "q_2": {
//!         "d_2": 2,
//!         "d_4": 1
//!     }
//! }"#;
//!
//! let pred_rels_data = r#"
//! {
//!     "q_1": {
//!         "d_1": 0.5,
//!         "d_2": 0.4,
//!         "d_3": 0.3
//!     },
//!     "q_2": {
//!         "d_3": 0.3,
//!         "d_1": 0.2,
//!         "d_4": 0.1
//!     }
//! }"#;
//!
//! let gold_rels_map: HashMap<String, HashMap<String, GoldScore>> =
//!     serde_json::from_str(gold_rels_data)?;
//! let pred_rels_map: HashMap<String, HashMap<String, PredScore>> =
//!     serde_json::from_str(pred_rels_data)?;
//!
//! let gold_rels = GoldRelStore::from_map(gold_rels_map);
//! let pred_rels = PredRelStore::from_map(pred_rels_map);
//!
//! assert_eq!(gold_rels.n_queries(), 2);
//! assert_eq!(gold_rels.n_docs(), 5);
//! assert_eq!(pred_rels.n_queries(), 2);
//! assert_eq!(pred_rels.n_docs(), 6);
//! # Ok(())
//! # }
//! ```
//!
//! # Crate features
//!
//! * `serde` - Enables Serde for [`PredScore`].
#![deny(missing_docs)]

pub mod errors;
pub mod metrics;
pub mod relevance;
pub mod statistical_tests;
pub mod trec;

use std::collections::HashMap;

use ordered_float::OrderedFloat;

pub use errors::ElinorError;
pub use metrics::Metric;
pub use relevance::Relevance;

/// Data type to store a gold relevance score.
/// In binary relevance, 0 means non-relevant and the others mean relevant.
pub type GoldScore = u32;

/// Data type to store a predicted relevance score.
/// A higher score means more relevant.
pub type PredScore = OrderedFloat<f64>;

/// Data structure to store gold relevance scores.
pub type GoldRelStore<K> = relevance::RelevanceStore<K, GoldScore>;

/// Builder for [`GoldRelStore`].
pub type GoldRelStoreBuilder<K> = relevance::RelevanceStoreBuilder<K, GoldScore>;

/// Data structure to store predicted relevance scores.
pub type PredRelStore<K> = relevance::RelevanceStore<K, PredScore>;

/// Builder for [`PredRelStore`].
pub type PredRelStoreBuilder<K> = relevance::RelevanceStoreBuilder<K, PredScore>;

/// Struct to store evaluated results.
pub struct Evaluated<K> {
    scores: HashMap<K, f64>,
    mean_score: f64,
}

impl<K> Evaluated<K> {
    /// Returns the reference to the mappping from query ids to scores.
    pub const fn scores(&self) -> &HashMap<K, f64> {
        &self.scores
    }

    /// Returns the macro-averaged score.
    pub const fn mean_score(&self) -> f64 {
        self.mean_score
    }
}

/// Evaluates the given gold_rels and pred_rels data using the specified metrics.
pub fn evaluate<K>(
    gold_rels: &GoldRelStore<K>,
    pred_rels: &PredRelStore<K>,
    metric: Metric,
) -> Result<Evaluated<K>, ElinorError>
where
    K: Clone + Eq + Ord + std::hash::Hash + std::fmt::Display,
{
    let scores = metrics::compute_metric(gold_rels, pred_rels, metric)?;
    let mean_score = scores.values().sum::<f64>() / scores.len() as f64;
    Ok(Evaluated { scores, mean_score })
}

/// Extracts paired scores from two [`Evaluated`] results.
///
/// # Errors
///
/// * [`ElinorError::InvalidArgument`] if the two evaluated results have different sets of queries.
pub fn paired_scores_from_evaluated<K>(
    a: &Evaluated<K>,
    b: &Evaluated<K>,
) -> Result<Vec<(f64, f64)>, ElinorError>
where
    K: Clone + Eq + Ord + std::hash::Hash + std::fmt::Display,
{
    let a = a.scores();
    let b = b.scores();
    if a.len() != b.len() {
        return Err(ElinorError::InvalidArgument(
            "The two evaluated results must have the same number of queries.".to_string(),
        ));
    }

    // Sort query ids to ensure the order of paired scores.
    let mut query_ids = a.keys().cloned().collect::<Vec<_>>();
    query_ids.sort_unstable();

    let mut paired_scores = vec![];
    for query_id in query_ids {
        let score_a = a.get(&query_id).unwrap();
        let score_b = b.get(&query_id).ok_or_else(|| {
            ElinorError::InvalidArgument(format!(
                "The query id {} is not found in the second evaluated result.",
                query_id
            ))
        })?;
        paired_scores.push((*score_a, *score_b));
    }
    Ok(paired_scores)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use maplit::hashmap;

    #[test]
    fn test_evaluate() {
        let mut b = GoldRelStoreBuilder::new();
        b.add_score("q_1", "d_1", 1).unwrap();
        b.add_score("q_1", "d_2", 0).unwrap();
        b.add_score("q_1", "d_3", 2).unwrap();
        b.add_score("q_2", "d_2", 2).unwrap();
        b.add_score("q_2", "d_4", 1).unwrap();
        let gold_rels = b.build();

        let mut b = PredRelStoreBuilder::new();
        b.add_score("q_1", "d_1", 0.5.into()).unwrap();
        b.add_score("q_1", "d_2", 0.4.into()).unwrap();
        b.add_score("q_1", "d_3", 0.3.into()).unwrap();
        b.add_score("q_2", "d_4", 0.1.into()).unwrap();
        b.add_score("q_2", "d_1", 0.2.into()).unwrap();
        b.add_score("q_2", "d_3", 0.3.into()).unwrap();
        let pred_rels = b.build();

        let evaluated = evaluate(&gold_rels, &pred_rels, Metric::Precision { k: 3 }).unwrap();
        assert_relative_eq!(evaluated.mean_score(), (2. / 3. + 1. / 3.) / 2.);

        let scores = evaluated.scores();
        assert_eq!(scores.len(), 2);
        assert_relative_eq!(scores["q_1"], 2. / 3.);
        assert_relative_eq!(scores["q_2"], 1. / 3.);
    }

    #[test]
    fn test_paired_scores_from_evaluated() {
        let evaluated_a = Evaluated {
            scores: hashmap! {
                "q_1" => 2.,
                "q_2" => 5.,
            },
            mean_score: 3.5,
        };
        let evaluated_b = Evaluated {
            scores: hashmap! {
                "q_1" => 1.,
                "q_2" => 0.,
            },
            mean_score: 0.5,
        };
        let paired_scores = paired_scores_from_evaluated(&evaluated_a, &evaluated_b).unwrap();
        assert_eq!(paired_scores, vec![(2., 1.), (5., 0.)]);
    }

    #[test]
    fn test_paired_scores_from_evaluated_different_n_queries() {
        let evaluated_a = Evaluated {
            scores: hashmap! {
                "q_1" => 2.,
                "q_2" => 5.,
            },
            mean_score: 3.5,
        };
        let evaluated_b = Evaluated {
            scores: hashmap! {
                "q_1" => 1.,
            },
            mean_score: 1.0,
        };
        let result = paired_scores_from_evaluated(&evaluated_a, &evaluated_b);
        assert_eq!(
            result.unwrap_err(),
            ElinorError::InvalidArgument(
                "The two evaluated results must have the same number of queries.".to_string()
            )
        );
    }

    #[test]
    fn test_paired_scores_from_evaluated_missing_query_id() {
        let evaluated_a = Evaluated {
            scores: hashmap! {
                "q_1" => 2.,
                "q_2" => 5.,
            },
            mean_score: 3.5,
        };
        let evaluated_b = Evaluated {
            scores: hashmap! {
                "q_1" => 1.,
                "q_3" => 0.,
            },
            mean_score: 0.5,
        };
        let result = paired_scores_from_evaluated(&evaluated_a, &evaluated_b);
        assert_eq!(
            result.unwrap_err(),
            ElinorError::InvalidArgument(
                "The query id q_2 is not found in the second evaluated result.".to_string()
            )
        );
    }
}
