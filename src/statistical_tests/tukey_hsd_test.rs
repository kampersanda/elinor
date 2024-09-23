//!
//! https://codapi.org/embed/?sandbox=r&code=data%3A%3Bbase64%2CZZFNawMhEIbv%2FoohvSg1i%2B5HWkr3Vkrvm3uxqyZLErYYE%2FDS3150V5jgZZDhmXce9Tz9OOUC9ZMOjhFyDVdvLhLetzBSUb0IDqJqUq1T3aXaoipSXUiJ%2BK5gdqjfIL5D%2BVKwVaLOEh1CBUJbFCSK6KYQlehco45EcvmqWaLJEqJYL4tlspBrC1H8lnWR1j3wjBBto4BWXlXWqYuh6wdxWB8pHxpWsvv5dxp7%2BRY3aBuB7%2FjPkToofzSOasvhZEI%2FpAwOd3W%2BmX4YZ2c4bNM8I%2BQJPqcDtNUr2d9OJnwNH1TNd5ow%2BAOrRj87umQweM6NZZwnI%2Bhh3c44bB4nNuwf
use std::collections::HashMap;

use itertools::Itertools;
use statrs::distribution::ContinuousCDF;
use statrs::distribution::StudentsT;
use statrs::statistics::Statistics;

use crate::errors::ElinorError;

/// Tukey Hsd Test with Paired Observations.
#[derive(Debug, Clone)]
pub struct TukeyHsdTest {
    n_systems: usize,
    system_means: Vec<f64>,
    residual_var: f64,
    scaled_t_dist: StudentsT,
    t_stats: HashMap<(usize, usize), f64>,
    p_values: HashMap<(usize, usize), f64>,
    effect_sizes: HashMap<(usize, usize), f64>,
}

impl TukeyHsdTest {
    /// Creates a new Tukey HSD test.
    pub fn from_tupled_samples<I, S>(samples: I, n_systems: usize) -> Result<Self, ElinorError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<[f64]>,
    {
        let samples: Vec<Vec<f64>> = samples
            .into_iter()
            .map(|sample| {
                let sample = sample.as_ref();
                if sample.len() != n_systems {
                    return Err(ElinorError::InvalidArgument(
                        "The length of each sample must be equal to the number of systems."
                            .to_string(),
                    ));
                }
                Ok(sample.to_vec())
            })
            .collect::<Result<_, _>>()?;

        if samples.len() <= 1 {
            return Err(ElinorError::InvalidArgument(
                "The input must have at least two samples.".to_string(),
            ));
        }

        let n_samples = samples.len() as f64;

        // Mean of all samples (x_{..}).
        let overall_mean = samples.iter().flatten().mean();

        // Mean of each system (x_{i.*}).
        let system_means = (0..n_systems)
            .map(|j| samples.iter().map(|sample| sample[j]).sum::<f64>() / n_samples)
            .collect::<Vec<_>>();

        // Mean of each topic (x_{*.j}).
        let topic_means = samples
            .iter()
            .map(|sample| sample.mean())
            .collect::<Vec<_>>();

        // Residual sum of squares S_E.
        let s_e = samples
            .iter()
            .enumerate()
            .map(|(j, topic_samples)| {
                topic_samples
                    .iter()
                    .enumerate()
                    .map(|(i, &x_ij)| {
                        let x_i_dot = system_means[i];
                        let x_dot_j = topic_means[j];
                        (x_ij - x_i_dot - x_dot_j + overall_mean).powi(2)
                    })
                    .sum::<f64>()
            })
            .sum::<f64>();

        // Residual variance V_E.
        let freedom = ((n_systems - 1) * (samples.len() - 1)) as f64;
        let v_e = s_e / freedom;

        let mut t_stats = HashMap::new();
        let mut p_values = HashMap::new();
        let mut effect_sizes = HashMap::new();
        let scale = (v_e / n_samples).sqrt();

        for combi in (0..n_systems).combinations(2) {
            let ai = combi[0];
            let bi = combi[1];
            let t_stat = (system_means[ai] - system_means[bi]) / scale;
            let t_dist = StudentsT::new(0.0, 1.0, freedom).unwrap();
            let p_value = t_dist.sf(t_stat.abs()) * 2.0; // two-tailed
            let effect_size = (system_means[ai] - system_means[bi]) / v_e.sqrt();
            t_stats.insert((ai, bi), t_stat);
            p_values.insert((ai, bi), p_value);
            effect_sizes.insert((ai, bi), effect_size);
        }

        Ok(Self {
            n_systems,
            system_means,
            residual_var: v_e,
            scaled_t_dist: StudentsT::new(0.0, scale, freedom).unwrap(),
            t_stats,
            p_values,
            effect_sizes,
        })
    }

