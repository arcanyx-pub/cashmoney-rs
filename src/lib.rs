//! Cashmoney is a library for expressing monetary values and performing safe
//! monetary calculations suitable for financial applications.

mod currency;
mod error;
mod fractional_money;
mod macros;
mod money;

pub use crate::currency::{Currency, UnknownCurrencyError};
pub use crate::error::Error;
pub use crate::fractional_money::FractionalMoney;
pub use crate::money::Money;
