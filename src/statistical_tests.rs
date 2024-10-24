//! Statistical tests.
//!
//! This module provides several statistical tests for comparing systems:
//!
//! * [Student's t-test](StudentTTest) for comparing two systems.
//! * [Bootstrap test](BootstrapTest) for comparing two systems.
//! * [Two-way ANOVA without replication](TwoWayAnovaWithoutReplication) for comparing three or more systems.
//! * [Tukey HSD test](TukeyHsdTest) for comparing three or more systems.
//! * [Randomized Tukey HSD test](RandomizedTukeyHsdTest) for comparing two or more systems.
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
