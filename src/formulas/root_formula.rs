use crate::__lib::boxed::Box;
use crate::__lib::fmt::Debug;
use crate::__lib::ops::{Add, Div, Mul, Sub};
use crate::formulas::root_formula::formula_argument::FormulaArgument;
use crate::formulas::root_formula::lexer::lex_expression;
use crate::formulas::{Evaluate, EvaluationError, FunctionLike, IsConst, MathError, ParserError};
use crate::function_stores::GetFunction;
use crate::tokens::{BaseToken, Operator};
use crate::variable_stores::{EmptyVariableStore, GetVariable, Variable};

use crate::__lib::sync::Arc;
use crate::formulas::root_formula::parser::parse_tokens;

mod formula_argument {
    use super::{Arc, BaseToken, Debug, FunctionLike, Variable};
    use crate::__lib::boxed::Box;
    use crate::tokens::NumberLike;
    use core::convert::TryFrom;

    #[derive(Debug)]
    #[non_exhaustive]
    pub enum FormulaArgument {
        Number(f64),
        Variable(Variable),
        OwnedFunction(Box<dyn FunctionLike>),
        SharedFunction(Arc<dyn FunctionLike>),
    }

    pub const TMP_FORMULA_ARGUMENT: FormulaArgument = FormulaArgument::Number(f64::NAN);

    impl Default for FormulaArgument {
        fn default() -> Self {
            TMP_FORMULA_ARGUMENT
        }
    }

    impl From<NumberLike> for FormulaArgument {
        fn from(value: NumberLike) -> Self {
            match value {
                NumberLike::Number(num) => Self::Number(num),
                NumberLike::Variable(var) => Self::Variable(var),
            }
        }
    }

