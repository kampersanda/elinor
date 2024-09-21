//! Bootstrap test.

use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;

use crate::errors::ElinorError;
use crate::statistical_tests::student_t_test::compute_t_stat;

/// Paired bootstrap test.
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
    /// # Errors
    ///
    /// See [`BootstrapTester::test`].
    pub fn from_paired_samples<I>(paired_samples: I) -> Result<Self, ElinorError>
    where
        I: IntoIterator<Item = (f64, f64)>,
    {
        let (a, b): (Vec<f64>, Vec<f64>) = paired_samples.into_iter().unzip();
        let samples = a.iter().zip(b.iter()).map(|(a, b)| a - b);
        BootstrapTester::new().test(samples)
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
#[derive(Debug, Clone, Copy)]
pub struct BootstrapTester {
    n_resamples: usize,
    random_state: Option<u64>,
}

impl BootstrapTester {
    /// Creates a new bootstrap tester.
    ///
    /// # Default parameters
    ///
    /// * `n_resamples`: 1000
    /// * `random_state`: None
    pub fn new() -> Self {
        Self {
            n_resamples: 1000,
            random_state: None,
        }
    }

    /// Sets the number of resamples.
    pub fn with_resamples(mut self, n_resamples: usize) -> Self {
        self.n_resamples = n_resamples;
        self
    }

    /// Sets the random state.
    pub fn with_random_state(mut self, random_state: u64) -> Self {
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

        let (t_stat, mean, var) = compute_t_stat(&samples)?;

        let random_state = match self.random_state {
            Some(seed) => seed,
            None => rand::thread_rng().gen(),
        };
        let mut rng = StdRng::seed_from_u64(random_state);

        let mut count: usize = 0;
        for _ in 0..self.n_resamples {
            let resampled: Vec<f64> = (0..samples.len())
                .map(|_| samples[rng.gen_range(0..samples.len())])
                .collect();
            let (resampled_t_stat, _, _) = compute_t_stat(&resampled)?;
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
}
