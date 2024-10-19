//! Tukey HSD test.

use crate::errors::ElinorError;
use crate::statistical_tests::TwoWayAnovaWithoutReplication;

/// Tukey HSD test.
///
/// It can be used to compare three or more systems.
///
/// # Notes
///
/// This struct does not provide p-values and only provides effect sizes
/// because we are unaware of Rust libraries that can calculate the studentized range distribution.
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

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_tukey_hsd_test_sakai_book_15() {
        // From Table 5.1 in Sakai's book, "情報アクセス評価方法論".
        let a = vec![
            0.70, 0.30, 0.20, 0.60, 0.40, 0.40, 0.00, 0.70, 0.10, 0.30, //
            0.50, 0.40, 0.00, 0.60, 0.50, 0.30, 0.10, 0.50, 0.20, 0.10,
        ];
        let b = vec![
            0.50, 0.10, 0.00, 0.20, 0.40, 0.30, 0.00, 0.50, 0.30, 0.30, //
            0.40, 0.40, 0.10, 0.40, 0.20, 0.10, 0.10, 0.60, 0.30, 0.20,
        ];
        let c = vec![
            0.00, 0.00, 0.20, 0.10, 0.30, 0.30, 0.10, 0.20, 0.40, 0.40, //
            0.40, 0.30, 0.30, 0.20, 0.20, 0.20, 0.10, 0.50, 0.40, 0.30,
        ];
        let tupled_samples = a
            .iter()
            .zip(b.iter())
            .zip(c.iter())
            .map(|((&a, &b), &c)| [a, b, c]);
        let stat = TukeyHsdTest::from_tupled_samples(tupled_samples, 3).unwrap();
        assert_eq!(stat.n_systems(), 3);

        // Comparing with the values in 情報アクセス評価方法論.
        let effect_sizes = stat.effect_sizes();
        assert_eq!(effect_sizes.len(), 3);
        assert_eq!(effect_sizes[0].len(), 3);
        assert_eq!(effect_sizes[1].len(), 3);
        assert_eq!(effect_sizes[2].len(), 3);
        assert_abs_diff_eq!(effect_sizes[0][0], 0.0000, epsilon = 1e-4);
        assert_abs_diff_eq!(effect_sizes[0][1], 0.5070, epsilon = 1e-4);
        assert_abs_diff_eq!(effect_sizes[0][2], 0.6760, epsilon = 1e-4);
        assert_abs_diff_eq!(effect_sizes[1][0], -0.5070, epsilon = 1e-4);
        assert_abs_diff_eq!(effect_sizes[1][1], 0.0000, epsilon = 1e-4);
        assert_abs_diff_eq!(effect_sizes[1][2], 0.1690, epsilon = 1e-4);
        assert_abs_diff_eq!(effect_sizes[2][0], -0.6760, epsilon = 1e-4);
        assert_abs_diff_eq!(effect_sizes[2][1], -0.1690, epsilon = 1e-4);
        assert_abs_diff_eq!(effect_sizes[2][2], 0.0000, epsilon = 1e-4);
    }
}
