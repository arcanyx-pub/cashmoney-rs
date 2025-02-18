use crate::currency;
use crate::currency::Currency;
use crate::error::Error;
use crate::money::Money;
use rust_decimal::{Decimal, RoundingStrategy};
use std::cmp::{max, Ordering};
use std::iter;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// A monetary value in a certain currency with a possibly invalid denomination, e.g., 13.37 USD or
/// 1.337 USD.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FractionalMoney {
    /// The (possibly) fractional amount, which may or may not be a valid denomination of the
    /// currency.
    amount: Decimal,
    currency: Currency,
}

impl FractionalMoney {
    /// Creates a new fractional amount of the given currency. The only restriction is that `amount`
    /// must be zero if currency is `ZeroNone`.
    pub fn new(amount: Decimal, currency: Currency) -> Result<Self, Error> {
        if let Currency::ZeroNone = currency {
            return if amount.is_zero() {
                Ok(Self::default())
            } else {
                Err(Error::ZeroCurrencyWithNonZeroAmount)
            };
        }
        Ok(Self { amount, currency })
    }

    pub fn amount(&self) -> Decimal {
        self.amount
    }

    pub fn currency(&self) -> Currency {
        self.currency
    }

    /// Attempts to add another monetary value to this one. Returns an error if the currencies do
    /// not match.
    pub fn try_add(&self, rhs: &Self) -> Result<Self, Error> {
        let currency = currency::combine_currency(self.currency, rhs.currency)?;
        let maybe_amount = self.amount.checked_add(rhs.amount);
        let Some(mut amount) = maybe_amount else {
            return Err(Error::Overflow);
        };
        // Decimal has strange scale rules. For example:
        //   `(dec!(0) + dec!(0.00)).to_string() == "0.00"
        //   `(dec!(0.00) + dec!(0)).to_string() == "0"
        //   `(dec!(1.50) + dec!(0)).to_string() == "1.50"
        // This becomes an issue when adding or subtracting the `ZeroNone` currency, since it has
        // zero decimal places, and when we are only using FractionalMoney as the inner value for
        // Money, which we assume is scaled to the max for the given currency. Thus, we explicitly
        // retain the max scale of the operands.
        amount.rescale(max(self.amount.scale(), rhs.amount.scale()));

        Ok(Self { currency, amount })
    }

    /// Attempts to subtract another monetary value from this one. Returns an error if the
    /// currencies do not match.
    pub fn try_subtract(&self, rhs: &Self) -> Result<Self, Error> {
        let currency = currency::combine_currency(self.currency, rhs.currency)?;
        let maybe_amount = self.amount.checked_sub(rhs.amount);
        let Some(mut amount) = maybe_amount else {
            return Err(Error::Overflow);
        };
        // See implementation comments for `try_add`.
        amount.rescale(max(self.amount.scale(), rhs.amount.scale()));

        Ok(Self { currency, amount })
    }

    /// Round FractionalMoney to the maximum precision allowed by the currency and return a Money
    /// object. The rounding method is "Banker's rounding" a.k.a. "midpoint nearest even".
    pub fn round(&self) -> Money {
        let precision = self.currency.max_precision();
        let mut rounded = self
            .amount
            .round_dp_with_strategy(precision, RoundingStrategy::MidpointNearestEven);
        rounded.rescale(precision);

        Money::new_unchecked(Self {
            amount: rounded,
            currency: self.currency,
        })
    }

    /// Similar to `round()` except that the rounding method is "midpoint away from zero"
    pub fn round_up(&self) -> Money {
        let precision = self.currency.max_precision();
        let mut rounded = self
            .amount
            .round_dp_with_strategy(precision, RoundingStrategy::MidpointAwayFromZero);
        rounded.rescale(precision);

        Money::new_unchecked(Self {
            amount: rounded,
            currency: self.currency,
        })
    }

    /// Returns true if the `amount` is zero, regardless of currency.
    pub fn is_zero(&self) -> bool {
        self.amount.is_zero()
    }

    /// Returns true if `amount` > 0.
    pub fn is_positive(&self) -> bool {
        self.amount.is_sign_positive() && !self.amount.is_zero()
    }

    /// Returns true if `amount` < 0.
    pub fn is_negative(&self) -> bool {
        self.amount.is_sign_negative() && !self.amount.is_zero()
    }

    /// Creates a zero-valued `FractionalMoney` with `ZeroNone` currency.
    pub fn zero() -> Self {
        Self::default()
    }
}

