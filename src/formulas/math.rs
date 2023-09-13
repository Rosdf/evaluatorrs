impl_many_one_arg_functions!(
    acos, Acos;
    acosh, Acosh;
    asin, Asin;
    asinh, Asinh;
    atan, Atan;
    atanh, Atanh;
    cbrt, Cbrt;
    ceil, Ceil;
    cos, Cos;
    cosh, Cosh;
    exp, Exp;
    floor, Floor;
    sin, Sin;
    sinh, Sinh;
    sqrt, Sqrt;
    tan, Tan;
    tanh, Tanh;
);

impl_one_arg_function!(
    "log", (f64::ln), (libm::Libm::<f64>::log),
    /// Natural logarithm function.
    pub Log
);

#[cfg(feature = "libm")]
impl_one_arg_function!(
    "erf", (libm::Libm::<f64>::erf),
    /// Error function.
    pub Erf
);
