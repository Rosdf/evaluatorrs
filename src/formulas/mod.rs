use crate::__lib::boxed::Box;
#[cfg(any(feature = "std", nightly))]
use crate::__lib::error::Error;
use crate::__lib::fmt::{Debug, Display, Formatter};
use crate::__lib::string::String;
use crate::__lib::sync::Arc;
use crate::function_stores::GetFunction;
use crate::variable_stores::{GetVariable, Variable};

/// Provides macros for fast construction of functions.
#[macro_use]
pub mod macros;
/// Provides base mathematical functions.
pub mod math;
mod min;
pub(crate) mod operator;
mod root_formula;

pub use min::Min;
pub use root_formula::RootFormula;

/// The error type which is returned from parsing unknown token in expression.
#[derive(Debug)]
pub struct UnknownTokenError(String);

impl Display for UnknownTokenError {
    fn fmt(&self, f: &mut Formatter<'_>) -> crate::__lib::fmt::Result {
        write!(f, "Got unknown token {}", self.0)
    }
}

#[cfg(any(feature = "std", nightly))]
impl Error for UnknownTokenError {}

/// The error type which is returned when expression contains wrong number of opening and closing parenthesis.
#[derive(Debug)]
pub struct ParenthesisError;

impl Display for ParenthesisError {
    fn fmt(&self, f: &mut Formatter<'_>) -> crate::__lib::fmt::Result {
        write!(f, "Wrong number of parenthesis")
    }
}

#[cfg(any(feature = "std", nightly))]
impl Error for ParenthesisError {}

/// The error type which is returned when function gets wrong number of arguments.
#[derive(Debug)]
pub struct ArgumentsError(String);

impl Display for ArgumentsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> crate::__lib::fmt::Result {
        write!(f, "wrong number of arguments for {}", self.0)
    }
}

#[cfg(any(feature = "std", nightly))]
impl Error for ArgumentsError {}

/// The error type which is returned when function can not be evaluated.
/// This error meant to represent something totally wrong with evaluation, when even NAN can not be returned.
#[derive(Debug)]
pub struct MathError(String);

impl Display for MathError {
    fn fmt(&self, f: &mut Formatter<'_>) -> crate::__lib::fmt::Result {
        write!(f, "Failed to evaluate {}", self.0)
    }
}

#[cfg(any(feature = "std", nightly))]
impl Error for MathError {}

/// The error type which is returned when [`GetVariable`] does not contain variable, that is needed for formula evaluation.
#[derive(Debug)]
pub struct NoVariableError {
    name: Variable,
}

impl NoVariableError {
    /// Creates new `NoVariableError`.
    pub const fn new(name: Variable) -> Self {
        Self { name }
    }
}

impl Display for NoVariableError {
    fn fmt(&self, f: &mut Formatter<'_>) -> crate::__lib::fmt::Result {
        write!(f, "No variable with name {} in variable store", self.name)
    }
}

#[cfg(any(feature = "std", nightly))]
impl Error for NoVariableError {}

/// The error variants which are returned when failed to evaluate formula for any reason.
#[derive(Debug)]
#[non_exhaustive]
pub enum EvaluationError {
    /// Something wrong with math inside of formula.
    MathError(MathError),
    /// Some variable's value is not set.
    NoVariableError(NoVariableError),
}

impl Display for EvaluationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> crate::__lib::fmt::Result {
        match self {
            Self::MathError(e) => Display::fmt(e, f),
            Self::NoVariableError(e) => Display::fmt(e, f),
        }
    }
}

#[cfg(any(feature = "std", nightly))]
impl Error for EvaluationError {}

/// The error variants which are returned when failed to parse formula for any reason.
#[derive(Debug)]
#[non_exhaustive]
pub enum ParserError {
    /// Got unknown token.
    UnknownTokenError(UnknownTokenError),
    /// Expression has unmatched parenthesis.
    ParenthesisError(ParenthesisError),
    /// Function has wrong number of arguments.
    ArgumentsError(ArgumentsError),
    /// Failed to evaluate constant function.
    EvaluationError(EvaluationError),
}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> crate::__lib::fmt::Result {
        match self {
            Self::UnknownTokenError(e) => Display::fmt(e, f),
            Self::ParenthesisError(e) => Display::fmt(e, f),
            Self::ArgumentsError(e) => Display::fmt(e, f),
            Self::EvaluationError(e) => Display::fmt(e, f),
        }
    }
}

#[cfg(any(feature = "std", nightly))]
impl Error for ParserError {}

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

