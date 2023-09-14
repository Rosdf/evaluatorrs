use crate::__lib::sync::Arc;
use crate::formulas::RootFormula;
use crate::variable_stores::{GetVariable, PopVariable, SetVariable, Variable};

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

impl PopVariable for EmptyVariableStore {
    fn pop(&mut self, _variable: &Variable) -> Option<Arc<RootFormula>> {
        None
    }
}

#[cfg(test)]
mod test {
    use crate::variable_stores::{
        EmptyVariableStore, GetVariable, PopVariable, SetVariable, Variable,
    };

    #[test]
    fn test_set_and_get() {
        let var = Variable::new("a");
        EmptyVariableStore.set(var.clone(), 1.0);
        assert!(EmptyVariableStore.get(&var).is_none());
    }

    #[test]
    fn test_set_and_pop() {
        let var = Variable::new("a");
        EmptyVariableStore.set(var.clone(), 1.0);
        assert!(EmptyVariableStore.pop(&var).is_none());
    }
}
