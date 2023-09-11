mod empty_store;
mod hashmap_store;

use crate::formulas::RootFormula;
use crate::formulas::{Evaluate, EvaluationError, IsConst, NoVariableError};
pub use crate::variable_stores::empty_store::EmptyVariableStore;
pub use crate::variable_stores::hashmap_store::HashMapVariableStore;
use std::fmt::{Display, Formatter};
use std::sync::Arc;

/// Type that stored in variable store as "key"
#[derive(Debug, PartialEq, Hash, Eq, Clone)]
pub struct Variable(String);

impl Display for Variable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Variable {
    /// Methode for creating new Variable
    #[inline]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

impl IsConst for Variable {
    #[inline]
    fn is_const(&self) -> bool {
        false
    }
}

impl Evaluate for Variable {
    #[inline]
    fn eval(&self, args: &dyn GetVariable) -> Result<f64, EvaluationError> {
        args.eval(self)
    }
}

impl<T: Into<String>> From<T> for Variable {
    #[inline]
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl AsRef<str> for Variable {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Trait for obtaining variable values.
pub trait GetVariable {
    /// Returns a reference to the function corresponding to the variable.
    fn get(&self, name: &Variable) -> Option<&Arc<RootFormula>>;

    /// Helper methode for converting self into trait object.
    fn as_dyn(&self) -> &dyn GetVariable;

    /// Evaluates variable with self as variable store.
    ///
    /// # Errors
    ///
    ///Will return Err if name is not in store or formula in store errors.
    fn eval(&self, name: &Variable) -> Result<f64, EvaluationError> {
        self.get(name).map_or_else(
            || {
                Err(EvaluationError::NoVariableError(NoVariableError::new(
                    name.clone(),
                )))
            },
            |formula| formula.eval(self.as_dyn()),
        )
    }
}

/// Trait for setting variables in variable store.
pub trait SetVariable {
    /// Sets variable by 'value'.
    fn set(&mut self, name: impl Into<Variable>, value: impl Into<RootFormula>);
}
