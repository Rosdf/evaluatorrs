use crate::formulas::{Formula, FormulaLike, ParserError};

pub type FormulaParser = dyn Fn(&[&str]) -> Result<Box<dyn FormulaLike>, ParserError>;

pub struct ArgumentBounds {
    min: usize,
    max: usize,
}

pub trait FormulaStore {
    fn register<T: Formula>();
    fn get_parser(&self, formula_name: &str) -> Option<(&FormulaParser, ArgumentBounds)>;
}
