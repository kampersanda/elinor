//! Data structures for storing relevance scores.
use std::borrow::Borrow;
use std::collections::BTreeMap;
use std::fmt::Display;

#[cfg(feature = "serde")]
use serde::Deserialize;
#[cfg(feature = "serde")]
use serde::Serialize;

use crate::errors::ElinorError;
use crate::errors::Result;

/// Record of a query-document pair.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Record<K, T> {
    /// Query id.
    pub query_id: K,

    /// Document id.
    pub doc_id: K,

    /// Relevance score.
    pub score: T,
}

/// Data to store a relevance score for a document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Relevance<K, T> {
    /// Document id.
    pub doc_id: K,

    /// Relevance score.
    pub score: T,
}

struct RelevanceData<K, T> {
    sorted: Vec<Relevance<K, T>>,
    map: BTreeMap<K, T>,
}

/// Data structure for storing relevance scores.
///
/// # Type parameters
///
/// * `K` - Query/document id.
/// * `T` - Relevance score.
pub struct RelevanceStore<K, T> {
    // Mapping from query ids to:
    //  - Sorted list of relevance scores in descending order.
    //  - Mapping from document ids to relevance scores.
    map: BTreeMap<K, RelevanceData<K, T>>,
}

impl<K, T> RelevanceStore<K, T>
where
    K: Eq + Ord + Clone + Display,
    T: Ord + Clone,
{
    /// Creates an instance from records.
    pub fn from_records<I>(records: I) -> Result<Self>
    where
        I: IntoIterator<Item = Record<K, T>>,
    {
        let mut b = RelevanceStoreBuilder::new();
        for record in records {
            b.add_record(record.query_id, record.doc_id, record.score)?;
        }
        Ok(b.build())
    }

    /// Exports the relevance store into records.
    pub fn into_records(self) -> Vec<Record<K, T>> {
        self.map
            .into_iter()
            .flat_map(|(query_id, data)| {
                data.sorted.into_iter().map(move |rel| Record {
                    query_id: query_id.clone(),
                    doc_id: rel.doc_id,
                    score: rel.score,
                })
            })
            .collect()
    }

    /// Returns the relevance store as records.
    pub fn records(&self) -> Vec<Record<K, T>> {
        self.map
            .iter()
            .flat_map(|(query_id, data)| {
                data.sorted.iter().map(move |rel| Record {
                    query_id: query_id.clone(),
                    doc_id: rel.doc_id.clone(),
                    score: rel.score.clone(),
                })
            })
            .collect()
    }

    /// Returns the score for a given query-document pair.
    pub fn get_score<Q>(&self, query_id: &Q, doc_id: &Q) -> Option<&T>
    where
        K: Borrow<Q>,
        Q: Eq + Ord + ?Sized,
    {
        self.map.get(query_id).and_then(|data| data.map.get(doc_id))
    }

    /// Returns the relevance map for a given query id.
    pub fn get_map<Q>(&self, query_id: &Q) -> Option<&BTreeMap<K, T>>
    where
        K: Borrow<Q>,
        Q: Eq + Ord + ?Sized,
    {
        self.map.get(query_id).map(|data| &data.map)
    }

    /// Returns the sorted list of relevance scores in descending order
    /// for a given query id.
    pub fn get_sorted<Q>(&self, query_id: &Q) -> Option<&[Relevance<K, T>]>
    where
        K: Borrow<Q>,
        Q: Eq + Ord + ?Sized,
    {
        self.map.get(query_id).map(|data| data.sorted.as_slice())
    }
}

impl<K, T> RelevanceStore<K, T> {
    /// Returns the number of query ids in the store.
    pub fn n_queries(&self) -> usize {
        self.map.len()
    }

    /// Returns the number of document ids in the store.
    pub fn n_docs(&self) -> usize {
        self.map.values().map(|data| data.map.len()).sum()
    }

    /// Returns an iterator over the query ids in random order.
    pub fn query_ids(&self) -> impl Iterator<Item = &K> {
        self.map.keys()
    }
}

/// Builder for [`RelevanceStore`].
pub struct RelevanceStoreBuilder<K, T> {
    map: BTreeMap<K, BTreeMap<K, T>>,
}

