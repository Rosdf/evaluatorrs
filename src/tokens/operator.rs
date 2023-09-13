use crate::formulas::operator::OperatorFormula;
use crate::formulas::RootFormula;

#[cfg(all(not(feature = "std"), feature = "libm"))]
fn exponential_function(base: f64, power: f64) -> f64 {
    libm::Libm::<f64>::pow(base, power)
}

#[cfg(feature = "std")]
fn exponential_function(base: f64, power: f64) -> f64 {
    base.powf(power)
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    #[cfg(any(feature = "std", feature = "libm"))]
    Exponent,
}

impl From<Operator> for &'static str {
    fn from(value: Operator) -> Self {
        match value {
            Operator::Plus => "+",
            Operator::Minus => "-",
            Operator::Multiply => "*",
            Operator::Divide => "/",
            #[cfg(any(feature = "std", feature = "libm"))]
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
            Self::Multiply | Self::Divide => 2,
            #[cfg(any(feature = "std", feature = "libm"))]
            Self::Exponent => 2,
        }
    }

    pub(crate) const fn side(&self) -> Side {
        match self {
            Self::Divide | Self::Multiply | Self::Minus | Self::Plus => Side::Left,
            #[cfg(any(feature = "std", feature = "libm"))]
            Self::Exponent => Side::Right,
        }
    }
    pub(crate) const fn parse(elem: char) -> Option<Self> {
        match elem {
            '+' => Some(Self::Plus),
            '-' => Some(Self::Minus),
            '*' => Some(Self::Multiply),
            '/' => Some(Self::Divide),
            #[cfg(any(feature = "std", feature = "libm"))]
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
            #[cfg(any(feature = "std", feature = "libm"))]
            Self::Exponent => exponential_function(first, second),
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
        #[cfg(any(feature = "std", feature = "libm"))]
        assert_eq!(Operator::parse('^').unwrap(), Operator::Exponent);
    }

    #[test]
    fn test_side() {
        assert_eq!(Operator::Plus.side(), Side::Left);
        assert_eq!(Operator::Minus.side(), Side::Left);
        assert_eq!(Operator::Multiply.side(), Side::Left);
        assert_eq!(Operator::Divide.side(), Side::Left);
        #[cfg(any(feature = "std", feature = "libm"))]
        assert_eq!(Operator::Exponent.side(), Side::Right);
    }

    #[test]
    fn test_priority() {
        assert_eq!(Operator::Plus.priority(), Operator::Minus.priority());
        assert_eq!(Operator::Multiply.priority(), Operator::Divide.priority());
        #[cfg(any(feature = "std", feature = "libm"))]
        assert_eq!(Operator::Exponent.priority(), Operator::Multiply.priority());
        assert!(Operator::Plus.priority() > Operator::Multiply.priority());
    }
}
