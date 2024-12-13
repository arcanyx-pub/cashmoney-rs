use anyhow::Result;
use cashmoney::{cad, usd, Currency, Money};
use expecting::*;
use rust_decimal_macros::dec;

#[test]
fn basic_calculations() -> Result<()> {
    let a = usd!(13.37);
    expect_eq!(a.to_string(), "13.37 USD");
    expect_eq!(a.currency(), Currency::USD);
    expect_eq!(a.amount().to_string(), "13.37");

    let double_a = a * dec!(2);
    expect_eq!(double_a.round().to_string(), "26.74 USD");

    let half_a = a / dec!(2);
    // Rounded to even from 6.685
    expect_eq!(half_a.round().to_string(), "6.68 USD");

    expect_err!(a.try_add(&cad!(0.63)));

    let b = expect_ok!(Money::new(dec!(0.63), Currency::USD));
    expect_eq!((a + b).to_string(), "14.00 USD");
    expect_eq!((a - b).to_string(), "12.74 USD");
    Ok(())
}

#[test]
fn chain_calculation_rounded_at_end() -> Result<()> {
    let a = usd!(1);
    let factor1 = dec!(0.99499);
    let factor2 = dec!(0.98);

    let mut product = a * factor1;
    product = product * factor2;
    expect_eq!(product.round().to_string(), "0.98 USD");

    // ... versus rounding at each step.
    let mut rounded_every_step = (a * factor1).round();
    rounded_every_step = (rounded_every_step * factor2).round();
    expect_eq!(rounded_every_step.to_string(), "0.97 USD");

    Ok(())
}
