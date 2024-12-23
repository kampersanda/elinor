//! Statistical tests.
//!
//! This module provides several statistical tests for comparing systems:
//!
//! * [Student's t-test](StudentTTest) for comparing two systems.
//! * [Bootstrap test](BootstrapTest) for comparing two systems.
//! * [Two-way ANOVA without replication](TwoWayAnovaWithoutReplication) for comparing three or more systems.
//! * [Tukey HSD test](TukeyHsdTest) for comparing three or more systems.
//! * [Randomized Tukey HSD test](RandomizedTukeyHsdTest) for comparing two or more systems.
pub mod bootstrap_test;
pub mod randomized_tukey_hsd_test;
pub mod student_t_test;
pub mod tukey_hsd_test;
pub mod two_way_anova_without_replication;

pub use bootstrap_test::BootstrapTest;
pub use randomized_tukey_hsd_test::RandomizedTukeyHsdTest;
pub use student_t_test::StudentTTest;
pub use tukey_hsd_test::TukeyHsdTest;
pub use two_way_anova_without_replication::TwoWayAnovaWithoutReplication;

use std::collections::BTreeMap;

use crate::errors::ElinorError;
use crate::errors::Result;

/// Converts two maps of scores, $`A`$ and $`B`$, into a vector of paired scores $`X`$:
///
/// - $`A = \{ (k^A_1 \mapsto v^A_1), (k^A_2 \mapsto v^A_2), \dots, (k^A_n \mapsto v^A_n) \}`$,
/// - $`B = \{ (k^B_1 \mapsto v^B_1), (k^B_2 \mapsto v^B_2), \dots, (k^B_n \mapsto v^B_n) \}`$, and
/// - $`X = [(v^A_1, v^B_1), (v^A_2, v^B_2), \dots, (v^A_n, v^B_n)]`$,
///
/// where $`k^A_i = k^B_i`$ for all $`i`$.
///
/// # Examples
///
/// ```
/// use elinor::statistical_tests::pairs_from_maps;
///
/// let map_a = [("a", 0.70), ("b", 0.30), ("c", 0.20)].into();
/// let map_b = [("a", 0.50), ("b", 0.10), ("c", 0.00)].into();
/// let pairs = pairs_from_maps(&map_a, &map_b).unwrap();
/// assert_eq!(pairs, vec![(0.70, 0.50), (0.30, 0.10), (0.20, 0.00)]);
/// ```
///
/// # Errors
///
/// * [`ElinorError::InvalidArgument`] if maps have different sets of keys.
pub fn pairs_from_maps<K>(
    map_a: &BTreeMap<K, f64>,
    map_b: &BTreeMap<K, f64>,
) -> Result<Vec<(f64, f64)>>
where
    K: Clone + Eq + Ord + std::fmt::Display,
{
    tuples_from_maps([map_a, map_b]).map(|tuples| {
        tuples
            .into_iter()
            .map(|tuple| (tuple[0], tuple[1]))
            .collect()
    })
}

/// Converts maps of scores, $`A_1, A_2, \dots, A_m`$, into a vector of tupled scores $`X`$:
///
/// - $`A_j = \{ (k^j_1 \mapsto v^j_1), (k^j_2 \mapsto v^j_2), \dots, (k^j_n \mapsto v^j_n) \}`$ for all $`j`$,
/// - $`X = [(v^1_1, v^2_1, \dots, v^m_1), (v^1_2, v^2_2, \dots, v^m_2), \dots, (v^1_n, v^2_n, \dots, v^m_n)]`$,
///
/// where $`k^1_i = k^2_i = \dots = k^m_i`$ for all $`i`$.
///
/// # Examples
///
/// ```
/// use elinor::statistical_tests::tuples_from_maps;
///
/// let map_a = [("a", 0.70), ("b", 0.30), ("c", 0.20)].into();
/// let map_b = [("a", 0.50), ("b", 0.10), ("c", 0.00)].into();
/// let map_c = [("a", 0.60), ("b", 0.20), ("c", 0.10)].into();
/// let tuples = tuples_from_maps([&map_a, &map_b, &map_c]).unwrap();
/// assert_eq!(tuples, vec![vec![0.70, 0.50, 0.60], vec![0.30, 0.10, 0.20], vec![0.20, 0.00, 0.10]]);
/// ```
///
/// # Errors
///
/// * [`ElinorError::InvalidArgument`] if maps have different sets of keys.
pub fn tuples_from_maps<'a, I, K>(maps: I) -> Result<Vec<Vec<f64>>>
where
    I: IntoIterator<Item = &'a BTreeMap<K, f64>>,
    K: Clone + Eq + Ord + std::fmt::Display + 'a,
{
    let maps = maps.into_iter().collect::<Vec<_>>();
    for i in 1..maps.len() {
        if maps[0].len() != maps[i].len() {
            return Err(ElinorError::InvalidArgument(format!(
                "The number of keys in maps must be the same, but got maps[0].len()={} and maps[{}].len()={}.",
                maps[0].len(),
                i,
                maps[i].len()
            )));
        }
        if maps[0].keys().ne(maps[i].keys()) {
            return Err(ElinorError::InvalidArgument(
                "The keys in the maps must be the same.".to_string(),
            ));
        }
    }
    let mut tuples = vec![];
    for query_id in maps[0].keys() {
        let mut tuple = vec![];
        for &map in &maps {
            tuple.push(*map.get(query_id).unwrap());
        }
        tuples.push(tuple);
    }
    Ok(tuples)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pairs_from_maps_different_keys() {
        let map_a = [("a", 0.70), ("b", 0.30), ("c", 0.20)].into();
        let map_b = [("a", 0.50), ("b", 0.10), ("d", 0.00)].into();
        assert_eq!(
            pairs_from_maps(&map_a, &map_b),
            Err(ElinorError::InvalidArgument(
                "The keys in the maps must be the same.".to_string()
            ))
        );
    }

    #[test]
    fn test_tuples_from_maps_different_keys() {
        let map_a = [("a", 0.70), ("b", 0.30), ("c", 0.20)].into();
        let map_b = [("a", 0.50), ("b", 0.10), ("d", 0.00)].into();
        let map_c = [("a", 0.60), ("b", 0.20), ("c", 0.10)].into();
        assert_eq!(
            tuples_from_maps([&map_a, &map_b, &map_c]),
            Err(ElinorError::InvalidArgument(
                "The keys in the maps must be the same.".to_string()
            ))
        );
    }

    #[test]
    fn test_tuples_from_maps_single_map() {
        let map_a = [("a", 0.70), ("b", 0.30), ("c", 0.20)].into();
        assert_eq!(
            tuples_from_maps([&map_a]),
            Ok(vec![vec![0.70], vec![0.30], vec![0.20]])
        );
    }
}
