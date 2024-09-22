//! Randomized Tukey HSD test.
//!
//! https://doi.org/10.1145/2094072.2094076

use std::collections::HashMap;

use itertools::Itertools;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::Rng;
use rand::SeedableRng;
use statrs::statistics::Statistics;

use crate::errors::ElinorError;

/// Randomized Tukey HSD test.
#[derive(Debug, Clone)]
pub struct RandomizedTukeyHsdTest {
    n_systems: usize,
    n_iters: usize,
    random_state: u64,
    p_values: HashMap<(usize, usize), f64>,
}

impl RandomizedTukeyHsdTest {
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

    /// p-values.
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

    /// p-values.
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
    pub const fn with_n_iters(mut self, n_iters: usize) -> Self {
        self.n_iters = n_iters;
        self
    }

    /// Sets the random state.
    pub const fn with_random_state(mut self, random_state: u64) -> Self {
        self.random_state = Some(random_state);
        self
    }

    /// Computes a randomized Tukey HSD test for the samples.
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
