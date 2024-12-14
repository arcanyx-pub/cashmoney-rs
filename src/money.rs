use crate::currency::Currency;
use crate::error::Error;
use crate::fractional_money::FractionalMoney;
use rust_decimal::Decimal;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::iter;
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};

/// A monetary value in a certain currency with a valid denomination, e.g., 13.37 USD but not
/// 1.337 USD.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Money {
    /// The validated and normalized FractionalMoney based on `currency`.
    money: FractionalMoney,
}

impl Money {
    /// Creates a new, validated, normalized monetary value. The given decimal must be a valid
    /// representation for the given currency, or else an InvalidMoneyValue error will be returned.
    ///
    /// `currency` cannot be `Currency::Zero`; you should instead specify a real currency.
    pub fn new(amount: Decimal, currency: Currency) -> Result<Self, Error> {
        let normed_amt = validate_and_normalize(amount, currency)?;

        Ok(Money {
            money: FractionalMoney::new(normed_amt, currency)?,
        })
    }

    pub(crate) fn new_unchecked(money: FractionalMoney) -> Self {
        Money { money }
    }

    /// Returns the decimal amount. This value is guaranteed to be valid and normalized based on its
    /// currency, and calling `to_string()` will produce a valid string representation.
    /// Normalization includes using the smallest conventional denomination (i.e., the maximum
    /// number of decimal places for the currency), so US$1 will become `dec!(1.00)` here.
    pub fn amount(&self) -> Decimal {
        self.money.amount()
    }

    pub fn currency(&self) -> Currency {
        self.money.currency()
    }

    /// Attempts to add another monetary value to this one. Returns an error if the currencies do
    /// not match.
    pub fn try_add(&self, rhs: &Self) -> Result<Self, Error> {
        Ok(Self {
            money: self.money.try_add(&rhs.money)?,
        })
    }

    /// Attempts to subtract another monetary value from this one. Returns an error if the
    /// currencies do not match.
    pub fn try_subtract(&self, rhs: &Self) -> Result<Self, Error> {
        Ok(Self {
            money: self.money.try_subtract(&rhs.money)?,
        })
    }
}

impl From<Money> for FractionalMoney {
    fn from(money: Money) -> Self {
        money.money
    }
}

impl Display for Money {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {:?}", self.money.amount(), self.money.currency())
    }
}

impl Add for Money {
    type Output = Money;

    fn add(self, rhs: Self) -> Self::Output {
        self.try_add(&rhs).unwrap()
    }
}

impl AddAssign for Money {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.add(rhs);
    }
}

impl Sub for Money {
    type Output = Money;

    fn sub(self, rhs: Self) -> Self::Output {
        self.try_subtract(&rhs).unwrap()
    }
}

impl SubAssign for Money {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.sub(rhs);
    }
}

impl Mul<Decimal> for Money {
    type Output = FractionalMoney;

    fn mul(self, rhs: Decimal) -> Self::Output {
        self.money * rhs
    }
}

impl Div<Decimal> for Money {
    type Output = FractionalMoney;

    fn div(self, rhs: Decimal) -> Self::Output {
        self.money / rhs
    }
}

impl Neg for Money {
    type Output = Money;

    fn neg(self) -> Self::Output {
        Self { money: -self.money }
    }
}

/// If the iterator is empty, then the special `Zero` currency will be the result.
impl iter::Sum for Money {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Default::default(), Add::add)
    }
}

impl Ord for Money {
    fn cmp(&self, other: &Self) -> Ordering {
        self.money.cmp(&other.money)
    }
}

impl PartialOrd for Money {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn validate_and_normalize(amt: Decimal, currency: Currency) -> Result<Decimal, Error> {
    match currency {
        Currency::Zero => Err(Error::ZeroCurrencyUsedUnnecessarily),
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
    fn add_assign() -> Result<()> {
        let mut a = usd!(1);
        a += usd!(68);
        expect_eq!(a, usd!(69));
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
    fn sub_assign() -> Result<()> {
        let mut a = usd!(1);
        a -= usd!(68);
        expect_eq!(a, usd!(-67));
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

    #[test]
    fn neg() -> Result<()> {
        let a = usd!(1);
        expect_eq!(-a, usd!(-1));
        Ok(())
    }

    #[test]
    fn compare() -> Result<()> {
        expect!(usd!(1) < usd!(2));
        expect!(usd!(2) > usd!(1));
        expect!(usd!(2) >= usd!(1));
        expect_eq!(usd!(2).min(usd!(1)), usd!(1));
        Ok(())
    }

    #[test]
    #[should_panic]
    fn compare_different_currencies() {
        let _ = usd!(1) < cad!(2);
    }
}
