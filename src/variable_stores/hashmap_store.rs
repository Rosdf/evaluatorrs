use crate::formulas::root_formula::RootFormula;
use crate::variable_stores::{GetVariable, SetVariable, Variable};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Default, Debug)]
pub struct HashMapStore(HashMap<Variable, Arc<RootFormula>>);

impl HashMapStore {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}

impl GetVariable for HashMapStore {
    fn get(&self, name: &Variable) -> Option<&Arc<RootFormula>> {
        self.0.get(name)
    }

    fn as_dyn(&self) -> &dyn GetVariable {
        self
    }

    fn as_dyn_mut(&mut self) -> &mut dyn GetVariable
    where
        Self: Sized,
    {
        self
    }
}

impl SetVariable for HashMapStore {
    fn set(&mut self, name: impl Into<Variable>, value: RootFormula) {
        self.0.insert(name.into(), Arc::new(value));
    }
}