    impl From<Variable> for FormulaArgument {
        fn from(value: Variable) -> Self {
            Self::Variable(value)
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

    impl From<Box<dyn FunctionLike>> for FormulaArgument {
        fn from(value: Box<dyn FunctionLike>) -> Self {
            Self::OwnedFunction(value)
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
}

/// Struct to store information about the formula.
#[derive(Debug, Default)]
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

impl Clone for RootFormula {
    fn clone(&self) -> Self {
        match &self.tree {
            FormulaArgument::Number(num) => Self::new(*num),
            FormulaArgument::Variable(var) => Self::new(var.clone()),
            FormulaArgument::OwnedFunction(func) => Self {
                tree: FormulaArgument::OwnedFunction(func.clone_into_box()),
            },
            FormulaArgument::SharedFunction(func) => Self {
                tree: FormulaArgument::SharedFunction(Arc::clone(func)),
            },
        }
    }
}

impl FunctionLike for RootFormula {
    fn collapse_inner(&mut self) -> Result<(), MathError> {
        if self.is_const() {
            self.tree = FormulaArgument::Number(match self.eval(&EmptyVariableStore) {
                Ok(val) => val,
                Err(EvaluationError::MathError(e)) => return Err(e),
                Err(EvaluationError::NoVariableError(_)) => unreachable!(),
            });
            return Ok(());
        }
        if let FormulaArgument::SharedFunction(function) = &mut self.tree {
            if let Some(func) = Arc::get_mut(function) {
                self.tree = FormulaArgument::OwnedFunction(func.clone_into_box());
            }
        }
        if let FormulaArgument::OwnedFunction(function) = &mut self.tree {
            function.collapse_inner()
        } else {
            Ok(())
        }
    }

    fn set_all_variables_shared(&mut self, args: &dyn GetVariable) {
        match self.tree {
            FormulaArgument::Variable(ref variable) => {
                if let Some(function) = args.get(variable) {
                    self.tree = FormulaArgument::SharedFunction(
                        Arc::clone(function) as Arc<dyn FunctionLike>
                    );
                }
            }
            FormulaArgument::OwnedFunction(ref mut function) => {
                function.set_all_variables_shared(args);
            }
            _ => {}
        }
    }

    fn set_all_variables_owned(&mut self, args: &dyn GetVariable) {
        match self.tree {
            FormulaArgument::Variable(ref variable) => {
                if let Some(function) = args.get(variable) {
                    self.tree = FormulaArgument::OwnedFunction(function.clone_into_box());
                }
            }
            FormulaArgument::OwnedFunction(ref mut function) => {
                function.set_all_variables_shared(args);
            }
            _ => {}
        }
    }

    fn set_variable_shared(&mut self, name: &Variable, function: &Arc<RootFormula>) {
        match self.tree {
            FormulaArgument::Variable(ref variable) => {
                if variable == name {
                    self.tree = FormulaArgument::SharedFunction(
                        Arc::clone(function) as Arc<dyn FunctionLike>
                    );
                }
            }
            FormulaArgument::OwnedFunction(ref mut local_function) => {
                local_function.set_variable_shared(name, function);
            }
            _ => {}
        }
    }

    fn set_variable_owned(&mut self, name: &Variable, function: &RootFormula) {
        match self.tree {
            FormulaArgument::Variable(ref variable) => {
                if variable == name {
                    self.tree = FormulaArgument::OwnedFunction(function.clone_into_box());
                }
            }
            FormulaArgument::OwnedFunction(ref mut local_function) => {
                local_function.set_variable_owned(name, function);
            }
            _ => {}
        }
    }

    fn clone_into_box(&self) -> Box<dyn FunctionLike> {
        Box::new(self.clone())
    }
}

mod lexer {
    use crate::__lib::boxed::Box;
    use crate::__lib::collections::VecDeque;
    use crate::__lib::str::FromStr;
    use crate::__lib::string::ToString;
    use crate::__lib::vec::Vec;
    use crate::formulas::{ArgumentsError, FunctionLike, ParserError, UnknownTokenError};
    use crate::function_stores::GetFunction;
    use crate::tokens::{BaseToken, Bracket, Operator};
    use crate::variable_stores::Variable;

    fn lex_parenthesis(expression: &mut &str) -> Option<Bracket> {
        if expression.is_empty() {
            return None;
        }
        let res = Bracket::parse(expression.chars().next().unwrap());
        if res.is_some() {
            *expression = &expression[1..];
        }
        res
    }

    fn lex_number(expression: &mut &str) -> Option<f64> {
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

    fn lex_operator(expression: &mut &str) -> Option<Operator> {
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

    fn lex_variable(expression: &mut &str) -> Option<Variable> {
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

    fn collect_arguments<'a>(expression: &mut &'a str) -> Box<[&'a str]> {
        let mut arguments = Vec::new();
        let mut brackets: usize = 1;
        let mut prev_comma: usize = 0;
        let mut early_exit = false;
        let mut last_index = 0;
        for (index, elem) in expression.chars().enumerate() {
            match elem {
                '(' => brackets += 1,
                ')' => {
                    brackets -= 1;
                    if brackets == 0 {
                        last_index = index;
                        early_exit = true;
                        arguments.push(&expression[prev_comma..index]);
                        break;
                    }
                }
                ',' if brackets == 1 => {
                    arguments.push(&expression[prev_comma..index]);
                    prev_comma = index + 1;
                }
                _ => {}
            }
        }
        if !early_exit {
            last_index = expression.len() - 1;
        }

        *expression = &expression[last_index + 1..];
        arguments.into_boxed_slice()
    }

    fn lex_function<T: for<'a> GetFunction<'a>>(
        expression: &mut &str,
        functions: &T,
    ) -> Option<Result<Box<dyn FunctionLike>, ParserError>> {
        for function_name in functions.iter() {
            if let Some(without_name) = expression.strip_prefix(function_name) {
                if let Some(mut without_name) = without_name.strip_prefix('(') {
                    let arguments = collect_arguments(&mut without_name);
                    let (parser, arg_num) = functions.function_parser(function_name).unwrap();
                    if arguments.len() < arg_num.min || arguments.len() > arg_num.max {
                        return Some(Err(ParserError::ArgumentsError(ArgumentsError(
                            function_name.into(),
                        ))));
                    }
                    *expression = without_name;
                    return Some(parser(arguments.as_ref()));
                }
            }
        }
        None
    }

    pub(super) fn lex_expression<T: for<'a> GetFunction<'a>>(
        mut expression: &str,
        formulas: &T,
    ) -> Result<VecDeque<BaseToken>, ParserError> {
        let mut res = VecDeque::new();
        let mut prev_len = expression.len() + 1;
        while expression.len() < prev_len {
            prev_len = expression.len();
            remove_spaces(&mut expression);
            if let Some(bra) = lex_parenthesis(&mut expression) {
                res.push_back(bra.into());
                remove_spaces(&mut expression);
            }

            if let Some(num) = lex_number(&mut expression) {
                res.push_back(num.into());
                remove_spaces(&mut expression);
            }

            if let Some(operator) = lex_operator(&mut expression) {
                res.push_back(operator.into());
                remove_spaces(&mut expression);
            }

            if let Some(function) = lex_function(&mut expression, formulas) {
                res.push_back(function?.into());
                remove_spaces(&mut expression);
            }

            if let Some(variable) = lex_variable(&mut expression) {
                res.push_back(variable.into());
                remove_spaces(&mut expression);
            }
        }
        if expression.is_empty() {
            return Ok(res);
        }
        Err(ParserError::UnknownTokenError(UnknownTokenError(
            expression.to_string(),
        )))
    }

    #[cfg(test)]
    mod test {
        use crate::__lib::convert::identity;
        use crate::formulas::root_formula::lexer::{
            collect_arguments, lex_expression, lex_function, lex_number, lex_parenthesis,
            remove_spaces,
        };
        use crate::function_stores::{EmptyFunctionStore, RegisterParser, VectorFunctionStore};
        use crate::tokens::{BaseToken, Bracket, NumberLike, Operator};

        impl_one_arg_function!(
            "ident",
            identity,
            /// Natural logarithm function.
            Ident
        );

        #[test]
        fn test_lex_open_bracket() {
            let mut expression = "(a";
            let res = lex_parenthesis(&mut expression);
            assert!(res.is_some());
            let res = res.unwrap();
            assert!(matches!(res, Bracket::OpenBracket(_)));
            assert_eq!(expression, "a", "{expression}");
        }

        #[test]
        fn test_lex_close_bracket() {
            let mut expression = ")a";
            let res = lex_parenthesis(&mut expression);
            assert!(res.is_some());
            let res = res.unwrap();
            assert!(matches!(res, Bracket::CloseBracket(_)));
            assert_eq!(expression, "a", "{expression}");
        }

        #[test]
        fn test_remove_all_spaces() {
            let mut expression = "     ";
            remove_spaces(&mut expression);
            assert_eq!(expression, "", "{expression}");
        }

        #[test]
        fn test_remove_spaces_with_text() {
            let mut expression = "     asd";
            remove_spaces(&mut expression);
            assert_eq!(expression, "asd", "{expression}");
        }

        #[test]
        fn test_remove_spaces() {
            let mut expression = "     asd   ";
            remove_spaces(&mut expression);
            assert_eq!(expression, "asd   ", "{expression}");
        }

        #[test]
        fn number_parser() {
            assert_eq!(lex_number(&mut "1"), Some(1.0));
            assert_eq!(lex_number(&mut "-1"), Some(-1.0));
            assert_eq!(lex_number(&mut "1.0"), Some(1.0));
            assert_eq!(lex_number(&mut "1.1"), Some(1.1));
            assert_eq!(lex_number(&mut "0.1"), Some(0.1));
            assert_eq!(lex_number(&mut "0.0"), Some(0.0));
            assert_eq!(lex_number(&mut "+"), None);
            assert_eq!(lex_number(&mut "1.0001"), Some(1.0001));
            assert_eq!(lex_number(&mut "-1.03456"), Some(-1.03456));
        }

        #[test]
        fn basic_test_lex() {
            let expression = "1+2";
            let result = lex_expression(expression, &EmptyFunctionStore);

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
            let result = lex_expression(expression, &EmptyFunctionStore);

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
        fn test_function_lex() {
            let mut store = VectorFunctionStore::new();
            store.register::<Ident>();
            let mut expression = "ident(a)";
            let res = lex_function(&mut expression, &store);
            assert!(res.is_some(), "{res:?}");
            let res = res.unwrap();
            assert!(res.is_ok(), "{res:?}");
            assert_eq!(expression, "", "{expression}");
            // let res = res.unwrap();
        }

        #[test]
        fn test_collect_arguments() {
            let mut expression = "1, 2, (3, 4))";
            let arguments = collect_arguments(&mut expression);
            assert_eq!(arguments.len(), 3, "{arguments:?}");
            assert_eq!(arguments.as_ref(), ["1", " 2", " (3, 4)"]);
            assert_eq!(expression, "", "{expression}");
        }
    }
}

mod parser {
    use crate::__lib::collections::VecDeque;
    use crate::__lib::convert::TryInto;
    use crate::__lib::string::ToString;
    use crate::__lib::vec::Vec;
    use crate::formulas::root_formula::formula_argument::FormulaArgument;
    use crate::formulas::root_formula::RootFormula;
    use crate::formulas::{
        ArgumentsError, EvaluationError, FunctionLike, MathError, ParenthesisError, ParserError,
    };
    use crate::tokens::{BaseToken, Bracket, OpenBracket, Operator, Side};
    use crate::variable_stores::EmptyVariableStore;

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
                    process_operator(operator, &mut tokens, &mut stack);
                }
                BaseToken::Bracket(bra) => process_bracket(bra, &mut tokens, &mut stack)?,
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
            rpn.push_back(match formula.eval(&EmptyVariableStore) {
                Ok(val) => val.into(),
                Err(EvaluationError::MathError(e)) => return Err(e),
                Err(EvaluationError::NoVariableError(_)) => unreachable!(),
            });
            return Ok(());
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
                        None => {
                            return Err(ParserError::ArgumentsError(ArgumentsError(
                                Into::<&str>::into(operator).into(),
                            )))
                        }
                        Some(val) => RootFormula::new::<FormulaArgument>(val.try_into().unwrap()),
                    };
                    let first = match rpn.pop_back() {
                        None => {
                            return Err(ParserError::ArgumentsError(ArgumentsError(
                                Into::<&str>::into(operator).into(),
                            )))
                        }
                        Some(val) => RootFormula::new::<FormulaArgument>(val.try_into().unwrap()),
                    };
                    let operator_formula = operator.into_formula(first, second);
                    push_formula(&mut rpn, operator_formula)?;
                }
                BaseToken::Formula(formula) => {
                    push_formula(&mut rpn, formula)?;
                }
                BaseToken::Bracket(_) => unreachable!(),
            }
        }
        if rpn.len() == 1 {
            return Ok(rpn.pop_front().unwrap().try_into().unwrap());
        }
        Err(ParserError::ArgumentsError(ArgumentsError(
            "no operator".to_string(),
        )))
    }

