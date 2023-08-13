use crate::variable_stores::Variable;

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum NumberLike {
    Number(f64),
    Variable(Variable),
}

impl From<f64> for NumberLike {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}

impl From<String> for NumberLike {
    fn from(value: String) -> Self {
        Self::Variable(Variable::new(value))
    }
}

impl From<Variable> for NumberLike {
    fn from(value: Variable) -> Self {
        Self::Variable(value)
    }
}
