use std::collections::HashMap;

pub(crate) trait GetVariable {
    fn get(&self, name: &str) -> Option<f64>;
}

pub struct HashMapStore(HashMap<String, f64>);

impl HashMapStore {
    pub fn new() -> Self {
        Self { 0: HashMap::new() }
    }
}

impl GetVariable for HashMapStore {
    fn get(&self, name: &str) -> Option<f64> {
        self.0.get(name).copied()
    }
}
