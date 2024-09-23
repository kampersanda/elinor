//! Randomized Tukey HSD test.
use std::collections::HashMap;

use itertools::Itertools;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::Rng;
use rand::SeedableRng;
use statrs::statistics::Statistics;

use crate::errors::ElinorError;

/// Randomized Tukey HSD test.
///
/// It can be used to compare two or more systems.
/// When comparing two systems, it is equivalent to Fisher's randomization test.
///
/// # Examples
///
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use approx::assert_abs_diff_eq;
/// use elinor::statistical_tests::RandomizedTukeyHsdTest;
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
/// let c = vec![
///     0.00, 0.00, 0.20, 0.10, 0.30, 0.30, 0.10, 0.20, 0.40, 0.40, //
///     0.40, 0.30, 0.30, 0.20, 0.20, 0.20, 0.10, 0.50, 0.40, 0.30,
/// ];
///
/// // Comparing two systems, equivalent to Fisher's randomization test.
/// let tupled_samples = a.iter().zip(b.iter()).map(|(&a, &b)| [a, b]);
/// let result = RandomizedTukeyHsdTest::from_tupled_samples(tupled_samples, 2)?;
/// assert!((0.0..=1.0).contains(&result.p_value(0, 1)?));
///
/// // Comparing three systems.
/// let tupled_samples = a
///     .iter()
///     .zip(b.iter())
///     .zip(c.iter())
///     .map(|((&a, &b), &c)| [a, b, c]);
/// let result = RandomizedTukeyHsdTest::from_tupled_samples(tupled_samples, 3)?;
/// assert!((0.0..=1.0).contains(&result.p_value(0, 1)?));  // a vs. b
/// assert!((0.0..=1.0).contains(&result.p_value(0, 2)?));  // a vs. c
/// assert!((0.0..=1.0).contains(&result.p_value(1, 2)?));  // b vs. c
/// # Ok(())
/// # }
/// ```
///
/// # References
///
/// * Mark D. Smucker, James Allan, and Ben Carterette.
///   [A comparison of statistical significance tests for information retrieval evaluation](https://doi.org/10.1145/1321440.1321528).
///   CIKM 2007.
/// * Benjamin A. Carterette.
///   [Multiple testing in statistical analysis of systems-based information retrieval experiments](https://doi.org/10.1145/2094072.2094076).
///   TOIS 2012.
#[derive(Debug, Clone)]
pub struct RandomizedTukeyHsdTest {
    n_systems: usize,
    n_iters: usize,
    random_state: u64,
    p_values: HashMap<(usize, usize), f64>,
}

impl RandomizedTukeyHsdTest {
    /// Creates a new randomized Tukey HSD test.
    ///
    /// # Errors
    ///
    /// See [`RandomizedTukeyHsdTester::test`].
    pub fn from_tupled_samples<I, S>(samples: I, n_systems: usize) -> Result<Self, ElinorError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<[f64]>,
    {
        RandomizedTukeyHsdTester::new(n_systems).test(samples)
    }

    /// Number of systems.
    pub const fn n_systems(&self) -> usize {
        self.n_systems
    }

    /// Number of iterations.
    pub const fn n_iters(&self) -> usize {
        self.n_iters
    }

    /// Random state.
    pub const fn random_state(&self) -> u64 {
        self.random_state
    }

    /// p-value between systems i and j.
    ///
    /// # Errors
    ///
    /// * [`ElinorError::InvalidArgument`] if the indices are out of bounds or the indices are the same.
    pub fn p_value(&self, i: usize, j: usize) -> Result<f64, ElinorError> {
        if i >= self.n_systems || j >= self.n_systems {
            return Err(ElinorError::InvalidArgument(
                "The indices must be less than the number of systems.".to_string(),
            ));
        }
        if i == j {
            return Err(ElinorError::InvalidArgument(
                "The indices must be different.".to_string(),
            ));
        }
        let (i, j) = if i < j { (i, j) } else { (j, i) };
        Ok(*self.p_values.get(&(i, j)).unwrap())
    }

    /// p-values for all pairs of systems, returning `(i, j, p-value)` such that `i < j`.
    ///
    /// The results are sorted by `(i, j)`.
    pub fn p_values(&self) -> Vec<(usize, usize, f64)> {
        let mut p_values = self
            .p_values
            .iter()
            .map(|(&(i, j), &p)| (i, j, p))
            .collect_vec();
        p_values.sort_unstable_by(|(ai, aj, _), (bi, bj, _)| ai.cmp(bi).then(aj.cmp(bj)));
        p_values
    }
}

/// Randomized Tukey HSD tester.
///
/// # Default parameters
///
/// * `n_iters`: `10000`
/// * `random_state`: `None`
#[derive(Debug, Clone)]
pub struct RandomizedTukeyHsdTester {
    n_systems: usize,
    n_iters: usize,
    random_state: Option<u64>,
}

