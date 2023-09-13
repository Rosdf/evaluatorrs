use crate::formulas::root_formula::RootFormula;
use crate::formulas::{
    Evaluate, EvaluationError, Function, FunctionLike, IsConst, MathError, ParserError,
};
use crate::function_stores::GetFunction;
use crate::lib::boxed::Box;
use crate::lib::sync::Arc;
use crate::variable_stores::{GetVariable, Variable};

#[cfg(feature = "libm")]
fn sin_function(argument: f64) -> f64 {
    libm::Libm::<f64>::sin(argument)
}

#[cfg(feature = "std")]
fn sin_function(argument: f64) -> f64 {
    argument.sin()
}

/// Function for calculating `Sin` of argument.
#[derive(Debug)]
pub struct Sin {
    argument: RootFormula,
}

impl IsConst for Sin {
    fn is_const(&self) -> bool {
        self.argument.is_const()
    }
}

impl Evaluate for Sin {
    fn eval(&self, args: &dyn GetVariable) -> Result<f64, EvaluationError> {
        Ok(sin_function(self.argument.eval(args)?))
    }
}

impl FunctionLike for Sin {
    fn collapse_inner(&mut self) -> Result<(), MathError> {
        self.argument.collapse_inner()
    }

    #[allow(clippy::semicolon_if_nothing_returned)]
    #[inline]
    fn set_all_variables_shared(&mut self, args: &dyn GetVariable) {
        self.argument.set_all_variables_shared(args)
    }

    #[allow(clippy::semicolon_if_nothing_returned)]
    #[inline]
    fn set_all_variables_owned(&mut self, args: &dyn GetVariable) {
        self.argument.set_all_variables_owned(args)
    }

    #[allow(clippy::semicolon_if_nothing_returned)]
    #[inline]
    fn set_variable_shared(&mut self, name: &Variable, function: &Arc<RootFormula>) {
        self.argument.set_variable_shared(name, function)
    }

    #[allow(clippy::semicolon_if_nothing_returned)]
    #[inline]
    fn set_variable_owned(&mut self, name: &Variable, function: &RootFormula) {
        self.argument.set_variable_owned(name, function)
    }

    fn clone_into_box(&self) -> Box<dyn FunctionLike> {
        Box::new(Self {
            argument: self.argument.clone(),
        })
    }
}

impl Function for Sin {
    const MIN_NUMBER_OF_ARGUMENTS: usize = 1;
    const MAX_NUMBER_OF_ARGUMENTS: usize = 1;
    const NAME: &'static str = "sin";

    fn parse<T: for<'a> GetFunction<'a>>(
        arguments: &[&str],
        formulas: &T,
    ) -> Result<Self, ParserError>
    where
        Self: Sized,
    {
        Ok(Self {
            argument: RootFormula::parse(arguments[0], formulas)?,
        })
    }
}
