use crate::variable_stores::GetVariable;
use std::collections::{HashMap, VecDeque};
use std::fmt::{Debug, Formatter};
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
    fn eval_dyn(&mut self, args: &dyn GetVariable) -> f64 {
        self.eval(args)
    }
    fn eval<T: GetVariable>(&mut self, args: &T) -> f64
    where
        Self: Sized;
}

pub(crate) trait IsConst {
    fn is_const(&self) -> bool;
}

pub(crate) trait Formula: Evaluate + Debug {
    fn parse(arguments: Vec<String>) -> Result<Self, ParserError>
    where
        Self: Sized;
    fn num_of_arguments() -> usize
    where
        Self: Sized;

    fn is_const(&self) -> bool;
}

#[derive(Debug, PartialEq)]
enum Side {
    Left,
    Right,
}

#[derive(Debug)]
pub(crate) enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
}
