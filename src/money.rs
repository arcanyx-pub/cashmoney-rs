use crate::currency::Currency;
use crate::error::Error;
use crate::fractional_money::FractionalMoney;
use rust_decimal::Decimal;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Sub};

/// A monetary value in a certain currency with a valid denomination, e.g., 13.37 USD but not
/// 1.337 USD.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Money {
    /// The validated and normalized amount based on `currency`.
    amount: Decimal,
    currency: Currency,
}

impl Money {
    /// Creates a new, validated, normalized monetary value. The given decimal must be a valid
    /// representation for the given currency, or else an InvalidMoneyValue error will be returned.
    pub fn new(value: Decimal, currency: Currency) -> Result<Self, Error> {
        let normed_amt = validate_and_normalize(value, currency)?;

        Ok(Money {
            currency,
            amount: normed_amt,
        })
    }

    /// Returns the decimal amount. This value is guaranteed to be valid and normalized based on its
    /// currency, and calling `to_string()` will produce a valid string representation.
    /// Normalization includes using the smallest conventional denomination (i.e., the maximum
    /// number of decimal places for the currency), so US$1 will become `dec!(1.00)` here.
    pub fn amount(&self) -> Decimal {
        self.amount
    }

    pub fn currency(&self) -> Currency {
        self.currency
    }

    /// Attempts to add another monetary value to this one. Returns an error if the currencies do
    /// not match.
    pub fn try_add(&self, rhs: &Self) -> Result<Self, Error> {
        if self.currency != rhs.currency {
            return Err(Error::MismatchedCurrency);
        }
        Ok(Self {
            currency: self.currency,
            amount: self.amount + rhs.amount,
        })
    }

    /// Attempts to subtract another monetary value from this one. Returns an error if the
    /// currencies do not match.
    pub fn try_subtract(&self, rhs: &Self) -> Result<Self, Error> {
        if self.currency != rhs.currency {
            return Err(Error::MismatchedCurrency);
        }
        Ok(Self {
            currency: self.currency,
            amount: self.amount - rhs.amount,
        })
    }
}

impl Display for Money {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {:?}", self.amount, self.currency)
    }
}

impl Add for Money {
    type Output = Money;

    fn add(self, rhs: Self) -> Self::Output {
        self.try_add(&rhs).unwrap()
    }
}

impl Sub for Money {
    type Output = Money;

    fn sub(self, rhs: Self) -> Self::Output {
        self.try_subtract(&rhs).unwrap()
    }
}

impl Mul<Decimal> for Money {
    type Output = FractionalMoney;

    fn mul(self, rhs: Decimal) -> Self::Output {
        let frac: FractionalMoney = self.into();
        frac * rhs
    }
}

impl Div<Decimal> for Money {
    type Output = FractionalMoney;

    fn div(self, rhs: Decimal) -> Self::Output {
        let frac: FractionalMoney = self.into();
        frac / rhs
    }
}

