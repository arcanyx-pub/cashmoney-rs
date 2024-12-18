use crate::Error;

/// Supported currencies, identified by their ISO 4217 code.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Currency {
    // United States Dollar
    USD,
    // Canadian Dollar
    CAD,
}

impl Currency {
    pub fn max_precision(&self) -> u32 {
        match self {
            Currency::USD => 2,
            Currency::CAD => 2,
        }
    }
}

/// Returns the result of operating on two currencies. They should be the same, or else
/// a MismatchedCurrency error is returned.
pub fn combine_currency(lhs: Currency, rhs: Currency) -> Result<Currency, Error> {
    let currency = if lhs == rhs {
        lhs
    } else {
        return Err(Error::MismatchedCurrency);
    };

    Ok(currency)
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;
    use anyhow::Result;
    use expecting::*;

    #[test]
    fn combine_currency__same__returns_same() -> Result<()> {
        let combined = expect_ok!(combine_currency(Currency::USD, Currency::USD));
        expect_eq!(combined, Currency::USD);
        Ok(())
    }

    #[test]
    fn combine_currency__difference__returns_error() -> Result<()> {
        let e = expect_err!(combine_currency(Currency::USD, Currency::CAD));
        expect_eq!(e, Error::MismatchedCurrency);
        Ok(())
    }
}
