#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    clippy::clone_on_ref_ptr,
    clippy::dbg_macro,
    clippy::exhaustive_enums
)]
#![allow(clippy::redundant_pub_crate, clippy::must_use_candidate)]

use std::marker::PhantomData;

pub mod formula_stores;
pub mod formulas;
mod tokens;
pub mod variable_stores;

pub(crate) struct FormulaCompressor<T> {
    _inner: PhantomData<T>,
}

pub(crate) struct CompressError<T> {
    pub(crate) function: T,
    pub(crate) kind: formulas::EvaluationError,
}

// impl<T: formulas::FunctionLike> FormulaCompressor<T> {
//     pub(crate) fn compress(function: T) -> Result<f64, CompressError<T>> {
//         if function.is_const() {
//             match function.eval(&variable_stores::EmptyVariableStore) {
//                 Ok(val) => return Ok(val),
//                 Err(e) => return Err(CompressError { function, kind: e }),
//             }
//         }
//     }
// }
