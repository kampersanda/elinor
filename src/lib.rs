//! Elinor (**E**valuation **l**ibrary in **in**f**o**rmation **r**etrieval) is a library
//! for evaluating information retrieval (IR) systems.
//! It provides a comprehensive set of tools and metrics tailored for IR engineers,
//! offering an intuitive and easy-to-use interface.
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
//! * **Statistical testing with in-depth reporting:**
//!     Elinor includes several statistical tests, such as Student's t-test or Randomized Tukey HSD test, to verify the generalizability of results.
//!     Not only p-values but also other statistics, such as effect sizes and confidence intervals, are provided for thorough reporting.
//!     See the [`statistical_tests`] module for more details.
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
//! # Relevance stores from [`HashMap`]
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

/// Evaluates the given predicted relevance scores against the gold relevance scores.
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

/// Extracts tupled scores from multiple [`Evaluated`] results.
///
/// # Errors
///
/// * [`ElinorError::InvalidArgument`] if the evaluated results have different sets of queries.
pub fn tupled_scores_from_evaluated<K>(
    evaluateds: &[Evaluated<K>],
) -> Result<Vec<Vec<f64>>, ElinorError>
where
    K: Clone + Eq + Ord + std::hash::Hash + std::fmt::Display,
{
    if evaluateds.len() < 2 {
        return Err(ElinorError::InvalidArgument(
            "The number of evaluated results must be at least 2.".to_string(),
        ));
    }

    let score_maps = evaluateds.iter().map(|e| e.scores()).collect::<Vec<_>>();
    for i in 1..score_maps.len() {
        if score_maps[i].len() != score_maps[0].len() {
            return Err(ElinorError::InvalidArgument(
                "The evaluated results must have the same number of queries.".to_string(),
            ));
        }
    }

    let mut query_ids = score_maps[0].keys().cloned().collect::<Vec<_>>();
    query_ids.sort_unstable();

    let mut tupled_scores = vec![];
    for query_id in query_ids {
        let mut scores = vec![];
        for score_map in &score_maps {
            if let Some(score) = score_map.get(&query_id) {
                scores.push(*score);
            } else {
                return Err(ElinorError::InvalidArgument(format!(
                    "The query id {} is not found in the evaluated results.",
                    query_id
                )));
            }
        }
        tupled_scores.push(scores);
    }
    Ok(tupled_scores)
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

    #[test]
    fn test_tupled_scores_from_evaluated() {
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
        let evaluated_c = Evaluated {
            scores: hashmap! {
                "q_1" => 2.,
                "q_2" => 1.,
            },
            mean_score: 1.5,
        };
        let tupled_scores =
            tupled_scores_from_evaluated(&[evaluated_a, evaluated_b, evaluated_c]).unwrap();
        assert_eq!(tupled_scores, vec![vec![2., 1., 2.], vec![5., 0., 1.]]);
    }
}