/// Implementing `Default` is useful for summing iterators and other cases where a default
/// zero-value is required.
impl Default for FractionalMoney {
    fn default() -> Self {
        Self {
            amount: Decimal::default(),
            currency: Currency::ZeroNone,
        }
    }
}

impl Add for FractionalMoney {
    type Output = FractionalMoney;

    fn add(self, rhs: Self) -> Self::Output {
        self.try_add(&rhs).unwrap()
    }
}

impl AddAssign for FractionalMoney {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.add(rhs);
    }
}

impl Sub for FractionalMoney {
    type Output = FractionalMoney;

    fn sub(self, rhs: Self) -> Self::Output {
        self.try_subtract(&rhs).unwrap()
    }
}

impl SubAssign for FractionalMoney {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.sub(rhs);
    }
}

impl Mul<Decimal> for FractionalMoney {
    type Output = FractionalMoney;

    fn mul(self, scalar: Decimal) -> Self::Output {
        Self {
            amount: self.amount * scalar,
            currency: self.currency,
        }
    }
}

impl MulAssign<Decimal> for FractionalMoney {
    fn mul_assign(&mut self, rhs: Decimal) {
        *self = self.mul(rhs);
    }
}

impl Div<Decimal> for FractionalMoney {
    type Output = FractionalMoney;

    fn div(self, scalar: Decimal) -> Self::Output {
        Self {
            amount: self.amount / scalar,
            currency: self.currency,
        }
    }
}

impl DivAssign<Decimal> for FractionalMoney {
    fn div_assign(&mut self, rhs: Decimal) {
        *self = self.div(rhs);
    }
}

impl Neg for FractionalMoney {
    type Output = FractionalMoney;

    fn neg(self) -> Self::Output {
        Self {
            amount: self.amount.neg(),
            currency: self.currency,
        }
    }
}

/// If the iterator is empty, then the special `ZeroNone` currency will be the result.
impl iter::Sum for FractionalMoney {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Default::default(), Add::add)
    }
}

impl Ord for FractionalMoney {
    fn cmp(&self, other: &Self) -> Ordering {
        if currency::combine_currency(self.currency, other.currency).is_err() {
            panic!("tried to compare different types of currency")
        }
        self.amount.cmp(&other.amount)
    }
}

impl PartialOrd for FractionalMoney {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;
    use crate::usd;
    use anyhow::Result;
    use expecting::*;
    use rust_decimal_macros::dec;

    fn usd(d: &str) -> FractionalMoney {
        FractionalMoney::new(Decimal::from_str_exact(d).unwrap(), Currency::USD).unwrap()
    }
    fn cad(d: &str) -> FractionalMoney {
        FractionalMoney::new(Decimal::from_str_exact(d).unwrap(), Currency::CAD).unwrap()
    }
    fn zero() -> FractionalMoney {
        FractionalMoney::default()
    }

    #[test]
    fn round() -> Result<()> {
        expect_eq!(usd("1").round(), usd!(1.00));
        expect_eq!(usd("11").round(), usd!(11.00));
        expect_eq!(usd("11.1").round(), usd!(11.10));
        expect_eq!(usd("11.11").round(), usd!(11.11));
        expect_eq!(usd("11.111").round(), usd!(11.11));

        expect_eq!(usd("1.005").round(), usd!(1.00));
        expect_eq!(usd("1.0051").round(), usd!(1.01));
        expect_eq!(usd("1.0149").round(), usd!(1.01));
        expect_eq!(usd("1.015").round(), usd!(1.02));
        Ok(())
    }

    #[test]
    fn round_up() -> Result<()> {
        expect_eq!(usd("1").round_up(), usd!(1.00));
        expect_eq!(usd("11").round_up(), usd!(11.00));
        expect_eq!(usd("11.1").round_up(), usd!(11.10));
        expect_eq!(usd("11.11").round_up(), usd!(11.11));
        expect_eq!(usd("11.111").round_up(), usd!(11.11));

        expect_eq!(usd("1.0049").round_up(), usd!(1.00));
        expect_eq!(usd("1.005").round_up(), usd!(1.01));
        expect_eq!(usd("1.0149").round_up(), usd!(1.01));
        expect_eq!(usd("1.015").round_up(), usd!(1.02));
        Ok(())
    }