impl RandomizedTukeyHsdTester {
    /// Creates a new randomized Tukey HSD tester.
    pub const fn new(n_systems: usize) -> Self {
        Self {
            n_systems,
            n_iters: 10000,
            random_state: None,
        }
    }

    /// Sets the number of iterations.
    ///
    /// If the input is less than `1`, it is modified to `1`.
    pub fn with_n_iters(mut self, n_iters: usize) -> Self {
        self.n_iters = n_iters.max(1);
        self
    }

    /// Sets the random state.
    pub const fn with_random_state(mut self, random_state: u64) -> Self {
        self.random_state = Some(random_state);
        self
    }

    /// Computes a randomized Tukey HSD test for the samples.
    ///
    /// # Errors
    ///
    /// * [`ElinorError::InvalidArgument`] if the length of each sample is not equal to the number of systems.
    pub fn test<I, S>(&self, samples: I) -> Result<RandomizedTukeyHsdTest, ElinorError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<[f64]>,
    {
        let samples: Vec<Vec<f64>> = samples
            .into_iter()
            .map(|sample| {
                let sample = sample.as_ref();
                if sample.len() != self.n_systems {
                    return Err(ElinorError::InvalidArgument(
                        "The length of each sample must be equal to the number of systems."
                            .to_string(),
                    ));
                }
                Ok(sample.to_vec())
            })
            .collect::<Result<_, _>>()?;

        if samples.is_empty() {
            return Err(ElinorError::InvalidArgument(
                "The input must have at least one sample.".to_string(),
            ));
        }

        let n_samples = samples.len() as f64;

        // Prepare the random number generator.
        let random_state = self
            .random_state
            .map_or_else(|| rand::thread_rng().gen(), |seed| seed);
        let mut rng = StdRng::seed_from_u64(random_state);

        // Compute the means of each system.
        let means = (0..self.n_systems)
            .map(|i| samples.iter().map(|sample| sample[i]).sum::<f64>() / n_samples)
            .collect_vec();

        // All possible combinations of two systems.
        let combis = (0..self.n_systems)
            .combinations(2)
            .map(|c| (c[0], c[1]))
            .collect_vec();

        // Compute the differences between the means for each pair of systems.
        let diffs = combis
            .iter()
            .map(|&(a, b)| means[a] - means[b])
            .collect_vec();

        let mut counts = vec![0usize; diffs.len()];
        for _ in 0..self.n_iters {
            let mut shuffled_samples = Vec::with_capacity(samples.len());
            for sample in &samples {
                let mut shuffled_sample = sample.clone();
                shuffled_sample.shuffle(&mut rng);
                shuffled_samples.push(shuffled_sample);
            }

            let shuffled_means = (0..self.n_systems)
                .map(|i| shuffled_samples.iter().map(|sample| sample[i]).sum::<f64>() / n_samples)
                .collect_vec();

            let shuffled_diff = shuffled_means.as_slice().max() - shuffled_means.as_slice().min();
            for (&diff, count) in diffs.iter().zip(counts.iter_mut()) {
                if shuffled_diff >= diff.abs() {
                    *count += 1;
                }
            }
        }

        let p_values: HashMap<_, _> = combis
            .iter()
            .zip(counts.iter())
            .map(|(&(a, b), &count)| ((a, b), count as f64 / self.n_iters as f64))
            .collect();

        Ok(RandomizedTukeyHsdTest {
            n_systems: self.n_systems,
            n_iters: self.n_iters,
            random_state,
            p_values,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_randomized_tukey_hsd_test_from_tupled_samples_empty() {
        let samples: Vec<[f64; 2]> = vec![];
        let result = RandomizedTukeyHsdTest::from_tupled_samples(samples, 2);
        assert_eq!(
            result.unwrap_err(),
            ElinorError::InvalidArgument("The input must have at least one sample.".to_string())
        );
    }

    #[test]
    fn test_randomized_tukey_hsd_test_from_tupled_samples_single() {
        let samples = vec![[1.0, 2.0]];
        let result = RandomizedTukeyHsdTest::from_tupled_samples(samples, 2).unwrap();
        assert_eq!(result.n_systems(), 2);
    }

    #[test]
    fn test_randomized_tukey_hsd_test_from_tupled_samples_invalid_length() {
        let samples = vec![vec![1.0, 2.0], vec![3.0]];
        let result = RandomizedTukeyHsdTest::from_tupled_samples(samples, 2);
        assert_eq!(
            result.unwrap_err(),
            ElinorError::InvalidArgument(
                "The length of each sample must be equal to the number of systems.".to_string()
            )
        );
    }
}
