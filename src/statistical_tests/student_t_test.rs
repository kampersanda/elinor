//! Two-sided Student's t-test

use statrs::distribution::ContinuousCDF;
use statrs::distribution::StudentsT;
use statrs::statistics::Statistics;

use crate::errors::ElinorError;
use crate::errors::Result;

/// Two-sided Student's t-test.
///
/// # Examples
///
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use approx::assert_abs_diff_eq;
/// use elinor::statistical_tests::StudentTTest;
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
/// let samples = a.into_iter().zip(b.into_iter()).map(|(x, y)| x - y);
/// let result = StudentTTest::from_samples(samples)?;
///
/// // Various statistics can be obtained.
/// assert_abs_diff_eq!(result.mean(), 0.0750, epsilon = 1e-4);
/// assert_abs_diff_eq!(result.variance(), 0.0251, epsilon = 1e-4);
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
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct StudentTTest {
    mean: f64,
    variance: f64,
    t_stat: f64,
    p_value: f64,
    scaled_t_dist: StudentsT,
}

impl StudentTTest {
    /// Computes a Student's t-test for $`n`$ samples $`x_{1},x_{2},\dots,x_{n}`$.
    ///
    /// # Errors
    ///
    /// * [`ElinorError::InvalidArgument`] if the input does not have at least two samples.
    /// * [`ElinorError::Uncomputable`] if the variance is zero.
    pub fn from_samples<I>(samples: I) -> Result<Self>
    where
        I: IntoIterator<Item = f64>,
    {
        let samples: Vec<f64> = samples.into_iter().collect();
        if samples.len() <= 1 {
            return Err(ElinorError::InvalidArgument(
                "The input must have at least two samples.".to_string(),
            ));
        }
        let (t_stat, mean, variance) = compute_t_stat(&samples)?;
        let n = samples.len() as f64;
        let t_dist = StudentsT::new(0.0, 1.0, n - 1.0).unwrap();
        let p_value = t_dist.sf(t_stat.abs()) * 2.0; // two-tailed
        let scaled_t_dist = StudentsT::new(0.0, (variance / n).sqrt(), n - 1.0).unwrap();
        Ok(Self {
            mean,
            variance,
            t_stat,
            p_value,
            scaled_t_dist,
        })
    }

    /// Mean of the samples.
    ///
    /// # Formula
    ///
    /// ```math
    /// \bar{x} = \frac{1}{n} \sum_{i=1}^{n} x_{i}
    /// ```
    pub const fn mean(&self) -> f64 {
        self.mean
    }

    /// Unbiased population variance.
    ///
    /// # Formula
    ///
    /// ```math
    /// V = \frac{1}{n-1} \sum_{i=1}^{n} (x_{i} - \bar{x})^{2}
    /// ```
    pub const fn variance(&self) -> f64 {
        self.variance
    }

    /// Sample effect size.
    ///
    /// # Formula
    ///
    /// ```math
    /// \text{ES} = \frac{\bar{x}}{\sqrt{V}}
    /// ```
    pub fn effect_size(&self) -> f64 {
        self.mean / self.variance.sqrt()
    }

    /// t-statistic.
    ///
    /// # Formula
    ///
    /// ```math
    /// t_0 = \frac{\bar{x}}{\sqrt{V/n}}
    /// ```
    pub const fn t_stat(&self) -> f64 {
        self.t_stat
    }

    /// p-value for the two-sided test.
    ///
    /// # Formula
    ///
    /// ```math
    /// p = P(|t_0| > t_{\alpha/2}(n-1))
    /// ```
    ///
    /// where $`t_{\alpha/2}(n-1)`$ is the $`1 - \alpha/2`$ quantile of the Student's t-distribution
    /// with $`n-1`$ degrees of freedom.
    pub const fn p_value(&self) -> f64 {
        self.p_value
    }

    /// Margin of error at a given significance level $`\alpha`$.
    ///
    /// # Errors
    ///
    /// * [`ElinorError::InvalidArgument`] if the significance level is not in the range `(0, 1]`.
    ///
    /// # Formula
    ///
    /// ```math
    /// \text{MOE} = t_{\alpha/2}(n-1) \sqrt{\frac{V}{n}}
    /// ```
    pub fn margin_of_error(&self, significance_level: f64) -> Result<f64> {
        if significance_level <= 0.0 || significance_level > 1.0 {
            return Err(ElinorError::InvalidArgument(
                "The significance level must be in the range (0, 1].".to_string(),
            ));
        }
        Ok(self
            .scaled_t_dist
            .inverse_cdf(1.0 - (significance_level / 2.0)))
    }

    /// Confidence interval at a given significance level $`\alpha`$.
    ///
    /// # Errors
    ///
    /// * [`ElinorError::InvalidArgument`] if the significance level is not in the range `(0, 1]`.
    ///
    /// # Formula
    ///
    /// ```math
    /// \text{CI} = [\bar{x} - \text{MOE}, \bar{x} + \text{MOE}]
    /// ```
    pub fn confidence_interval(&self, significance_level: f64) -> Result<(f64, f64)> {
        let moe = self.margin_of_error(significance_level)?;
        Ok((self.mean - moe, self.mean + moe))
    }
}

/// Computes a t-statistic, returning:
///
/// * the t-statistic,
/// * the mean, and
/// * the unbiased population variance.
///
/// # Errors
///
/// * [`ElinorError::Uncomputable`] if the variance is zero.
pub fn compute_t_stat(samples: &[f64]) -> Result<(f64, f64, f64)> {
    let mean = Statistics::mean(samples);
    let variance = Statistics::variance(samples);
    if variance == 0.0 {
        return Err(ElinorError::Uncomputable(
            "The variance is zero.".to_string(),
        ));
    }
    let n = samples.len() as f64;
    let t_stat = mean / (variance / n).sqrt();
    Ok((t_stat, mean, variance))
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_student_t_test_compute_empty() {
        let result = StudentTTest::from_samples(Vec::<f64>::new());
        assert_eq!(
            result.unwrap_err(),
            ElinorError::InvalidArgument("The input must have at least two samples.".to_string())
        );
    }

    #[test]
    fn test_student_t_test_compute_one_sample() {
        let result = StudentTTest::from_samples(vec![1.0]);
        assert_eq!(
            result.unwrap_err(),
            ElinorError::InvalidArgument("The input must have at least two samples.".to_string())
        );
    }

    #[test]
    fn test_student_t_test_compute_two_samples() {
        let result = StudentTTest::from_samples(vec![1.0, 3.0]);
        let expected = (1.0 + 3.0) / 2.0;
        assert_abs_diff_eq!(result.unwrap().mean(), expected, epsilon = 1e-4);
    }

    #[test]
    fn test_student_t_test_compute_zero_variance() {
        let result = StudentTTest::from_samples(vec![1.0, 1.0]);
        assert_eq!(
            result.unwrap_err(),
            ElinorError::Uncomputable("The variance is zero.".to_string())
        );
    }

    #[test]
    fn test_student_t_test_margin_of_error_invalid_argument() {
        let result = StudentTTest::from_samples(vec![1.0, 1.5]);
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
    fn test_student_t_test_confidence_interval_invalid_argument() {
        let result = StudentTTest::from_samples(vec![1.0, 1.5]);
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