    #[test]
    fn add__matching_currency() -> Result<()> {
        expect_eq!(usd("1") + usd("2.99"), usd("3.99"));
        expect_eq!(usd("1") + usd("-2.99"), usd("-1.99"));
        expect_eq!(usd("1") + usd("2.12345"), usd("3.12345"));

        expect_eq!(cad("1") + cad("-1"), cad("0"));
        Ok(())
    }

    #[test]
    fn add__zero_currency() -> Result<()> {
        expect_eq!(usd("1") + zero(), usd("1"));
        expect_eq!(zero() + usd("1"), usd("1"));
        Ok(())
    }

    #[test]
    #[should_panic]
    fn add__mismatched_currencies__panics() {
        let _ = usd("1") + cad("2.99");
    }

    #[test]
    fn try_add__mismatched_currency__returns_err() -> Result<()> {
        expect_err!(usd("1").try_add(&cad("1")));
        Ok(())
    }

    #[test]
    fn subtract__matching_currency() -> Result<()> {
        expect_eq!(usd("1") - usd("2.99"), usd("-1.99"));
        expect_eq!(usd("1") - usd("-2.99"), usd("3.99"));
        expect_eq!(usd("1") - usd("2.12345"), usd("-1.12345"));

        expect_eq!(cad("1") - cad("-1"), cad("2"));
        Ok(())
    }

    #[test]
    fn subtract__zero_currency() -> Result<()> {
        expect_eq!(usd("1") - zero(), usd("1"));
        expect_eq!(zero() - usd("1"), usd("-1"));
        Ok(())
    }

    #[test]
    #[should_panic]
    fn subtract__mismatched_currencies__panics() {
        let _ = usd("1") - cad("0.50");
    }

    #[test]
    fn try_subtract__mismatched_currency__returns_err() -> Result<()> {
        expect_err!(usd("1").try_subtract(&cad("1")));
        Ok(())
    }

    #[test]
    fn multiply_by_0() -> Result<()> {
        expect_eq!(usd("1") * dec!(0), usd("0"));
        expect_eq!(usd("1") * dec!(0.0), usd("0"));
        expect_eq!(usd("1.000") * dec!(0.0), usd("0"));
        Ok(())
    }

    #[test]
    fn multiply_by_1() -> Result<()> {
        expect_eq!(usd("1") * dec!(1), usd("1"));
        expect_eq!(usd("1") * dec!(1.0), usd("1"));
        expect_eq!(usd("1.000") * dec!(1.0), usd("1"));
        Ok(())
    }

    #[test]
    fn multiply_by_positive() -> Result<()> {
        expect_eq!(usd("1") * dec!(2), usd("2"));
        expect_eq!(usd("1") * dec!(1.50), usd("1.5"));
        expect_eq!(usd("1") * dec!(0.50), usd("0.5"));
        expect_eq!(usd("2.25") * dec!(1.25), usd("2.8125"));
        Ok(())
    }

    #[test]
    fn multiply_by_negative() -> Result<()> {
        expect_eq!(usd("1") * dec!(-2), usd("-2"));
        Ok(())
    }

    #[test]
    #[should_panic]
    fn divide_by_0() {
        let _ = usd("1") / dec!(0);
    }

    #[test]
    fn divide_by_1() -> Result<()> {
        expect_eq!(usd("1") / dec!(1), usd("1"));
        expect_eq!(usd("1") / dec!(1.0), usd("1"));
        expect_eq!(usd("1.000") / dec!(1.0), usd("1"));
        Ok(())
    }

    #[test]
    fn divide_by_positive() -> Result<()> {
        expect_eq!(usd("1") / dec!(2), usd("0.5"));
        expect_eq!(usd("1") / dec!(0.5), usd("2"));
        expect_eq!(usd("2.25") / dec!(2), usd("1.125"));
        Ok(())
    }

    #[test]
    fn divide_by_negative() -> Result<()> {
        expect_eq!(usd("1") / dec!(-2), usd("-0.5"));
        Ok(())
    }

    #[test]
    fn divide_resulting_in_non_terminating() -> Result<()> {
        expect_eq!(usd("1") / dec!(3), usd("0.3333333333333333333333333333"));
        Ok(())
    }

    #[test]
    fn compare() -> Result<()> {
        expect!(usd("1") < usd("2"));
        expect!(usd("2") > usd("1"));
        expect!(usd("2") >= usd("1"));
        expect_eq!(usd("2").min(usd("1")), usd("1"));
        Ok(())
    }

    #[test]
    #[should_panic]
    fn compare_different_currencies() {
        let _ = usd("1") < cad("2");
    }
}
