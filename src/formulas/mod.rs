use crate::variable_stores::GetVariable;
use std::fmt::Debug;
use std::str::FromStr;

pub(crate) mod base_formula;
pub(crate) mod sin;

#[derive(Debug)]
pub(crate) struct ExpressionParsingError {
    left_expression: String,
}

#[derive(Debug)]
pub(crate) struct ParenthesisError;

#[derive(Debug)]
pub(crate) struct OpernandNumError;

#[derive(Debug)]
pub(crate) enum ParserError {
    ExpressionParsingError(ExpressionParsingError),
    ParenthesisError(ParenthesisError),
    OpernandNumError(OpernandNumError),
}

pub(crate) trait Evaluate {
    fn eval_dyn(&mut self, args: &dyn GetVariable) -> f64;
    fn eval<T: GetVariable + ?Sized>(&mut self, args: &T) -> f64
    where
        Self: Sized;
}

pub(crate) trait IsConst {
    fn is_const(&self) -> bool;
}

pub(crate) trait FormulaLike: Evaluate + IsConst + Debug {}

pub(crate) trait Formula: FormulaLike {
    const MIN_NUMBER_OF_ARGUMENTS: usize;
    const MAX_NUMBER_OF_ARGUMENTS: usize;
    const NAME: &'static str;

    fn parse(arguments: &[&str]) -> Result<Self, ParserError>
    where
        Self: Sized;
}
