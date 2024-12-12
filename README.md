# 🤑 Cashmoney 💸

[![Crates.io](https://img.shields.io/crates/v/cashmoney)](https://crates.io/crates/cashmoney)
[![Documentation](https://docs.rs/cashmoney/badge.svg)](https://docs.rs/cashmoney)
[![Crates.io](https://img.shields.io/crates/l/cashmoney)](LICENSE)

Cashmoney is a Rust library for expressing monetary values and performing safe
monetary calculations suitable for financial applications.

## Features

 - Ensure monetary values have the correct number of decimal places
   according to their currency.
 - Prevent accidental summation of disparate currencies.
 - Use exact or high-precision representation for intermediate calculations.
 - Round monetary values as the final step in a series of calculations by an 
   explicit API call.

## Usage

### Creating a Money object

```rust
let value = dec!(13.37);
let a: Result<Money> = Money::from(value, Currency::USD);

assert_eq!(a, b);

// Convenience macros. Note that these macros rely on the rust_decimal crate's
// `dec!` macro to parse the values.
let c = cad!(13.37;
let d = usd!(13.37);
```
### Adding and subtracting

```rust
let sum: Money = usd!(13) + usd!(0.37);
assert_eq!(sum, usd!(13.37));

// Returns Err:
let _ = usd!(13) + cad!(0.37);

let difference: Money = usd!(14.00) - usd!(0.63);
assert_eq!(difference, usd!(13.37));
```

### Scalar multiplication and division

```rust
// FractionalMoney represents a monetary value that may be a more precise value
// than the smallest denomination for the given currency. For example, the
// value below would be USD $6.685.
let product: FractionalMoney = usd!(13.37) * dec!(0.5);
assert_eq!(product.fractional_value(), dec!(6.685));

// "Standard" rounding (midpoint rounds away from zero)
let rounded_up: Money = product.round_up();
assert_eq!(rounded_up, usd!(6.69));

// "Banker's rounding"
let rounded: Money = product.round();
assert_eq!(rounded, usd!(6.68));
```
