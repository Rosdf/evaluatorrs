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
