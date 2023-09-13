use crate::formulas::FunctionLike;
use crate::lib::boxed::Box;
use crate::tokens::number_like::NumberLike;
use crate::tokens::operator::Operator;
use crate::variable_stores::Variable;

#[derive(Debug, PartialEq)]
pub(crate) struct OpenBracket;

#[derive(Debug, PartialEq)]
pub(crate) struct CloseBracket;

#[derive(Debug, PartialEq)]
pub(crate) enum Bracket {
    OpenBracket(OpenBracket),
    CloseBracket(CloseBracket),
}

impl Bracket {
    pub(crate) const fn parse(elem: char) -> Option<Self> {
        match elem {
            '(' => Some(Self::OpenBracket(OpenBracket)),
            ')' => Some(Self::CloseBracket(CloseBracket)),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub(crate) enum BaseToken {
    NumberLike(NumberLike),
    Operator(Operator),
    Bracket(Bracket),
    Formula(Box<dyn FunctionLike>),
}

impl From<f64> for BaseToken {
    fn from(value: f64) -> Self {
        Self::NumberLike(NumberLike::Number(value))
    }
}

impl From<Variable> for BaseToken {
    fn from(value: Variable) -> Self {
        Self::NumberLike(NumberLike::Variable(value))
    }
}

impl From<Operator> for BaseToken {
    fn from(value: Operator) -> Self {
        Self::Operator(value)
    }
}

impl From<Bracket> for BaseToken {
    fn from(value: Bracket) -> Self {
        Self::Bracket(value)
    }
}

impl<T: FunctionLike + 'static> From<T> for BaseToken {
    fn from(value: T) -> Self {
        Self::Formula(Box::new(value))
    }
}

#[cfg(test)]
mod test {
    use crate::tokens::Bracket;

    #[test]
    fn parse_open_bracket() {
        let res = Bracket::parse('(');
        assert!(res.is_some());
        let res = res.unwrap();
        assert!(matches!(res, Bracket::OpenBracket(_)));
    }

    #[test]
    fn parse_close_bracket() {
        let res = Bracket::parse(')');
        assert!(res.is_some());
        let res = res.unwrap();
        assert!(matches!(res, Bracket::CloseBracket(_)));
    }

    #[test]
    fn fail_parse() {
        let res = Bracket::parse('3');
        assert!(res.is_none());
    }
}
