use hashbrown::HashMap;

pub struct Run {
    name: String,
    run_map: HashMap<String, Vec<(String, f64)>>,
}

pub struct RunBuilder {
    run_map: HashMap<String, HashMap<String, f64>>,
}

impl RunBuilder {}
