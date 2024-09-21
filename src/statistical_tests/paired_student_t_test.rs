//! Paired Student's t-test

use statrs::distribution::ContinuousCDF;
use statrs::distribution::StudentsT;
use statrs::statistics::Statistics;

use crate::errors::ElinorError;

/// Paired Student's t-test.
///
/// # Examples
///
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use approx::assert_abs_diff_eq;
/// use elinor::statistical_tests::PairedStudentTTest;
///
/// // From Table 5.1 in Sakai's book, "情報アクセス評価方法論".
/// let a = vec![
///     0.70, 0.30, 0.20, 0.60, 0.40, 0.40, 0.00, 0.70, 0.10, 0.30, //
///     0.50, 0.40, 0.00, 0.60, 0.50, 0.30, 0.10, 0.50, 0.20, 0.10,
/// ];
/// let b = vec![
///     0.50, 0.10, 0.00, 0.20, 0.40, 0.30, 0.00, 0.50, 0.30, 0.30, //
///     0.40, 0.40, 0.10, 0.40, 0.20, 0.10, 0.10, 0.60, 0.30, 0.20,
/// ];
///
/// let paired_samples = a.into_iter().zip(b.into_iter()).map(|(x, y)| (x, y));
/// let result = PairedStudentTTest::compute(paired_samples)?;
///
/// // Various statistics can be obtained.
/// assert_abs_diff_eq!(result.mean(), 0.0750, epsilon = 1e-4);
/// assert_abs_diff_eq!(result.var(), 0.0251, epsilon = 1e-4);
/// assert_abs_diff_eq!(result.effect_size(), 0.473, epsilon = 1e-3);
/// assert_abs_diff_eq!(result.t_stat(), 2.116, epsilon = 1e-3);
/// assert_abs_diff_eq!(result.p_value(), 0.048, epsilon = 1e-3);
///
/// // Margin of error at a 95% confidence level.
/// assert_abs_diff_eq!(result.margin_of_error(0.05)?, 0.0742, epsilon = 1e-4);
///
/// // Confidence interval at a 95% confidence level.
/// let (ci95_btm, ci95_top) = result.confidence_interval(0.05)?;
/// assert_abs_diff_eq!(ci95_btm, 0.0750 - 0.0742, epsilon = 1e-4);
/// assert_abs_diff_eq!(ci95_top, 0.0750 + 0.0742, epsilon = 1e-4);
///
/// // Check if the difference is significant at a 95% confidence level.
/// assert!(result.is_significant(0.05));
/// # Ok(())
/// # }
/// ```
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

    /// Mean difference, i.e., `mean(a - b)` for `(a, b)` in `paired_samples`.
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

    /// Margin of error at a `1 - significance_level` confidence level.
    ///
    /// # Errors
    ///
    /// * [`ElinorError::InvalidArgument`] if the significance level is not in the range `(0, 1]`.
    pub fn margin_of_error(&self, significance_level: f64) -> Result<f64, ElinorError> {
        if significance_level <= 0.0 || significance_level > 1.0 {
            return Err(ElinorError::InvalidArgument(
                "The significance level must be in the range (0, 1].".to_string(),
            ));
        }
        Ok(self
            .scaled_t_dist
            .inverse_cdf(1.0 - (significance_level / 2.0)))
    }

    /// Confidence interval at a `1 - significance_level` confidence level.
    ///
    /// # Errors
    ///
    /// * [`ElinorError::InvalidArgument`] if the significance level is not in the range `(0, 1]`.
    pub fn confidence_interval(&self, significance_level: f64) -> Result<(f64, f64), ElinorError> {
        let moe = self.margin_of_error(significance_level)?;
        Ok((self.mean - moe, self.mean + moe))
    }

    /// Returns true if the difference is significant at the given significance level.
    pub fn is_significant(&self, significance_level: f64) -> bool {
        self.p_value < significance_level
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

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

    #[test]
    fn test_paired_student_t_test_margin_of_error_invalid_argument() {
        let result = PairedStudentTTest::compute(vec![(1.0, 2.0), (3.0, 4.5)]);
        let result = result.unwrap();
        let moe = result.margin_of_error(0.0);
        assert_eq!(
            moe.unwrap_err(),
            ElinorError::InvalidArgument(
                "The significance level must be in the range (0, 1].".to_string()
            )
        );
        let moe = result.margin_of_error(1.0).unwrap();
        assert_abs_diff_eq!(moe, 0.0, epsilon = 1e-4);
    }

    #[test]
    fn test_paired_student_t_test_confidence_interval_invalid_argument() {
        let result = PairedStudentTTest::compute(vec![(1.0, 2.0), (3.0, 4.5)]);
        let result = result.unwrap();
        let ci = result.confidence_interval(0.0);
        assert_eq!(
            ci.unwrap_err(),
            ElinorError::InvalidArgument(
                "The significance level must be in the range (0, 1].".to_string()
            )
        );
        let (ci95_btm, ci95_top) = result.confidence_interval(1.0).unwrap();
        assert_abs_diff_eq!(ci95_btm, result.mean(), epsilon = 1e-4);
        assert_abs_diff_eq!(ci95_top, result.mean(), epsilon = 1e-4);
    }
}
