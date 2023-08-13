use crate::formula_stores::GetParser;
use crate::formulas::{
    Evaluate, EvaluationError, FunctionLike, IsConst, MathError, OpernandNumError,
    ParenthesisError, ParserError, UnknownTokenError,
};
use crate::tokens::{BaseToken, Bracket, NumberLike, OpenBracket, Operator, Side};
use crate::variable_stores::{EmptyVariableStore, GetVariable, Variable};
use std::collections::VecDeque;
use std::fmt::Debug;
use std::str::FromStr;
use std::sync::Arc;

#[derive(Debug)]
#[non_exhaustive]
pub enum FormulaArgument {
    Number(f64),
    Variable(Variable),
    OwnedFunction(Box<dyn FunctionLike>),
    SharedFunction(Arc<dyn FunctionLike>),
}

impl From<NumberLike> for FormulaArgument {
    fn from(value: NumberLike) -> Self {
        match value {
            NumberLike::Number(num) => Self::Number(num),
            NumberLike::Variable(var) => Self::Variable(var),
        }
    }
}

impl From<f64> for FormulaArgument {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}

impl From<Arc<dyn FunctionLike>> for FormulaArgument {
    fn from(value: Arc<dyn FunctionLike>) -> Self {
        Self::SharedFunction(value)
    }
}

impl TryFrom<BaseToken> for FormulaArgument {
    type Error = ();

    fn try_from(value: BaseToken) -> Result<Self, Self::Error> {
        match value {
            BaseToken::NumberLike(val) => Ok(val.into()),
            BaseToken::Operator(_) | BaseToken::Bracket(_) => Err(()),
            BaseToken::Formula(val) => Ok(Self::OwnedFunction(val)),
        }
    }
}

#[derive(Debug)]
pub struct RootFormula {
    tree: FormulaArgument,
}

impl Evaluate for RootFormula {
    fn eval(&self, args: &dyn GetVariable) -> Result<f64, EvaluationError> {
        match &self.tree {
            FormulaArgument::OwnedFunction(formula) => formula.eval(args),
            FormulaArgument::SharedFunction(formula) => formula.eval(args),
            FormulaArgument::Number(num) => Ok(*num),
            FormulaArgument::Variable(var) => args.eval(var),
        }
    }
}

impl IsConst for RootFormula {
    fn is_const(&self) -> bool {
        match &self.tree {
            FormulaArgument::OwnedFunction(val) => val.is_const(),
            FormulaArgument::SharedFunction(val) => val.is_const(),
            FormulaArgument::Number(_) => true,
            FormulaArgument::Variable(_) => false,
        }
    }
}

impl FunctionLike for RootFormula {
    fn collapse_inner(&mut self) -> Result<(), MathError> {
        if self.is_const() {
            self.tree = FormulaArgument::Number(match self.eval(&EmptyVariableStore) {
                Ok(val) => val,
                Err(EvaluationError::MathError(e)) => return Err(e),
                Err(EvaluationError::NoVariableError) => unreachable!(),
            });
            return Ok(());
        }
        if let FormulaArgument::OwnedFunction(function) = &mut self.tree {
            function.collapse_inner()
        } else {
            Ok(())
        }
    }

