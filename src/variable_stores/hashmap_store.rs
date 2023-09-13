use crate::formulas::RootFormula;
use crate::variable_stores::{GetVariable, SetVariable, Variable};
// We can use std, because this module is not imported, when no_std feature is enabled.
use std::collections::HashMap;
use std::sync::Arc;

/// Variable store based on [`HashMap`].
#[derive(Debug, Clone, Default)]
pub struct HashMapVariableStore(HashMap<Variable, Arc<RootFormula>>);

impl HashMapVariableStore {
    /// Creates an empty `HashMapVariableStore`.
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}

impl GetVariable for HashMapVariableStore {
    fn get(&self, name: &Variable) -> Option<&Arc<RootFormula>> {
        self.0.get(name)
    }

    fn as_dyn(&self) -> &dyn GetVariable {
        self
    }
}

impl SetVariable for HashMapVariableStore {
    fn set(&mut self, name: impl Into<Variable>, value: impl Into<RootFormula>) {
        self.0.insert(name.into(), Arc::new(value.into()));
    }
}
