use crate::__lib::boxed::Box;
use crate::__lib::sync::Arc;
use crate::formulas::Function;
use crate::formulas::RootFormula;
use crate::function_stores::{ArgumentBounds, GetFunction, Parser, RegisterParser};
use crate::variable_stores::{GetVariable, PopVariable, SetVariable, Variable};

/// Struct for interacting with variable store and function store.
#[derive(Debug, Default, Clone)]
pub struct Context<T, U> {
    variable_store: T,
    function_store: U,
}

impl<T, U> Context<T, U> {
    /// Creates new `Context`.
    #[inline]
    pub const fn new(variable_store: T, function_store: U) -> Self {
        Self {
            variable_store,
            function_store,
        }
    }
}

impl<T: GetVariable, U> GetVariable for Context<T, U> {
    #[inline]
    fn get(&self, name: &Variable) -> Option<&Arc<RootFormula>> {
        self.variable_store.get(name)
    }

    #[inline]
    fn as_dyn(&self) -> &dyn GetVariable {
        self
    }
}

impl<T: SetVariable, U> SetVariable for Context<T, U> {
    #[allow(clippy::semicolon_if_nothing_returned)]
    #[inline]
    fn set(&mut self, name: impl Into<Variable>, value: impl Into<RootFormula>) {
        self.variable_store.set(name, value)
    }
}

impl<'a, T, U> GetFunction<'a> for Context<T, U>
where
    U: GetFunction<'a>,
{
    type Iter = U::Iter;

    #[inline]
    fn function_parser<'b>(
        &'b self,
        formula_name: &str,
    ) -> Option<(Box<Parser<'b>>, ArgumentBounds)> {
        self.function_store.function_parser(formula_name)
    }

    #[inline]
    fn iter(&'a self) -> Self::Iter {
        self.function_store.iter()
    }
}

impl<T, U: RegisterParser> RegisterParser for Context<T, U> {
    #[allow(clippy::semicolon_if_nothing_returned)]
    #[inline]
    fn register<W: Function + 'static>(&mut self) {
        self.function_store.register::<W>()
    }
}

impl<T: PopVariable, U> PopVariable for Context<T, U> {
    #[inline]
    fn pop(&mut self, variable: &Variable) -> Option<Arc<RootFormula>> {
        self.variable_store.pop(variable)
    }
}
