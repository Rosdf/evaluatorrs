use crate::formulas::root_formula::RootFormula;
use crate::formulas::{
    Evaluate, EvaluationError, Function, FunctionLike, IsConst, MathError, ParserError,
};
use crate::function_stores::GetFunction;
use crate::lib::boxed::Box;
use crate::lib::sync::Arc;
use crate::lib::vec::Vec;
use crate::variable_stores::{GetVariable, Variable};

/// Function for calculating min of it's arguments.
#[derive(Debug)]
pub struct Min {
    arguments: Box<[RootFormula]>,
}

impl IsConst for Min {
    fn is_const(&self) -> bool {
        self.arguments.iter().all(IsConst::is_const)
    }
}

impl Evaluate for Min {
    fn eval(&self, args: &dyn GetVariable) -> Result<f64, EvaluationError> {
        let mut min = f64::MAX;
        for val in self.arguments.as_ref() {
            min = min.min(val.eval(args)?);
        }
        Ok(min)
    }
}

impl FunctionLike for Min {
    fn collapse_inner(&mut self) -> Result<(), MathError> {
        for val in self.arguments.as_mut() {
            val.collapse_inner()?;
        }
        Ok(())
    }

    fn set_all_variables_shared(&mut self, args: &dyn GetVariable) {
        for val in self.arguments.as_mut() {
            val.set_all_variables_shared(args);
        }
    }

    fn set_all_variables_owned(&mut self, args: &dyn GetVariable) {
        for val in self.arguments.as_mut() {
            val.set_all_variables_owned(args);
        }
    }

    fn set_variable_shared(&mut self, name: &Variable, function: &Arc<RootFormula>) {
        for val in self.arguments.as_mut() {
            val.set_variable_shared(name, function);
        }
    }

    fn set_variable_owned(&mut self, name: &Variable, function: &RootFormula) {
        for val in self.arguments.as_mut() {
            val.set_variable_owned(name, function);
        }
    }

    fn clone_into_box(&self) -> Box<dyn FunctionLike> {
        Box::new(Self {
            arguments: self.arguments.clone(),
        })
    }
}

impl Function for Min {
    const MIN_NUMBER_OF_ARGUMENTS: usize = 2;
    const MAX_NUMBER_OF_ARGUMENTS: usize = usize::MAX;
    const NAME: &'static str = "min";

    fn parse<T: for<'a> GetFunction<'a>>(
        arguments: &[&str],
        formulas: &T,
    ) -> Result<Self, ParserError>
    where
        Self: Sized,
    {
        let mut parsed_arguments = Vec::with_capacity(arguments.len());
        for argument in arguments {
            parsed_arguments.push(RootFormula::parse(argument, formulas)?);
        }
        Ok(Self {
            arguments: parsed_arguments.into_boxed_slice(),
        })
    }
}