    pub(super) fn parse_tokens(
        tokens: VecDeque<BaseToken>,
    ) -> Result<FormulaArgument, ParserError> {
        compress_rpn(build_rpn(tokens)?)
    }

    #[cfg(test)]
    mod rpn_test {
        use crate::__lib::collections::VecDeque;
        use crate::formulas::root_formula::parser::build_rpn;
        use crate::tokens::{BaseToken, Bracket, CloseBracket, NumberLike, OpenBracket, Operator};

        #[test]
        fn easy_rpn_test() {
            let mut initial: VecDeque<BaseToken> = VecDeque::with_capacity(3);
            initial.push_back(1.0.into());
            initial.push_back(Operator::Plus.into());
            initial.push_back(2.0.into());
            let result = build_rpn(initial);
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
            let result = build_rpn(initial);
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
            let result = build_rpn(initial);
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

        #[cfg(any(feature = "std", feature = "libm"))]
        #[test]
        fn power_test() {
            let mut initial: VecDeque<BaseToken> = VecDeque::with_capacity(7);
            initial.push_back(5.0.into());
            initial.push_back(Operator::Exponent.into());
            initial.push_back(6.0.into());
            initial.push_back(Operator::Exponent.into());
            initial.push_back(7.0.into());
            let result = build_rpn(initial);
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
            let result = build_rpn(initial);
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
            let result = build_rpn(initial);
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
        use crate::__lib::collections::VecDeque;
        use crate::formulas::root_formula::parser::compress_rpn;
        use crate::formulas::root_formula::FormulaArgument;
        use crate::tokens::{BaseToken, Operator};

        #[test]
        fn easy_test() {
            let mut initial: VecDeque<BaseToken> = VecDeque::with_capacity(3);
            initial.push_back(1.0.into());
            initial.push_back(2.0.into());
            initial.push_back(Operator::Plus.into());
            let result = compress_rpn(initial);
            assert!(result.is_ok(), "{result:?}");
            let result = result.unwrap();
            assert!(
                matches!(result, FormulaArgument::Number(val) if (val - 3.0).abs() < f64::EPSILON),
                "{result:?}"
            );
        }
    }
}

impl RootFormula {
    /// Creates new `RootFormula`.
    pub fn new<T: Into<FormulaArgument>>(value: T) -> Self {
        Self { tree: value.into() }
    }

