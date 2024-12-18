use crate::{Currency, Error, Money};
use rust_decimal_macros::dec;

/// Attempts to compute the sum of `moneys`, which must all match `currency` or else a
/// `MismatchedCurrency` error is thrown. If `moneys` is empty, then a zero-valued `Money` of the
/// given `currency` is returned.
pub fn try_sum(moneys: &[Money], currency: Currency) -> Result<Money, Error> {
    let default = Money::new(dec!(0), currency)?;

    moneys
        .iter()
        .try_fold(default, |acc, x| Money::try_add(&acc, x))
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;
    use crate::{cad, usd};
    use anyhow::Result;
    use expecting::*;

    #[test]
    fn try_sum__empty() -> Result<()> {
        let moneys: Vec<Money> = vec![];

        let sum = expect_ok!(try_sum(&moneys, Currency::CAD));
        expect_eq!(sum, cad!(0));
        Ok(())
    }

    #[test]
    fn try_sum__one_item() -> Result<()> {
        let moneys: Vec<Money> = vec![cad!(9.99)];

        let sum = expect_ok!(try_sum(&moneys, Currency::CAD));
        expect_eq!(sum, cad!(9.99));
        Ok(())
    }

    #[test]
    fn try_sum__one_item__wrong_currency() -> Result<()> {
        let moneys: Vec<Money> = vec![cad!(9.99)];

        let err = expect_err!(try_sum(&moneys, Currency::USD));
        expect_eq!(err, Error::MismatchedCurrency);
        Ok(())
    }

    #[test]
    fn try_sum__two_items() -> Result<()> {
        let moneys: Vec<Money> = vec![cad!(9.99), cad!(0.01)];

        let sum = expect_ok!(try_sum(&moneys, Currency::CAD));
        expect_eq!(sum, cad!(10));
        Ok(())
    }

    #[test]
    fn try_sum__two_items__wrong_currency() -> Result<()> {
        let moneys: Vec<Money> = vec![cad!(9.99), cad!(0.01)];

        let err = expect_err!(try_sum(&moneys, Currency::USD));
        expect_eq!(err, Error::MismatchedCurrency);
        Ok(())
    }

    #[test]
    fn try_sum__two_items__different_currency() -> Result<()> {
        let moneys: Vec<Money> = vec![cad!(9.99), usd!(0.01)];

        let err = expect_err!(try_sum(&moneys, Currency::CAD));
        expect_eq!(err, Error::MismatchedCurrency);
        Ok(())
    }
}
