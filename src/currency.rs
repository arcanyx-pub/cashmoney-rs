/// Supported currencies, identified by their ISO 4217 code.
#[derive(Copy, Clone, Debug, PartialEq)]
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