fn validate_and_normalize(amt: Decimal, currency: Currency) -> Result<Decimal, Error> {
    match currency {
        Currency::USD | Currency::CAD => {
            let scale = amt.scale();
            // We don't allow scale=1 since it is unconventional and likely indicates the calling
            // code has a bug.
            if scale != 0 && scale != 2 {
                return Err(Error::InvalidMoneyValue(format!(
                    "expected 0 or 2 decimal places for {currency:?}, but '{amt}' has {scale}"
                )));
            }
            // Normalize to 2 decimal places.
            let mut value = amt;
            value.rescale(2);

            Ok(value)
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;
    use crate::{cad, usd};
    use anyhow::Result;
    use expecting::*;
    use rust_decimal_macros::dec;

    #[test]
    fn new__usd__2_decimals() -> Result<()> {
        let a = expect_ok!(Money::new(dec!(0.00), Currency::USD));
        expect_eq!(a.to_string(), "0.00 USD");
        expect_eq!(a.amount().to_string(), "0.00");

        let a = expect_ok!(Money::new(dec!(1.00), Currency::USD));
        expect_eq!(a.to_string(), "1.00 USD");
        expect_eq!(a.amount().to_string(), "1.00");

        let a = expect_ok!(Money::new(dec!(-1.00), Currency::USD));
        expect_eq!(a.to_string(), "-1.00 USD");
        expect_eq!(a.amount().to_string(), "-1.00");

        let a = expect_ok!(Money::new(dec!(13.37), Currency::USD));
        expect_eq!(a.to_string(), "13.37 USD");
        expect_eq!(a.amount().to_string(), "13.37");
        Ok(())
    }

    #[test]
    fn new__usd__0_decimals() -> Result<()> {
        let a = expect_ok!(Money::new(dec!(0), Currency::USD));
        expect_eq!(a.to_string(), "0.00 USD");
        expect_eq!(a.amount().to_string(), "0.00");

        let a = expect_ok!(Money::new(dec!(1), Currency::USD));
        expect_eq!(a.to_string(), "1.00 USD");
        expect_eq!(a.amount().to_string(), "1.00");

        let a = expect_ok!(Money::new(dec!(-1), Currency::USD));
        expect_eq!(a.to_string(), "-1.00 USD");
        expect_eq!(a.amount().to_string(), "-1.00");
        Ok(())
    }

    #[test]
    fn new__usd__1_decimals__fails() -> Result<()> {
        expect_err!(Money::new(dec!(0.0), Currency::USD));
        expect_err!(Money::new(dec!(0.1), Currency::USD));
        expect_err!(Money::new(dec!(1.1), Currency::USD));
        expect_err!(Money::new(dec!(-1.1), Currency::USD));
        Ok(())
    }

    #[test]
    fn new__usd__gt_2_decimals__fails() -> Result<()> {
        expect_err!(Money::new(dec!(0.000), Currency::USD));
        expect_err!(Money::new(dec!(0.101), Currency::USD));
        expect_err!(Money::new(dec!(1.111), Currency::USD));
        expect_err!(Money::new(dec!(-1.110), Currency::USD));
        expect_err!(Money::new(dec!(-1.1102345), Currency::USD));
        Ok(())
    }

    #[test]
    fn new__cad__2_decimals() -> Result<()> {
        let a = expect_ok!(Money::new(dec!(0.00), Currency::CAD));
        expect_eq!(a.to_string(), "0.00 CAD");
        expect_eq!(a.amount().to_string(), "0.00");

        let a = expect_ok!(Money::new(dec!(1.00), Currency::CAD));
        expect_eq!(a.to_string(), "1.00 CAD");
        expect_eq!(a.amount().to_string(), "1.00");

        let a = expect_ok!(Money::new(dec!(-1.00), Currency::CAD));
        expect_eq!(a.to_string(), "-1.00 CAD");
        expect_eq!(a.amount().to_string(), "-1.00");

        let a = expect_ok!(Money::new(dec!(13.37), Currency::CAD));
        expect_eq!(a.to_string(), "13.37 CAD");
        expect_eq!(a.amount().to_string(), "13.37");
        Ok(())
    }

    #[test]
    fn new__cad__0_decimals() -> Result<()> {
        let a = expect_ok!(Money::new(dec!(0), Currency::CAD));
        expect_eq!(a.to_string(), "0.00 CAD");
        expect_eq!(a.amount().to_string(), "0.00");

        let a = expect_ok!(Money::new(dec!(1), Currency::CAD));
        expect_eq!(a.to_string(), "1.00 CAD");
        expect_eq!(a.amount().to_string(), "1.00");

        let a = expect_ok!(Money::new(dec!(-1), Currency::CAD));
        expect_eq!(a.to_string(), "-1.00 CAD");
        expect_eq!(a.amount().to_string(), "-1.00");
        Ok(())
    }

    #[test]
    fn new__cad__1_decimals__fails() -> Result<()> {
        expect_err!(Money::new(dec!(0.0), Currency::CAD));
        expect_err!(Money::new(dec!(0.1), Currency::CAD));
        expect_err!(Money::new(dec!(1.1), Currency::CAD));
        expect_err!(Money::new(dec!(-1.1), Currency::CAD));
        Ok(())
    }

    #[test]
    fn new__cad__gt_2_decimals__fails() -> Result<()> {
        expect_err!(Money::new(dec!(0.000), Currency::CAD));
        expect_err!(Money::new(dec!(0.101), Currency::CAD));
        expect_err!(Money::new(dec!(1.111), Currency::CAD));
        expect_err!(Money::new(dec!(-1.110), Currency::CAD));
        expect_err!(Money::new(dec!(-1.1102345), Currency::CAD));
        Ok(())
    }

    #[test]
    fn add__matching_currency() -> Result<()> {
        expect_eq!(usd!(1) + usd!(2.99), usd!(3.99));
        expect_eq!(usd!(1) + usd!(-2.99), usd!(-1.99));

        expect_eq!(cad!(1) + cad!(-1), cad!(0));
        Ok(())
    }

    #[test]
    #[should_panic]
    fn add__mismatched_currencies__panics() {
        let _ = usd!(1) + cad!(2.99);
    }

    #[test]
    fn try_add__mismatched_currency__returns_err() -> Result<()> {
        expect_err!(usd!(1).try_add(&cad!(1)));
        Ok(())
    }

    #[test]
    fn subtract__matching_currency() -> Result<()> {
        expect_eq!(usd!(1) - usd!(2.99), usd!(-1.99));
        expect_eq!(usd!(1) - usd!(-2.99), usd!(3.99));

        expect_eq!(cad!(1) - cad!(1), cad!(0));
        Ok(())
    }

    #[test]
    #[should_panic]
    fn subtract__mismatched_currencies__panics() {
        let _ = usd!(1) - cad!(0.50);
    }

    #[test]
    fn try_subtract__mismatched_currency__returns_err() -> Result<()> {
        expect_err!(usd!(1).try_subtract(&cad!(1)));
        Ok(())
    }

    #[test]
    fn multiply() -> Result<()> {
        let product = usd!(2.23) * dec!(2);
        expect_eq!(product.round(), usd!(4.46));
        Ok(())
    }

    #[test]
    fn divide() -> Result<()> {
        let quotient = usd!(2.23) / dec!(2);
        expect_eq!(quotient.round(), usd!(1.12));
        Ok(())
    }
}
