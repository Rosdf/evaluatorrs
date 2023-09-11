use crate::formulas::{Function, FunctionLike, ParserError};
use crate::function_stores::{ArgumentBounds, GetFunction, Parser, RegisterParser};
use std::collections::hash_map::Keys;
use std::collections::HashMap;

type InnerFunctionParser =
    fn(&[&str], &HashMapFunctionStore) -> Result<Box<dyn FunctionLike>, ParserError>;

/// Function store based on [`HashMap`].
#[derive(Default, Debug, Clone)]
pub struct HashMapFunctionStore(HashMap<&'static str, (InnerFunctionParser, ArgumentBounds)>);

impl HashMapFunctionStore {
    /// Creates an empty `HashMapFunctionStore`.
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}

#[derive(Debug)]
pub struct FunctionNamesIterator<'a>(Keys<'a, &'static str, (InnerFunctionParser, ArgumentBounds)>);

impl<'a> Iterator for FunctionNamesIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().copied()
    }
}

impl<'a> GetFunction<'a> for HashMapFunctionStore {
    type Iter = FunctionNamesIterator<'a>;

    fn function_parser<'b>(
        &'b self,
        formula_name: &str,
    ) -> Option<(Box<Parser<'b>>, ArgumentBounds)> {
        self.0.iter();
        self.0.get(formula_name).map(|(parser, bounds)| {
            (
                Box::new(move |arguments: &[&str]| (*parser)(arguments, self)) as Box<Parser>,
                bounds.clone(),
            )
        })
    }

    fn iter(&'a self) -> Self::Iter {
        FunctionNamesIterator(self.0.keys())
    }
}

impl RegisterParser for HashMapFunctionStore {
    fn register<T: Function + 'static>(&mut self) {
        self.0.insert(
            T::NAME,
            (
                T::parse_into_box,
                ArgumentBounds {
                    min: T::MIN_NUMBER_OF_ARGUMENTS,
                    max: T::MAX_NUMBER_OF_ARGUMENTS,
                },
            ),
        );
    }
}
