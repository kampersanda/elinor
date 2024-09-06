use hashbrown::HashMap;

pub struct Qrels {
    map: HashMap<String, HashMap<String, usize>>,
}

impl Qrels {
    pub fn get_rels_map(&self, query_id: &str) -> Option<&HashMap<String, usize>> {
        self.map.get(query_id)
    }
}

pub struct QrelsBuilder {
    map: HashMap<String, HashMap<String, usize>>,
}

impl QrelsBuilder {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
}
