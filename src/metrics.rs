//! Metrics for evaluating information retrieval systems.
pub(crate) mod average_precision;
pub(crate) mod f1;
pub(crate) mod hits;
pub(crate) mod ndcg;
pub(crate) mod precision;
pub(crate) mod recall;
pub(crate) mod reciprocal_rank;

use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;

use regex::Regex;

use crate::errors::EmirError;
use crate::GoldScore;
use crate::Qrels;
use crate::Run;

pub(crate) const RELEVANT_LEVEL: GoldScore = 1;

/// Metrics for evaluating information retrieval systems.
///
/// # Arguments
///
/// * `k` - Number of top documents to consider. if `k` is set to 0, all documents are considered.
///
/// # Conversion from/into string
///
/// The [`FromStr`] trait is implemented to allow
/// instantiating a [`Metric`] from a string, as follows:
///
/// ```rust
/// use emir::Metric;
///
/// assert_eq!("hits".parse::<Metric>(), Ok(Metric::Hits { k: 0 }));
/// assert_eq!("hits@3".parse::<Metric>(), Ok(Metric::Hits { k: 3 }));
/// ```
///
/// The [`Display`] trait is also implemented to allow
/// formatting a [`Metric`] into a string, as follows:
///
/// ```rust
/// use emir::Metric;
///
/// assert_eq!(format!("{}", Metric::Hits { k: 0 }), "hits");
/// assert_eq!(format!("{}", Metric::Hits { k: 3 }), "hits@3");
/// ```
///
/// ## Conversion table
///
/// | Metric | String |
/// | ------ | ------ |
/// | [`Metric::Hits`] | `hits` |
/// | [`Metric::Success`] | `success` |
/// | [`Metric::Precision`] | `precision` |
/// | [`Metric::Recall`] | `recall` |
/// | [`Metric::F1`] | `f1` |
/// | [`Metric::AP`] | `ap` |
/// | [`Metric::RR`] | `rr` |
/// | [`Metric::DCG`] | `dcg` |
/// | [`Metric::NDCG`] | `ndcg` |
/// | [`Metric::DCGBurges`] | `dcg_burges` |
/// | [`Metric::NDCGBurges`] | `ndcg_burges` |
///
/// ## Parameters
///
/// The `@k` suffix is used to specify the value of `k`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Metric {
    /// Number of relevant documents retrieved:
    ///
    /// ```math
    /// \text{Hits} = | \text{Res} \cap \text{Rel} |
    /// ```
    ///
    /// where:
    ///
    /// * $`\text{Res}`$ is the set of retrieved documents.
    /// * $`\text{Rel}`$ is the set of relevant documents.
    Hits {
        /// See the [Arguments](enum.Metric.html#arguments) section.
        k: usize,
    },

    /// Binary metric indicating whether at least one relevant document is retrieved:
    ///
    /// ```math
    /// \text{Success} = \left\{ \begin{array}{ll}
    ///     1 & \text{if Hits} > 0 \\
    ///     0 & \text{otherwise}
    /// \end{array} \right.
    /// ```
    Success {
        /// See the [Arguments](enum.Metric.html#arguments) section.
        k: usize,
    },

    /// Proportion of the retrieved documents that are relevant:
    ///
    /// ```math
    /// \text{Precision} = \frac{\text{Hits}}{|\text{Res}|}
    /// ```
    ///
    /// Note that, when `k` is set, the denominator is fixed to `k`
    /// even if the number of retrieved documents is fewer than `k`:
    ///
    /// ```math
    /// \text{Precision}@k = \frac{\text{Hits}}{k}
    /// ```
    Precision {
        /// See the [Arguments](enum.Metric.html#arguments) section.
        k: usize,
    },

    /// Ratio between the retrieved documents that are relevant and
    /// the total number of relevant documents:
    ///
    /// ```math
    /// \text{Recall} = \frac{\text{Hits}}{| \text{Rel} |}
    /// ```
    Recall {
        /// See the [Arguments](enum.Metric.html#arguments) section.
        k: usize,
    },

    /// Harmonic mean of precision and recall:
    ///
    /// ```math
    /// \text{F1} = 2 \times \frac{\text{Precision} \times \text{Recall}}{\text{Precision} + \text{Recall}}
    /// ```
    F1 {
        /// See the [Arguments](enum.Metric.html#arguments) section.
        k: usize,
    },

    /// Average of the Precision scores computed after each relevant document is retrieved:
    ///
    /// ```math
    /// \text{AP} = \frac{1}{| \text{Rel} |} \sum_{i=1}^{| \text{Res} |} \text{Precision}@i \times
    /// \left\{ \begin{array}{ll} 1 & \text{if the } i \text{-th document is relevant} \\ 0 & \text{otherwise} \end{array} \right.
    /// ```
    AP {
        /// See the [Arguments](enum.Metric.html#arguments) section.
        k: usize,
    },

    /// Multiplicative inverse of the rank of the first retrieved relevant document:
    ///
    /// ```math
    /// \text{RR} = \frac{1}{\text{the rank of the first retrieved relevant document}}
    /// ```
    RR {
        /// See the [Arguments](enum.Metric.html#arguments) section.
        k: usize,
    },

    /// Discounted cumulative gain proposed in
    /// [Järvelin et al., TOIS 2002](https://dl.acm.org/doi/10.1145/582415.582418).
    ///
    /// ```math
    /// \text{DCG}@k = \sum_{i=1}^k \frac{\text{rel}_i}{\log_2(i + 1)}
    /// ```
    ///
    /// where `rel_i` is the relevance score of the `i`-th document.
    DCG {
        /// See the [Arguments](enum.Metric.html#arguments) section.
        k: usize,
    },

    /// Normalized version of the DCG score:
    ///
    /// ```math
    /// \text{NDCG}@k = \frac{\text{DCG}@k}{\text{IDCG}@k}
    /// ```
    ///
    /// where `IDCG` is the ideal DCG score, i.e., the max possible DCG score.
    NDCG {
        /// See the [Arguments](enum.Metric.html#arguments) section.
        k: usize,
    },

    /// Discounted cumulative gain proposed in
    /// [Burges et al. ICML 2005](https://dl.acm.org/doi/10.1145/1102351.1102363).
    ///
    /// ```math
    /// \text{DCG}_\text{Burges}@k = \sum_{i=1}^k \frac{2^{\text{rel}_i} - 1}{\log_2(i + 1)}
    /// ```
    DCGBurges {
        /// See the [Arguments](enum.Metric.html#arguments) section.
        k: usize,
    },

    /// Normalized version of the Burges' DCG score:
    ///
    /// ```math
    /// \text{NDCG}_\text{Burges}@k = \frac{\text{DCG}_\text{Burges}@k}{\text{IDCG}_\text{Burges}@k}
    /// ```
    NDCGBurges {
        /// See the [Arguments](enum.Metric.html#arguments) section.
        k: usize,
    },
}

