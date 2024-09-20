//! Paired Student's t-test

use core::str;

use statrs::distribution::ContinuousCDF;
use statrs::distribution::StudentsT;
use statrs::statistics::Statistics;

///
pub struct PairedStudentTTest {
    /// Mean difference.
    pub mean: f64,

    /// Unbiased population variance.
    pub var: f64,

    /// t-statistic.
    pub t_stat: f64,

    /// p-value.
    pub p_value: f64,

    /// Effect size.
    pub effect_size: f64,
}

/// Computes the paired Student's t-test.
pub fn compute_paired_student_t_test(a: &[f64], b: &[f64]) -> PairedStudentTTest {
    assert_eq!(a.len(), b.len());

    let diffs: Vec<f64> = a.iter().zip(b.iter()).map(|(x, y)| x - y).collect();
    let mean = Statistics::mean(&diffs);
    let var = Statistics::variance(&diffs);
    let n = diffs.len() as f64;
    let t_stat = mean / (var / n).sqrt();
    let t_dist = StudentsT::new(0.0, 1.0, n - 1.0).unwrap();
    let p_value = t_dist.sf(t_stat.abs());
    let effect_size = mean / var.sqrt();

    PairedStudentTTest {
        mean,
        var,
        t_stat,
        p_value,
        effect_size,
    }
}
