use crate::__lib::marker::PhantomData;
use crate::__lib::str::FromStr;
use crate::__lib::string::ToString;
use crate::formulas::{ParserError, RootFormula, WrongTypeError};
use crate::function_stores::GetFunction;

/// Interface for parsing [`&str`] into "T".
#[derive(Debug)]
pub struct Parser<T> {
    _inner: PhantomData<T>,
}

impl Parser<RootFormula> {
    /// Parses [`&str`] into  [`RootFormula`].
    ///
    /// ## Errors
    ///
    /// Will return Err if non valid expression is passed.
    #[inline]
    pub fn parse<T: for<'a> GetFunction<'a>>(
        value: &str,
        formulas: &T,
    ) -> Result<RootFormula, ParserError> {
        RootFormula::parse(value, formulas)
    }
}

macro_rules! impl_parser {
    ($parse_type:ty, $error_msg:expr) => {
        impl Parser<$parse_type> {
            #[doc = concat!("Parses [`&str`] into [`", stringify!($parse_type), "`].\n")]
            #[doc = "## Errors\n"]
            #[doc = concat!("Will return Err, if provided not with ", $error_msg, ".")]
            #[inline]
            pub fn parse(value: &str) -> Result<$parse_type, WrongTypeError> {
                <$parse_type>::from_str(value.trim())
                    .map_err(|_| WrongTypeError::new(value.to_string(), $error_msg))
            }
        }
    };

    ($parser_type:ty) => {
        impl_parser!($parser_type, stringify!($parser_type));
    };
}

impl_parser!(bool, "\"true\" or \"false\"");
impl_parser!(i8);
impl_parser!(i16);
impl_parser!(i32);
impl_parser!(i64);
impl_parser!(i128);
impl_parser!(u8);
impl_parser!(u16);
impl_parser!(u32);
impl_parser!(u64);
impl_parser!(u128);
impl_parser!(isize);
impl_parser!(usize);
impl_parser!(f32);
impl_parser!(f64);
