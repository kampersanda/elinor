//! Bootstrap test.

use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;

use crate::errors::ElinorError;
use crate::statistical_tests::student_t_test::compute_t_stat;

/// Bootstrap test.
///
/// # Examples
///
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use approx::assert_abs_diff_eq;
/// use elinor::statistical_tests::BootstrapTest;
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
/// let result = BootstrapTest::from_paired_samples(paired_samples)?;
///
/// // Various statistics can be obtained.
/// assert_abs_diff_eq!(result.mean(), 0.0750, epsilon = 1e-4);
/// assert_abs_diff_eq!(result.var(), 0.0251, epsilon = 1e-4);
/// assert_abs_diff_eq!(result.effect_size(), 0.473, epsilon = 1e-3);
/// assert!((0.0..=1.0).contains(&result.p_value()));
/// # Ok(())
/// # }
/// ```
///
/// # References
///
/// * Bradley Efron and R.J. Tibshirani.
///   [An Introduction to the Bootstrap](https://doi.org/10.1201/9780429246593).
///   Chapman & Hall/CRC, 1994.
/// * Tetsuya Sakai.
///   [Evaluating evaluation metrics based on the bootstrap](https://doi.org/10.1145/1148170.1148261).
///   SIGIR 2006.
#[derive(Debug, Clone, Copy)]
pub struct BootstrapTest {
    n_resamples: usize,
    random_state: u64,
    mean: f64,
    var: f64,
    p_value: f64,
}

impl BootstrapTest {
    /// Computes a bootstrap test for the samples.
    ///
    /// It uses the default parameters defined in [`BootstrapTester`].
    /// To customize the parameters, use [`BootstrapTester`].
    ///
    /// # Errors
    ///
    /// See [`BootstrapTester::test`].
    pub fn from_samples<I>(samples: I) -> Result<Self, ElinorError>
    where
        I: IntoIterator<Item = f64>,
    {
        BootstrapTester::new().test(samples)
    }

    /// Computes a paired bootstrap test for differences between paired samples.
    ///
    /// It uses the default parameters defined in [`BootstrapTester`].
    /// To customize the parameters, use [`BootstrapTester`].
    ///
    /// # Errors
    ///
    /// See [`BootstrapTester::test`].
    pub fn from_paired_samples<I>(paired_samples: I) -> Result<Self, ElinorError>
    where
        I: IntoIterator<Item = (f64, f64)>,
    {
        BootstrapTester::new().test_for_paired_samples(paired_samples)
    }

    /// Mean.
    pub const fn mean(&self) -> f64 {
        self.mean
    }

    /// Unbiased population variance.
    pub const fn var(&self) -> f64 {
        self.var
    }

    /// Effect size.
    pub fn effect_size(&self) -> f64 {
        self.mean / self.var.sqrt()
    }

    /// p-value.
    pub const fn p_value(&self) -> f64 {
        self.p_value
    }

    /// Number of resamples.
    pub const fn n_resamples(&self) -> usize {
        self.n_resamples
    }

    /// Random state used for the resampling.
    pub const fn random_state(&self) -> u64 {
        self.random_state
    }
}

/// Bootstrap tester.
///
/// # Default parameters
///
/// * `n_resamples`: `1000`
/// * `random_state`: `None`
///
/// # References
///
/// The default parameter `n_resamples = 1000` is based on the paper,
/// [Tetsuya Sakai. Evaluation with informational and navigational intents. WWW 2012](https://doi.org/10.1145/2187836.2187904).
#[derive(Debug, Clone, Copy)]
pub struct BootstrapTester {
    n_resamples: usize,
    random_state: Option<u64>,
}

impl Default for BootstrapTester {
    fn default() -> Self {
        Self::new()
    }
}

impl BootstrapTester {
    /// Creates a new bootstrap tester.
    pub const fn new() -> Self {
        Self {
            n_resamples: 1000,
            random_state: None,
        }
    }

    /// Sets the number of resamples.
    pub const fn with_n_resamples(mut self, n_resamples: usize) -> Self {
        self.n_resamples = n_resamples;
        self
    }

    /// Sets the random state.
    pub const fn with_random_state(mut self, random_state: u64) -> Self {
        self.random_state = Some(random_state);
        self
    }

