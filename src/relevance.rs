//! Relevance store.
use std::collections::HashMap;
use std::hash::Hash;

use crate::errors::EmirError;

/// Data to store a relevance score for a document.
#[derive(Debug, Clone, PartialEq)]
pub struct Relevance<K, T> {
    /// Document id.
    pub doc_id: K,

    /// Relevance score.
    pub score: T,
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
    map: HashMap<K, (Vec<Relevance<K, T>>, HashMap<K, T>)>,
}

impl<K, T> RelevanceStore<K, T>
where
    K: Eq + PartialEq + Hash + Clone + std::fmt::Display,
    T: Eq + PartialEq + Ord + PartialOrd + Clone,
{
    /// Creates a relevance store from a map of query ids to relevance maps.
    pub fn from_map(map: HashMap<K, HashMap<K, T>>) -> Self {
        let b = RelevanceStoreBuilder { map };
        b.build()
    }

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

    /// Returns the relevance map for a given query id.
    pub fn get_map(&self, query_id: &K) -> Option<&HashMap<K, T>> {
        self.map.get(query_id).map(|(_, rels)| rels)
    }

    /// Returns the sorted list of relevance scores in descending order
    /// for a given query id.
    pub fn get_sorted(&self, query_id: &K) -> Option<&[Relevance<K, T>]> {
        self.map.get(query_id).map(|(rels, _)| rels.as_slice())
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

impl<K, T> RelevanceStoreBuilder<K, T>
where
    K: Eq + PartialEq + Hash + Clone + std::fmt::Display,
    T: Ord + PartialOrd + Clone,
{
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
    /// * [`EmirError::DuplicateEntry`] if the query-document pair already exists.
    pub fn add_score(&mut self, query_id: K, doc_id: K, score: T) -> Result<(), EmirError> {
        let rels = self
            .map
            .entry(query_id.clone())
            .or_insert_with(HashMap::new);
        if rels.contains_key(&doc_id) {
            return Err(EmirError::DuplicateEntry(format!(
                "Query: {query_id}, Doc: {doc_id}"
            )));
        }
        rels.insert(doc_id, score);
        Ok(())
    }

    /// Builds the relevance store.
    pub fn build(self) -> RelevanceStore<K, T> {
        let mut map = HashMap::new();
        for (query_id, rels) in self.map {
            let mut sorted = rels
                .iter()
                .map(|(doc_id, score)| Relevance {
                    doc_id: doc_id.clone(),
                    score: score.clone(),
                })
                .collect::<Vec<_>>();
            sorted.sort_by(|a, b| b.score.cmp(&a.score));
            map.insert(query_id, (sorted, rels));
        }
        RelevanceStore { name: None, map }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_relevance_store_name() {
        let store = RelevanceStore::from_map([('a', [('x', 1)].into())].into());
        assert_eq!(store.name(), None);
        let store = store.with_name("test");
        assert_eq!(store.name(), Some("test"));
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
    fn test_relevance_store_builder_duplicate_entry() {
        let mut b = RelevanceStoreBuilder::new();
        b.add_score('a', 'x', 1).unwrap();
        assert_eq!(
            b.add_score('a', 'x', 2),
            Err(EmirError::DuplicateEntry("Query: a, Doc: x".to_string()))
        );
    }
}
