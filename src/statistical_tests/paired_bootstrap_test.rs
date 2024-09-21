//! Paired bootstrap test.

use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;

use crate::errors::ElinorError;

/// Paired bootstrap test.
#[derive(Debug, Clone, Copy)]
pub struct BootstrapTested {
    n_resamples: usize,
    random_state: u64,
    mean: f64,
    p_value: f64,
}

impl BootstrapTested {}

#[derive(Debug, Clone, Copy)]
pub struct BootstrapTester {
    n_resamples: usize,
    random_state: Option<u64>,
}

impl BootstrapTester {
    pub fn new() -> Self {
        Self {
            n_resamples: 9999,
            random_state: None,
        }
    }

    pub fn with_resamples(mut self, n_resamples: usize) -> Self {
        self.n_resamples = n_resamples;
        self
    }

    pub fn with_random_state(mut self, random_state: u64) -> Self {
        self.random_state = Some(random_state);
        self
    }

    pub fn test<I>(&self, samples: I) -> Result<BootstrapTested, ElinorError>
    where
        I: IntoIterator<Item = f64>,
    {
        let mut rng = match self.random_state {
            Some(seed) => StdRng::seed_from_u64(seed),
            None => StdRng::from_entropy(),
        };
        let samples: Vec<f64> = samples.into_iter().collect();
        let mut count = 0;
        for _ in 0..self.n_resamples {
            let resampled: Vec<f64> = (0..samples.len())
                .map(|_| samples[rng.gen_range(0..samples.len())])
                .collect();
            let mean = resample.iter().sum::<f64>() / resample.len() as f64;
            if mean >= 0.0 {
                count += 1;
            }
        }
        let p_value = count as f64 / self.n_resamples as f64;
    }
}
