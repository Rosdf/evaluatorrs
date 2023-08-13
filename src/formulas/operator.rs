use crate::formulas::root_formula::RootFormula;
use crate::formulas::{Evaluate, EvaluationError, FunctionLike, IsConst, MathError};
use crate::tokens::Operator;
use crate::variable_stores::GetVariable;

#[derive(Debug)]
pub(crate) struct OperatorFormula {
    first: RootFormula,
    second: RootFormula,
    operator: Operator,
}

impl OperatorFormula {
    pub(crate) const fn new(first: RootFormula, second: RootFormula, operator: Operator) -> Self {
        Self {
            first,
            second,
            operator,
        }
    }
}

impl IsConst for OperatorFormula {
    fn is_const(&self) -> bool {
        self.first.is_const() && self.second.is_const()
    }
}

impl Evaluate for OperatorFormula {
    fn eval(&self, args: &dyn GetVariable) -> Result<f64, EvaluationError> {
        let first = self.first.eval(args)?;
        let second = self.second.eval(args)?;

        Ok(self.operator.eval(first, second))
    }
}

impl FunctionLike for OperatorFormula {
    fn collapse_inner(&mut self) -> Result<(), MathError> {
        self.first.collapse_inner()?;
        self.second.collapse_inner()?;
        Ok(())
    }

    fn set_variables_shared(&mut self, args: &dyn GetVariable) {
        self.first.set_variables_shared(args);
        self.second.set_variables_shared(args);
    }
}
