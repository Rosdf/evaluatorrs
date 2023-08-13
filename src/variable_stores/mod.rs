mod empty_store;
mod hashmap_store;

use crate::formulas::root_formula::RootFormula;
use crate::formulas::{Evaluate, EvaluationError, IsConst};
pub use crate::variable_stores::empty_store::EmptyVariableStore;
pub use crate::variable_stores::hashmap_store::HashMapStore;
use std::sync::Arc;

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
pub struct Variable(String);

impl Variable {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

impl IsConst for Variable {
    fn is_const(&self) -> bool {
        false
    }
}

impl Evaluate for Variable {
    fn eval(&self, args: &dyn GetVariable) -> Result<f64, EvaluationError> {
        args.eval(self)
    }
}

impl<T: Into<String>> From<T> for Variable {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl AsRef<str> for Variable {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

pub trait GetVariable {
    fn get(&self, name: &Variable) -> Option<&Arc<RootFormula>>;
    fn as_dyn(&self) -> &dyn GetVariable;
    fn as_dyn_mut(&mut self) -> &mut dyn GetVariable;

    /// # Errors
    ///
    ///Will return Err if name is not in store or formula in store errors
    fn eval(&self, name: &Variable) -> Result<f64, EvaluationError> {
        self.get(name)
            .map_or(Err(EvaluationError::NoVariableError), |formula| {
                formula.eval(self.as_dyn())
            })
    }
}

pub trait SetVariable: GetVariable {
    fn set(&mut self, name: impl Into<Variable>, value: RootFormula);
}
