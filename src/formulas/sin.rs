use crate::formula_stores::GetParser;
use crate::formulas::root_formula::RootFormula;
use crate::formulas::{
    Evaluate, EvaluationError, Function, FunctionLike, IsConst, MathError, ParserError,
};
use crate::variable_stores::GetVariable;

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
        Ok(self.argument.eval(args)?.sin())
    }
}

impl FunctionLike for Sin {
    fn collapse_inner(&mut self) -> Result<(), MathError> {
        self.argument.collapse_inner()
    }

    fn set_variables_shared(&mut self, args: &dyn GetVariable) {
        self.argument.set_variables_shared(args);
    }
}

impl Function for Sin {
    const MIN_NUMBER_OF_ARGUMENTS: usize = 1;
    const MAX_NUMBER_OF_ARGUMENTS: usize = 1;
    const NAME: &'static str = "sin";

    fn parse<T: GetParser>(arguments: &[&str], formulas: &T) -> Result<Self, ParserError>
    where
        Self: Sized,
    {
        Ok(Self {
            argument: RootFormula::parse(arguments[0], formulas)?,
        })
    }
}
