//! Two-way ANOVA without replication.
use statrs::distribution::ContinuousCDF;
use statrs::distribution::FisherSnedecor;
use statrs::distribution::StudentsT;
use statrs::statistics::Statistics;

use crate::errors::ElinorError;

/// Two-way ANOVA without replication.
///
/// # Notations
///
/// * $`m`$: Number of systems.
/// * $`n`$: Number of topics.
/// * $`x_{ij}`$: Score of the $`i`$-th system on the $`j`$-th topic.
/// * $`\bar{x}`$: Mean of all scores $`x_{ij}`$.
#[derive(Debug, Clone)]
pub struct TwoWayAnovaWithoutReplication {
    n_systems: usize,
    n_topics: usize,
    system_means: Vec<f64>,
    topic_means: Vec<f64>,
    between_system_variation: f64, // S_A
    between_system_variance: f64,  // V_A
    between_topic_variation: f64,  // S_B
    between_topic_variance: f64,   // V_B
    residual_variation: f64,       // S_E
    residual_variance: f64,        // V_E
    between_system_f_stat: f64,    // F (between-system factor)
    between_topic_f_stat: f64,     // F (between-topic factor)
    between_system_p_value: f64,   // p-value (between-system factor)
    between_topic_p_value: f64,    // p-value (between-topic factor)
    system_t_dist: StudentsT,
}

impl TwoWayAnovaWithoutReplication {
    /// Computes a new Two-way ANOVA without replication
    /// from scores $`x_{ij}`$ of $`i \in [1,m]`$ systems and $`j \in [1,n]`$ topics.
    ///
    /// # Arguments
    ///
    /// * `samples` - Iterator of tupled samples, where each sample is an array of $`m`$ system scores.
    /// * `n_systems` - Number of systems, $`m`$.
    ///
    /// # Errors
    ///
    /// * [`ElinorError::InvalidArgument`] if the length of each sample is not equal to the number of systems.
    /// * [`ElinorError::InvalidArgument`] if the input does not have at least two samples.
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

        let n_topics_f = samples.len() as f64;
        let n_systems_f = n_systems as f64;

        // Mean of all samples (x_{..}).
        let overall_mean = samples.iter().flatten().mean();

        // Mean of each system (x_{i.*}).
        let system_means = (0..n_systems)
            .map(|j| samples.iter().map(|sample| sample[j]).sum::<f64>() / n_topics_f)
            .collect::<Vec<_>>();

        // Mean of each topic (x_{*.j}).
        let topic_means = samples
            .iter()
            .map(|sample| sample.mean())
            .collect::<Vec<_>>();

        // S_A
        let between_system_variation = system_means
            .iter()
            .map(|&x_i_dot| (x_i_dot - overall_mean).powi(2))
            .sum::<f64>()
            * n_topics_f;

        // S_B
        let between_topic_variation = topic_means
            .iter()
            .map(|&x_dot_j| (x_dot_j - overall_mean).powi(2))
            .sum::<f64>()
            * n_systems_f;

        // S_E
        let residual_variation = samples
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

        // V_A
        let between_system_freedom = n_systems_f - 1.;
        let between_system_variance = between_system_variation / between_system_freedom;

        // V_B
        let between_topic_freedom = n_topics_f - 1.;
        let between_topic_variance = between_topic_variation / between_topic_freedom;

        // V_E
        let residual_freedom = (n_systems_f - 1.) * (n_topics_f - 1.);
        let residual_variance = residual_variation / residual_freedom;

        // F and p-value for the between-system factor.
        let between_system_f_dist = FisherSnedecor::new(between_system_freedom, residual_freedom)
            .expect("Failed to create a Fisher-Snedecor distribution.");
        let between_system_f_stat = between_system_variance / residual_variance;
        let between_system_p_value = between_system_f_dist.sf(between_system_f_stat);

        // F and p-value for the between-topic factor.
        let between_topic_f_dist = FisherSnedecor::new(between_topic_freedom, residual_freedom)
            .expect("Failed to create a Fisher-Snedecor distribution.");
        let between_topic_f_stat = between_topic_variance / residual_variance;
        let between_topic_p_value = between_topic_f_dist.sf(between_topic_f_stat);

