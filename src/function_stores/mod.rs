mod array_store;
pub use array_store::VecFunctionStore;
mod empty_store;
pub use empty_store::EmptyFunctionStore;
#[cfg(feature = "std")]
mod hashmap_store;
#[cfg(feature = "std")]
pub use hashmap_store::HashMapFunctionStore;

use crate::__lib::boxed::Box;
use crate::formulas::{Function, FunctionLike, ParserError};

/// Provides information about bounds on arguments number of function.
#[derive(Clone, Debug)]
pub struct ArgumentBounds {
    pub(crate) min: usize,
    pub(crate) max: usize,
}

/// Signature of parser that function store uses.
pub type Parser<'a> =
    dyn for<'b, 'c> Fn(&'b [&'c str]) -> Result<Box<dyn FunctionLike>, ParserError> + 'a;

/// Trait for obtaining function parser by function name.
pub trait GetFunction<'a> {
    /// Type of iterator for iterating over function names present in function store.
    type Iter: Iterator<Item = &'a str>;

    /// Returns a function parser and [`ArgumentBounds`] for function name.
    fn function_parser<'b>(
        &'b self,
        formula_name: &str,
    ) -> Option<(Box<Parser<'b>>, ArgumentBounds)>;

    /// Methode to get iterator over function names stored in function store.
    fn iter(&'a self) -> Self::Iter;
}

/// Trait for registering new functions in function store.
pub trait RegisterParser {
    /// Methode for registering new functions in function store.
    fn register<T: Function + 'static>(&mut self);
}
