use crate::formulas::root_formula::RootFormula;
use crate::variable_stores::{GetVariable, Variable};
use std::sync::Arc;

pub struct EmptyVariableStore;

impl GetVariable for EmptyVariableStore {
    fn get(&self, _name: &Variable) -> Option<&Arc<RootFormula>> {
        None
    }

    fn as_dyn(&self) -> &dyn GetVariable {
        self
    }

    fn as_dyn_mut(&mut self) -> &mut dyn GetVariable {
        self
    }
}
