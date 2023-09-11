#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    clippy::clone_on_ref_ptr,
    clippy::dbg_macro,
    clippy::exhaustive_enums
)]
#![allow(clippy::redundant_pub_crate, clippy::must_use_candidate)]
#![deny(missing_debug_implementations, missing_docs)]

//! # Evaluator rs
//!
//! evaluatorrs is a library for evaluating mathematical expressions into [`f64`] numbers.
//!
//! ## Design
//!
//! Where other libraries have predefined set of functions, evaluatorrs has ways to define new once
//! by user.
//!
//! ## Evaluate simple expression
//! ```
//! use evaluatorrs::eval;
//!
//! fn evaluate() {
//!     let expression = "1 + 2";
//!     let result = eval(expression).unwrap();
//!     debug_assert_eq!(result, 3.0);
//! }
//! ```

mod context;

/// Provides basic ways to store function parsers and traits to implement new once.
pub mod function_stores;

/// Provides basic set of functions and traits to implement new once.
pub mod formulas;
mod tokens;

/// Provides basic ways to store variables and traits to implement new once.
pub mod variable_stores;

use crate::formulas::RootFormula;
use crate::formulas::{Evaluate, ParserError};
use crate::function_stores::EmptyFunctionStore;
use crate::variable_stores::EmptyVariableStore;
pub use context::Context;

/// Parses and evaluates [`&str`] into [`f64`].
///
/// # Errors
///
/// will return Err if failed to parse expression, or evaluate it
#[inline]
pub fn eval(expression: &str) -> Result<f64, ParserError> {
    Ok(RootFormula::parse(expression, &EmptyFunctionStore)?.eval(&EmptyVariableStore)?)
}
