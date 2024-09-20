//! Relevance store.
use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;

use crate::errors::ElinorError;

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
    map: HashMap<K, T>,
}

/// Data structure for storing relevance scores.
///
/// # Type parameters
///
/// * `K` - Query/document id.
/// * `T` - Relevance score.
pub struct RelevanceStore<K, T> {
    // Name.
    name: Option<String>,

    // Mapping from query ids to:
    //  - Sorted list of relevance scores in descending order.
    //  - Mapping from document ids to relevance scores.
    map: HashMap<K, RelevanceData<K, T>>,
}

impl<K, T> RelevanceStore<K, T>
where
    K: Eq + Ord + Hash + Clone,
    T: Ord + Clone,
{
    /// Creates a relevance store from a map of query ids to relevance maps.
    pub fn from_map(map: HashMap<K, HashMap<K, T>>) -> Self
    where
        K: Display,
    {
        let b = RelevanceStoreBuilder { map };
        b.build()
    }

    /// Exports the relevance store as a map of query ids to relevance maps.
    pub fn into_map(self) -> HashMap<K, HashMap<K, T>> {
        self.map.into_iter().map(|(k, v)| (k, v.map)).collect()
    }

    /// Returns the score for a given query-document pair.
    pub fn get_score<Q>(&self, query_id: &Q, doc_id: &Q) -> Option<&T>
    where
        K: Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        self.map.get(query_id).and_then(|data| data.map.get(doc_id))
    }

    /// Returns the relevance map for a given query id.
    pub fn get_map<Q>(&self, query_id: &Q) -> Option<&HashMap<K, T>>
    where
        K: Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        self.map.get(query_id).map(|data| &data.map)
    }

    /// Returns the sorted list of relevance scores in descending order
    /// for a given query id.
    pub fn get_sorted<Q>(&self, query_id: &Q) -> Option<&[Relevance<K, T>]>
    where
        K: Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        self.map.get(query_id).map(|data| data.sorted.as_slice())
    }
}

impl<K, T> RelevanceStore<K, T> {
    /// Sets the name of the relevance store.
    pub fn with_name(self, name: &str) -> Self {
        Self {
            name: Some(name.to_string()),
            ..self
        }
    }

    /// Returns the name of the relevance store.
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

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
    map: HashMap<K, HashMap<K, T>>,
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
            map: HashMap::new(),
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
    pub fn add_score(&mut self, query_id: K, doc_id: K, score: T) -> Result<(), ElinorError>
    where
        K: Eq + Hash + Clone + Display,
    {
        let rels = self.map.entry(query_id.clone()).or_default();
        if rels.contains_key(&doc_id) {
            return Err(ElinorError::DuplicateEntry(format!(
                "Query: {query_id}, Doc: {doc_id}"
            )));
        }
        rels.insert(doc_id, score);
        Ok(())
    }

    /// Builds the relevance store.
    pub fn build(self) -> RelevanceStore<K, T>
    where
        K: Eq + Ord + Hash + Clone + Display,
        T: Ord + Clone,
    {
        let mut map = HashMap::new();
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
        RelevanceStore { name: None, map }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_relevance_store_from_into_map() {
        let map1: HashMap<char, HashMap<char, u32>> =
            [('a', [('x', 1), ('y', 2)].into()), ('b', [('x', 1)].into())].into();
        let store = RelevanceStore::from_map(map1.clone());
        let map2 = store.into_map();
        assert_eq!(map1, map2);
    }

    #[test]
    fn test_relevance_store_name() {
        let store = RelevanceStore::from_map([('a', [('x', 1)].into())].into());
        assert_eq!(store.name(), None);
        let store = store.with_name("test");
        assert_eq!(store.name(), Some("test"));
    }

    #[test]
    fn test_relevance_store_n_queries() {
        let store = RelevanceStore::from_map(
            [
                ('a', [('x', 1)].into()),
                ('b', [('x', 1), ('y', 2)].into()),
                ('c', [('x', 1)].into()),
            ]
            .into(),
        );
        assert_eq!(store.n_queries(), 3);
    }

    #[test]
    fn test_relevance_store_n_docs() {
        let store = RelevanceStore::from_map(
            [
                ('a', [('x', 1)].into()),
                ('b', [('x', 1), ('y', 2)].into()),
                ('c', [('x', 1)].into()),
            ]
            .into(),
        );
        assert_eq!(store.n_docs(), 4);
    }

    #[test]
    fn test_relevance_store_get_score() {
        let store = RelevanceStore::from_map([('a', [('x', 1)].into())].into());
        assert_eq!(store.get_score(&'a', &'x'), Some(&1));
        assert_eq!(store.get_score(&'a', &'y'), None);
        assert_eq!(store.get_score(&'b', &'x'), None);
    }

    #[test]
    fn test_relevance_store_get_map() {
        let store = RelevanceStore::from_map([('a', [('x', 1), ('y', 2)].into())].into());
        assert_eq!(store.get_map(&'a'), Some(&[('x', 1), ('y', 2)].into()));
        assert_eq!(store.get_map(&'b'), None);
    }

    #[test]
    fn test_relevance_store_get_sorted() {
        let store = RelevanceStore::from_map([('a', [('x', 1), ('y', 2)].into())].into());
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
        let store = RelevanceStore::from_map(
            [
                ('a', [('x', 1)].into()),
                ('b', [('x', 1)].into()),
                ('c', [('x', 1)].into()),
            ]
            .into(),
        );
        let expected = HashSet::from_iter([&'a', &'b', &'c']);
        let actual = store.query_ids().collect::<HashSet<_>>();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_relevance_store_builder() {
        let mut b = RelevanceStoreBuilder::new();
        b.add_score('a', 'x', 1).unwrap();
        b.add_score('a', 'y', 2).unwrap();
        b.add_score('b', 'x', 3).unwrap();
        let store = b.build();
        assert_eq!(store.get_map(&'a'), Some(&[('x', 1), ('y', 2)].into()));
        assert_eq!(store.get_map(&'b'), Some(&[('x', 3)].into()));
        assert_eq!(store.get_map(&'c'), None);
    }

    #[test]
    fn test_relevance_store_builder_duplicate_entry() {
        let mut b = RelevanceStoreBuilder::new();
        b.add_score('a', 'x', 1).unwrap();
        assert_eq!(
            b.add_score('a', 'x', 2),
            Err(ElinorError::DuplicateEntry("Query: a, Doc: x".to_string()))
        );
    }
}