    /// Parses [`&str`] into `RootFormula`.
    ///
    /// # Errors
    ///
    /// will return Err if non valid expression is passed.
    pub fn parse<T: for<'a> GetFunction<'a>>(
        expression: &str,
        formulas: &T,
    ) -> Result<Self, ParserError> {
        let parsed = lex_expression(expression, formulas)?;
        Ok(Self {
            tree: parse_tokens(parsed)?,
        })
    }
}

impl<T: Into<FormulaArgument>> From<T> for RootFormula {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl_operation_for_formula!(Add, add, Operator::Plus);
impl_operation_for_formula!(Sub, sub, Operator::Minus);
impl_operation_for_formula!(Mul, mul, Operator::Multiply);
impl_operation_for_formula!(Div, div, Operator::Divide);

#[cfg(test)]
mod test_root_formula {
    use crate::__lib::boxed::Box;
    use crate::formulas::operator::OperatorFormula;
    use crate::formulas::root_formula::formula_argument::FormulaArgument;
    use crate::formulas::{FunctionLike, RootFormula};
    use crate::tokens::Operator;
    use crate::variable_stores::{SetVariable, Variable, VectorVariableStore};

    #[test]
    fn test_collapse() {
        let mut formula = RootFormula::new(Box::new(OperatorFormula::new(
            RootFormula::new(1.0),
            RootFormula::new(2.0),
            Operator::Plus,
        )) as Box<dyn FunctionLike>);
        assert!(formula.collapse_inner().is_ok());
        assert!(
            matches!(formula.tree, FormulaArgument::Number(num) if f64::abs(num - 3.0) <= f64::EPSILON)
        );
    }

    #[test]
    fn test_collapse_shared_to_owned() {
        let mut formula = RootFormula::new(Variable::new("A"));
        let mut variable_store = VectorVariableStore::new();
        variable_store.set(
            "A",
            RootFormula::new(Box::new(OperatorFormula::new(
                RootFormula::new(Variable::new("B")),
                RootFormula::new(Variable::new("C")),
                Operator::Plus,
            )) as Box<dyn FunctionLike>),
        );
        formula.set_all_variables_shared(&variable_store);
        drop(variable_store);
        assert!(formula.collapse_inner().is_ok());
        assert!(matches!(formula.tree, FormulaArgument::OwnedFunction(_)));
    }
}
