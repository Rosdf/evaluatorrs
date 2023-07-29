use crate::formulas::{Evaluate, ExpressionParsingError, Formula, ParenthesisError, ParserError};
use crate::utils::{Queue, Stack};
use crate::variable_stores::{GetVariable, HashMapStore};
use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;
use std::str::FromStr;

#[derive(Debug)]
pub(crate) struct BaseFormula {
    tree: OperatorFormulaArgument,
}

impl Evaluate for BaseFormula {
    fn eval<T: GetVariable>(&mut self, args: &T) -> f64
    where
        Self: Sized,
    {
        self.tree.eval(args)
    }
}

impl Formula for BaseFormula {
    fn parse(arguments: Vec<String>) -> Result<Self, ParserError>
    where
        Self: Sized,
    {
        todo!()
    }

    fn num_of_arguments() -> usize
    where
        Self: Sized,
    {
        todo!()
    }

    fn is_const(&self) -> bool {
        self.tree.
    }
}

#[derive(Debug)]
enum OperatorFormulaArgument {
    NumberLike(NumberLike),
    FormulaLike(FormulaLike),
}

impl From<NumberLike> for OperatorFormulaArgument {
    fn from(value: NumberLike) -> Self {
        OperatorFormulaArgument::NumberLike(value.into())
    }
}

impl From<f64> for OperatorFormulaArgument {
    fn from(value: f64) -> Self {
        OperatorFormulaArgument::NumberLike(value.into())
    }
}

impl From<FormulaLike> for OperatorFormulaArgument {
    fn from(value: FormulaLike) -> Self {
        Self::FormulaLike(value.into())
    }
}

impl From<Box<dyn Formula>> for OperatorFormulaArgument {
    fn from(value: Box<dyn Formula>) -> Self {
        Self::FormulaLike(value.into())
    }
}

impl From<Box<dyn OperatorFormula>> for OperatorFormulaArgument {
    fn from(value: Box<dyn OperatorFormula>) -> Self {
        Self::FormulaLike(value.into())
    }
}

impl TryFrom<RpnToken> for OperatorFormulaArgument {
    type Error = ();

    fn try_from(value: RpnToken) -> Result<Self, Self::Error> {
        match value {
            RpnToken::NumberLike(number_like) => Ok(Self::NumberLike(number_like)),
            RpnToken::Operator(_) => Err(()),
            RpnToken::Formula(formula) => Ok(Self::FormulaLike(formula.into())),
        }
    }
}

impl Evaluate for OperatorFormulaArgument {
    // fn eval_dyn(&mut self, args: &dyn GetVariable) -> f64 {
    //     match self {
    //         OperatorFormulaArgument::NumberLike(num) => num.eval_dyn(args),
    //         OperatorFormulaArgument::FormulaLike(formula) => formula.eval_dyn(args),
    //     }
    // }

    fn eval<T: GetVariable>(&mut self, args: &T) -> f64
    where
        Self: Sized,
    {
        match self {
            OperatorFormulaArgument::NumberLike(num) => num.eval(args),
            OperatorFormulaArgument::FormulaLike(formula) => formula.eval(args),
        }
    }
}

#[derive(Debug)]
enum FormulaLike {
    Formula(Box<dyn Formula>),
    OperatorFormula(Box<dyn OperatorFormula>),
}

impl From<Box<dyn Formula>> for FormulaLike {
    fn from(value: Box<dyn Formula>) -> Self {
        Self::Formula(value)
    }
}

impl From<Box<dyn OperatorFormula>> for FormulaLike {
    fn from(value: Box<dyn OperatorFormula>) -> Self {
        Self::OperatorFormula(value)
    }
}

impl Evaluate for FormulaLike {
    fn eval<T: GetVariable>(&mut self, args: &T) -> f64
    where
        Self: Sized,
    {
        return match self {
            FormulaLike::Formula(formula) => formula.eval_dyn(args),
            FormulaLike::OperatorFormula(operator) => operator.eval_dyn(args),
        };
    }
}

trait OperatorFormula: Evaluate + Debug {
    fn new(first: OperatorFormulaArgument, second: OperatorFormulaArgument) -> Self
    where
        Self: Sized;

    fn is_const(&self) -> bool;
}

#[derive(Debug)]
enum NumberLike {
    Number(f64),
    Variable(String),
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
    fn eval<T: GetVariable + ?Sized>(&mut self, args: &T) -> f64 {
        return match self {
            NumberLike::Number(num) => num.clone(),
            NumberLike::Variable(var) => args.get(var).unwrap(),
        };
    }
}

#[derive(Debug)]
struct OpenBracket;

#[derive(Debug)]
struct CloseBracket;

#[derive(Debug)]
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

#[derive(Debug)]
pub(crate) enum Operator {
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

