use crate::formulas::RootFormula;
use crate::lib::sync::Arc;
use crate::variable_stores::{GetVariable, SetVariable, Variable};

/// Variable store that can not contain any functions.
#[derive(Debug, Copy, Clone)]
pub struct EmptyVariableStore;

impl GetVariable for EmptyVariableStore {
    fn get(&self, _name: &Variable) -> Option<&Arc<RootFormula>> {
        None
    }

    fn as_dyn(&self) -> &dyn GetVariable {
        self
    }
}

impl SetVariable for EmptyVariableStore {
    fn set(&mut self, _name: impl Into<Variable>, _value: impl Into<RootFormula>) {}
}