    fn set_variables_shared(&mut self, args: &dyn GetVariable) {
        if let FormulaArgument::Variable(variable) = &self.tree {
            if let Some(function) = args.get(variable) {
                self.tree =
                    FormulaArgument::SharedFunction(Arc::clone(function) as Arc<dyn FunctionLike>);
            }
        }
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

impl RootFormula {
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
                    parsed += 1;
                }
                _ => break,
            }
        }
        if encountered_digit {
            let (for_num, rest_str) = expression.split_at(parsed);
            let ret = f64::from_str(for_num).unwrap();
            *expression = rest_str;
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

    fn parse_variable(expression: &mut &str) -> Option<Variable> {
        let mut parsed: usize = 0;
        let mut started = false;
        for elem in expression.chars() {
            if elem.is_alphabetic() {
                parsed += 1;
                started = true;
                continue;
            }
            if started && elem.is_ascii_digit() {
                parsed += 1;
            }
            break;
        }
        if !started {
            return None;
        }
        let (for_var, rest) = expression.split_at(parsed);
        let result = Variable::from(for_var);
        *expression = rest;
        Some(result)
    }

    fn lex_expression<T: GetParser>(
        mut expression: &str,
        formulas: &T,
    ) -> Result<VecDeque<BaseToken>, UnknownTokenError> {
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

            Self::remove_spaces(&mut expression);
            if let Some(variable) = Self::parse_variable(&mut expression) {
                res.push_back(variable.into());
            }
        }
        if expression.is_empty() {
            return Ok(res);
        }
        Err(UnknownTokenError)
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
                        rpn.push_back(operator_stack.pop().unwrap().into());
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
                    tokens.push_back(BaseToken::NumberLike(num_like));
                }
                BaseToken::Operator(operator) => {
                    Self::process_operator(operator, &mut tokens, &mut stack);
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

    fn push_formula<T: FunctionLike + Into<BaseToken>>(
        rpn: &mut VecDeque<BaseToken>,
        mut formula: T,
    ) -> Result<(), MathError> {
        if formula.is_const() {
            rpn.push_back(formula.eval(&EmptyVariableStore)?.into())
        }
        formula.collapse_inner()?;
        rpn.push_back(formula.into());
        Ok(())
    }

    fn compress_rpn(mut rpn: VecDeque<BaseToken>) -> Result<FormulaArgument, ParserError> {
        let initial_len = rpn.len();
        for _ in 0..initial_len {
            let token = rpn.pop_front().unwrap();
            match token {
                BaseToken::NumberLike(num_like) => rpn.push_back(BaseToken::NumberLike(num_like)),
                BaseToken::Operator(operator) => {
                    let second = match rpn.pop_back() {
                        None => return Err(ParserError::OpernandNumError(OpernandNumError)),
                        Some(val) => Self::new::<FormulaArgument>(val.try_into().unwrap()),
                    };
                    let first = match rpn.pop_back() {
                        None => return Err(ParserError::OpernandNumError(OpernandNumError)),
                        Some(val) => Self::new::<FormulaArgument>(val.try_into().unwrap()),
                    };
                    let operator_formula = operator.into_formula(first, second);
                    Self::push_formula(&mut rpn, operator_formula)?;
                }
                BaseToken::Formula(formula) => {
                    Self::push_formula(&mut rpn, formula)?;
                }
                BaseToken::Bracket(_) => unreachable!(),
            }
        }
        if rpn.len() == 1 {
            return Ok(rpn.pop_front().unwrap().try_into().unwrap());
        }
        Err(ParserError::OpernandNumError(OpernandNumError))
    }

    pub fn new<T: Into<FormulaArgument>>(value: T) -> Self {
        Self { tree: value.into() }
    }

    /// # Errors
    ///
    /// will return Err if non valid expression is passed
    pub fn parse<T: GetParser>(expression: &str, formulas: &T) -> Result<Self, ParserError> {
        let parsed = Self::lex_expression(expression, formulas)?;
        let rpn = Self::build_rpn(parsed)?;
        Ok(Self {
            tree: Self::compress_rpn(rpn)?,
        })
    }
}

#[cfg(test)]
mod lexer_test {
    use crate::formula_stores::EmptyFormulaStore;
    use crate::formulas::root_formula::RootFormula;
    use crate::tokens::{BaseToken, NumberLike, Operator};

    #[test]
    fn remove_all_spaces() {
        let mut expression = "     ";
        RootFormula::remove_spaces(&mut expression);
        assert_eq!(expression, "", "{expression}");
    }

    #[test]
    fn remove_spaces_with_text() {
        let mut expression = "     asd";
        RootFormula::remove_spaces(&mut expression);
        assert_eq!(expression, "asd", "{expression}");
    }

    #[test]
    fn remove_spaces() {
        let mut expression = "     asd   ";
        RootFormula::remove_spaces(&mut expression);
        assert_eq!(expression, "asd   ", "{expression}");
    }

