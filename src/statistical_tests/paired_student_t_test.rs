//! Paired Student's t-test

use statrs::distribution::Continuous;
use statrs::distribution::StudentsT;
use statrs::statistics::Statistics;

pub struct PairedStudentTTest {
    pub mean_diff: f64,
    pub var_diff: f64,
    pub t: f64,
    pub effect_size: f64,
}

/// Computes the paired Student's t-test.
pub fn compute_paired_student_t_test(a: &[f64], b: &[f64]) -> f64 {
    assert_eq!(a.len(), b.len());

    let n_samples = a.len() as f64;
    let diffs: Vec<f64> = a.iter().zip(b.iter()).map(|(x, y)| x - y).collect();
    let mean = Statistics::mean(&diffs);
    let var = Statistics::variance(&diffs);
    let std = var.sqrt();
    let effect_size = mean / std;

    let freedom = n_samples - 1.0;
    let scale = var / n_samples;
    let students_t = StudentsT::new(0.0, 1.0, freedom).unwrap();

    let n = a.len() as f64;
    let t = mean_diff / (var_diff / n).sqrt();

    0.0
}
