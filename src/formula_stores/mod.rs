use crate::formulas::{Function, FunctionLike, ParserError};

pub type FormulaParser = dyn Fn(&[&str]) -> Result<Box<dyn FunctionLike>, ParserError>;

pub struct ArgumentBounds {
    min: usize,
    max: usize,
}

pub trait GetParser {
    fn get_parser(&self, formula_name: &str) -> Option<(&FormulaParser, ArgumentBounds)>;
}

pub trait RegisterParser: GetParser {
    fn register<T: Function>();
}

pub struct EmptyFormulaStore;

impl GetParser for EmptyFormulaStore {
    fn get_parser(&self, _formula_name: &str) -> Option<(&FormulaParser, ArgumentBounds)> {
        None
    }
}

impl RegisterParser for EmptyFormulaStore {
    fn register<T: Function>() {}
}