impl<K, T> Default for RelevanceStoreBuilder<K, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K, T> RelevanceStoreBuilder<K, T> {
    /// Creates a new builder.
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }

    /// Adds a relevance score to the store.
    ///
    /// # Arguments
    ///
    /// * `query_id` - Query id.
    /// * `doc_id` - Document id.
    /// * `score` - Relevance score.
    ///
    /// # Errors
    ///
    /// * [`ElinorError::DuplicateEntry`] if the query-document pair already exists.
    pub fn add_record(&mut self, query_id: K, doc_id: K, score: T) -> Result<()>
    where
        K: Eq + Ord + Clone + Display,
    {
        let rels = self.map.entry(query_id.clone()).or_default();
        if rels.contains_key(&doc_id) {
            return Err(ElinorError::DuplicateEntry(format!(
                "Input query-doc pair must be unique, but got query_id={query_id}, doc_id={doc_id}"
            )));
        }
        rels.insert(doc_id, score);
        Ok(())
    }

    /// Builds the relevance store.
    pub fn build(self) -> RelevanceStore<K, T>
    where
        K: Eq + Ord + Clone + Display,
        T: Ord + Clone,
    {
        let mut map = BTreeMap::new();
        for (query_id, rels) in self.map {
            let mut sorted = rels
                .iter()
                .map(|(doc_id, score)| Relevance {
                    doc_id: doc_id.clone(),
                    score: score.clone(),
                })
                .collect::<Vec<_>>();
            sorted.sort_by(|a, b| b.score.cmp(&a.score).then(a.doc_id.cmp(&b.doc_id)));
            map.insert(query_id, RelevanceData { sorted, map: rels });
        }
        RelevanceStore { map }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_relevance_store_from_into_records() {
        let mut records = vec![
            Record {
                query_id: 'a',
                doc_id: 'x',
                score: 1,
            },
            Record {
                query_id: 'b',
                doc_id: 'x',
                score: 1,
            },
            Record {
                query_id: 'b',
                doc_id: 'y',
                score: 2,
            },
            Record {
                query_id: 'c',
                doc_id: 'x',
                score: 1,
            },
        ];
        let store = RelevanceStore::from_records(records.iter().cloned()).unwrap();
        let mut other = store.into_records();
        records.sort();
        other.sort();
        assert_eq!(records, other);
    }

    #[test]
    fn test_relevance_store_n_queries() {
        let store = RelevanceStore::from_records([
            Record {
                query_id: 'a',
                doc_id: 'x',
                score: 1,
            },
            Record {
                query_id: 'b',
                doc_id: 'x',
                score: 1,
            },
            Record {
                query_id: 'b',
                doc_id: 'y',
                score: 2,
            },
            Record {
                query_id: 'c',
                doc_id: 'x',
                score: 1,
            },
        ])
        .unwrap();
        assert_eq!(store.n_queries(), 3);
    }

    #[test]
    fn test_relevance_store_n_docs() {
        let store = RelevanceStore::from_records([
            Record {
                query_id: 'a',
                doc_id: 'x',
                score: 1,
            },
            Record {
                query_id: 'b',
                doc_id: 'x',
                score: 1,
            },
            Record {
                query_id: 'b',
                doc_id: 'y',
                score: 2,
            },
            Record {
                query_id: 'c',
                doc_id: 'x',
                score: 1,
            },
        ])
        .unwrap();
        assert_eq!(store.n_docs(), 4);
    }

    #[test]
    fn test_relevance_store_get_score() {
        let store = RelevanceStore::from_records([Record {
            query_id: 'a',
            doc_id: 'x',
            score: 1,
        }])
        .unwrap();
        assert_eq!(store.get_score(&'a', &'x'), Some(&1));
        assert_eq!(store.get_score(&'a', &'y'), None);
        assert_eq!(store.get_score(&'b', &'x'), None);
    }

    #[test]
    fn test_relevance_store_get_map() {
        let store = RelevanceStore::from_records([
            Record {
                query_id: 'a',
                doc_id: 'x',
                score: 1,
            },
            Record {
                query_id: 'a',
                doc_id: 'y',
                score: 2,
            },
        ])
        .unwrap();
        assert_eq!(store.get_map(&'a'), Some(&[('x', 1), ('y', 2)].into()));
        assert_eq!(store.get_map(&'b'), None);
    }

    #[test]
    fn test_relevance_store_get_sorted() {
        let store = RelevanceStore::from_records([
            Record {
                query_id: 'a',
                doc_id: 'x',
                score: 1,
            },
            Record {
                query_id: 'a',
                doc_id: 'y',
                score: 2,
            },
        ])
        .unwrap();
        let expected = vec![
            Relevance {
                doc_id: 'y',
                score: 2,
            },
            Relevance {
                doc_id: 'x',
                score: 1,
            },
        ];
        assert_eq!(store.get_sorted(&'a'), Some(expected.as_slice()));
        assert_eq!(store.get_sorted(&'b'), None);
    }

    #[test]
    fn test_relevance_store_query_ids() {
        let store = RelevanceStore::from_records([
            Record {
                query_id: 'a',
                doc_id: 'x',
                score: 1,
            },
            Record {
                query_id: 'b',
                doc_id: 'x',
                score: 1,
            },
            Record {
                query_id: 'c',
                doc_id: 'x',
                score: 1,
            },
        ])
        .unwrap();
        let expected = HashSet::from_iter([&'a', &'b', &'c']);
        let actual = store.query_ids().collect::<HashSet<_>>();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_relevance_store_builder() {
        let mut b = RelevanceStoreBuilder::new();
        b.add_record('a', 'x', 1).unwrap();
        b.add_record('a', 'y', 2).unwrap();
        b.add_record('b', 'x', 3).unwrap();
        let store = b.build();
        assert_eq!(store.get_map(&'a'), Some(&[('x', 1), ('y', 2)].into()));
        assert_eq!(store.get_map(&'b'), Some(&[('x', 3)].into()));
        assert_eq!(store.get_map(&'c'), None);
    }

    #[test]
    fn test_relevance_store_builder_duplicate_entry() {
        let mut b = RelevanceStoreBuilder::new();
        b.add_record('a', 'x', 1).unwrap();
        assert_eq!(
            b.add_record('a', 'x', 2),
            Err(ElinorError::DuplicateEntry(
                "Input query-doc pair must be unique, but got query_id=a, doc_id=x".to_string()
            ))
        );
    }
}
