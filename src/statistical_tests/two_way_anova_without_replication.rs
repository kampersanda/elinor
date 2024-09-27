//!

use statrs::distribution::ContinuousCDF;
use statrs::distribution::FisherSnedecor;
use statrs::distribution::StudentsT;
use statrs::statistics::Statistics;

use crate::errors::ElinorError;

/// Two-Way ANOVA without replication.
///
/// # Examples
///
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use approx::assert_abs_diff_eq;
/// use elinor::statistical_tests::TwoWayAnovaWithoutReplication;
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
/// // Comparing three systems.
/// let tupled_samples = a
///     .iter()
///     .zip(b.iter())
///     .zip(c.iter())
///     .map(|((&a, &b), &c)| [a, b, c]);
/// let result = TwoWayAnovaWithoutReplication::from_tupled_samples(tupled_samples, 3)?;
///
/// assert_abs_diff_eq!(result.between_system_variation(), 0.1083, epsilon = 1e-4);
/// assert_abs_diff_eq!(result.between_topic_variation(), 1.0293, epsilon = 1e-4);
/// assert_abs_diff_eq!(result.residual_variation(), 0.8317, epsilon = 1e-4);
///
/// assert_abs_diff_eq!(result.between_system_variance(), 0.0542, epsilon = 1e-4);
/// assert_abs_diff_eq!(result.between_topic_variance(), 0.0542, epsilon = 1e-4);
/// assert_abs_diff_eq!(result.residual_variance(), 0.0219, epsilon = 1e-4);
///
/// assert_abs_diff_eq!(result.between_system_f_stat(), 2.475, epsilon = 1e-4);
/// assert_abs_diff_eq!(result.between_system_f_stat(), 2.475, epsilon = 1e-4);
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct TwoWayAnovaWithoutReplication {
    n_systems: usize,
    n_topics: usize,
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
    system_means: Vec<f64>,
    scaled_t_dist: StudentsT,
}

impl TwoWayAnovaWithoutReplication {
    /// Creates a new Two-Way ANOVA without replication.
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

        let scaled_t_dist = StudentsT::new(
            0.0,
            (residual_variance / n_topics_f).sqrt(),
            residual_freedom,
        )
        .expect("Failed to create a Student's t distribution.");

        Ok(Self {
            n_topics: samples.len(),
            n_systems,
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
            system_means,
            scaled_t_dist,
        })
    }

    /// Number of systems.
    pub const fn n_systems(&self) -> usize {
        self.n_systems
    }

    /// Number of topics.
    pub const fn n_topics(&self) -> usize {
        self.n_topics
    }

    /// Between-system sum of squares.
    pub const fn between_system_variation(&self) -> f64 {
        self.between_system_variation
    }

    /// Between-system mean square.
    pub const fn between_system_variance(&self) -> f64 {
        self.between_system_variance
    }

    /// Between-topic sum of squares.
    pub const fn between_topic_variation(&self) -> f64 {
        self.between_topic_variation
    }

    /// Between-topic mean square.
    pub const fn between_topic_variance(&self) -> f64 {
        self.between_topic_variance
    }

    /// Residual sum of squares.
    pub const fn residual_variation(&self) -> f64 {
        self.residual_variation
    }

    /// Residual mean square.
    pub const fn residual_variance(&self) -> f64 {
        self.residual_variance
    }

    /// Between-system F-statistic.
    pub const fn between_system_f_stat(&self) -> f64 {
        self.between_system_f_stat
    }

    /// Between-topic F-statistic.
    pub const fn between_topic_f_stat(&self) -> f64 {
        self.between_topic_f_stat
    }

    /// Between-system p-value.
    pub const fn between_system_p_value(&self) -> f64 {
        self.between_system_p_value
    }

    /// Between-topic p-value.
    pub const fn between_topic_p_value(&self) -> f64 {
        self.between_topic_p_value
    }

    /// Means of each system.
    pub fn system_means(&self) -> Vec<f64> {
        self.system_means.clone()
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

    /// Effect sizes for all combinations of systems.
    pub fn effect_sizes(&self) -> Vec<Vec<f64>> {
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
