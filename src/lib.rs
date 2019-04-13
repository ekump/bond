#![crate_name = "bond"]

extern crate bigdecimal;

use bigdecimal::BigDecimal;



/// Returns the current yield of an instrument.
///
/// # Arguments
/// * 'annual_dollar_coupon_interest' - The dollar value of the annual coupon ammount
/// * 'price' - The current price of the instrument
///
/// # Example
///
/// ```
/// extern crate bigdecimal;
/// use bigdecimal::BigDecimal;
/// use std::str::FromStr;
/// use bond::*;
/// fn main() {
///     let annual_dollar_coupon_interest = BigDecimal::from_str("10.00").unwrap();
///     let price = BigDecimal::from_str("102.00").unwrap();
///     let curr_yield = current_yield(annual_dollar_coupon_interest, price);
/// }
/// ```
pub fn current_yield(annual_dollar_coupon_interest: BigDecimal, price: BigDecimal) -> BigDecimal {
    annual_dollar_coupon_interest / price
}

#[cfg(test)]
use std::str::FromStr;

#[test]
fn current_yield_test() {
    let annual_dollar_coupon_interest = BigDecimal::from_str("15.00").unwrap();
    let price = BigDecimal::from_str("100.00").unwrap();
    let actual_current_yield = current_yield(annual_dollar_coupon_interest, price);
    let expected_current_yield = BigDecimal::from_str("0.15").unwrap();
    assert_eq!(actual_current_yield, expected_current_yield);
}
