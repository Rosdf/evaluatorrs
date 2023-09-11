use crate::formulas::operator::OperatorFormula;
use crate::formulas::RootFormula;

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Exponent,
}

impl From<Operator> for &'static str {
    fn from(value: Operator) -> Self {
        match value {
            Operator::Plus => "+",
            Operator::Minus => "-",
            Operator::Multiply => "*",
            Operator::Divide => "/",
            Operator::Exponent => "^",
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Side {
    Left,
    Right,
}

impl Operator {
    pub(crate) const fn priority(&self) -> u8 {
        match self {
            Self::Minus | Self::Plus => 3,
            Self::Multiply | Self::Divide | Self::Exponent => 2,
        }
    }

    pub(crate) const fn side(&self) -> Side {
        match self {
            Self::Divide | Self::Multiply | Self::Minus | Self::Plus => Side::Left,
            Self::Exponent => Side::Right,
        }
    }
    pub(crate) const fn parse(elem: char) -> Option<Self> {
        match elem {
            '+' => Some(Self::Plus),
            '-' => Some(Self::Minus),
            '*' => Some(Self::Multiply),
            '/' => Some(Self::Divide),
            '^' => Some(Self::Exponent),
            _ => None,
        }
    }

    pub(crate) const fn into_formula(
        self,
        first: RootFormula,
        second: RootFormula,
    ) -> OperatorFormula {
        OperatorFormula::new(first, second, self)
    }

    pub(crate) fn eval(&self, first: f64, second: f64) -> f64 {
        match self {
            Self::Plus => first + second,
            Self::Minus => first - second,
            Self::Multiply => first * second,
            Self::Divide => first / second,
            Self::Exponent => first.powf(second),
        }
    }
}

#[cfg(test)]
mod correctness_tests {
    use crate::tokens::{Operator, Side};

    #[test]
    fn test_parse() {
        assert_eq!(Operator::parse('+').unwrap(), Operator::Plus);
        assert_eq!(Operator::parse('-').unwrap(), Operator::Minus);
        assert_eq!(Operator::parse('*').unwrap(), Operator::Multiply);
        assert_eq!(Operator::parse('/').unwrap(), Operator::Divide);
        assert_eq!(Operator::parse('^').unwrap(), Operator::Exponent);
    }

    #[test]
    fn test_side() {
        assert_eq!(Operator::Plus.side(), Side::Left);
        assert_eq!(Operator::Minus.side(), Side::Left);
        assert_eq!(Operator::Multiply.side(), Side::Left);
        assert_eq!(Operator::Divide.side(), Side::Left);
        assert_eq!(Operator::Exponent.side(), Side::Right);
    }

    #[test]
    fn test_priority() {
        assert_eq!(Operator::Plus.priority(), Operator::Minus.priority());
        assert_eq!(Operator::Multiply.priority(), Operator::Divide.priority());
        assert_eq!(Operator::Exponent.priority(), Operator::Multiply.priority());
        assert!(Operator::Plus.priority() > Operator::Multiply.priority());
    }
}
