use crate::variable_stores::{GetVariable, SetVariable};
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct HashMapStore(HashMap<String, f64>);

impl HashMapStore {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}

impl GetVariable for HashMapStore {
    fn get(&self, name: &str) -> Option<f64> {
        self.0.get(name).copied()
    }

    fn as_dyn(&self) -> &dyn GetVariable {
        self
    }
}

impl SetVariable for HashMapStore {
    fn set(&mut self, name: String, value: f64) {
        self.0.insert(name, value);
    }
}
