use crate::formulas::{Evaluate, ExpressionParsingError, FormulaLike, IsConst, ParenthesisError};
use crate::variable_stores::{GetVariable, HashMapStore};
use std::collections::VecDeque;
use std::fmt::Debug;
use std::str::FromStr;

#[derive(Debug)]
pub struct BaseFormula {
    tree: FormulaArgument,
}

impl Evaluate for BaseFormula {
    fn eval_dyn(&mut self, args: &dyn GetVariable) -> f64 {
        self.tree.eval(args)
    }

    fn eval<T: GetVariable + ?Sized>(&mut self, args: &T) -> f64
    where
        Self: Sized,
    {
        self.tree.eval(args)
    }
}

impl IsConst for BaseFormula {
    fn is_const(&self) -> bool {
        self.tree.is_const()
    }
}

impl FormulaLike for BaseFormula {}

#[derive(Debug)]
enum FormulaArgument {
    NumberLike(NumberLike),
    Formula(Box<dyn FormulaLike>),
}

impl IsConst for FormulaArgument {
    fn is_const(&self) -> bool {
        match self {
            FormulaArgument::NumberLike(val) => val.is_const(),
            FormulaArgument::Formula(val) => val.is_const(),
        }
    }
}

impl From<NumberLike> for FormulaArgument {
    fn from(value: NumberLike) -> Self {
        FormulaArgument::NumberLike(value)
    }
}

impl From<f64> for FormulaArgument {
    fn from(value: f64) -> Self {
        FormulaArgument::NumberLike(value.into())
    }
}

impl From<Box<dyn FormulaLike>> for FormulaArgument {
    fn from(value: Box<dyn FormulaLike>) -> Self {
        Self::Formula(value)
    }
}

impl TryFrom<BaseToken> for FormulaArgument {
    type Error = ();

    fn try_from(value: BaseToken) -> Result<Self, Self::Error> {
        match value {
            BaseToken::NumberLike(val) => Ok(val.into()),
            BaseToken::Operator(_) => Err(()),
            BaseToken::Bracket(_) => Err(()),
            BaseToken::Formula(val) => Ok(val.into()),
        }
    }
}

impl Evaluate for FormulaArgument {
    fn eval_dyn(&mut self, args: &dyn GetVariable) -> f64 {
        match self {
            FormulaArgument::NumberLike(num) => num.eval(args),
            FormulaArgument::Formula(formula) => formula.eval_dyn(args.as_dyn()),
        }
    }

    fn eval<T: GetVariable + ?Sized>(&mut self, args: &T) -> f64
    where
        Self: Sized,
    {
        match self {
            FormulaArgument::NumberLike(num) => num.eval(args),
            FormulaArgument::Formula(formula) => formula.eval_dyn(args.as_dyn()),
        }
    }
}

#[derive(Debug)]
struct OperatorFormula {
    first: FormulaArgument,
    second: FormulaArgument,
    operator: Operator,
}

impl IsConst for OperatorFormula {
    fn is_const(&self) -> bool {
        self.first.is_const() && self.second.is_const()
    }
}

impl Evaluate for OperatorFormula {
    fn eval_dyn(&mut self, args: &dyn GetVariable) -> f64 {
        self.operator
            .eval(self.first.eval(args), self.second.eval(args))
    }

    fn eval<T: GetVariable + ?Sized>(&mut self, args: &T) -> f64
    where
        Self: Sized,
    {
        self.operator
            .eval(self.first.eval(args), self.second.eval(args))
    }
}

impl FormulaLike for OperatorFormula {}

impl From<OperatorFormula> for BaseToken {
    fn from(value: OperatorFormula) -> Self {
        Self::Formula(Box::new(value))
    }
}

#[derive(Debug, PartialEq)]
enum NumberLike {
    Number(f64),
    Variable(String),
}

impl IsConst for NumberLike {
    fn is_const(&self) -> bool {
        match self {
            NumberLike::Number(_) => true,
            NumberLike::Variable(_) => false,
        }
    }
}

impl From<f64> for NumberLike {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}

impl From<String> for NumberLike {
    fn from(value: String) -> Self {
        Self::Variable(value)
    }
}

