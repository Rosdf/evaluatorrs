use crate::__lib::boxed::Box;
use crate::__lib::fmt::Debug;
use crate::__lib::slice::Iter;
use crate::__lib::vec::Vec;
use crate::formulas::{Function, FunctionLike, ParserError};
use crate::function_stores::{ArgumentBounds, GetFunction, Parser, RegisterParser};

type InnerFunctionParser =
    fn(&[&str], &VecFunctionStore) -> Result<Box<dyn FunctionLike>, ParserError>;

/// Function store based on [`Vec`]. Might be faster then [`HashMapFunctionStore`] for small number of functions.
#[derive(Default, Clone, Debug)]
pub struct VecFunctionStore(Vec<(&'static str, (InnerFunctionParser, ArgumentBounds))>);

impl VecFunctionStore {
    /// Creates an empty `VecFunctionStore`.
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

#[derive(Debug)]
pub struct FunctionNamesIterator<'a>(
    Iter<'a, (&'static str, (InnerFunctionParser, ArgumentBounds))>,
);

impl<'a> Iterator for FunctionNamesIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|x| x.0)
    }
}

impl<'a> GetFunction<'a> for VecFunctionStore {
    type Iter = FunctionNamesIterator<'a>;

    fn function_parser<'b>(
        &'b self,
        formula_name: &str,
    ) -> Option<(Box<Parser<'b>>, ArgumentBounds)> {
        for (name, (parser, bounds)) in &self.0 {
            if *name == formula_name {
                return Some((
                    Box::new(move |arguments: &[&str]| (*parser)(arguments, self)) as Box<Parser>,
                    bounds.clone(),
                ));
            }
        }
        None
    }

    fn iter(&'a self) -> Self::Iter {
        FunctionNamesIterator(self.0.iter())
    }
}

impl RegisterParser for VecFunctionStore {
    fn register<T: Function + 'static>(&mut self) {
        self.0.push((
            T::NAME,
            (
                T::parse_into_box,
                ArgumentBounds {
                    min: T::MIN_NUMBER_OF_ARGUMENTS,
                    max: T::MAX_NUMBER_OF_ARGUMENTS,
                },
            ),
        ));
    }
}
