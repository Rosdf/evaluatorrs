use crate::formulas::{Formula, FormulaLike, ParserError};

pub(crate) struct ArgumentBounds {
    min: usize,
    max: usize,
}

pub(crate) trait FormulaStore {
    fn register<T: Formula>();
    fn get_constructor(
        &self,
        formula_name: &str,
    ) -> Option<(
        &dyn Fn(&[&str]) -> Result<Box<dyn FormulaLike>, ParserError>,
        ArgumentBounds,
    )>;
}