    /// Number of systems.
    pub const fn n_systems(&self) -> usize {
        self.n_systems
    }

    /// Means of each system.
    pub fn system_means(&self) -> Vec<f64> {
        self.system_means.clone()
    }

    /// Residual variance.
    pub fn residual_var(&self) -> f64 {
        self.residual_var
    }

    /// Effect size between systems i and j.
    pub fn effect_size(&self, i: usize, j: usize) -> Result<f64, ElinorError> {
        self.check_indices(i, j)?;
        let (i, j) = if i < j { (i, j) } else { (j, i) };
        Ok(*self.effect_sizes.get(&(i, j)).unwrap())
    }

    /// Effect sizes for all pairs of systems, returning `(i, j, effect size)` such that `i < j`.
    ///
    /// The results are sorted by `(i, j)`.
    pub fn effect_sizes(&self) -> Vec<(usize, usize, f64)> {
        let mut effect_sizes = self
            .effect_sizes
            .iter()
            .map(|(&(i, j), &effect_size)| (i, j, effect_size))
            .collect_vec();
        effect_sizes.sort_unstable_by(|(ai, aj, _), (bi, bj, _)| ai.cmp(bi).then(aj.cmp(bj)));
        effect_sizes
    }

    /// t-statistic between systems i and j.
    pub fn t_stat(&self, i: usize, j: usize) -> Result<f64, ElinorError> {
        self.check_indices(i, j)?;
        let (i, j) = if i < j { (i, j) } else { (j, i) };
        Ok(*self.t_stats.get(&(i, j)).unwrap())
    }

    /// t-statistics for all pairs of systems, returning `(i, j, t-statistic)` such that `i < j`.
    ///
    /// The results are sorted by `(i, j)`.
    pub fn t_stats(&self) -> Vec<(usize, usize, f64)> {
        let mut t_stats = self
            .t_stats
            .iter()
            .map(|(&(i, j), &t)| (i, j, t))
            .collect_vec();
        t_stats.sort_unstable_by(|(ai, aj, _), (bi, bj, _)| ai.cmp(bi).then(aj.cmp(bj)));
        t_stats
    }

    /// p-value between systems i and j.
    pub fn p_value(&self, i: usize, j: usize) -> Result<f64, ElinorError> {
        self.check_indices(i, j)?;
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

    /// Confidence intervals at a `1 - significance_level` confidence level.
    ///
    /// # Errors
    ///
    /// * [`ElinorError::InvalidArgument`] if the significance level is not in the range `(0, 1]`.
    pub fn confidence_intervals(
        &self,
        significance_level: f64,
    ) -> Result<Vec<(f64, f64)>, ElinorError> {
        let moe = self.margin_of_error(significance_level)?;
        Ok(self
            .system_means
            .iter()
            .map(|&mean| (mean - moe, mean + moe))
            .collect())
    }

    fn check_indices(&self, i: usize, j: usize) -> Result<(), ElinorError> {
        if i >= self.n_systems || j >= self.n_systems {
            return Err(ElinorError::InvalidArgument(
                "The system index is out of range.".to_string(),
            ));
        }
        if i == j {
            return Err(ElinorError::InvalidArgument(
                "The indices must be different.".to_string(),
            ));
        }
        Ok(())
    }
}
