use std::collections::HashMap;

use crate::errors::EmirError;
use crate::Relevance;
use crate::RelevanceMap;

pub struct Run {
    // Name of the run.
    name: Option<String>,

    // Mapping from query identifiers to sorted list of relevance scores in descending order.
    map: HashMap<String, Vec<Relevance<f64>>>,
}

impl Run {
    /// Returns the name of the run.
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Returns the sorted list of relevance scores in descending order for a given query identifier.
    pub fn get_sorted_rels(&self, query_id: &str) -> Option<&[Relevance<f64>]> {
        self.map.get(query_id).map(|v| v.as_slice())
    }

    /// Returns an iterator over the query identifiers in random order.
    pub fn query_ids(&self) -> impl Iterator<Item = &String> {
        self.map.keys()
    }

    /// Returns an iterator over the query identifiers and their sorted relevance scores.
    pub fn query_ids_and_sorted_rels(&self) -> impl Iterator<Item = (&String, &[Relevance<f64>])> {
        self.map.iter().map(|(k, v)| (k, v.as_slice()))
    }

    /// Creates a run from a map of query identifiers to relevance maps.
    pub fn from_map(name: Option<String>, map: HashMap<String, RelevanceMap<f64>>) -> Self {
        let b = RunBuilder { name, map };
        b.build()
    }
}

pub struct RunBuilder {
    name: Option<String>,
    map: HashMap<String, RelevanceMap<f64>>,
}

impl RunBuilder {
    /// Creates a new builder.
    pub fn new() -> Self {
        Self {
            name: None,
            map: HashMap::new(),
        }
    }

    /// Sets the name of the run.
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// Adds an estimated relevance score to the run.
    ///
    /// # Arguments
    ///
    /// * `query_id` - Query identifier.
    /// * `doc_id` - Document identifier.
    /// * `score` - Estimated relevance score.
    ///
    /// # Errors
    ///
    /// * [`EmirError::DuplicateQueryDoc`]
    pub fn add_score(
        &mut self,
        query_id: String,
        doc_id: String,
        score: f64,
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

    /// Builds the run.
    pub fn build(self) -> Run {
        let name = self.name;
        let mut map = HashMap::new();
        for (query_id, rels) in self.map {
            let mut sorted = rels
                .into_iter()
                .map(|(doc_id, score)| Relevance { doc_id, score })
                .collect::<Vec<_>>();
            sorted.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
            map.insert(query_id, sorted);
        }
        Run { name, map }
    }
}