impl Evaluate for NumberLike {
    fn eval_dyn(&mut self, args: &dyn GetVariable) -> f64 {
        match self {
            NumberLike::Number(num) => *num,
            NumberLike::Variable(var) => args.get(var).unwrap(),
        }
    }

    fn eval<T: GetVariable + ?Sized>(&mut self, args: &T) -> f64 {
        match self {
            NumberLike::Number(num) => *num,
            NumberLike::Variable(var) => args.get(var).unwrap(),
        }
    }
}

#[derive(Debug, PartialEq)]
struct OpenBracket;

#[derive(Debug, PartialEq)]
struct CloseBracket;

#[derive(Debug, PartialEq)]
enum Bracket {
    OpenBracket(OpenBracket),
    CloseBracket(CloseBracket),
}

impl Bracket {
    fn parse(elem: char) -> Option<Self> {
        match elem {
            '(' => Some(Bracket::OpenBracket(OpenBracket)),
            ')' => Some(Bracket::CloseBracket(CloseBracket)),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
}

#[derive(Debug, PartialEq)]
enum Side {
    Left,
    Right,
}

impl Operator {
    fn priority(&self) -> u8 {
        match self {
            Operator::Plus => 3,
            Operator::Minus => 3,
            Operator::Multiply => 2,
            Operator::Divide => 2,
            Operator::Power => 2,
        }
    }

    fn side(&self) -> Side {
        match self {
            Operator::Plus => Side::Left,
            Operator::Minus => Side::Left,
            Operator::Multiply => Side::Left,
            Operator::Divide => Side::Left,
            Operator::Power => Side::Right,
        }
    }
    fn parse(elem: char) -> Option<Operator> {
        match elem {
            '+' => Some(Operator::Plus),
            '-' => Some(Operator::Minus),
            '*' => Some(Operator::Multiply),
            '/' => Some(Operator::Divide),
            '^' => Some(Operator::Power),
            _ => None,
        }
    }

    fn into_formula(self, first: FormulaArgument, second: FormulaArgument) -> OperatorFormula {
        OperatorFormula {
            first,
            second,
            operator: self,
        }
    }

    fn eval(&self, first: f64, second: f64) -> f64 {
        match self {
            Operator::Plus => first + second,
            Operator::Minus => first - second,
            Operator::Multiply => first * second,
            Operator::Divide => first / second,
            Operator::Power => first.powf(second),
        }
    }
}

#[derive(Debug)]
enum BaseToken {
    NumberLike(NumberLike),
    Operator(Operator),
    Bracket(Bracket),
    Formula(Box<dyn FormulaLike>),
}

impl<T: Into<NumberLike>> From<T> for BaseToken {
    fn from(value: T) -> Self {
        Self::NumberLike(value.into())
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

impl From<Box<dyn FormulaLike>> for BaseToken {
    fn from(value: Box<dyn FormulaLike>) -> Self {
        Self::Formula(value)
    }
}

enum OperatorStackToken {
    Operator(Operator),
    OpenBracket(OpenBracket),
}

impl From<OperatorStackToken> for BaseToken {
    fn from(value: OperatorStackToken) -> Self {
        match value {
            OperatorStackToken::Operator(operator) => Self::Operator(operator),
            OperatorStackToken::OpenBracket(bra) => Self::Bracket(Bracket::OpenBracket(bra)),
        }
    }
}

impl BaseFormula {
    fn parse_parenthesis(expression: &mut &str) -> Option<Bracket> {
        if expression.is_empty() {
            return None;
        }
        let res = Bracket::parse(expression.chars().next().unwrap());
        if res.is_some() {
            *expression = &expression[1..];
        }
        res
    }

    fn parse_number(expression: &mut &str) -> Option<f64> {
        if expression.is_empty() {
            return None;
        }
        let mut encountered_digit = false;
        let mut encountered_dot = false;
        let mut parsed = 0;
        let mut starting_char: usize = 0;

        match expression.chars().next().unwrap() {
            '-' => {
                parsed += 1;
                starting_char = 1;
            }
            '.' => {
                return None;
            }
            _ => {}
        }
        for elem in expression[starting_char..].chars() {
            match elem {
                x if x.is_ascii_digit() => {
                    parsed += 1;
                    encountered_digit = true;
                }
                '.' => {
                    if encountered_dot {
                        break;
                    }
                    if !encountered_digit {
                        return None;
                    }
                    encountered_dot = true;
                    parsed += 1
                }
                _ => break,
            }
        }
        if encountered_digit {
            let ret = f64::from_str(&expression[..parsed]).unwrap();
            *expression = &expression[parsed..];
            return Some(ret);
        }
        None
    }

    fn parse_operator(expression: &mut &str) -> Option<Operator> {
        if expression.is_empty() {
            return None;
        }
        let res = Operator::parse(expression.chars().next().unwrap());
        if res.is_some() {
            *expression = &expression[1..];
        }
        res
    }

    fn remove_spaces(expression: &mut &str) {
        let mut spaces: usize = 0;
        for elem in expression.chars() {
            if elem == ' ' {
                spaces += 1;
            } else {
                break;
            }
        }
        *expression = &expression[spaces..];
    }

    fn parse_variable(expression: &mut &str) -> String {
        todo!()
    }

    fn lex_expression(mut expression: &str) -> Result<VecDeque<BaseToken>, ExpressionParsingError> {
        let mut res = VecDeque::new();
        let mut prev_len = expression.len() + 1;
        while expression.len() < prev_len {
            prev_len = expression.len();
            Self::remove_spaces(&mut expression);
            if let Some(bra) = Self::parse_parenthesis(&mut expression) {
                res.push_back(bra.into());
            }

            Self::remove_spaces(&mut expression);
            if let Some(num) = Self::parse_number(&mut expression) {
                res.push_back(num.into());
            }

            Self::remove_spaces(&mut expression);
            if let Some(operator) = Self::parse_operator(&mut expression) {
                res.push_back(operator.into());
            }
        }
        if expression.is_empty() {
            return Ok(res);
        }
        Err(ExpressionParsingError)
    }

    fn process_bracket(
        bra: Bracket,
        rpn: &mut VecDeque<BaseToken>,
        operator_stack: &mut Vec<OperatorStackToken>,
    ) -> Result<(), ParenthesisError> {
        match bra {
            Bracket::OpenBracket(par) => operator_stack.push(OperatorStackToken::OpenBracket(par)),
            Bracket::CloseBracket(_) => {
                let mut found_open = false;
                while let Some(oper) = operator_stack.pop() {
                    match oper {
                        OperatorStackToken::Operator(operator) => {
                            rpn.push_back(BaseToken::Operator(operator));
                        }
                        OperatorStackToken::OpenBracket(_) => {
                            found_open = true;
                            break;
                        }
                    }
                }
                if !found_open {
                    return Err(ParenthesisError);
                }
            }
        }
        Ok(())
    }

    fn process_operator(
        op: Operator,
        rpn: &mut VecDeque<BaseToken>,
        operator_stack: &mut Vec<OperatorStackToken>,
    ) {
        if op.side() == Side::Right {
            operator_stack.push(OperatorStackToken::Operator(op));
            return;
        }
        while let Some(oper) = operator_stack.last() {
            match oper {
                OperatorStackToken::Operator(oper) => {
                    if op.priority() >= oper.priority() {
                        rpn.push_back(operator_stack.pop().unwrap().into())
                    } else {
                        break;
                    }
                }
                OperatorStackToken::OpenBracket(_) => break,
            }
        }
        operator_stack.push(OperatorStackToken::Operator(op));
    }

    fn build_rpn(mut tokens: VecDeque<BaseToken>) -> Result<VecDeque<BaseToken>, ParenthesisError> {
        let length = tokens.len();
        let mut stack = Vec::<OperatorStackToken>::new();
        for _ in 0..length {
            let token = tokens.pop_front().unwrap();
            match token {
                BaseToken::NumberLike(num_like) => {
                    tokens.push_back(BaseToken::NumberLike(num_like))
                }
                BaseToken::Operator(operator) => {
                    Self::process_operator(operator, &mut tokens, &mut stack)
                }
                BaseToken::Bracket(bra) => Self::process_bracket(bra, &mut tokens, &mut stack)?,
                BaseToken::Formula(formula) => tokens.push_back(BaseToken::Formula(formula)),
            }
        }
        while let Some(token) = stack.pop() {
            match token {
                OperatorStackToken::Operator(operator) => tokens.push_back(operator.into()),
                OperatorStackToken::OpenBracket(_) => {
                    return Err(ParenthesisError);
                }
            }
        }
        if stack.is_empty() {
            Ok(tokens)
        } else {
            Err(ParenthesisError)
        }
    }

    fn compress_rpn(mut rpn: VecDeque<BaseToken>) -> Option<FormulaArgument> {
        let empty_store = HashMapStore::new();
        let initial_len = rpn.len();
        for _ in 0..initial_len {
            let token = rpn.pop_front().unwrap();
            match token {
                BaseToken::NumberLike(num_like) => rpn.push_back(BaseToken::NumberLike(num_like)),
                BaseToken::Operator(operator) => {
                    let second = rpn.pop_back().unwrap();
                    let first = rpn.pop_back().unwrap();
                    let mut operator_formula = operator
                        .into_formula(first.try_into().unwrap(), second.try_into().unwrap());
                    rpn.push_back(if operator_formula.is_const() {
                        operator_formula.eval(&empty_store).into()
                    } else {
                        operator_formula.into()
                    });
                }
                BaseToken::Formula(formula) => rpn.push_back(BaseToken::Formula(formula)),
                BaseToken::Bracket(_) => unreachable!(),
            }
        }
        if rpn.len() == 1 {
            return Some(rpn.pop_front().unwrap().try_into().unwrap());
        }
        None
    }

    pub fn new(expression: &str) -> Self {
        let parsed = Self::lex_expression(expression).unwrap();
        let rpn = Self::build_rpn(parsed).unwrap();
        Self {
            tree: Self::compress_rpn(rpn).unwrap(),
        }
    }
}

#[cfg(test)]
mod lexer_test {
    use crate::formulas::base_formula::{BaseFormula, BaseToken, NumberLike, Operator};

    #[test]
    fn remove_all_spaces() {
        let mut expression = "     ";
        BaseFormula::remove_spaces(&mut expression);
        assert_eq!(expression, "", "{}", expression);
    }

    #[test]
    fn remove_spaces_with_text() {
        let mut expression = "     asd";
        BaseFormula::remove_spaces(&mut expression);
        assert_eq!(expression, "asd", "{}", expression);
    }

    #[test]
    fn remove_spaces() {
        let mut expression = "     asd   ";
        BaseFormula::remove_spaces(&mut expression);
        assert_eq!(expression, "asd   ", "{}", expression);
    }

    #[test]
    fn number_parser() {
        assert_eq!(BaseFormula::parse_number(&mut "1"), Some(1.0));
        assert_eq!(BaseFormula::parse_number(&mut "-1"), Some(-1.0));
        assert_eq!(BaseFormula::parse_number(&mut "1.0"), Some(1.0));
        assert_eq!(BaseFormula::parse_number(&mut "1.1"), Some(1.1));
        assert_eq!(BaseFormula::parse_number(&mut "0.1"), Some(0.1));
        assert_eq!(BaseFormula::parse_number(&mut "0.0"), Some(0.0));
        assert_eq!(BaseFormula::parse_number(&mut "+"), None);
        assert_eq!(BaseFormula::parse_number(&mut "1.0001"), Some(1.0001));
        assert_eq!(BaseFormula::parse_number(&mut "-1.03456"), Some(-1.03456));
    }

    #[test]
    fn basic_test_lex() {
        let expression = "1+2";
        let result = BaseFormula::lex_expression(expression);

        assert!(result.is_ok());
        let mut result = result.unwrap();
        assert_eq!(result.len(), 3);
        assert!(matches!(
            result.pop_front().unwrap(),
            BaseToken::NumberLike(NumberLike::Number(val)) if val == 1.0
        ));
        assert!(matches!(
            result.pop_front().unwrap(),
            BaseToken::Operator(Operator::Plus)
        ));
        assert!(matches!(
            result.pop_front().unwrap(),
            BaseToken::NumberLike(NumberLike::Number(val)) if val == 2.0
        ));
    }

    #[test]
    fn space_test_lex() {
        let expression = "  1 +   2 ";
        let result = BaseFormula::lex_expression(expression);

        assert!(result.is_ok());
        let mut result = result.unwrap();
        assert_eq!(result.len(), 3);
        assert!(matches!(
            result.pop_front().unwrap(),
            BaseToken::NumberLike(NumberLike::Number(val)) if val == 1.0
        ));
        assert!(matches!(
            result.pop_front().unwrap(),
            BaseToken::Operator(Operator::Plus)
        ));
        assert!(matches!(
            result.pop_front().unwrap(),
            BaseToken::NumberLike(NumberLike::Number(val)) if val == 2.0
        ));
    }
}

#[cfg(test)]
mod rpn_test {
    use crate::formulas::base_formula::{
        BaseFormula, BaseToken, Bracket, CloseBracket, NumberLike, OpenBracket, Operator,
    };
    use std::collections::VecDeque;

    #[test]
    fn easy_rpn_test() {
        let mut initial: VecDeque<BaseToken> = VecDeque::with_capacity(3);
        initial.push_back(1.0.into());
        initial.push_back(Operator::Plus.into());
        initial.push_back(2.0.into());
        let result = BaseFormula::build_rpn(initial);
        assert!(result.is_ok());
        let mut result = result.unwrap();
        assert_eq!(result.len(), 3, "{:?}", result);
        assert!(matches!(
            result.pop_front().unwrap(),
            BaseToken::NumberLike(NumberLike::Number(val)) if val == 1.0
        ));
        assert!(matches!(
            result.pop_front().unwrap(),
            BaseToken::NumberLike(NumberLike::Number(val)) if val == 2.0
        ));
        assert!(matches!(
            result.pop_front().unwrap(),
            BaseToken::Operator(Operator::Plus)
        ));
    }

    #[test]
    fn bracket_test() {
        let mut initial: VecDeque<BaseToken> = VecDeque::with_capacity(7);
        initial.push_back(Bracket::OpenBracket(OpenBracket).into());
        initial.push_back(1.0.into());
        initial.push_back(Operator::Plus.into());
        initial.push_back(2.0.into());
        initial.push_back(Bracket::CloseBracket(CloseBracket).into());
        initial.push_back(Operator::Multiply.into());
        initial.push_back(5.0.into());
        let result = BaseFormula::build_rpn(initial);
        assert!(result.is_ok(), "{:?}", result);
        let mut result = result.unwrap();
        assert_eq!(result.len(), 5, "{:?}", result);
        assert!(matches!(
            result.pop_front().unwrap(),
            BaseToken::NumberLike(NumberLike::Number(val)) if val == 1.0
        ));
        assert!(matches!(
            result.pop_front().unwrap(),
            BaseToken::NumberLike(NumberLike::Number(val)) if val == 2.0
        ));
        assert!(matches!(
            result.pop_front().unwrap(),
            BaseToken::Operator(Operator::Plus)
        ));
        assert!(matches!(
            result.pop_front().unwrap(),
            BaseToken::NumberLike(NumberLike::Number(val)) if val == 5.0
        ));
        assert!(matches!(
            result.pop_front().unwrap(),
            BaseToken::Operator(Operator::Multiply)
        ));
    }

    #[test]
    fn priority_test() {
        let mut initial: VecDeque<BaseToken> = VecDeque::with_capacity(7);
        initial.push_back(1.0.into());
        initial.push_back(Operator::Multiply.into());
        initial.push_back(2.0.into());
        initial.push_back(Operator::Plus.into());
        initial.push_back(5.0.into());
        initial.push_back(Operator::Multiply.into());
        initial.push_back(6.0.into());
        let result = BaseFormula::build_rpn(initial);
        assert!(result.is_ok(), "{:?}", result);
        let mut result = result.unwrap();
        assert_eq!(result.len(), 7, "{:?}", result);
        assert!(matches!(
            result.pop_front().unwrap(),
            BaseToken::NumberLike(NumberLike::Number(val)) if val == 1.0
        ));
        assert!(matches!(
            result.pop_front().unwrap(),
            BaseToken::NumberLike(NumberLike::Number(val)) if val == 2.0
        ));
        assert!(matches!(
            result.pop_front().unwrap(),
            BaseToken::Operator(Operator::Multiply)
        ));
        assert!(matches!(
            result.pop_front().unwrap(),
            BaseToken::NumberLike(NumberLike::Number(val)) if val == 5.0
        ));
        assert!(matches!(
            result.pop_front().unwrap(),
            BaseToken::NumberLike(NumberLike::Number(val)) if val == 6.0
        ));
        assert!(matches!(
            result.pop_front().unwrap(),
            BaseToken::Operator(Operator::Multiply)
        ));
        assert!(matches!(
            result.pop_front().unwrap(),
            BaseToken::Operator(Operator::Plus)
        ));
    }

    #[test]
    fn power_test() {
        let mut initial: VecDeque<BaseToken> = VecDeque::with_capacity(7);
        initial.push_back(5.0.into());
        initial.push_back(Operator::Power.into());
        initial.push_back(6.0.into());
        initial.push_back(Operator::Power.into());
        initial.push_back(7.0.into());
        let result = BaseFormula::build_rpn(initial);
        assert!(result.is_ok(), "{:?}", result);
        let mut result = result.unwrap();
        assert_eq!(result.len(), 5, "{:?}", result);
        assert!(matches!(
            result.pop_front().unwrap(),
            BaseToken::NumberLike(NumberLike::Number(val)) if val == 5.0
        ));
        assert!(matches!(
            result.pop_front().unwrap(),
            BaseToken::NumberLike(NumberLike::Number(val)) if val == 6.0
        ));
        assert!(matches!(
            result.pop_front().unwrap(),
            BaseToken::NumberLike(NumberLike::Number(val)) if val == 7.0
        ));
        assert!(matches!(
            result.pop_front().unwrap(),
            BaseToken::Operator(Operator::Power)
        ));
        assert!(matches!(
            result.pop_front().unwrap(),
            BaseToken::Operator(Operator::Power)
        ));
    }

    #[test]
    fn double_wrong_bracket_test() {
        let mut initial: VecDeque<BaseToken> = VecDeque::with_capacity(6);
        initial.push_back(Bracket::OpenBracket(OpenBracket).into());
        initial.push_back(Bracket::OpenBracket(OpenBracket).into());
        initial.push_back(1.0.into());
        initial.push_back(Operator::Plus.into());
        initial.push_back(2.0.into());
        initial.push_back(Bracket::CloseBracket(CloseBracket).into());
        let result = BaseFormula::build_rpn(initial);
        assert!(result.is_err(), "{:?}", result);
    }

    #[test]
    fn double_bracket_test() {
        let mut initial: VecDeque<BaseToken> = VecDeque::with_capacity(6);
        initial.push_back(Bracket::OpenBracket(OpenBracket).into());
        initial.push_back(Bracket::OpenBracket(OpenBracket).into());
        initial.push_back(1.0.into());
        initial.push_back(Operator::Plus.into());
        initial.push_back(2.0.into());
        initial.push_back(Bracket::CloseBracket(CloseBracket).into());
        initial.push_back(Bracket::CloseBracket(CloseBracket).into());
        let result = BaseFormula::build_rpn(initial);
        assert!(result.is_ok());
        let mut result = result.unwrap();
        assert_eq!(result.len(), 3, "{:?}", result);
        assert!(matches!(
            result.pop_front().unwrap(),
            BaseToken::NumberLike(NumberLike::Number(val)) if val == 1.0
        ));
        assert!(matches!(
            result.pop_front().unwrap(),
            BaseToken::NumberLike(NumberLike::Number(val)) if val == 2.0
        ));
        assert!(matches!(
            result.pop_front().unwrap(),
            BaseToken::Operator(Operator::Plus)
        ));
    }
}

#[cfg(test)]
mod compress_test {
    use crate::formulas::base_formula::{
        BaseFormula, BaseToken, FormulaArgument, NumberLike, Operator,
    };
    use std::collections::VecDeque;

    #[test]
    fn easy_test() {
        let mut initial: VecDeque<BaseToken> = VecDeque::with_capacity(3);
        initial.push_back(1.0.into());
        initial.push_back(2.0.into());
        initial.push_back(Operator::Plus.into());
        let result = BaseFormula::compress_rpn(initial);
        assert!(result.is_some());
        let result = result.unwrap();
        assert!(
            matches!(result, FormulaArgument::NumberLike(NumberLike::Number(val)) if val == 3.0),
            "{:?}",
            result
        );
    }
}
