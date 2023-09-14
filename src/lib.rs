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
#![cfg_attr(not(feature = "std"), no_std)]

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
//! ```rust
//! use evaluatorrs::eval;
//!
//! fn evaluate() {
//!     let expression = "1 + 2";
//!     let result = eval(expression).unwrap();
//!     debug_assert_eq!(result, 3.0);
//! }
//! ```
//!
//! ## Create your own functions
//! Create function, that computes average of all it's arguments.
//! ```rust
//! use std::sync::Arc;
//! use evaluatorrs::formulas::{IsConst, Evaluate, FunctionLike, Function, RootFormula, EvaluationError, MathError, ParserError};
//! use evaluatorrs::function_stores::{GetFunction, RegisterParser, VectorFunctionStore};
//! use evaluatorrs::variable_stores::{EmptyVariableStore, GetVariable, Variable};
//!
//! #[derive(Debug)]
//! struct Average {
//!     arguments: Box<[RootFormula]>,
//! }
//!
//! impl IsConst for Average {
//!     fn is_const(&self) -> bool {
//!        self.arguments.iter().all(|x| x.is_const())
//!     }
//! }
//!
//! impl Evaluate for Average {
//!     fn eval(&self, args: &dyn GetVariable) -> Result<f64, EvaluationError> {
//!         let mut res = 0.0;
//!         for val in self.arguments.iter() {
//!             let new_val = val.eval(args)?;
//!             if new_val.is_nan() {
//!                 return Ok(f64::NAN);
//!             }
//!             res += new_val;
//!         }
//!         Ok(res / self.arguments.len() as f64)
//!     }
//! }
//!
//! impl FunctionLike for Average {
//!     fn collapse_inner(&mut self) -> Result<(), MathError> {
//!         for val in self.arguments.iter_mut() {
//!             val.collapse_inner()?;
//!         }
//!         Ok(())
//!     }
//!
//!     fn set_all_variables_shared(&mut self, args: &dyn GetVariable) {
//!         for val in self.arguments.iter_mut() {
//!             val.set_all_variables_shared(args);
//!         }
//!     }
//!
//!     fn set_all_variables_owned(&mut self, args: &dyn GetVariable) {
//!         for val in self.arguments.iter_mut() {
//!             val.set_all_variables_owned(args);
//!         }
//!     }
//!
//!     fn set_variable_shared(&mut self, name: &Variable, function: &Arc<RootFormula>) {
//!         for val in self.arguments.iter_mut() {
//!             val.set_variable_shared(name, function);
//!         }
//!     }
//!
//!     fn set_variable_owned(&mut self, name: &Variable, function: &RootFormula) {
//!         for val in self.arguments.iter_mut() {
//!             val.set_variable_owned(name, function);
//!         }
//!     }
//!
//!     fn clone_into_box(&self) -> Box<dyn FunctionLike> {
//!         Box::new(Self {arguments: self.arguments.iter().map(|x| RootFormula::new(x.clone_into_box())).collect()})
//!     }
//! }
//!
//! impl Function for Average {
//!     const MIN_NUMBER_OF_ARGUMENTS: usize = 1;
//!     const MAX_NUMBER_OF_ARGUMENTS: usize = 999;
//!     const NAME: &'static str = "avg";
//!
//!     fn parse<T: for<'a> GetFunction<'a>>(arguments: &[&str], formulas: &T) -> Result<Self, ParserError>
//!     where
//!         Self: Sized
//!     {
//!         let parsed_arguments: Result<Box<[_]>, _> = arguments.iter().map(|x| RootFormula::parse(*x, formulas)).collect();
//!         parsed_arguments.map(|x| Self {arguments: x})
//!     }
//! }
//!
//! fn try_parse() {
//!     let mut function_store = VectorFunctionStore::new();
//!     function_store.register::<Average>();
//!     let parsed = RootFormula::parse("avg(1, 2, 3)", &function_store);
//!     assert!(parsed.is_ok());
//!     let parsed = parsed.unwrap();
//!     let res = parsed.eval(&EmptyVariableStore);
//!     assert!(res.is_ok());
//!     assert_eq!(res.unwrap(), 2.0);
//! }
//! ```
//!
//! ## Evaluate functions with variables
//! ```rust
//! # use evaluatorrs::formulas::{Evaluate, RootFormula};
//! # use evaluatorrs::function_stores::EmptyFunctionStore;
//! # #[cfg(feature = "std")]
//! fn example() {
//!     # use evaluatorrs::variable_stores::{HashMapVariableStore, SetVariable};
//!
//!     let formula = RootFormula::parse("a + b", &EmptyFunctionStore).unwrap();
//!     let mut variable_store = HashMapVariableStore::new();
//!     variable_store.set("a", RootFormula::parse("1", &EmptyFunctionStore).unwrap());
//!     variable_store.set("b", RootFormula::parse("10", &EmptyFunctionStore).unwrap());
//!     let evaluated = formula.eval(&variable_store);
//!     assert!(evaluated.is_ok());
//!     let evaluated = evaluated.unwrap();
//!     assert_eq!(evaluated, 11.0);
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