    fn to_formula(
        self,
        first: OperatorFormulaArgument,
        second: OperatorFormulaArgument,
    ) -> Box<dyn OperatorFormula> {
        todo!()
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

enum BaseToken {
    NumberLike(NumberLike),
    Operator(Operator),
    Bracket(Bracket),
    Formula(Box<dyn Formula>),
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

impl From<Box<dyn Formula>> for BaseToken {
    fn from(value: Box<dyn Formula>) -> Self {
        Self::Formula(value)
    }
}

enum RpnToken {
    NumberLike(NumberLike),
    Operator(Operator),
    Formula(Box<dyn Formula>),
}

enum OperatorStackToken {
    Operator(Operator),
    OpenBracket(OpenBracket),
}

impl TryFrom<OperatorStackToken> for RpnToken {
    type Error = ();

    fn try_from(value: OperatorStackToken) -> Result<Self, Self::Error> {
        match value {
            OperatorStackToken::Operator(operator) => Ok(RpnToken::Operator(operator)),
            OperatorStackToken::OpenBracket(_) => Err(()),
        }
    }
}

impl From<f64> for RpnToken {
    fn from(value: f64) -> Self {
        Self::NumberLike(value.into())
    }
}

impl BaseFormula {
    fn parse_parenthesis(expression: &mut String) -> Option<Bracket> {
        if expression.is_empty() {
            return None;
        }
        let res = Bracket::parse(expression.chars().next().unwrap());
        if res.is_some() {
            expression.remove(0);
        }
        res
    }

    fn parse_number(expression: &mut String) -> Option<f64> {
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
            *expression = expression[parsed..].to_string();
            return Some(ret);
        }
        None
    }

    fn parse_operator(expression: &mut String) -> Option<Operator> {
        if expression.is_empty() {
            return None;
        }
        let res = Operator::parse(expression.chars().next().unwrap());
        if res.is_some() {
            expression.remove(0);
        }
        res
    }

    fn parse_expression(
        mut expression: String,
    ) -> Result<VecDeque<BaseToken>, ExpressionParsingError> {
        let mut res = VecDeque::new();
        let mut prev_len = expression.len() + 1;
        while expression.len() < prev_len {
            prev_len = expression.len();

            if let Some(bra) = Self::parse_parenthesis(&mut expression) {
                res.push_back(bra.into());
            }

            if let Some(num) = Self::parse_number(&mut expression) {
                res.push_back(num.into());
            }

            if let Some(operator) = Self::parse_operator(&mut expression) {
                res.push_back(operator.into());
            }
        }
        if expression.is_empty() {
            return Ok(res);
        }
        Err(ExpressionParsingError {
            left_expression: expression,
        })
    }

    fn process_bracket(
        bra: Bracket,
        rpn: &mut impl Queue<RpnToken>,
        operator_stack: &mut impl Stack<OperatorStackToken>,
    ) -> Result<(), ParenthesisError> {
        match bra {
            Bracket::OpenBracket(par) => operator_stack.push(OperatorStackToken::OpenBracket(par)),
            Bracket::CloseBracket(_) => {
                let mut found_open = false;
                while let Some(oper) = operator_stack.pop() {
                    match oper {
                        OperatorStackToken::Operator(operator) => {
                            rpn.push(RpnToken::Operator(operator));
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
        rpn: &mut impl Queue<RpnToken>,
        operator_stack: &mut impl Stack<OperatorStackToken>,
    ) {
        if op.side() == Side::Right {
            operator_stack.push(OperatorStackToken::Operator(op));
            return;
        }
        while let Some(oper) = operator_stack.pop() {
            match oper {
                OperatorStackToken::Operator(oper) => {
                    if op.priority() >= oper.priority() {
                        rpn.push(operator_stack.pop().unwrap().try_into().unwrap())
                    } else {
                        break;
                    }
                }
                OperatorStackToken::OpenBracket(_) => break,
            }
        }
        operator_stack.push(OperatorStackToken::Operator(op));
    }

    fn build_rpn(
        mut tokens: impl Queue<BaseToken>,
    ) -> Result<VecDeque<RpnToken>, ParenthesisError> {
        let mut res = VecDeque::new();
        let mut stack = VecDeque::<OperatorStackToken>::new();
        while let Some(token) = tokens.pop() {
            match token {
                BaseToken::NumberLike(num_like) => res.push_back(RpnToken::NumberLike(num_like)),
                BaseToken::Operator(operator) => {
                    Self::process_operator(operator, &mut res, &mut stack)
                }
                BaseToken::Bracket(bra) => Self::process_bracket(bra, &mut res, &mut stack)?,
                BaseToken::Formula(formula) => res.push_back(RpnToken::Formula(formula)),
            }
        }
        Ok(res)
    }

    fn compress_rpn(mut rpn: VecDeque<RpnToken>) -> Option<OperatorFormulaArgument> {
        let const_store = HashMapStore::new();
        let initial_len = rpn.len();
        for _ in 0..initial_len {
            let token = rpn.pop_front().unwrap();
            match token {
                RpnToken::NumberLike(num_like) => rpn.push_front(RpnToken::NumberLike(num_like)),
                RpnToken::Operator(operator) => {
                    let second = rpn.pop_front().unwrap();
                    let first = rpn.pop_front().unwrap();
                    let mut operator_formula =
                        operator.to_formula(first.try_into().unwrap(), second.try_into().unwrap());
                    rpn.push_front(if operator_formula.is_const() {
                        operator_formula.eval_dyn(&const_store).into()
                    } else {
                        operator_formula.into()
                    });
                }
                RpnToken::Formula(formula) => rpn.push_front(RpnToken::Formula(formula)),
            }
        }
        if rpn.len() == 1 {
            return Some(rpn.pop_front().unwrap().into());
        }
        None
    }

    pub fn new(mut expression: String) -> Self {
        let mut parsed = Self::parse_expression(expression).unwrap();
        let mut rpn = Self::build_rpn(parsed).unwrap();
        Self {
            tree: Self::compress_rpn(rpn).unwrap(),
        }
    }
}
