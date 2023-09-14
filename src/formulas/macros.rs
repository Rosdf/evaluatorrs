#[cfg(doc)]
use crate::formulas::{Evaluate, Function, FunctionLike, IsConst};

/// Macros for creating function with only one argument. And implementing [`IsConst`], [`Evaluate`], [`FunctionLike`], [`Function`].
///
/// ## Examples
/// Duplication function
/// ```rust
/// use evaluatorrs::formulas::macros::impl_one_arg_function;
///
/// impl_one_arg_function!(
///     "double", (|x: f64| x * 2.0),
///     /// Function duplicates it's argument.
///     pub Cos
/// );
/// ```
///
/// If function has different implementations for std and libm features, two variants of function can be passed
/// ```rust
/// use evaluatorrs::formulas::macros::impl_one_arg_function;
///
/// impl_one_arg_function!(
///     "sin", (f64::sin), (libm::Libm::<f64>::sin),
///     /// Sin function.
///     pub Sin
/// );
/// ```
#[macro_export(local_inner_macros)]
macro_rules! impl_one_arg_function {
    (
        $parser_name:expr, $function_std:tt, $function_libm:tt,
        $(#[$meta: meta])*
        $vis:vis $StructName:ident
    ) => {
        #[cfg(feature = "std")]
        impl_one_arg_function!(
            $parser_name,
            $function_std,
            $(#[$meta])*
            $vis $StructName
        );
        #[cfg(all(not(feature = "std"), feature = "libm"))]
        impl_one_arg_function!(
            $parser_name,
            $function_libm,
            $(#[$meta])*
            $vis $StructName
        );
    };
    (
        $parser_name:expr, $function:tt,
        $(#[$meta: meta])*
        $vis:vis $StructName:ident
    ) => {
        $(#[$meta])*
        #[derive(Debug)]
        $vis struct $StructName {
            argument: $crate::formulas::RootFormula,
        }

        impl $crate::formulas::IsConst for $StructName {
            #[inline]
            fn is_const(&self) -> bool {
                self.argument.is_const()
            }
        }

        impl $crate::formulas::Evaluate for $StructName {
            fn eval(&self, args: &dyn $crate::variable_stores::GetVariable) -> Result<f64, $crate::formulas::EvaluationError> {
                Ok($function(self.argument.eval(args)?))
            }
        }

        impl $crate::formulas::FunctionLike for $StructName {
            #[inline]
            fn collapse_inner(&mut self) -> Result<(), $crate::formulas::MathError> {
                self.argument.collapse_inner()
            }

            #[allow(clippy::semicolon_if_nothing_returned)]
            #[inline]
            fn set_all_variables_shared(&mut self, args: &dyn $crate::variable_stores::GetVariable) {
                self.argument.set_all_variables_shared(args)
            }

            #[allow(clippy::semicolon_if_nothing_returned)]
            #[inline]
            fn set_all_variables_owned(&mut self, args: &dyn $crate::variable_stores::GetVariable) {
                self.argument.set_all_variables_owned(args)
            }

            #[allow(clippy::semicolon_if_nothing_returned)]
            #[inline]
            fn set_variable_shared(&mut self, name: &$crate::variable_stores::Variable, function: &$crate::__lib::sync::Arc<$crate::formulas::RootFormula>) {
                self.argument.set_variable_shared(name, function)
            }

            #[allow(clippy::semicolon_if_nothing_returned)]
            #[inline]
            fn set_variable_owned(&mut self, name: &$crate::variable_stores::Variable, function: &$crate::formulas::RootFormula) {
                self.argument.set_variable_owned(name, function)
            }

            fn clone_into_box(&self) -> $crate::__lib::boxed::Box<dyn $crate::formulas::FunctionLike> {
                $crate::__lib::boxed::Box::new(Self {
                    argument: self.argument.clone(),
                })
            }
        }

        impl $crate::formulas::Function for $StructName {
            const MIN_NUMBER_OF_ARGUMENTS: usize = 1;
            const MAX_NUMBER_OF_ARGUMENTS: usize = 1;
            const NAME: &'static str = $parser_name;

            fn parse<T: for<'a> $crate::function_stores::GetFunction<'a>>(
                arguments: &[&str],
                formulas: &T,
            ) -> Result<Self, $crate::formulas::ParserError>
            where
                Self: Sized,
            {
                Ok(Self {
                    argument: $crate::formulas::RootFormula::parse(arguments[0], formulas)?,
                })
            }
        }
    };
}

pub use impl_one_arg_function;

macro_rules! impl_many_one_arg_functions {
    ($func_name:tt, $struct_name:tt) => {
        impl_one_arg_function!(
            stringify!($func_name), (f64::$func_name), (libm::Libm::<f64>::$func_name),
            #[doc = concat!(stringify!($struct_name), " function.")]
            pub $struct_name
        );
    };
    ($($func_name:tt, $struct_name:tt);+ $(;)?) => {
        $(impl_many_one_arg_functions!($func_name, $struct_name);)*
    };
}

macro_rules! impl_operation_for_formula {
    ($operation:ident, $method:ident, $operator:expr) => {
        impl<T: Into<Self>> $operation<T> for $crate::formulas::RootFormula {
            type Output = Self;

            fn $method(self, rhs: T) -> Self::Output {
                let formula =
                    $crate::formulas::operator::OperatorFormula::new(self, rhs.into(), $operator);
                formula.eval(&EmptyVariableStore).map_or_else(
                    |_| Self::new(Box::new(formula) as Box<dyn FunctionLike>),
                    Self::new,
                )
            }
        }
    };
}
