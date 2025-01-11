use std::fmt;
use std::fmt::Formatter;

/// Error type for the library.
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    /// The amount provided is not valid for the given currency.
    InvalidMoneyValue(String),
    /// Attempted to create (Fractional)Money with `Zero` currency but non-zero amount.
    ZeroCurrencyWithNonZeroAmount,
    /// A mathematical operation was attempted on monetary values of different currencies.
    MismatchedCurrency,
    /// There was an overflow error in the underlying Decimal library.
    Overflow,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidMoneyValue(details) => {
                write!(f, "Invalid money value for the given currency: {details}")
            }
            Self::ZeroCurrencyWithNonZeroAmount => {
                write!(f, "Attempted to use non-zero amount for Zero currency.")
            }
            Self::MismatchedCurrency => {
                write!(
                    f,
                    "A mathematical operation was attempted on values of different currencies"
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
