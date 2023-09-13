use crate::__lib::boxed::Box;
use crate::__lib::iter::{empty, Empty};
use crate::formulas::Function;
use crate::function_stores::{ArgumentBounds, GetFunction, Parser, RegisterParser};

/// Function store that can not contain any functions.
#[derive(Debug, Default, Copy, Clone)]
pub struct EmptyFunctionStore;

impl<'a> GetFunction<'a> for EmptyFunctionStore {
    type Iter = Empty<&'a str>;

    fn function_parser<'b>(
        &'b self,
        _formula_name: &str,
    ) -> Option<(Box<Parser<'b>>, ArgumentBounds)> {
        None
    }

    fn iter(&self) -> Self::Iter {
        empty()
    }
}

impl RegisterParser for EmptyFunctionStore {
    fn register<T: Function + 'static>(&mut self) {}
}
