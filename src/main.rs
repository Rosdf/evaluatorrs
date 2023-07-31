// use crate::formulas::{build_rpn, eval_rpn, parse_expression};
// use std::collections::HashMap;

use crate::formulas::FormulaLike;
use crate::variable_stores::HashMapStore;
use std::collections::HashMap;

mod formula_stores;
mod formulas;
mod utils;
mod variable_stores;

fn main() {
    // let a = variable_stores::HashMapStore::new();
    // let expr: String = "(25^0.5+2^2)^0.5*3^4^0".to_string();
    // let res = parse_expression(expr);
    // println!("{:?}", res);
    // let res = build_rpn(res.unwrap());
    // println!("{:?}", res);
    // let aa: HashMap<String, f64> = HashMap::new();
    // print!("{:?}", eval_rpn(res.unwrap(), &aa));
    // println!("{}", expr);
    let a = &mut formulas::base_formula::BaseFormula::new("1+1".into()) as &mut dyn FormulaLike;
    println!("{}", a.eval_dyn(&HashMapStore::new()))
}
