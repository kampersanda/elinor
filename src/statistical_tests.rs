//! Statistical tests.
//!
//! This module provides several statistical tests for comparing systems:
//!
//! * [Student's t-test](StudentTTest) for comparing two systems.
//! * [Bootstrap test](BootstrapTest) for comparing two systems.
//! * [Randomized Tukey HSD test](RandomizedTukeyHsdTest) for comparing two or more systems.
//!
//! # Example: Statistical tests for comparing two systems
//!
//! This example shows how to perform [Student's t-test](StudentTTest) for Precision scores between two systems.
//! Not only the p-value but also various statistics, such as variance and effect size, are provided for thorough reporting.
//!
//! ```
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use approx::assert_relative_eq;
//! use elinor::{GoldRelStoreBuilder, PredRelStoreBuilder, Metric};
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
//! let metric = Metric::Precision { k: 0 };
//! let evaluated_a = elinor::evaluate(&gold_rels, &pred_rels_a, metric)?;
//! let evaluated_b = elinor::evaluate(&gold_rels, &pred_rels_b, metric)?;
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
//! assert!((0.0..=1.0).contains(&result.p_value()));
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
//! # Example: Statistical tests for comparing three systems
//!
//! [Randomized Tukey HSD test](RandomizedTukeyHsdTest) can be used to compare two or more systems.
//!
//! ```
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use elinor::{GoldRelStoreBuilder, PredRelStoreBuilder, Metric};
//! use elinor::statistical_tests::RandomizedTukeyHsdTest;
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
//! // Prepare predicted relevance scores for system C.
//! let mut b = PredRelStoreBuilder::new();
//! b.add_score("q_1", "d_1", 0.2.into())?;
//! b.add_score("q_2", "d_2", 0.1.into())?;
//! b.add_score("q_2", "d_4", 0.2.into())?;
//! let pred_rels_c = b.build();
//!
//! // Evaluate Precision for both systems.
//! let metric = Metric::Precision { k: 0 };
//! let evaluated_a = elinor::evaluate(&gold_rels, &pred_rels_a, metric)?;
//! let evaluated_b = elinor::evaluate(&gold_rels, &pred_rels_b, metric)?;
//! let evaluated_c = elinor::evaluate(&gold_rels, &pred_rels_c, metric)?;
//!
//! // Perform Randomized Tukey HSD test.
//! let tupled_scores = elinor::tupled_scores_from_evaluated(&[evaluated_a, evaluated_b, evaluated_c])?;
//! let result = RandomizedTukeyHsdTest::from_tupled_samples(tupled_scores, 3)?;
//! assert!((0.0..=1.0).contains(&result.p_value(0, 1)?));  // A vs. B
//! assert!((0.0..=1.0).contains(&result.p_value(0, 2)?));  // A vs. C
//! assert!((0.0..=1.0).contains(&result.p_value(1, 2)?));  // B vs. C
//! # Ok(())
//! # }
//! ```
pub mod bootstrap_test;
pub mod randomized_tukey_hsd_test;
pub mod student_t_test;
pub mod two_way_anova_without_replication;

pub use bootstrap_test::BootstrapTest;
pub use randomized_tukey_hsd_test::RandomizedTukeyHsdTest;
pub use student_t_test::StudentTTest;
pub use two_way_anova_without_replication::TwoWayAnovaWithoutReplication;
