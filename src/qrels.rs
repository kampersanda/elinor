use crate::EmirError;
use crate::RelevanceMap;

use std::collections::HashMap;

pub struct Qrels {
    name: Option<String>,
    map: HashMap<String, RelevanceMap<i32>>,
}

impl Qrels {
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn get_rels(&self, query_id: &str) -> Option<&RelevanceMap<i32>> {
        self.map.get(query_id)
    }

    pub fn query_ids(&self) -> impl Iterator<Item = &String> {
        self.map.keys()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &RelevanceMap<i32>)> {
        self.map.iter()
    }

    pub fn from_map(name: Option<String>, map: HashMap<String, RelevanceMap<i32>>) -> Self {
        Self { map, name }
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
        Qrels {
            name: self.name,
            map: self.map,
        }
    }
}
