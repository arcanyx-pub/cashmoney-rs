use crate::Error;

/// Supported currencies, identified by their ISO 4217 code.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Currency {
    // Only valid when `amount` is 0. Used when constructing the default value for Money. Can be
    // added to or subtracted from any other currency, and can be divided or multiplied (which will
    // of course result in a zero value).
    Zero,
    // United States Dollar
    USD,
    // Canadian Dollar
    CAD,
}

impl Currency {
    pub fn max_precision(&self) -> u32 {
        match self {
            Currency::Zero => 0,
            Currency::USD => 2,
            Currency::CAD => 2,
        }
    }
}

/// Returns the result of operating on two currencies. Generally, they should be the same, or else
/// a MismatchedCurrency error is returned. The `Zero` Currency is an exception; it takes on the
/// currency of the other operand.
pub fn combine_currency(lhs: Currency, rhs: Currency) -> Result<Currency, Error> {
    let currency = if lhs == rhs {
        lhs
    } else if lhs == Currency::Zero {
        rhs
    } else if rhs == Currency::Zero {
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

    #[test]
    fn combine_currency__zero_and_zero__returns_zero() -> Result<()> {
        let combined = expect_ok!(combine_currency(Currency::Zero, Currency::Zero));
        expect_eq!(combined, Currency::Zero);
        Ok(())
    }

    #[test]
    fn combine_currency__zero_and_usd__returns_usd() -> Result<()> {
        let combined = expect_ok!(combine_currency(Currency::Zero, Currency::USD));
        expect_eq!(combined, Currency::USD);
        Ok(())
    }

    #[test]
    fn combine_currency__usd_and_zero__returns_usd() -> Result<()> {
        let combined = expect_ok!(combine_currency(Currency::USD, Currency::Zero));
        expect_eq!(combined, Currency::USD);
        Ok(())
    }
}
