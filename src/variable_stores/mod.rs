mod hashmap_store;
pub use crate::variable_stores::hashmap_store::HashMapStore;

pub trait GetVariable {
    fn get(&self, name: &str) -> Option<f64>;
    fn as_dyn(&self) -> &dyn GetVariable;
}

pub trait SetVariable: GetVariable {
    fn set(&mut self, name: String, value: f64);
}

pub trait SetInnerVariable: GetVariable {
    fn set(&self, name: String, value: f64);
}
