use std::fmt;
use std::fmt::Formatter;

/// Error type for the library.
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    /// The amount provided is not valid for the given currency.
    InvalidMoneyValue(String),
    /// A mathematical operation was attempted on monetary values of different currencies.
    MismatchedCurrency,
    /// `Money::new` was called with `Currency::Zero`. You should specify a real currency instead.
    /// If you really want to create a zero-valued Money with `Zero` currency, use
    /// `Money::default()` instead.
    ZeroCurrencyUsedUnnecessarily,
    /// There was an overflow error in the underlying Decimal library.
    Overflow,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidMoneyValue(details) => {
                write!(f, "Invalid money value for the given currency: {details}")
            }
            Self::MismatchedCurrency => {
                write!(
                    f,
                    "A mathematical operation was attempted on values of different currencies"
                )
            }
            Self::ZeroCurrencyUsedUnnecessarily => {
                write!(
                    f,
                    "`Money::new` cannot be called with `Currency::Zero`. Use `Money::default()` instead."
                )
            }
            Self::Overflow => {
                write!(
                    f,
                    "There was an overflow error in the underlying Decimal library."
                )
            }
        }
    }
}

impl std::error::Error for Error {}
