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

///
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

/// Converts maps of scores into a vector of tupled scores, where each tuple contains the scores for each key.
///
/// This function is expected to be used to prepare data for statistical tests.
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
    if maps.len() < 2 {
        return Err(ElinorError::InvalidArgument(format!(
            "The number of maps maps must be at least 2, but got {}.",
            maps.len()
        )));
    }
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
