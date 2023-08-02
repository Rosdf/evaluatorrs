use crate::variable_stores::GetVariable;
use std::fmt::Debug;

pub mod base_formula;
pub mod sin;

#[derive(Debug)]
pub struct ExpressionParsingError;

#[derive(Debug)]
pub struct ParenthesisError;

#[derive(Debug)]
pub struct OpernandNumError;

#[derive(Debug)]
pub enum ParserError {
    ExpressionParsingError(ExpressionParsingError),
    ParenthesisError(ParenthesisError),
    OpernandNumError(OpernandNumError),
}

pub trait Evaluate {
    fn eval_dyn(&mut self, args: &dyn GetVariable) -> f64;
    fn eval<T: GetVariable + ?Sized>(&mut self, args: &T) -> f64
    where
        Self: Sized;
}

pub trait IsConst {
    fn is_const(&self) -> bool;
}

pub trait FormulaLike: Evaluate + IsConst + Debug {}

pub trait Formula: FormulaLike {
    const MIN_NUMBER_OF_ARGUMENTS: usize;
    const MAX_NUMBER_OF_ARGUMENTS: usize;
    const NAME: &'static str;

    fn parse(arguments: &[&str]) -> Result<Self, ParserError>
    where
        Self: Sized;
}
