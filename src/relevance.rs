use std::collections::HashMap;
use std::hash::Hash;

use crate::errors::EmirError;

/// Data to store a relevance score for a document.
#[derive(Debug, Clone)]
pub struct Relevance<K, T> {
    pub doc_id: K,
    pub score: T,
}

/// Mapping from document ids to relevance scores.
pub type RelevanceMap<K, T> = HashMap<K, T>;

/// Data structure for storing relevance scores for a given query id.
pub struct RelevanceStore<K, T> {
    // Name.
    name: Option<String>,

    // Mapping from query ids to:
    //  - Sorted list of relevance scores in descending order.
    //  - Mapping from document ids to relevance scores.
    map: HashMap<K, (Vec<Relevance<K, T>>, RelevanceMap<K, T>)>,
}

impl<K, T> RelevanceStore<K, T>
where
    K: Eq + PartialEq + Hash + Clone + std::fmt::Display,
    T: Ord + PartialOrd + Clone,
{
    /// Returns the name of the relevance store.
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Sets the name of the relevance store.
    pub fn with_name(self, name: &str) -> Self {
        Self {
            name: Some(name.to_string()),
            ..self
        }
    }

    /// Returns the relevance map for a given query id.
    pub fn get_map(&self, query_id: &K) -> Option<&RelevanceMap<K, T>> {
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

    /// Returns an iterator over the query ids and their relevance maps.
    pub fn query_ids_and_maps(&self) -> impl Iterator<Item = (&K, &RelevanceMap<K, T>)> {
        self.map.iter().map(|(k, (_, v))| (k, v))
    }

    /// Returns an iterator over the query ids and their sorted relevance scores.
    pub fn query_ids_and_sorted(&self) -> impl Iterator<Item = (&K, &[Relevance<K, T>])> {
        self.map.iter().map(|(k, (v, _))| (k, v.as_slice()))
    }

    /// Creates a relevance store from a map of query ids to relevance maps.
    pub fn from_map(map: HashMap<K, RelevanceMap<K, T>>) -> Self {
        let b = RelevanceStoreBuilder { map };
        b.build()
    }
}

/// Builder for creating a relevance store.
pub struct RelevanceStoreBuilder<K, T> {
    map: HashMap<K, RelevanceMap<K, T>>,
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
    /// * [`EmirError::DuplicateDocId`] if the document id already exists for the query.
    pub fn add_score(&mut self, query_id: K, doc_id: K, score: T) -> Result<(), EmirError<K>> {
        let rels = self
            .map
            .entry(query_id.clone())
            .or_insert_with(HashMap::new);
        if rels.contains_key(&doc_id) {
            return Err(EmirError::DuplicateDocId(query_id, doc_id));
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