    #[test]
    fn number_parser() {
        assert_eq!(RootFormula::parse_number(&mut "1"), Some(1.0));
        assert_eq!(RootFormula::parse_number(&mut "-1"), Some(-1.0));
        assert_eq!(RootFormula::parse_number(&mut "1.0"), Some(1.0));
        assert_eq!(RootFormula::parse_number(&mut "1.1"), Some(1.1));
        assert_eq!(RootFormula::parse_number(&mut "0.1"), Some(0.1));
        assert_eq!(RootFormula::parse_number(&mut "0.0"), Some(0.0));
        assert_eq!(RootFormula::parse_number(&mut "+"), None);
        assert_eq!(RootFormula::parse_number(&mut "1.0001"), Some(1.0001));
        assert_eq!(RootFormula::parse_number(&mut "-1.03456"), Some(-1.03456));
    }

    #[test]
    fn basic_test_lex() {
        let expression = "1+2";
        let result = RootFormula::lex_expression(expression, &EmptyFormulaStore);

        assert!(result.is_ok());
        let mut result = result.unwrap();
        assert_eq!(result.len(), 3);
        assert!(matches!(
            result.pop_front().unwrap(),
            BaseToken::NumberLike(NumberLike::Number(val)) if (val - 1.0).abs() < f64::EPSILON
        ));
        assert!(matches!(
            result.pop_front().unwrap(),
            BaseToken::Operator(Operator::Plus)
        ));
        assert!(matches!(
            result.pop_front().unwrap(),
            BaseToken::NumberLike(NumberLike::Number(val)) if (val - 2.0).abs() < f64::EPSILON
        ));
    }

    #[test]
    fn space_test_lex() {
        let expression = "  1 +   2 ";
        let result = RootFormula::lex_expression(expression, &EmptyFormulaStore);

        assert!(result.is_ok());
        let mut result = result.unwrap();
        assert_eq!(result.len(), 3);
        assert!(matches!(
            result.pop_front().unwrap(),
            BaseToken::NumberLike(NumberLike::Number(val)) if (val - 1.0).abs() < f64::EPSILON
        ));
        assert!(matches!(
            result.pop_front().unwrap(),
            BaseToken::Operator(Operator::Plus)
        ));
        assert!(matches!(
            result.pop_front().unwrap(),
            BaseToken::NumberLike(NumberLike::Number(val)) if (val - 2.0).abs() < f64::EPSILON
        ));
    }
}

#[cfg(test)]
mod rpn_test {
    use crate::formulas::root_formula::RootFormula;
    use crate::tokens::{BaseToken, Bracket, CloseBracket, NumberLike, OpenBracket, Operator};
    use std::collections::VecDeque;