impl Display for Metric {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Hits { k } => {
                write!(f, "{}", format_metric("hits", *k))
            }
            Self::Success { k } => {
                write!(f, "{}", format_metric("success", *k))
            }
            Self::Precision { k } => {
                write!(f, "{}", format_metric("precision", *k))
            }
            Self::Recall { k } => {
                write!(f, "{}", format_metric("recall", *k))
            }
            Self::F1 { k } => {
                write!(f, "{}", format_metric("f1", *k))
            }
            Self::AP { k } => {
                write!(f, "{}", format_metric("ap", *k))
            }
            Self::RR { k } => {
                write!(f, "{}", format_metric("rr", *k))
            }
            Self::DCG { k } => {
                write!(f, "{}", format_metric("dcg", *k))
            }
            Self::NDCG { k } => {
                write!(f, "{}", format_metric("ndcg", *k))
            }
            Self::DCGBurges { k } => {
                write!(f, "{}", format_metric("dcg_burges", *k))
            }
            Self::NDCGBurges { k } => {
                write!(f, "{}", format_metric("ndcg_burges", *k))
            }
        }
    }
}

fn format_metric(name: &str, k: usize) -> String {
    if k == 0 {
        name.to_string()
    } else {
        format!("{name}@{k}")
    }
}

impl FromStr for Metric {
    type Err = EmirError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^(?<metric>[a-z1-9_]+)(@(?<k>\d+))?$").unwrap();
        let caps = re
            .captures(s)
            .ok_or_else(|| EmirError::InvalidFormat(s.to_string()))?;
        let name = caps.name("metric").unwrap().as_str();
        let k = caps
            .name("k")
            .map(|m| m.as_str().parse::<usize>())
            .transpose()
            .map_err(|_| EmirError::InvalidFormat(s.to_string()))?
            .unwrap_or(0);
        match name {
            "hits" => Ok(Self::Hits { k }),
            "success" => Ok(Self::Success { k }),
            "precision" => Ok(Self::Precision { k }),
            "recall" => Ok(Self::Recall { k }),
            "f1" => Ok(Self::F1 { k }),
            "ap" => Ok(Self::AP { k }),
            "rr" => Ok(Self::RR { k }),
            "dcg" => Ok(Self::DCG { k }),
            "ndcg" => Ok(Self::NDCG { k }),
            "dcg_burges" => Ok(Self::DCGBurges { k }),
            "ndcg_burges" => Ok(Self::NDCGBurges { k }),
            _ => Err(EmirError::InvalidFormat(s.to_string())),
        }
    }
}

