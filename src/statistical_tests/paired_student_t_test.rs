//! Paired Student's t-test

use statrs::distribution::ContinuousCDF;
use statrs::distribution::StudentsT;
use statrs::statistics::Statistics;

use crate::errors::ElinorError;

/// Paired Student's t-test.
#[derive(Debug, Clone, Copy)]
pub struct PairedStudentTTest {
    mean: f64,
    var: f64,
    t_stat: f64,
    p_value: f64,
    effect_size: f64,
    scaled_t_dist: StudentsT,
}

impl PairedStudentTTest {
    /// Computes a paired Student's t-test.
    ///
    /// # Errors
    ///
    /// * [`ElinorError::InvalidArgument`] if the input does not have at least two samples.
    /// * [`ElinorError::Uncomputable`] if the variance is zero.
    pub fn compute<I>(paired_samples: I) -> Result<Self, ElinorError>
    where
        I: IntoIterator<Item = (f64, f64)>,
    {
        let (a, b): (Vec<f64>, Vec<f64>) = paired_samples.into_iter().unzip();
        if a.len() <= 1 {
            return Err(ElinorError::InvalidArgument(
                "The input must have at least two samples.".to_string(),
            ));
        }
        let diffs: Vec<f64> = a
            .into_iter()
            .zip(b.into_iter())
            .map(|(x, y)| x - y)
            .collect();
        let mean = Statistics::mean(&diffs);
        let var = Statistics::variance(&diffs);
        if var == 0.0 {
            return Err(ElinorError::Uncomputable(
                "The variance of the differences is zero.".to_string(),
            ));
        }
        let n = diffs.len() as f64;
        let t_stat = mean / (var / n).sqrt();
        let t_dist = StudentsT::new(0.0, 1.0, n - 1.0).unwrap();
        let p_value = t_dist.sf(t_stat.abs()) * 2.0; // two-tailed
        let effect_size = mean / var.sqrt();
        dbg!(var, n, (var / n).sqrt());
        let scaled_t_dist = StudentsT::new(0.0, (var / n).sqrt(), n - 1.0).unwrap();
        Ok(Self {
            mean,
            var,
            t_stat,
            p_value,
            effect_size,
            scaled_t_dist,
        })
    }

    /// Mean difference.
    pub const fn mean(&self) -> f64 {
        self.mean
    }

    /// Unbiased population variance.
    pub const fn var(&self) -> f64 {
        self.var
    }

    /// t-statistic.
    pub const fn t_stat(&self) -> f64 {
        self.t_stat
    }

    /// p-value.
    pub const fn p_value(&self) -> f64 {
        self.p_value
    }

    /// Effect size.
    pub const fn effect_size(&self) -> f64 {
        self.effect_size
    }

    /// Margin of error at a (1 - significance_level) confidence level.
    pub fn margin_of_error(&self, significance_level: f64) -> f64 {
        self.scaled_t_dist
            .inverse_cdf(1.0 - (significance_level / 2.0))
    }

    /// Confidence interval at a (1 - significance_level) confidence level.
    pub fn confidence_interval(&self, significance_level: f64) -> (f64, f64) {
        let moe = self.margin_of_error(significance_level);
        (self.mean - moe, self.mean + moe)
    }

    /// Returns true if the p-value is less than the significance level.
    pub fn is_significant(&self, significance_level: f64) -> bool {
        self.p_value < significance_level
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_paired_student_t_test_compute() {
        // From sakai's book
        let a = vec![
            0.70, 0.30, 0.20, 0.60, 0.40, 0.40, 0.00, 0.70, 0.10, 0.30, //
            0.50, 0.40, 0.00, 0.60, 0.50, 0.30, 0.10, 0.50, 0.20, 0.10,
        ];
        let b = vec![
            0.50, 0.10, 0.00, 0.20, 0.40, 0.30, 0.00, 0.50, 0.30, 0.30, //
            0.40, 0.40, 0.10, 0.40, 0.20, 0.10, 0.10, 0.60, 0.30, 0.20,
        ];
        let result =
            PairedStudentTTest::compute(a.into_iter().zip(b.into_iter()).map(|(x, y)| (x, y)))
                .unwrap();
        assert_abs_diff_eq!(result.mean(), 0.0750, epsilon = 1e-4);
        assert_abs_diff_eq!(result.var(), 0.0251, epsilon = 1e-4);
        assert_abs_diff_eq!(result.t_stat(), 2.116, epsilon = 1e-3);
        assert_abs_diff_eq!(result.p_value(), 0.048, epsilon = 1e-3);
        assert_abs_diff_eq!(result.effect_size(), 0.473, epsilon = 1e-3);
        assert_abs_diff_eq!(result.margin_of_error(0.05), 0.0742, epsilon = 1e-4);

        let ci_95 = result.confidence_interval(0.05);
        assert_abs_diff_eq!(ci_95.0, 0.0750 - 0.0742, epsilon = 1e-4);
        assert_abs_diff_eq!(ci_95.1, 0.0750 + 0.0742, epsilon = 1e-4);

        assert_eq!(result.is_significant(0.05), true);
    }

    #[test]
    fn test_paired_student_t_test_compute_empty() {
        let result = PairedStudentTTest::compute(Vec::<(f64, f64)>::new());
        assert_eq!(
            result.unwrap_err(),
            ElinorError::InvalidArgument("The input must have at least two samples.".to_string())
        );
    }

    #[test]
    fn test_paired_student_t_test_compute_one_sample() {
        let result = PairedStudentTTest::compute(vec![(1.0, 2.0)]);
        assert_eq!(
            result.unwrap_err(),
            ElinorError::InvalidArgument("The input must have at least two samples.".to_string())
        );
    }

    #[test]
    fn test_paired_student_t_test_compute_two_samples() {
        let result = PairedStudentTTest::compute(vec![(1.0, 2.0), (3.0, 4.5)]);
        let expected = (1.0 - 2.0 + 3.0 - 4.5) / 2.0;
        assert_abs_diff_eq!(result.unwrap().mean(), expected, epsilon = 1e-4);
    }

    #[test]
    fn test_paired_student_t_test_compute_zero_variance() {
        let result = PairedStudentTTest::compute(vec![(1.0, 2.0), (2.0, 3.0)]);
        assert_eq!(
            result.unwrap_err(),
            ElinorError::Uncomputable("The variance of the differences is zero.".to_string())
        );
    }
}