    /// Computes a bootstrap test for the samples.
    ///
    /// # Errors
    ///
    /// * [`ElinorError::InvalidArgument`] if the input does not have at least two samples.
    /// * [`ElinorError::Uncomputable`] if the variance is zero.
    pub fn test<I>(&self, samples: I) -> Result<BootstrapTest, ElinorError>
    where
        I: IntoIterator<Item = f64>,
    {
        let samples: Vec<f64> = samples.into_iter().collect();
        if samples.len() <= 1 {
            return Err(ElinorError::InvalidArgument(
                "The input must have at least two samples.".to_string(),
            ));
        }

        // Prepare the random number generator.
        let random_state = self
            .random_state
            .map_or_else(|| rand::thread_rng().gen(), |seed| seed);
        let mut rng = StdRng::seed_from_u64(random_state);

        // Compute the t-statistic for the original samples.
        let (t_stat, mean, var) = compute_t_stat(&samples)?;

        // Shift the samples to have a mean of zero.
        let samples: Vec<f64> = samples.iter().map(|x| x - mean).collect();

        // Perform the bootstrap test.
        let mut count: usize = 0;
        for _ in 0..self.n_resamples {
            let resampled: Vec<f64> = (0..samples.len())
                .map(|_| samples[rng.gen_range(0..samples.len())])
                .collect();
            // If samples.len() is small, the variance may be zero.
            // In that unfortunate case, we skip the counting.
            let (resampled_t_stat, _, _) = compute_t_stat(&resampled).unwrap_or((0.0, 0.0, 0.0));
            if resampled_t_stat.abs() >= t_stat.abs() {
                count += 1;
            }
        }
        let p_value = count as f64 / self.n_resamples as f64;

        Ok(BootstrapTest {
            n_resamples: self.n_resamples,
            random_state,
            mean,
            var,
            p_value,
        })
    }

    /// Computes a paired bootstrap test for differences between paired samples.
    ///
    /// # Errors
    ///
    /// See [`BootstrapTester::test`].
    pub fn test_for_paired_samples<I>(
        &self,
        paired_samples: I,
    ) -> Result<BootstrapTest, ElinorError>
    where
        I: IntoIterator<Item = (f64, f64)>,
    {
        let diffs = paired_samples.into_iter().map(|(x, y)| x - y);
        self.test(diffs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::relative_eq;

    #[test]
    fn test_bootstrap_test_from_samples_empty() {
        let samples = vec![];
        let result = BootstrapTest::from_samples(samples);
        assert_eq!(
            result.unwrap_err(),
            ElinorError::InvalidArgument("The input must have at least two samples.".to_string())
        );
    }

    #[test]
    fn test_bootstrap_test_from_samples_single() {
        let samples = vec![1.0];
        let result = BootstrapTest::from_samples(samples);
        assert_eq!(
            result.unwrap_err(),
            ElinorError::InvalidArgument("The input must have at least two samples.".to_string())
        );
    }

    #[test]
    fn test_bootstrap_test_from_samples_zero_variance() {
        let samples = vec![1.0, 1.0];
        let result = BootstrapTest::from_samples(samples);
        assert_eq!(
            result.unwrap_err(),
            ElinorError::Uncomputable("The variance is zero.".to_string())
        );
    }

    #[test]
    fn test_bootstrap_tester_with_parameters() {
        let tester = BootstrapTester::new()
            .with_n_resamples(334)
            .with_random_state(42);
        let samples = (0..10).map(|x| x as f64).collect::<Vec<f64>>();
        let result = tester.test(samples).unwrap();
        assert_eq!(result.n_resamples(), 334);
        assert_eq!(result.random_state(), 42);
    }

    #[test]
    fn test_bootstrap_tester_with_random_state_consistency() {
        let samples = (0..10).map(|x| x as f64).collect::<Vec<f64>>();
        let p_values: Vec<f64> = (0..10)
            .map(|_| {
                let tester = BootstrapTester::new().with_random_state(42);
                let result = tester.test(samples.clone()).unwrap();
                result.p_value()
            })
            .collect();
        let x = p_values[0];
        assert!(p_values.iter().all(|&y| relative_eq!(x, y)));
    }
}
