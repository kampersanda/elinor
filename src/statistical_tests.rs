//! Statistical tests.

pub mod bootstrap_test;
pub mod randomized_tukey_hsd_test;
pub mod student_t_test;

pub use bootstrap_test::BootstrapTest;
pub use student_t_test::StudentTTest;
