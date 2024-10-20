//! Randomized Tukey HSD test.
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::Rng;
use rand::SeedableRng;
use statrs::statistics::Statistics;

use crate::errors::ElinorError;

/// Randomized Tukey HSD test for comparing two or more systems.
///
/// When comparing two systems, it is equivalent to Fisher's randomization test.
///
/// # Examples
///
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use approx::assert_abs_diff_eq;
/// use elinor::statistical_tests::RandomizedTukeyHsdTest;
///
/// let a = vec![0.70, 0.30, 0.20, 0.60, 0.40];
/// let b = vec![0.50, 0.10, 0.00, 0.20, 0.40];
/// let c = vec![0.00, 0.00, 0.20, 0.10, 0.30];
///
/// let tupled_samples = a
///     .iter()
///     .zip(b.iter())
///     .zip(c.iter())
///     .map(|((&a, &b), &c)| [a, b, c]);
/// let result = RandomizedTukeyHsdTest::from_tupled_samples(tupled_samples, 3)?;
/// assert_eq!(result.n_systems(), 3);
/// assert_eq!(result.n_topics(), 5);
///
/// let p_values = result.p_values();
/// assert!((0.0..=1.0).contains(&p_values[0][1]));  // a vs. b
/// assert!((0.0..=1.0).contains(&p_values[0][2]));  // a vs. c
/// assert!((0.0..=1.0).contains(&p_values[1][2]));  // b vs. c
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
/// * Tetsuya Sakai.
///   [Laboratory Experiments in Information Retrieval: Sample Sizes, Effect Sizes, and Statistical Power](https://doi.org/10.1007/978-981-13-1199-4).
///   Chapter 4. Springer, 2018.
#[derive(Debug, Clone)]
pub struct RandomizedTukeyHsdTest {
    n_systems: usize,
    n_topics: usize,
    n_iters: usize,
    random_state: u64,
    p_values: Vec<Vec<f64>>,
}

impl RandomizedTukeyHsdTest {
    /// Creates a new randomized Tukey HSD test.
    /// from samples $`x_{ij}`$ for $`i \in [1,m]`$ systems and $`j \in [1,n]`$ topics.
    ///
    /// # Arguments
    ///
    /// * `samples` - Iterator of tupled samples, where each record is an array of $`m`$ system samples for a topic.
    /// * `n_systems` - Number of systems, $`m`$.
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

    /// Number of systems, $`m`$.
    pub const fn n_systems(&self) -> usize {
        self.n_systems
    }

    /// Number of topics, $`n`$.
    pub const fn n_topics(&self) -> usize {
        self.n_topics
    }

    /// Number of iterations.
    pub const fn n_iters(&self) -> usize {
        self.n_iters
    }

    /// Random state.
    pub const fn random_state(&self) -> u64 {
        self.random_state
    }

    /// p-values for all combinations of systems,
    /// returning a matrix of size $`m \times m`$,
    /// where $`m`$ is the number of systems.
    ///
    /// The $`(i, j)`$-th element has the p-value
    /// between the $`i`$-th and $`j`$-th systems.
    /// The diagonal elements are always one.
    pub fn p_values(&self) -> Vec<Vec<f64>> {
        self.p_values.clone()
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
            .map(|topic| {
                let topic = topic.as_ref();
                if topic.len() != self.n_systems {
                    return Err(ElinorError::InvalidArgument(
                        "The length of each sample must be equal to the number of systems."
                            .to_string(),
                    ));
                }
                Ok(topic.to_vec())
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
        let means: Vec<_> = (0..self.n_systems)
            .map(|i| samples.iter().map(|sample| sample[i]).sum::<f64>() / n_samples)
            .collect();

        // Compute the differences between the means of each pair of systems.
        // i >= j, so the upper triangle is filled with zeros.
        let mut diffs = vec![vec![0_f64; self.n_systems]; self.n_systems];
        for i in 0..self.n_systems {
            for j in (i + 1)..self.n_systems {
                diffs[i][j] = means[i] - means[j];
            }
        }

        let mut counts = vec![vec![0_usize; self.n_systems]; self.n_systems];
        for _ in 0..self.n_iters {
            let mut shuffled_samples = Vec::with_capacity(samples.len());
            for sample in &samples {
                let mut shuffled_sample = sample.clone();
                shuffled_sample.shuffle(&mut rng);
                shuffled_samples.push(shuffled_sample);
            }

            let shuffled_means: Vec<_> = (0..self.n_systems)
                .map(|i| shuffled_samples.iter().map(|sample| sample[i]).sum::<f64>() / n_samples)
                .collect();

            let shuffled_diff = shuffled_means.as_slice().max() - shuffled_means.as_slice().min();
            for i in 0..self.n_systems {
                for j in (i + 1)..self.n_systems {
                    if shuffled_diff >= diffs[i][j].abs() {
                        counts[i][j] += 1;
                    }
                }
            }
        }

        let mut p_values = vec![vec![1_f64; self.n_systems]; self.n_systems];
        for i in 0..self.n_systems {
            for j in (i + 1)..self.n_systems {
                p_values[i][j] = counts[i][j] as f64 / self.n_iters as f64;
                p_values[j][i] = p_values[i][j];
            }
        }

        Ok(RandomizedTukeyHsdTest {
            n_systems: self.n_systems,
            n_topics: samples.len(),
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