        let system_t_dist = StudentsT::new(
            0.0,
            (residual_variance / n_topics_f).sqrt(),
            residual_freedom,
        )
        .expect("Failed to create a Student's t distribution.");

        Ok(Self {
            n_topics: samples.len(),
            n_systems,
            system_means,
            topic_means,
            between_system_variation,
            between_system_variance,
            between_topic_variation,
            between_topic_variance,
            residual_variation,
            residual_variance,
            between_system_f_stat,
            between_topic_f_stat,
            between_system_p_value,
            between_topic_p_value,
            system_t_dist,
        })
    }

    /// Number of systems, $`m`$.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use elinor::statistical_tests::TwoWayAnovaWithoutReplication;
    ///
    /// let stat = TwoWayAnovaWithoutReplication::from_tupled_samples([[1., 2., 3.], [2., 4., 2.]], 3)?;
    /// assert_eq!(stat.n_systems(), 3);
    /// # Ok(())
    /// # }
    /// ```
    pub const fn n_systems(&self) -> usize {
        self.n_systems
    }

    /// Number of topics, $`n`$.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use elinor::statistical_tests::TwoWayAnovaWithoutReplication;
    ///
    /// let stat = TwoWayAnovaWithoutReplication::from_tupled_samples([[1., 2., 3.], [2., 4., 2.]], 3)?;
    /// assert_eq!(stat.n_topics(), 2);
    /// # Ok(())
    /// # }
    /// ```
    pub const fn n_topics(&self) -> usize {
        self.n_topics
    }

    /// Means of each system.
    ///
    /// # Formula
    ///
    /// ```math
    /// \bar{x}_{i*} = \frac{1}{n} \sum_{i=1}^{m} x_{ij}
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use approx::assert_abs_diff_eq;
    /// use elinor::statistical_tests::TwoWayAnovaWithoutReplication;
    ///
    /// let stat = TwoWayAnovaWithoutReplication::from_tupled_samples([[1., 2., 3.], [2., 4., 2.]], 3)?;
    /// let system_means = stat.system_means();
    /// assert_eq!(system_means.len(), 3);
    /// assert_abs_diff_eq!(system_means[0], (1. + 2.) / 2., epsilon = 1e-10);
    /// assert_abs_diff_eq!(system_means[1], (2. + 4.) / 2., epsilon = 1e-10);
    /// assert_abs_diff_eq!(system_means[2], (3. + 2.) / 2., epsilon = 1e-10);
    /// # Ok(())
    /// # }
    /// ```
    pub fn system_means(&self) -> Vec<f64> {
        self.system_means.clone()
    }

    /// Means of each topic.
    ///
    /// # Formula
    ///
    /// ```math
    /// \bar{x}_{*j} = \frac{1}{m} \sum_{j=1}^{n} x_{ij}
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use approx::assert_abs_diff_eq;
    /// use elinor::statistical_tests::TwoWayAnovaWithoutReplication;
    ///
    /// let stat = TwoWayAnovaWithoutReplication::from_tupled_samples([[1., 2., 3.], [2., 4., 2.]], 3)?;
    /// let topic_means = stat.topic_means();
    /// assert_eq!(topic_means.len(), 2);
    /// assert_abs_diff_eq!(topic_means[0], (1. + 2. + 3.) / 3., epsilon = 1e-10);
    /// assert_abs_diff_eq!(topic_means[1], (2. + 4. + 2.) / 3., epsilon = 1e-10);
    /// # Ok(())
    /// # }
    /// ```
    pub fn topic_means(&self) -> Vec<f64> {
        self.topic_means.clone()
    }

    /// Between-system variation.
    ///
    /// # Formula
    ///
    /// ```math
    /// S_A = n \sum_{i=1}^{m} (\bar{x}_{i*} - \bar{x})^2
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use approx::assert_abs_diff_eq;
    /// use elinor::statistical_tests::TwoWayAnovaWithoutReplication;
    ///
    /// let stat = TwoWayAnovaWithoutReplication::from_tupled_samples([[1., 2., 3.], [2., 4., 2.]], 3)?;
    /// let mean: f64 = (1. + 2. + 2. + 4. + 3. + 2.) / 6.;
    /// let mean_system_a: f64 = (1. + 2.) / 2.;
    /// let mean_system_b: f64 = (2. + 4.) / 2.;
    /// let mean_system_c: f64 = (3. + 2.) / 2.;
    /// assert_abs_diff_eq!(
    ///     stat.between_system_variation(),
    ///     ((mean_system_a - mean).powi(2) + (mean_system_b - mean).powi(2) + (mean_system_c - mean).powi(2)) * 2.,
    ///     epsilon = 1e-10,
    /// );
    /// # Ok(())
    /// # }
    /// ```
    pub const fn between_system_variation(&self) -> f64 {
        self.between_system_variation
    }

    /// Between-topic variation.
    ///
    /// # Formula
    ///
    /// ```math
    /// S_B = m \sum_{j=1}^{n} (\bar{x}_{*j} - \bar{x})^2
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use approx::assert_abs_diff_eq;
    /// use elinor::statistical_tests::TwoWayAnovaWithoutReplication;
    ///
    /// let stat = TwoWayAnovaWithoutReplication::from_tupled_samples([[1., 2., 3.], [2., 4., 2.]], 3)?;
    /// let mean: f64 = (1. + 2. + 2. + 4. + 3. + 2.) / 6.;
    /// let mean_topic_1: f64 = (1. + 2. + 3.) / 3.;
    /// let mean_topic_2: f64 = (2. + 4. + 2.) / 3.;
    /// assert_abs_diff_eq!(
    ///     stat.between_topic_variation(),
    ///     ((mean_topic_1 - mean).powi(2) + (mean_topic_2 - mean).powi(2)) * 3.,
    ///     epsilon = 1e-10,
    /// );
    /// # Ok(())
    /// # }
    /// ```
    pub const fn between_topic_variation(&self) -> f64 {
        self.between_topic_variation
    }

    /// Residual variation.
    ///
    /// # Formula
    ///
    /// ```math
    /// S_E = \sum_{j=1}^{n} \sum_{i=1}^{m} (x_{ij} - \bar{x}_{i*} - \bar{x}_{*j} + \bar{x})^2
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use approx::assert_abs_diff_eq;
    /// use elinor::statistical_tests::TwoWayAnovaWithoutReplication;
    ///
    /// let stat = TwoWayAnovaWithoutReplication::from_tupled_samples([[1., 2., 3.], [2., 4., 2.]], 3)?;
    /// let mean: f64 = (1. + 2. + 2. + 4. + 3. + 2.) / 6.;
    /// let mean_system_a: f64 = (1. + 2.) / 2.;
    /// let mean_system_b: f64 = (2. + 4.) / 2.;
    /// let mean_system_c: f64 = (3. + 2.) / 2.;
    /// let mean_topic_1: f64 = (1. + 2. + 3.) / 3.;
    /// let mean_topic_2: f64 = (2. + 4. + 2.) / 3.;
    /// assert_abs_diff_eq!(
    ///     stat.residual_variation(),
    ///     (1.0 - mean_system_a - mean_topic_1 + mean).powi(2) + (2.0 - mean_system_a - mean_topic_2 + mean).powi(2) +
    ///     (2.0 - mean_system_b - mean_topic_1 + mean).powi(2) + (4.0 - mean_system_b - mean_topic_2 + mean).powi(2) +
    ///     (3.0 - mean_system_c - mean_topic_1 + mean).powi(2) + (2.0 - mean_system_c - mean_topic_2 + mean).powi(2),
    ///     epsilon = 1e-10,
    /// );
    /// # Ok(())
    /// # }
    /// ```
    pub const fn residual_variation(&self) -> f64 {
        self.residual_variation
    }

    /// Between-system variance.
    ///
    /// # Formula
    ///
    /// ```math
    /// V_A = \frac{S_A}{m - 1}
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use approx::assert_abs_diff_eq;
    /// use elinor::statistical_tests::TwoWayAnovaWithoutReplication;
    ///
    /// let stat = TwoWayAnovaWithoutReplication::from_tupled_samples([[1., 2., 3.], [2., 4., 2.]], 3)?;
    /// assert_abs_diff_eq!(
    ///     stat.between_system_variance(),
    ///     stat.between_system_variation() / (3. - 1.),
    ///     epsilon = 1e-10,
    /// );
    /// # Ok(())
    /// # }
    /// ```
    pub const fn between_system_variance(&self) -> f64 {
        self.between_system_variance
    }

    /// Between-topic variance.
    ///
    /// # Formula
    ///
    /// ```math
    /// V_B = \frac{S_B}{n - 1}
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use approx::assert_abs_diff_eq;
    /// use elinor::statistical_tests::TwoWayAnovaWithoutReplication;
    ///
    /// let stat = TwoWayAnovaWithoutReplication::from_tupled_samples([[1., 2., 3.], [2., 4., 2.]], 3)?;
    /// assert_abs_diff_eq!(
    ///     stat.between_topic_variance(),
    ///     stat.between_topic_variation() / (2. - 1.),
    ///     epsilon = 1e-10,
    /// );
    /// # Ok(())
    /// # }
    /// ```
    pub const fn between_topic_variance(&self) -> f64 {
        self.between_topic_variance
    }

    /// Residual variance.
    ///
    /// # Formula
    ///
    /// ```math
    /// V_E = \frac{S_E}{(m - 1)(n - 1)}
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use approx::assert_abs_diff_eq;
    /// use elinor::statistical_tests::TwoWayAnovaWithoutReplication;
    ///
    /// let stat = TwoWayAnovaWithoutReplication::from_tupled_samples([[1., 2., 3.], [2., 4., 2.]], 3)?;
    /// assert_abs_diff_eq!(
    ///     stat.residual_variance(),
    ///     stat.residual_variation() / ((3. - 1.) * (2. - 1.)),
    ///     epsilon = 1e-10,
    /// );
    /// # Ok(())
    /// # }
    /// ```
    pub const fn residual_variance(&self) -> f64 {
        self.residual_variance
    }

    /// Between-system F-statistic.
    ///
    /// # Formula
    ///
    /// ```math
    /// F_A = \frac{V_A}{V_E}
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use approx::assert_abs_diff_eq;
    /// use elinor::statistical_tests::TwoWayAnovaWithoutReplication;
    ///
    /// let stat = TwoWayAnovaWithoutReplication::from_tupled_samples([[1., 2., 3.], [2., 4., 2.]], 3)?;
    /// assert_abs_diff_eq!(
    ///     stat.between_system_f_stat(),
    ///     stat.between_system_variance() / stat.residual_variance(),
    ///     epsilon = 1e-10,
    /// );
    /// # Ok(())
    /// # }
    /// ```
    pub const fn between_system_f_stat(&self) -> f64 {
        self.between_system_f_stat
    }

    /// Between-topic F-statistic.
    ///
    /// # Formula
    ///
    /// ```math
    /// F_B = \frac{V_B}{V_E}
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use approx::assert_abs_diff_eq;
    /// use elinor::statistical_tests::TwoWayAnovaWithoutReplication;
    ///
    /// let stat = TwoWayAnovaWithoutReplication::from_tupled_samples([[1., 2., 3.], [2., 4., 2.]], 3)?;
    /// assert_abs_diff_eq!(
    ///     stat.between_topic_f_stat(),
    ///     stat.between_topic_variance() / stat.residual_variance(),
    ///     epsilon = 1e-10,
    /// );
    /// # Ok(())
    /// # }
    /// ```
    pub const fn between_topic_f_stat(&self) -> f64 {
        self.between_topic_f_stat
    }

    /// Between-system p-value.
    ///
    /// # Formula
    ///
    /// ```math
    /// p_A = P(F_A > F_{\alpha}(m - 1, (m - 1)(n - 1)))
    /// ```
    ///
    /// where $`F_{\alpha}(m - 1, (m - 1)(n - 1))`$ is the $`1 - \alpha`$ quantile of the $`F`$ distribution with $`m - 1`$ and $`(m - 1)(n - 1)`$ degrees of freedom.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use elinor::statistical_tests::TwoWayAnovaWithoutReplication;
    ///
    /// let stat = TwoWayAnovaWithoutReplication::from_tupled_samples([[1., 2., 3.], [2., 4., 2.]], 3)?;
    /// assert!((0.0..=1.0).contains(&stat.between_system_p_value()));
    /// # Ok(())
    /// # }
    /// ```
    pub const fn between_system_p_value(&self) -> f64 {
        self.between_system_p_value
    }

    /// Between-topic p-value.
    ///
    /// # Formula
    ///
    /// ```math
    /// p_B = P(F_B > F_{\alpha}(n - 1, (m - 1)(n - 1)))
    /// ```
    ///
    /// where $`F_{\alpha}(n - 1, (m - 1)(n - 1))`$ is the $`1 - \alpha`$ quantile of the $`F`$ distribution with $`n - 1`$ and $`(m - 1)(n - 1)`$ degrees of freedom.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use elinor::statistical_tests::TwoWayAnovaWithoutReplication;
    ///
    /// let stat = TwoWayAnovaWithoutReplication::from_tupled_samples([[1., 2., 3.], [2., 4., 2.]], 3)?;
    /// assert!((0.0..=1.0).contains(&stat.between_topic_p_value()));
    /// # Ok(())
    /// # }
    /// ```
    pub const fn between_topic_p_value(&self) -> f64 {
        self.between_topic_p_value
    }

    /// Margin of error at a given significance level.
    ///
    /// # Errors
    ///
    /// * [`ElinorError::InvalidArgument`] if the significance level is not in the range `(0, 1]`.
    ///
    /// # Formula
    ///
    /// ```math
    /// \text{MOE} = t_{\alpha/2}((m - 1)(n - 1)) \times \sqrt{\frac{V_E}{n}}
    /// ```
    ///
    /// where $`t_{\alpha/2}((m - 1)(n - 1))`$ is the $`1 - \alpha/2`$ quantile of the Student's $`t`$ distribution with $`(m - 1)(n - 1)`$ degrees of freedom.
    pub fn margin_of_error(&self, significance_level: f64) -> Result<f64, ElinorError> {
        if significance_level <= 0.0 || significance_level > 1.0 {
            return Err(ElinorError::InvalidArgument(
                "The significance level must be in the range (0, 1].".to_string(),
            ));
        }
        Ok(self
            .system_t_dist
            .inverse_cdf(1.0 - (significance_level / 2.0)))
    }

    /// Effect sizes for all combinations of systems,
    /// returning a matrix of size $`m \times m`$.
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
    /// # Examples
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use approx::assert_abs_diff_eq;
    /// use elinor::statistical_tests::TwoWayAnovaWithoutReplication;
    ///
    /// let stat = TwoWayAnovaWithoutReplication::from_tupled_samples([[1., 2., 3.], [2., 4., 2.]], 3)?;
    /// let effect_sizes = stat.between_system_effect_sizes();
    /// let mean_system_a: f64 = (1. + 2.) / 2.;
    /// let mean_system_b: f64 = (2. + 4.) / 2.;
    /// let mean_system_c: f64 = (3. + 2.) / 2.;
    ///
    /// assert_eq!(effect_sizes.len(), 3);
    /// assert_eq!(effect_sizes[0].len(), 3);
    /// assert_eq!(effect_sizes[1].len(), 3);
    /// assert_eq!(effect_sizes[2].len(), 3);
    /// assert_abs_diff_eq!(effect_sizes[0][0], 0.0);
    /// assert_abs_diff_eq!(effect_sizes[1][1], 0.0);
    /// assert_abs_diff_eq!(effect_sizes[2][2], 0.0);
    /// assert_abs_diff_eq!(effect_sizes[0][1], (mean_system_a - mean_system_b) / stat.residual_variance().sqrt(), epsilon = 1e-10);
    /// assert_abs_diff_eq!(effect_sizes[0][2], (mean_system_a - mean_system_c) / stat.residual_variance().sqrt(), epsilon = 1e-10);
    /// assert_abs_diff_eq!(effect_sizes[1][2], (mean_system_b - mean_system_c) / stat.residual_variance().sqrt(), epsilon = 1e-10);
    /// assert_abs_diff_eq!(effect_sizes[1][0], -effect_sizes[0][1], epsilon = 1e-10);
    /// assert_abs_diff_eq!(effect_sizes[2][0], -effect_sizes[0][2], epsilon = 1e-10);
    /// assert_abs_diff_eq!(effect_sizes[2][1], -effect_sizes[1][2], epsilon = 1e-10);
    /// # Ok(())
    /// # }
    /// ```
    pub fn between_system_effect_sizes(&self) -> Vec<Vec<f64>> {
        let mut effect_sizes = vec![vec![0.0; self.n_systems]; self.n_systems];
        for i in 0..self.n_systems {
            for j in (i + 1)..self.n_systems {
                let diff = self.system_means[i] - self.system_means[j];
                let effect_size = diff / self.residual_variance.sqrt();
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
    fn test_two_way_anova_without_replication_from_tupled_samples_empty() {
        let samples: Vec<[f64; 2]> = vec![];
        let stat = TwoWayAnovaWithoutReplication::from_tupled_samples(samples, 2);
        assert_eq!(
            stat.unwrap_err(),
            ElinorError::InvalidArgument("The input must have at least two samples.".to_string())
        );
    }

    #[test]
    fn test_two_way_anova_without_replication_from_tupled_samples_single() {
        let samples = vec![[1.0, 2.0]];
        let stat = TwoWayAnovaWithoutReplication::from_tupled_samples(samples, 2);
        assert_eq!(
            stat.unwrap_err(),
            ElinorError::InvalidArgument("The input must have at least two samples.".to_string())
        );
    }

    #[test]
    fn test_two_way_anova_without_replication_from_tupled_samples_invalid_length() {
        let samples = vec![vec![1.0, 2.0], vec![3.0]];
        let stat = TwoWayAnovaWithoutReplication::from_tupled_samples(samples, 2);
        assert_eq!(
            stat.unwrap_err(),
            ElinorError::InvalidArgument(
                "The length of each sample must be equal to the number of systems.".to_string()
            )
        );
    }

    #[test]
    fn test_two_way_anova_without_replication_sakai_book() {
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
        let stat = TwoWayAnovaWithoutReplication::from_tupled_samples(tupled_samples, 3).unwrap();
        assert_eq!(stat.n_systems(), 3);
        assert_eq!(stat.n_topics(), 20);

        // Comparing with the values in Sakai's book.
        assert_abs_diff_eq!(stat.between_system_variation(), 0.1083, epsilon = 1e-4);
        assert_abs_diff_eq!(stat.between_topic_variation(), 1.0293, epsilon = 1e-4);
        assert_abs_diff_eq!(stat.residual_variation(), 0.8317, epsilon = 1e-4);
        assert_abs_diff_eq!(stat.between_system_variance(), 0.0542, epsilon = 1e-4);
        assert_abs_diff_eq!(stat.between_topic_variance(), 0.0542, epsilon = 1e-4);
        assert_abs_diff_eq!(stat.residual_variance(), 0.0219, epsilon = 1e-4);
        assert_abs_diff_eq!(stat.between_system_f_stat(), 2.475, epsilon = 1e-3);
        assert_abs_diff_eq!(stat.between_topic_f_stat(), 2.475, epsilon = 1e-3);
        assert_abs_diff_eq!(stat.between_system_p_value(), 0.098, epsilon = 1e-3);
        assert_abs_diff_eq!(stat.between_topic_p_value(), 0.009, epsilon = 1e-3);
        assert_abs_diff_eq!(stat.margin_of_error(0.05).unwrap(), 0.0670, epsilon = 1e-4);
        let effect_sizes = stat.between_system_effect_sizes();
        assert_abs_diff_eq!(effect_sizes[0][1], 0.5070, epsilon = 1e-4);
        assert_abs_diff_eq!(effect_sizes[0][2], 0.6760, epsilon = 1e-4);
        assert_abs_diff_eq!(effect_sizes[1][2], 0.1690, epsilon = 1e-4);
    }
}
