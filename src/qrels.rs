use crate::errors::EmirError;
use crate::Relevance;
use crate::RelevanceMap;

use std::collections::HashMap;

pub struct Qrels {
    // Name of the qrels.
    name: Option<String>,

    // Mapping from query identifiers to:
    //  - Sorted list of relevance scores in descending order.
    //  - Mapping from document identifiers to relevance scores.
    map: HashMap<String, (Vec<Relevance<i32>>, RelevanceMap<i32>)>,
}

impl Qrels {
    /// Returns the name of the qrels.
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Returns the mapping from document identifiers to relevance scores
    /// for a given query identifier.
    pub fn get_rel_map(&self, query_id: &str) -> Option<&RelevanceMap<i32>> {
        self.map.get(query_id).map(|(_, rels)| rels)
    }

    /// Returns the sorted list of relevance scores in descending order
    /// for a given query identifier.
    pub fn get_sorted_rels(&self, query_id: &str) -> Option<&[Relevance<i32>]> {
        self.map.get(query_id).map(|(rels, _)| rels.as_slice())
    }

    /// Returns an iterator over the query identifiers in random order.
    pub fn query_ids(&self) -> impl Iterator<Item = &String> {
        self.map.keys()
    }

    /// Returns an iterator over the query identifiers and their relevance maps.
    pub fn query_ids_and_rel_maps(&self) -> impl Iterator<Item = (&String, &RelevanceMap<i32>)> {
        self.map.iter().map(|(k, (_, v))| (k, v))
    }

    /// Returns an iterator over the query identifiers and their sorted relevance scores.
    pub fn query_ids_and_sorted_rels(&self) -> impl Iterator<Item = (&String, &[Relevance<i32>])> {
        self.map.iter().map(|(k, (v, _))| (k, v.as_slice()))
    }

    /// Creates a qrels from a map of query identifiers to relevance maps.
    pub fn from_map(name: Option<String>, map: HashMap<String, RelevanceMap<i32>>) -> Self {
        let b = QrelsBuilder { name, map };
        b.build()
    }
}

pub struct QrelsBuilder {
    name: Option<String>,
    map: HashMap<String, RelevanceMap<i32>>,
}

impl QrelsBuilder {
    pub fn new() -> Self {
        Self {
            name: None,
            map: HashMap::new(),
        }
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// Adds a relevance score to the qrels.
    ///
    /// # Arguments
    ///
    /// * `query_id` - Query identifier.
    /// * `doc_id` - Document identifier.
    /// * `score` - Relevance score.
    ///
    /// # Errors
    ///
    /// * [`EmirError::DuplicateQueryDoc`] if the query and document identifiers already exist.
    pub fn add_score(
        &mut self,
        query_id: String,
        doc_id: String,
        score: i32,
    ) -> Result<(), EmirError> {
        if self
            .map
            .get(&query_id)
            .map_or(false, |m| m.contains_key(&doc_id))
        {
            return Err(EmirError::DuplicateQueryDoc(query_id, doc_id));
        }
        self.map
            .entry(query_id)
            .or_insert_with(RelevanceMap::new)
            .insert(doc_id, score);
        Ok(())
    }

    /// Builds the qrels.
    pub fn build(self) -> Qrels {
        let name = self.name;
        let mut map = HashMap::new();
        for (query_id, rels) in self.map {
            let mut sorted = rels
                .iter()
                .map(|(&doc_id, &score)| Relevance { doc_id, score })
                .collect::<Vec<_>>();
            sorted.sort_by(|a, b| b.score.cmp(&a.score));
            map.insert(query_id, (sorted, rels));
        }
        Qrels { name, map }
    }
}
