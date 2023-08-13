use crate::formula_stores::GetParser;
use crate::variable_stores::GetVariable;
use std::fmt::Debug;

pub(crate) mod operator;
pub mod root_formula;
pub mod sin;

#[derive(Debug)]
pub struct UnknownTokenError;

#[derive(Debug)]
pub struct ParenthesisError;

#[derive(Debug)]
pub struct OpernandNumError;

#[derive(Debug)]
pub struct MathError;

#[derive(Debug)]
#[non_exhaustive]
pub enum EvaluationError {
    MathError(MathError),
    NoVariableError,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum ParserError {
    UnknownTokenError(UnknownTokenError),
    ParenthesisError(ParenthesisError),
    OpernandNumError(OpernandNumError),
    EvaluationError(EvaluationError),
}

impl From<EvaluationError> for ParserError {
    fn from(value: EvaluationError) -> Self {
        Self::EvaluationError(value)
    }
}

impl From<ParenthesisError> for ParserError {
    fn from(value: ParenthesisError) -> Self {
        Self::ParenthesisError(value)
    }
}

impl From<OpernandNumError> for ParserError {
    fn from(value: OpernandNumError) -> Self {
        Self::OpernandNumError(value)
    }
}

impl From<UnknownTokenError> for ParserError {
    fn from(value: UnknownTokenError) -> Self {
        Self::UnknownTokenError(value)
    }
}

impl From<MathError> for ParserError {
    fn from(value: MathError) -> Self {
        Self::EvaluationError(EvaluationError::MathError(value))
    }
}

pub trait Evaluate {
    /// # Errors
    ///
    /// will return Err if failed to evaluate formula or variable is not in args
    fn eval(&self, args: &dyn GetVariable) -> Result<f64, EvaluationError>;
}

pub trait IsConst {
    fn is_const(&self) -> bool;
}

pub trait FunctionLike: Evaluate + IsConst + Debug {
    /// # Errors
    ///
    /// will return Err if got error on evaluation of inner formula
    fn collapse_inner(&mut self) -> Result<(), MathError>;
    fn set_variables_shared(&mut self, args: &dyn GetVariable);
}

impl IsConst for Box<dyn FunctionLike> {
    fn is_const(&self) -> bool {
        self.as_ref().is_const()
    }
}

impl Evaluate for Box<dyn FunctionLike> {
    fn eval(&self, args: &dyn GetVariable) -> Result<f64, EvaluationError> {
        self.as_ref().eval(args)
    }
}

impl FunctionLike for Box<dyn FunctionLike> {
    fn collapse_inner(&mut self) -> Result<(), MathError> {
        self.as_mut().collapse_inner()
    }
    fn set_variables_shared(&mut self, args: &dyn GetVariable) {
        self.as_mut().set_variables_shared(args);
    }
}

pub trait Function: FunctionLike {
    const MIN_NUMBER_OF_ARGUMENTS: usize;
    const MAX_NUMBER_OF_ARGUMENTS: usize;
    const NAME: &'static str;

    /// # Errors
    ///
    /// will return Err if expression is not a valid formula
    fn parse<T: GetParser>(arguments: &[&str], formulas: &T) -> Result<Self, ParserError>
    where
        Self: Sized;
}
