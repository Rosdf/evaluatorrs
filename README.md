This library provides ways to evaluate mathematical expressions at runtime.

## Feature switches

* `std` (*default*): enables use of std library
* `libm`: enables use of mathematical functions from libm, useful for no_std crates and non-standard functions

## Example 1

```rust
use evaluatorrs::eval;
use evaluatorrs::formulas::ParserError;

fn evaluate() -> Result<(), ParserError> {
    let expression = "1 + 2";
    let result = eval(expression)?;
    debug_assert_eq!(result, 3.0);
    Ok(())
}
```

## Example 2

```rust
use evaluatorrs::formulas::{Evaluate, RootFormula};
use evaluatorrs::function_stores::EmptyFunctionStore;
use evaluatorrs::variable_stores::{HashMapVariableStore, SetVariable};

fn example() -> Result<(), ParserError> {
    let formula = RootFormula::parse("a + b", &EmptyFunctionStore)?;
    let mut variable_store = HashMapVariableStore::new();
    variable_store.set("a", RootFormula::parse("1", &EmptyFunctionStore)?);
    variable_store.set("b", RootFormula::parse("10", &EmptyFunctionStore)?);
    let evaluated = formula.eval(&variable_store);
    assert!(evaluated.is_ok());
    let evaluated = evaluated?;
    assert_eq!(evaluated, 11.0);
 }
```