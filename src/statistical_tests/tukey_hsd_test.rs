//! Tukey HSD test.

use crate::errors::ElinorError;
use crate::statistical_tests::TwoWayAnovaWithoutReplication;

/// Tukey HSD test.
///
/// It can be used to compare three or more systems.
///
/// # Notes
///
/// We are unaware of Rust libraries that can calculate the studentized range distribution,
/// so this struct does not provide p-values. Only effect sizes are obtained.
/// You can use [`RandomizedTukeyHsdTest`](crate::statistical_tests::RandomizedTukeyHsdTest) instead if you need p-values.
#[derive(Debug, Clone)]
pub struct TukeyHsdTest {
    anova: TwoWayAnovaWithoutReplication,
}

impl TukeyHsdTest {
    /// Creates a new Tukey HSD test.
    pub fn from_tupled_samples<I, S>(samples: I, n_systems: usize) -> Result<Self, ElinorError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<[f64]>,
    {
        let anova = TwoWayAnovaWithoutReplication::from_tupled_samples(samples, n_systems)?;
        Ok(Self { anova })
    }

    /// Number of systems.
    pub const fn n_systems(&self) -> usize {
        self.anova.n_systems()
    }

    /// Effect sizes for all combinations of systems,
    /// returning a matrix of size $`m \times m`$.
    /// where $`m`$ is the number of systems.
    ///
    /// The $`(i, j)`$-th element is $`\text{ES}_{ij}`$.
    /// The diagonal elements are always zero.
    ///
    /// # Formula
    ///
    /// ```math
    /// \text{ES}_{ij} = \frac{\bar{x}_{i*} - \bar{x}_{j*}}{\sqrt{V_E}}
    /// ```
    ///
    /// where
    ///
    /// * $`\bar{x}_{i*}`$ is [the mean score of the $`i`$-th system](TwoWayAnovaWithoutReplication::system_means), and
    /// * $`V_E`$ is [the residual variance](TwoWayAnovaWithoutReplication::residual_variance).
    pub fn effect_sizes(&self) -> Vec<Vec<f64>> {
        let system_means = self.anova.system_means();
        let residual_stddev = self.anova.residual_variance().sqrt();
        let mut effect_sizes = vec![vec![0.0; self.n_systems()]; self.n_systems()];
        for i in 0..self.n_systems() {
            for j in (i + 1)..self.n_systems() {
                let diff = system_means[i] - system_means[j];
                let effect_size = diff / residual_stddev;
                effect_sizes[i][j] = effect_size;
                effect_sizes[j][i] = -effect_size;
            }
        }
        effect_sizes
    }
}
