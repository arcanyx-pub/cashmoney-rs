use std::str::FromStr;

/// Supported currencies, identified by their ISO 4217 code.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Currency {
    // United States Dollar
    USD,
    // Canadian Dollar
    CAD,
}

pub struct UnknownCurrencyError {}

impl Currency {
    pub fn max_precision(&self) -> u32 {
        match self {
            Currency::USD => 2,
            Currency::CAD => 2,
        }
    }
}

impl FromStr for Currency {
    type Err = UnknownCurrencyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_ascii_uppercase();
        match s.as_ref() {
            "USD" => Ok(Currency::USD),
            "CAD" => Ok(Currency::CAD),
            _ => Err(UnknownCurrencyError {}),
        }
    }
}
