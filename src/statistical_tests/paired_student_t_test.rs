//! Paired Student's t-test

use statrs::distribution::ContinuousCDF;
use statrs::distribution::StudentsT;
use statrs::statistics::Statistics;

///
#[derive(Debug)]
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

    /// Margin of error at 99% confidence level.
    pub moe_99: f64,

    /// Margin of error at 95% confidence level.
    pub moe_95: f64,

    /// Margin of error at 90% confidence level.
    pub moe_90: f64,
}

impl PairedStudentTTest {
    /// Returns the confidence interval at 99% confidence level.
    pub fn confidence_interval_99(&self) -> (f64, f64) {
        (self.mean - self.moe_99, self.mean + self.moe_99)
    }

    /// Returns the confidence interval at 95% confidence level.
    pub fn confidence_interval_95(&self) -> (f64, f64) {
        (self.mean - self.moe_95, self.mean + self.moe_95)
    }

    /// Returns the confidence interval at 90% confidence level.
    pub fn confidence_interval_90(&self) -> (f64, f64) {
        (self.mean - self.moe_90, self.mean + self.moe_90)
    }
}

/// Runs a paired Student's t-test.
pub fn run_paired_student_t_test(a: &[f64], b: &[f64]) -> PairedStudentTTest {
    assert_eq!(a.len(), b.len());

    let diffs: Vec<f64> = a.iter().zip(b.iter()).map(|(x, y)| x - y).collect();
    let mean = Statistics::mean(&diffs);
    let var = Statistics::variance(&diffs);
    let n = diffs.len() as f64;
    let t_stat = mean / (var / n).sqrt();
    let t_dist = StudentsT::new(0.0, 1.0, n - 1.0).unwrap();
    let p_value = t_dist.sf(t_stat.abs()) * 2.0; // two-tailed
    let effect_size = mean / var.sqrt();

    let scale = (var / n).sqrt();
    let moe_99 = t_dist.inverse_cdf(1.0 - (0.01 / 2.0)) * scale;
    let moe_95 = t_dist.inverse_cdf(1.0 - (0.05 / 2.0)) * scale;
    let moe_90 = t_dist.inverse_cdf(1.0 - (0.10 / 2.0)) * scale;

    PairedStudentTTest {
        mean,
        var,
        t_stat,
        p_value,
        effect_size,
        moe_99,
        moe_95,
        moe_90,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_compute_paired_student_t_test() {
        // From sakai's book
        let a = vec![
            0.70, 0.30, 0.20, 0.60, 0.40, 0.40, 0.00, 0.70, 0.10, 0.30, //
            0.50, 0.40, 0.00, 0.60, 0.50, 0.30, 0.10, 0.50, 0.20, 0.10,
        ];
        let b = vec![
            0.50, 0.10, 0.00, 0.20, 0.40, 0.30, 0.00, 0.50, 0.30, 0.30, //
            0.40, 0.40, 0.10, 0.40, 0.20, 0.10, 0.10, 0.60, 0.30, 0.20,
        ];
        let result = run_paired_student_t_test(&a, &b);
        assert_abs_diff_eq!(result.mean, 0.0750, epsilon = 1e-4);
        assert_abs_diff_eq!(result.var, 0.0251, epsilon = 1e-4);
        assert_abs_diff_eq!(result.t_stat, 2.116, epsilon = 1e-3);
        assert_abs_diff_eq!(result.p_value, 0.048, epsilon = 1e-3);
        assert_abs_diff_eq!(result.effect_size, 0.473, epsilon = 1e-3);
        assert_abs_diff_eq!(result.moe_99, 0.1014, epsilon = 1e-4);
        assert_abs_diff_eq!(result.moe_95, 0.0742, epsilon = 1e-4);
        assert_abs_diff_eq!(result.moe_90, 0.0613, epsilon = 1e-4);
    }
}
