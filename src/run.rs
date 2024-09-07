use std::collections::HashMap;

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

    pub fn query_ids(&self) -> impl Iterator<Item = &String> {
        self.map.keys()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &[Relevance<f64>])> {
        self.map.iter().map(|(k, v)| (k, v.as_slice()))
    }
}

pub struct RunBuilder {
    map: HashMap<String, RelevanceMap<f64>>,
    name: Option<String>,
}

impl RunBuilder {
    /// Creates a new builder.
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            name: None,
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
        let mut map = HashMap::new();
        for (query_id, rels) in self.map {
            let mut rels = rels
                .into_iter()
                .map(|(doc_id, score)| Relevance { doc_id, score })
                .collect::<Vec<_>>();
            rels.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
            map.insert(query_id, rels);
        }
        let name = self.name;
        Run { map, name }
    }
}
