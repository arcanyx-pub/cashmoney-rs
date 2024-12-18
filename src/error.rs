use std::fmt;
use std::fmt::Formatter;

/// Error type for the library.
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    /// The amount provided is not valid for the given currency.
    InvalidMoneyValue(String),
    /// A mathematical operation was attempted on monetary values of different currencies.
    MismatchedCurrency,
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
        }
    }
}

impl std::error::Error for Error {}
