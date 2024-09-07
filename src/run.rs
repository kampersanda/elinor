use hashbrown::HashMap;

use crate::EmirError;
use crate::Relevance;
use crate::RelevanceMap;

pub struct Run {
    map: HashMap<String, Vec<Relevance<f64>>>,
    name: Option<String>,
}

impl Run {
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn get_preds(&self, query_id: &str) -> Option<&[Relevance<f64>]> {
        self.map.get(query_id).map(|v| v.as_slice())
    }
}

pub struct RunBuilder {
    map: HashMap<String, RelevanceMap<f64>>,
    name: Option<String>,
}

impl RunBuilder {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            name: None,
        }
    }

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
    /// * [`EmirError::DuplicateEntry`] if the query and document identifiers already exist.
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
            return Err(EmirError::DuplicateEntry(query_id, doc_id));
        }
        self.map
            .entry(query_id)
            .or_insert_with(RelevanceMap::new)
            .insert(doc_id, score);
        Ok(())
    }
}