/// Computes the metric scores for the given Qrels and Run data.
pub fn compute_metric<K>(
    qrels: &Qrels<K>,
    run: &Run<K>,
    metric: Metric,
) -> Result<HashMap<K, f64>, EmirError>
where
    K: Clone + Eq + std::hash::Hash + std::fmt::Display,
{
    for query_id in run.query_ids() {
        if qrels.get_map(query_id).is_none() {
            return Err(EmirError::MissingEntry(format!("Query ID: {query_id}")));
        }
    }
    let mut results = HashMap::new();
    for query_id in run.query_ids() {
        let preds = run.get_sorted(query_id).unwrap();
        let rels = qrels.get_map(query_id).unwrap();
        let score = match metric {
            Metric::Hits { k } => hits::compute_hits(rels, preds, k, RELEVANT_LEVEL),
            Metric::Success { k } => hits::compute_success(rels, preds, k, RELEVANT_LEVEL),
            Metric::Precision { k } => precision::compute_precision(rels, preds, k, RELEVANT_LEVEL),
            Metric::Recall { k } => recall::compute_recall(rels, preds, k, RELEVANT_LEVEL),
            Metric::F1 { k } => f1::compute_f1(rels, preds, k, RELEVANT_LEVEL),
            Metric::AP { k } => {
                average_precision::compute_average_precision(rels, preds, k, RELEVANT_LEVEL)
            }
            Metric::RR { k } => {
                reciprocal_rank::compute_reciprocal_rank(rels, preds, k, RELEVANT_LEVEL)
            }
            Metric::DCG { k } => ndcg::compute_dcg(rels, preds, k, ndcg::DcgWeighting::Jarvelin),
            Metric::NDCG { k } => {
                let golds = qrels.get_sorted(query_id).unwrap();
                ndcg::compute_ndcg(rels, golds, preds, k, ndcg::DcgWeighting::Jarvelin)
            }
            Metric::DCGBurges { k } => {
                ndcg::compute_dcg(rels, preds, k, ndcg::DcgWeighting::Burges)
            }
            Metric::NDCGBurges { k } => {
                let golds = qrels.get_sorted(query_id).unwrap();
                ndcg::compute_ndcg(rels, golds, preds, k, ndcg::DcgWeighting::Burges)
            }
        };
        results.insert(query_id.clone(), score);
    }
    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use maplit::hashmap;
    use rstest::*;

    const LOG_2_2: f64 = 1.0;
    const LOG_2_3: f64 = 1.584962500721156;
    const LOG_2_4: f64 = 2.0;

    fn compare_hashmaps(a: &HashMap<char, f64>, b: &HashMap<char, f64>) {
        assert_eq!(a.len(), b.len());
        for (k, v) in a.iter() {
            assert_relative_eq!(v, b.get(k).unwrap());
        }
    }

    #[rstest]
    // Hits
    #[case::hits_k_0_rel_lvl_1(Metric::Hits { k: 0 }, hashmap! { 'A' => 2.0 })]
    #[case::hits_k_1_rel_lvl_1(Metric::Hits { k: 1 }, hashmap! { 'A' => 1.0 })]
    #[case::hits_k_2_rel_lvl_1(Metric::Hits { k: 2 }, hashmap! { 'A' => 1.0 })]
    #[case::hits_k_3_rel_lvl_1(Metric::Hits { k: 3 }, hashmap! { 'A' => 2.0 })]
    #[case::hits_k_4_rel_lvl_1(Metric::Hits { k: 4 }, hashmap! { 'A' => 2.0 })]
    #[case::hits_k_5_rel_lvl_1(Metric::Hits { k: 5 }, hashmap! { 'A' => 2.0 })]
    // Hit rate
    #[case::hit_rate_k_0_rel_lvl_1(Metric::Success { k: 0 }, hashmap! { 'A' => 1.0 })]
    #[case::hit_rate_k_1_rel_lvl_1(Metric::Success { k: 1 }, hashmap! { 'A' => 1.0 })]
    #[case::hit_rate_k_2_rel_lvl_1(Metric::Success { k: 2 }, hashmap! { 'A' => 1.0 })]
    #[case::hit_rate_k_3_rel_lvl_1(Metric::Success { k: 3 }, hashmap! { 'A' => 1.0 })]
    #[case::hit_rate_k_4_rel_lvl_1(Metric::Success { k: 4 }, hashmap! { 'A' => 1.0 })]
    #[case::hit_rate_k_5_rel_lvl_1(Metric::Success { k: 5 }, hashmap! { 'A' => 1.0 })]
    // Precision
    #[case::precision_k_0_rel_lvl_1(Metric::Precision { k: 0 }, hashmap! { 'A' => 2.0 / 4.0 })]
    #[case::precision_k_1_rel_lvl_1(Metric::Precision { k: 1 }, hashmap! { 'A' => 1.0 / 1.0 })]
    #[case::precision_k_2_rel_lvl_1(Metric::Precision { k: 2 }, hashmap! { 'A' => 1.0 / 2.0 })]
    #[case::precision_k_3_rel_lvl_1(Metric::Precision { k: 3 }, hashmap! { 'A' => 2.0 / 3.0 })]
    #[case::precision_k_4_rel_lvl_1(Metric::Precision { k: 4 }, hashmap! { 'A' => 2.0 / 4.0 })]
    #[case::precision_k_5_rel_lvl_1(Metric::Precision { k: 5 }, hashmap! { 'A' => 2.0 / 5.0 })]
    // Recall
    #[case::recall_k_0_rel_lvl_1(Metric::Recall { k: 0 }, hashmap! { 'A' => 2.0 / 2.0 })]
    #[case::recall_k_1_rel_lvl_1(Metric::Recall { k: 1 }, hashmap! { 'A' => 1.0 / 2.0 })]
    #[case::recall_k_2_rel_lvl_1(Metric::Recall { k: 2 }, hashmap! { 'A' => 1.0 / 2.0 })]
    #[case::recall_k_3_rel_lvl_1(Metric::Recall { k: 3 }, hashmap! { 'A' => 2.0 / 2.0 })]
    #[case::recall_k_4_rel_lvl_1(Metric::Recall { k: 4 }, hashmap! { 'A' => 2.0 / 2.0 })]
    #[case::recall_k_5_rel_lvl_1(Metric::Recall { k: 5 }, hashmap! { 'A' => 2.0 / 2.0 })]
    // F1
    #[case::f1_k_0_rel_lvl_1(Metric::F1 { k: 0 }, hashmap! { 'A' => 2.0 * (2.0 / 4.0) * (2.0 / 2.0) / ((2.0 / 4.0) + (2.0 / 2.0)) })]
    #[case::f1_k_1_rel_lvl_1(Metric::F1 { k: 1 }, hashmap! { 'A' => 2.0 * (1.0 / 1.0) * (1.0 / 2.0) / ((1.0 / 1.0) + (1.0 / 2.0)) })]
    #[case::f1_k_2_rel_lvl_1(Metric::F1 { k: 2 }, hashmap! { 'A' => 2.0 * (1.0 / 2.0) * (1.0 / 2.0) / ((1.0 / 2.0) + (1.0 / 2.0)) })]
    #[case::f1_k_3_rel_lvl_1(Metric::F1 { k: 3 }, hashmap! { 'A' => 2.0 * (2.0 / 3.0) * (2.0 / 2.0) / ((2.0 / 3.0) + (2.0 / 2.0)) })]
    #[case::f1_k_4_rel_lvl_1(Metric::F1 { k: 4 }, hashmap! { 'A' => 2.0 * (2.0 / 4.0) * (2.0 / 2.0) / ((2.0 / 4.0) + (2.0 / 2.0)) })]
    #[case::f1_k_5_rel_lvl_1(Metric::F1 { k: 5 }, hashmap! { 'A' => 2.0 * (2.0 / 5.0) * (2.0 / 2.0) / ((2.0 / 5.0) + (2.0 / 2.0)) })]
    // Average precision
    #[case::average_precision_k_0_rel_lvl_1(Metric::AP { k: 0 }, hashmap! { 'A' => ((1.0 / 1.0) + (2.0 / 3.0)) / 2.0 })]
    #[case::average_precision_k_1_rel_lvl_1(Metric::AP { k: 1 }, hashmap! { 'A' => (1.0 / 1.0) / 2.0 })]
    #[case::average_precision_k_2_rel_lvl_1(Metric::AP { k: 2 }, hashmap! { 'A' => (1.0 / 1.0) / 2.0 })]
    #[case::average_precision_k_3_rel_lvl_1(Metric::AP { k: 3 }, hashmap! { 'A' => ((1.0 / 1.0) + (2.0 / 3.0)) / 2.0 })]
    #[case::average_precision_k_4_rel_lvl_1(Metric::AP { k: 4 }, hashmap! { 'A' => ((1.0 / 1.0) + (2.0 / 3.0)) / 2.0 })]
    #[case::average_precision_k_5_rel_lvl_1(Metric::AP { k: 5 }, hashmap! { 'A' => ((1.0 / 1.0) + (2.0 / 3.0)) / 2.0 })]
    // Reciprocal rank
    #[case::reciprocal_rank_k_0_rel_lvl_1(Metric::RR { k: 0 }, hashmap! { 'A' => 1.0 / 1.0 })]
    #[case::reciprocal_rank_k_1_rel_lvl_1(Metric::RR { k: 1 }, hashmap! { 'A' => 1.0 / 1.0 })]
    #[case::reciprocal_rank_k_2_rel_lvl_1(Metric::RR { k: 2 }, hashmap! { 'A' => 1.0 / 1.0 })]
    #[case::reciprocal_rank_k_3_rel_lvl_1(Metric::RR { k: 3 }, hashmap! { 'A' => 1.0 / 1.0 })]
    #[case::reciprocal_rank_k_4_rel_lvl_1(Metric::RR { k: 4 }, hashmap! { 'A' => 1.0 / 1.0 })]
    #[case::reciprocal_rank_k_5_rel_lvl_1(Metric::RR { k: 5 }, hashmap! { 'A' => 1.0 / 1.0 })]
    // DCG (Jarvelin)
    #[case::dcg_k_0_jarvelin(Metric::DCG { k: 0 }, hashmap! { 'A' => 1.0 / LOG_2_2 + 2.0 / LOG_2_4 })]
    #[case::dcg_k_1_jarvelin(Metric::DCG { k: 1 }, hashmap! { 'A' => 1.0 / LOG_2_2 })]
    #[case::dcg_k_2_jarvelin(Metric::DCG { k: 2 }, hashmap! { 'A' => 1.0 / LOG_2_2 })]
    #[case::dcg_k_3_jarvelin(Metric::DCG { k: 3 }, hashmap! { 'A' => 1.0 / LOG_2_2 + 2.0 / LOG_2_4 })]
    #[case::dcg_k_4_jarvelin(Metric::DCG { k: 4 }, hashmap! { 'A' => 1.0 / LOG_2_2 + 2.0 / LOG_2_4 })]
    #[case::dcg_k_5_jarvelin(Metric::DCG { k: 5 }, hashmap! { 'A' => 1.0 / LOG_2_2 + 2.0 / LOG_2_4 })]
    // NDCG (Jarvelin)
    #[case::ndcg_k_0_jarvelin(Metric::NDCG { k: 0 }, hashmap! { 'A' => (1.0 / LOG_2_2 + 2.0 / LOG_2_4) / (2.0 / LOG_2_2 + 1.0 / LOG_2_3) })]
    #[case::ndcg_k_1_jarvelin(Metric::NDCG { k: 1 }, hashmap! { 'A' => (1.0 / LOG_2_2) / (2.0 / LOG_2_2) })]
    #[case::ndcg_k_2_jarvelin(Metric::NDCG { k: 2 }, hashmap! { 'A' => (1.0 / LOG_2_2) / (2.0 / LOG_2_2 + 1.0 / LOG_2_3) })]
    #[case::ndcg_k_3_jarvelin(Metric::NDCG { k: 3 }, hashmap! { 'A' => (1.0 / LOG_2_2 + 2.0 / LOG_2_4) / (2.0 / LOG_2_2 + 1.0 / LOG_2_3) })]
    #[case::ndcg_k_4_jarvelin(Metric::NDCG { k: 4 }, hashmap! { 'A' => (1.0 / LOG_2_2 + 2.0 / LOG_2_4) / (2.0 / LOG_2_2 + 1.0 / LOG_2_3) })]
    #[case::ndcg_k_5_jarvelin(Metric::NDCG { k: 5 }, hashmap! { 'A' => (1.0 / LOG_2_2 + 2.0 / LOG_2_4) / (2.0 / LOG_2_2 + 1.0 / LOG_2_3) })]
    // DCG (Burges)
    #[case::dcg_k_0_burges(Metric::DCGBurges { k: 0 }, hashmap! { 'A' => 1.0 / LOG_2_2 + 3.0 / LOG_2_4 })]
    #[case::dcg_k_1_burges(Metric::DCGBurges { k: 1 }, hashmap! { 'A' => 1.0 / LOG_2_2 })]
    #[case::dcg_k_2_burges(Metric::DCGBurges { k: 2 }, hashmap! { 'A' => 1.0 / LOG_2_2 })]
    #[case::dcg_k_3_burges(Metric::DCGBurges { k: 3 }, hashmap! { 'A' => 1.0 / LOG_2_2 + 3.0 / LOG_2_4 })]
    #[case::dcg_k_4_burges(Metric::DCGBurges { k: 4 }, hashmap! { 'A' => 1.0 / LOG_2_2 + 3.0 / LOG_2_4 })]
    #[case::dcg_k_5_burges(Metric::DCGBurges { k: 5 }, hashmap! { 'A' => 1.0 / LOG_2_2 + 3.0 / LOG_2_4 })]
    // NDCG (Burges)
    #[case::ndcg_k_0_burges(Metric::NDCGBurges { k: 0 }, hashmap! { 'A' => (1.0 / LOG_2_2 + 3.0 / LOG_2_4) / (3.0 / LOG_2_2 + 1.0 / LOG_2_3) })]
    #[case::ndcg_k_1_burges(Metric::NDCGBurges { k: 1 }, hashmap! { 'A' => (1.0 / LOG_2_2) / (3.0 / LOG_2_2) })]
    #[case::ndcg_k_2_burges(Metric::NDCGBurges { k: 2 }, hashmap! { 'A' => (1.0 / LOG_2_2) / (3.0 / LOG_2_2 + 1.0 / LOG_2_3) })]
    #[case::ndcg_k_3_burges(Metric::NDCGBurges { k: 3 }, hashmap! { 'A' => (1.0 / LOG_2_2 + 3.0 / LOG_2_4) / (3.0 / LOG_2_2 + 1.0 / LOG_2_3) })]
    #[case::ndcg_k_4_burges(Metric::NDCGBurges { k: 4 }, hashmap! { 'A' => (1.0 / LOG_2_2 + 3.0 / LOG_2_4) / (3.0 / LOG_2_2 + 1.0 / LOG_2_3) })]
    #[case::ndcg_k_5_burges(Metric::NDCGBurges { k: 5 }, hashmap! { 'A' => (1.0 / LOG_2_2 + 3.0 / LOG_2_4) / (3.0 / LOG_2_2 + 1.0 / LOG_2_3) })]
    fn test_compute_metric(#[case] metric: Metric, #[case] expected: HashMap<char, f64>) {
        let qrels = Qrels::from_map(hashmap! {
            'A' => hashmap! {
                'X' => 1,
                'Y' => 0,
                'Z' => 2,
            },
        });
        let run = Run::from_map(hashmap! {
            'A' => hashmap! {
                'X' => 0.5.into(),
                'Y' => 0.4.into(),
                'Z' => 0.3.into(),
                'W' => 0.2.into(),
            },
        });
        let results = compute_metric(&qrels, &run, metric).unwrap();
        compare_hashmaps(&results, &expected);
    }

    #[rstest]
    #[case::hits("hits", Metric::Hits { k: 0 })]
    #[case::hits_k0("hits@0", Metric::Hits { k: 0 })]
    #[case::hits_k1("hits@1", Metric::Hits { k: 1 })]
    #[case::hits_k100("hits@100", Metric::Hits { k: 100 })]
    #[case::success("success", Metric::Success { k: 0 })]
    #[case::success_k0("success@0", Metric::Success { k: 0 })]
    #[case::success_k1("success@1", Metric::Success { k: 1 })]
    #[case::success_k100("success@100", Metric::Success { k: 100 })]
    #[case::precision("precision", Metric::Precision { k: 0 })]
    #[case::precision_k0("precision@0", Metric::Precision { k: 0 })]
    #[case::precision_k1("precision@1", Metric::Precision { k: 1 })]
    #[case::precision_k100("precision@100", Metric::Precision { k: 100 })]
    #[case::recall("recall", Metric::Recall { k: 0 })]
    #[case::recall_k0("recall@0", Metric::Recall { k: 0 })]
    #[case::recall_k1("recall@1", Metric::Recall { k: 1 })]
    #[case::recall_k100("recall@100", Metric::Recall { k: 100 })]
    #[case::f1("f1", Metric::F1 { k: 0 })]
    #[case::f1_k0("f1@0", Metric::F1 { k: 0 })]
    #[case::f1_k1("f1@1", Metric::F1 { k: 1 })]
    #[case::f1_k100("f1@100", Metric::F1 { k: 100 })]
    #[case::average_precision("ap", Metric::AP { k: 0 })]
    #[case::average_precision_k0("ap@0", Metric::AP { k: 0 })]
    #[case::average_precision_k1("ap@1", Metric::AP { k: 1 })]
    #[case::average_precision_k100("ap@100", Metric::AP { k: 100 })]
    #[case::reciprocal_rank("rr", Metric::RR { k: 0 })]
    #[case::reciprocal_rank_k0("rr@0", Metric::RR { k: 0 })]
    #[case::reciprocal_rank_k1("rr@1", Metric::RR { k: 1 })]
    #[case::reciprocal_rank_k100("rr@100", Metric::RR { k: 100 })]
    #[case::dcg("dcg", Metric::DCG { k: 0 })]
    #[case::dcg_k0("dcg@0", Metric::DCG { k: 0 })]
    #[case::dcg_k1("dcg@1", Metric::DCG { k: 1 })]
    #[case::dcg_k100("dcg@100", Metric::DCG { k: 100 })]
    #[case::ndcg("ndcg", Metric::NDCG { k: 0 })]
    #[case::ndcg_k0("ndcg@0", Metric::NDCG { k: 0 })]
    #[case::ndcg_k1("ndcg@1", Metric::NDCG { k: 1 })]
    #[case::ndcg_k100("ndcg@100", Metric::NDCG { k: 100 })]
    #[case::dcg_burges("dcg_burges", Metric::DCGBurges { k: 0 })]
    #[case::dcg_burges_k0("dcg_burges@0", Metric::DCGBurges { k: 0 })]
    #[case::dcg_burges_k1("dcg_burges@1", Metric::DCGBurges { k: 1 })]
    #[case::dcg_burges_k100("dcg_burges@100", Metric::DCGBurges { k: 100 })]
    #[case::ndcg_burges("ndcg_burges", Metric::NDCGBurges { k: 0 })]
    #[case::ndcg_burges_k0("ndcg_burges@0", Metric::NDCGBurges { k: 0 })]
    #[case::ndcg_burges_k1("ndcg_burges@1", Metric::NDCGBurges { k: 1 })]
    #[case::ndcg_burges_k100("ndcg_burges@100", Metric::NDCGBurges { k: 100 })]
    fn test_metric_from_str(#[case] input: &str, #[case] expected: Metric) {
        let metric = Metric::from_str(input).unwrap();
        assert_eq!(metric, expected);
    }
}