    #[test]
    fn easy_rpn_test() {
        let mut initial: VecDeque<BaseToken> = VecDeque::with_capacity(3);
        initial.push_back(1.0.into());
        initial.push_back(Operator::Plus.into());
        initial.push_back(2.0.into());
        let result = RootFormula::build_rpn(initial);
        assert!(result.is_ok());
        let mut result = result.unwrap();
        assert_eq!(result.len(), 3, "{result:?}");
        assert!(matches!(
            result.pop_front().unwrap(),
            BaseToken::NumberLike(NumberLike::Number(val)) if (val - 1.0).abs() < f64::EPSILON
        ));
        assert!(matches!(
            result.pop_front().unwrap(),
            BaseToken::NumberLike(NumberLike::Number(val)) if (val - 2.0).abs() < f64::EPSILON
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
        let result = RootFormula::build_rpn(initial);
        assert!(result.is_ok(), "{result:?}");
        let mut result = result.unwrap();
        assert_eq!(result.len(), 5, "{result:?}");
        assert!(matches!(
            result.pop_front().unwrap(),
            BaseToken::NumberLike(NumberLike::Number(val)) if (val - 1.0).abs() < f64::EPSILON
        ));
        assert!(matches!(
            result.pop_front().unwrap(),
            BaseToken::NumberLike(NumberLike::Number(val)) if (val - 2.0).abs() < f64::EPSILON
        ));
        assert!(matches!(
            result.pop_front().unwrap(),
            BaseToken::Operator(Operator::Plus)
        ));
        assert!(matches!(
            result.pop_front().unwrap(),
            BaseToken::NumberLike(NumberLike::Number(val)) if (val - 5.0).abs() < f64::EPSILON
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
        let result = RootFormula::build_rpn(initial);
        assert!(result.is_ok(), "{result:?}");
        let mut result = result.unwrap();
        assert_eq!(result.len(), 7, "{result:?}");
        assert!(matches!(
            result.pop_front().unwrap(),
            BaseToken::NumberLike(NumberLike::Number(val)) if (val - 1.0).abs() < f64::EPSILON
        ));
        assert!(matches!(
            result.pop_front().unwrap(),
            BaseToken::NumberLike(NumberLike::Number(val)) if (val - 2.0).abs() < f64::EPSILON
        ));
        assert!(matches!(
            result.pop_front().unwrap(),
            BaseToken::Operator(Operator::Multiply)
        ));
        assert!(matches!(
            result.pop_front().unwrap(),
            BaseToken::NumberLike(NumberLike::Number(val)) if (val - 5.0).abs() < f64::EPSILON
        ));
        assert!(matches!(
            result.pop_front().unwrap(),
            BaseToken::NumberLike(NumberLike::Number(val)) if (val - 6.0).abs() < f64::EPSILON
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
        initial.push_back(Operator::Exponent.into());
        initial.push_back(6.0.into());
        initial.push_back(Operator::Exponent.into());
        initial.push_back(7.0.into());
        let result = RootFormula::build_rpn(initial);
        assert!(result.is_ok(), "{result:?}");
        let mut result = result.unwrap();
        assert_eq!(result.len(), 5, "{result:?}");
        assert!(matches!(
            result.pop_front().unwrap(),
            BaseToken::NumberLike(NumberLike::Number(val)) if (val - 5.0).abs() < f64::EPSILON
        ));
        assert!(matches!(
            result.pop_front().unwrap(),
            BaseToken::NumberLike(NumberLike::Number(val)) if (val - 6.0).abs() < f64::EPSILON
        ));
        assert!(matches!(
            result.pop_front().unwrap(),
            BaseToken::NumberLike(NumberLike::Number(val)) if (val - 7.0).abs() < f64::EPSILON
        ));
        assert!(matches!(
            result.pop_front().unwrap(),
            BaseToken::Operator(Operator::Exponent)
        ));
        assert!(matches!(
            result.pop_front().unwrap(),
            BaseToken::Operator(Operator::Exponent)
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
        let result = RootFormula::build_rpn(initial);
        assert!(result.is_err(), "{result:?}");
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
        let result = RootFormula::build_rpn(initial);
        assert!(result.is_ok());
        let mut result = result.unwrap();
        assert_eq!(result.len(), 3, "{result:?}");
        assert!(matches!(
            result.pop_front().unwrap(),
            BaseToken::NumberLike(NumberLike::Number(val)) if (val - 1.0).abs() < f64::EPSILON
        ));
        assert!(matches!(
            result.pop_front().unwrap(),
            BaseToken::NumberLike(NumberLike::Number(val)) if (val - 2.0).abs() < f64::EPSILON
        ));
        assert!(matches!(
            result.pop_front().unwrap(),
            BaseToken::Operator(Operator::Plus)
        ));
    }
}

#[cfg(test)]
mod compress_test {
    use crate::formulas::root_formula::FormulaArgument;
    use crate::formulas::root_formula::RootFormula;
    use crate::tokens::{BaseToken, Operator};
    use std::collections::VecDeque;

    #[test]
    fn easy_test() {
        let mut initial: VecDeque<BaseToken> = VecDeque::with_capacity(3);
        initial.push_back(1.0.into());
        initial.push_back(2.0.into());
        initial.push_back(Operator::Plus.into());
        let result = RootFormula::compress_rpn(initial);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(
            matches!(result, FormulaArgument::Number(val) if (val - 3.0).abs() < f64::EPSILON),
            "{result:?}"
        );
    }
}

#[cfg(test)]
mod parser_test {
    use crate::formula_stores::EmptyFormulaStore;
    use crate::formulas::root_formula::RootFormula;

    #[test]
    fn test_str_unchanged() {
        let expression = "5 + 1";
        let _ = RootFormula::parse(expression, &EmptyFormulaStore);
        assert_eq!(expression, "5 + 1");
    }
}
