use crate::__lib::sync::Arc;
use crate::__lib::vec::Vec;
use crate::formulas::RootFormula;
use crate::variable_stores::{GetVariable, PopVariable, SetVariable, Variable};

/// Variable store based on [`Vec`] of tuples.
#[derive(Debug, Clone, Default)]
pub struct VectorVariableStore(Vec<(Variable, Arc<RootFormula>)>);

impl VectorVariableStore {
    /// Creates an empty `VectorVariableStore`.
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    /// Creates an empty `VectorVariableStore` with given capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }
}

impl GetVariable for VectorVariableStore {
    fn get(&self, name: &Variable) -> Option<&Arc<RootFormula>> {
        for (variable, formula) in &self.0 {
            if *variable == *name {
                return Some(formula);
            }
        }
        None
    }

    fn as_dyn(&self) -> &dyn GetVariable {
        self
    }
}

impl SetVariable for VectorVariableStore {
    fn set(&mut self, name: impl Into<Variable>, value: impl Into<RootFormula>) {
        self.0.push((name.into(), Arc::new(value.into())));
    }
}

impl PopVariable for VectorVariableStore {
    fn pop(&mut self, variable: &Variable) -> Option<Arc<RootFormula>> {
        let index = self.0.iter().position(|(x, _)| *x == *variable);
        index.map(|x| self.0.remove(x).1)
    }
}
