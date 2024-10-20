//! Statistical tests.
//!
//! This module provides several statistical tests for comparing systems:
//!
//! * [Student's t-test](StudentTTest) for comparing two systems.
//! * [Bootstrap test](BootstrapTest) for comparing two systems.
//! * [Two-way ANOVA without replication](TwoWayAnovaWithoutReplication) for comparing three or more systems.
//! * [Tukey HSD test](TukeyHsdTest) for comparing three or more systems.
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
//! b.add_record("q_1", "d_1", 1)?;
//! b.add_record("q_1", "d_2", 1)?;
//! b.add_record("q_2", "d_1", 1)?;
//! b.add_record("q_2", "d_2", 1)?;
//! let gold_rels = b.build();
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
//! let result_a = elinor::evaluate(&gold_rels, &pred_rels_a, metric)?;
//! let result_b = elinor::evaluate(&gold_rels, &pred_rels_b, metric)?;
//!
//! // Perform two-sided paired Student's t-test.
//! let tupled_scores = elinor::tupled_scores_from_evaluations(&[&result_a, &result_b])?;
//! let stat = StudentTTest::from_samples(tupled_scores.iter().map(|x| x[0] - x[1]))?;
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
//! # Example: Statistical tests for comparing three systems
//!
//! This example shows how to perform [Tukey HSD test](TukeyHsdTest) and [Randomized Tukey HSD test](RandomizedTukeyHsdTest)
//! for Precision scores among three systems.
//!
//! ```
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use elinor::{GoldRelStoreBuilder, PredRelStoreBuilder, Metric};
//! use elinor::statistical_tests::{RandomizedTukeyHsdTest, TukeyHsdTest};
//!
//! // Prepare gold relevance scores.
//! let mut b = GoldRelStoreBuilder::new();
//! b.add_record("q_1", "d_1", 1)?;
//! b.add_record("q_1", "d_2", 1)?;
//! b.add_record("q_2", "d_1", 1)?;
//! b.add_record("q_2", "d_2", 1)?;
//! let gold_rels = b.build();
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
//! let result_a = elinor::evaluate(&gold_rels, &pred_rels_a, metric)?;
//! let result_b = elinor::evaluate(&gold_rels, &pred_rels_b, metric)?;
//! let result_c = elinor::evaluate(&gold_rels, &pred_rels_c, metric)?;
//!
//! // Prepare tupled scores for tests.
//! let tupled_scores = elinor::tupled_scores_from_evaluations(&[&result_a, &result_b, &result_c])?;
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
pub mod bootstrap_test;
pub mod randomized_tukey_hsd_test;
pub mod student_t_test;
pub mod tukey_hsd_test;
pub mod two_way_anova_without_replication;

pub use bootstrap_test::BootstrapTest;
pub use randomized_tukey_hsd_test::RandomizedTukeyHsdTest;
pub use student_t_test::StudentTTest;
pub use tukey_hsd_test::TukeyHsdTest;
pub use two_way_anova_without_replication::TwoWayAnovaWithoutReplication;
