/// Creates Money of the given amount with USD currency. Example: `usd!(13.37)`.
#[macro_export]
macro_rules! usd {
    ( $amount:expr ) => {{
        let val = rust_decimal_macros::dec!($amount);
        $crate::Money::new(val, $crate::Currency::USD).unwrap()
    }};
}

/// Creates Money of the given amount with CAD currency. Example: `cad!(13.37)`.
#[macro_export]
macro_rules! cad {
    ( $amount:expr ) => {{
        let val = rust_decimal_macros::dec!($amount);
        $crate::Money::new(val, $crate::Currency::CAD).unwrap()
    }};
}

/// Creates 0-valued money with the special `ZeroNone` currency.
#[macro_export]
macro_rules! zero {
    () => {{
        $crate::Money::default()
    }};
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use crate::{Currency, Money};
    use anyhow::Result;
    use expecting::*;
    use rust_decimal_macros::dec;

    #[test]
    fn usd__0_decimals() -> Result<()> {
        expect_eq!(usd!(0), Money::new(dec!(0), Currency::USD).unwrap());
        expect_eq!(usd!(1), Money::new(dec!(1), Currency::USD).unwrap());
        expect_eq!(usd!(-1), Money::new(dec!(-1), Currency::USD).unwrap());
        Ok(())
    }

    #[test]
    fn usd__2_decimals() -> Result<()> {
        expect_eq!(usd!(0.00), Money::new(dec!(0), Currency::USD).unwrap());
        expect_eq!(usd!(1.00), Money::new(dec!(1), Currency::USD).unwrap());
        expect_eq!(usd!(1.01), Money::new(dec!(1.01), Currency::USD).unwrap());
        expect_eq!(usd!(-1.01), Money::new(dec!(-1.01), Currency::USD).unwrap());
        Ok(())
    }

    #[test]
    #[should_panic]
    fn usd__3_decimals__panics() {
        usd!(0.123);
    }

    #[test]
    fn cad__0_decimals() -> Result<()> {
        expect_eq!(cad!(0), Money::new(dec!(0), Currency::CAD).unwrap());
        expect_eq!(cad!(1), Money::new(dec!(1), Currency::CAD).unwrap());
        expect_eq!(cad!(-1), Money::new(dec!(-1), Currency::CAD).unwrap());
        Ok(())
    }

    #[test]
    fn cad__2_decimals() -> Result<()> {
        expect_eq!(cad!(0.00), Money::new(dec!(0), Currency::CAD).unwrap());
        expect_eq!(cad!(1.00), Money::new(dec!(1), Currency::CAD).unwrap());
        expect_eq!(cad!(1.01), Money::new(dec!(1.01), Currency::CAD).unwrap());
        expect_eq!(cad!(-1.01), Money::new(dec!(-1.01), Currency::CAD).unwrap());
        Ok(())
    }

    #[test]
    #[should_panic]
    fn cad__3_decimals__panics() {
        cad!(0.123);
    }

    #[test]
    fn zero_none() -> Result<()> {
        let z = zero!();
        expect_eq!(z.amount(), dec!(0));
        expect_eq!(z.currency(), Currency::ZeroNone);
        Ok(())
    }
}