impl From<ArgumentsError> for ParserError {
    fn from(value: ArgumentsError) -> Self {
        Self::ArgumentsError(value)
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

/// Trait for evaluating structs.
pub trait Evaluate {
    /// Evaluates value of formula, args contains variables.
    ///
    /// # Errors
    ///
    /// Will return Err if failed to evaluate formula or variable is not in args.
    fn eval(&self, args: &dyn GetVariable) -> Result<f64, EvaluationError>;
}

/// Trait allows to determine if formula is constant.
pub trait IsConst {
    /// Returns `true` if formula returns same value for every .eval(...) call, else returns `false`.
    fn is_const(&self) -> bool;
}

/// Trait provides base functionality for functions.
pub trait FunctionLike: Evaluate + IsConst + Debug + Send + Sync {
    /// Simplifies function arguments. For example if one of the function arguments is "1 + 2",
    /// this method will simplify it to "3"
    ///
    /// # Errors
    ///
    /// will return Err if got error on evaluation of inner formula
    fn collapse_inner(&mut self) -> Result<(), MathError>;
    /// Sets variables present in function to formulas stored in `args` as shared function.
    /// If function has some inner state and set as shared in different formulas, inner state will be shared.
    fn set_all_variables_shared(&mut self, args: &dyn GetVariable);
    /// Sets variables present in function to formulas stored in `args` as owned function.
    /// If function has some inner state and set as owned in different formulas, inner state will be different.
    fn set_all_variables_owned(&mut self, args: &dyn GetVariable);
    /// Sets variable `name` in function to `function` as shared function.
    /// If function has some inner state and set as shared in different formulas, inner state will be shared.
    fn set_variable_shared(&mut self, name: &Variable, function: &Arc<RootFormula>);
    /// Sets variable `name` in function to `function` as owned function.
    /// If function has some inner state and set as owned in different formulas, inner state will be different.
    fn set_variable_owned(&mut self, name: &Variable, function: &RootFormula);
    /// Creates clone of object stored in [`Box`] as trait object
    fn clone_into_box(&self) -> Box<dyn FunctionLike>;
}

impl IsConst for Box<dyn FunctionLike> {
    #[inline]
    fn is_const(&self) -> bool {
        self.as_ref().is_const()
    }
}

impl Evaluate for Box<dyn FunctionLike> {
    #[inline]
    fn eval(&self, args: &dyn GetVariable) -> Result<f64, EvaluationError> {
        self.as_ref().eval(args)
    }
}

impl FunctionLike for Box<dyn FunctionLike> {
    fn collapse_inner(&mut self) -> Result<(), MathError> {
        self.as_mut().collapse_inner()
    }
    #[allow(clippy::semicolon_if_nothing_returned)]
    #[inline]
    fn set_all_variables_shared(&mut self, args: &dyn GetVariable) {
        self.as_mut().set_all_variables_shared(args)
    }
    #[allow(clippy::semicolon_if_nothing_returned)]
    #[inline]
    fn set_all_variables_owned(&mut self, args: &dyn GetVariable) {
        self.as_mut().set_all_variables_owned(args)
    }
    #[allow(clippy::semicolon_if_nothing_returned)]
    #[inline]
    fn set_variable_shared(&mut self, name: &Variable, function: &Arc<RootFormula>) {
        self.as_mut().set_variable_shared(name, function)
    }
    #[allow(clippy::semicolon_if_nothing_returned)]
    #[inline]
    fn set_variable_owned(&mut self, name: &Variable, function: &RootFormula) {
        self.as_mut().set_variable_owned(name, function);
    }

    #[inline]
    fn clone_into_box(&self) -> Box<dyn FunctionLike> {
        self.as_ref().clone_into_box()
    }
}

/// Trait provides methods to parse [`&str`] into [`FunctionLike`].
pub trait Function: FunctionLike {
    /// Minimal number of function arguments.
    const MIN_NUMBER_OF_ARGUMENTS: usize;
    /// Maximum number of function arguments.
    const MAX_NUMBER_OF_ARGUMENTS: usize;
    /// Name of function.
    const NAME: &'static str;

    /// Parses `arguments` and creates function.
    ///
    /// # Errors
    ///
    /// will return Err if expression is not a valid formula.
    fn parse<T: for<'a> GetFunction<'a>>(
        arguments: &[&str],
        formulas: &T,
    ) -> Result<Self, ParserError>
    where
        Self: Sized;

    /// Parses `arguments` and creates function stored in [`Box`] as trait object.
    ///
    /// # Errors
    ///
    /// will return Err if expression is not a valid formula
    #[inline]
    fn parse_into_box<T: for<'a> GetFunction<'a>>(
        arguments: &[&str],
        formulas: &T,
    ) -> Result<Box<dyn FunctionLike>, ParserError>
    where
        Self: Sized + 'static,
    {
        Self::parse(arguments, formulas).map(|function| Box::new(function) as Box<dyn FunctionLike>)
    }
}