#[cfg(not(feature = "std"))]
extern crate alloc;

// this mod is needed for easier access to non std.
// marked as pub for using inside of public macros.
#[doc(hidden)]
pub mod __lib {
    pub mod boxed {
        #[cfg(not(feature = "std"))]
        pub use alloc::boxed::Box;
        #[cfg(feature = "std")]
        pub use std::boxed::Box;
    }
    pub mod string {
        #[cfg(not(feature = "std"))]
        #[allow(clippy::module_name_repetitions)]
        pub use alloc::string::{String, ToString};
        #[cfg(feature = "std")]
        #[allow(clippy::module_name_repetitions)]
        pub use std::string::{String, ToString};
    }
    pub mod sync {
        #[cfg(not(feature = "std"))]
        pub use alloc::sync::Arc;
        #[cfg(feature = "std")]
        pub use std::sync::Arc;
    }
    pub mod fmt {
        #[cfg(not(feature = "std"))]
        pub use core::fmt::{Debug, Display, Formatter, Result};
        #[cfg(feature = "std")]
        pub use std::fmt::{Debug, Display, Formatter, Result};
    }
    pub mod error {
        #[cfg(all(feature = "std", nightly))]
        pub use core::error::Error;
        #[cfg(feature = "std")]
        pub use std::error::Error;
    }
    pub mod ops {
        #[cfg(not(feature = "std"))]
        pub use core::ops::{Add, Div, Mul, Sub};
        #[cfg(feature = "std")]
        pub use std::ops::{Add, Div, Mul, Sub};
    }
    pub mod str {
        #[cfg(not(feature = "std"))]
        #[allow(clippy::module_name_repetitions)]
        pub use core::str::FromStr;
        #[cfg(feature = "std")]
        #[allow(clippy::module_name_repetitions)]
        pub use std::str::FromStr;
    }
    pub mod collections {
        #[cfg(not(feature = "std"))]
        pub use alloc::collections::VecDeque;
        #[cfg(feature = "std")]
        pub use std::collections::VecDeque;
    }
    pub mod vec {
        #[cfg(not(feature = "std"))]
        pub use alloc::vec::Vec;
        #[cfg(feature = "std")]
        pub use std::vec::Vec;
    }
    pub mod convert {
        #[cfg(not(feature = "std"))]
        pub use core::convert::{identity, TryInto};
        #[cfg(feature = "std")]
        pub use std::convert::{identity, TryInto};
    }
    pub mod iter {
        #[cfg(not(feature = "std"))]
        pub use core::iter::{empty, Empty};
        #[cfg(feature = "std")]
        pub use std::iter::{empty, Empty};
    }
    pub mod slice {
        #[cfg(not(feature = "std"))]
        pub use core::slice::Iter;
        #[cfg(feature = "std")]
        pub use std::slice::Iter;
    }
}
