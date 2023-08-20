use crate::odds::american::{AmericanOdds, ToAmericanOdds};
use crate::probability::{Probability, ToProbability};

/// Represent odds in Decimal format, which is any real number greater than or equal to 1.
#[derive(Debug, PartialEq)]
pub struct DecimalOdds {
    value: f64,
}

impl DecimalOdds {
    /// Creates a new instance of `DecimalOdds`.
    ///
    /// # Arguments
    ///
    /// * `value` - The decimal odds value.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `DecimalOdds` instance if the value is greater than 1,
    /// otherwise returns an error message.
    pub fn new(value: f64) -> Result<Self, &'static str> {
        if value < 1.0 {
            return Err("Decimal odds must be greater than 1");
        }

        Ok(Self { value })
    }
}

impl ToAmericanOdds for DecimalOdds {
    fn to_american(&self) -> Result<AmericanOdds, &'static str> {
        if self.value >= 2.0 {
            let american_odds = (self.value - 1.0) * 100.0;
            return AmericanOdds::new(american_odds.round() as i32);
        } else {
            let american_odds = -100.0 / (self.value - 1.0);
            return AmericanOdds::new(american_odds.round() as i32);
        }
    }
}

impl ToProbability for DecimalOdds {
    fn to_probability(&self) -> Result<Probability, &'static str> {
        return Probability::new(1.0 / self.value);
    }
}

pub trait ToDecimalOdds {
    fn to_decimal(&self) -> Result<DecimalOdds, &'static str>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decimal_odds_to_american() {
        let odds_favorite = DecimalOdds::new(1.9090909090909092).unwrap();
        assert_eq!(
            odds_favorite.to_american().unwrap(),
            AmericanOdds::new(-110).unwrap()
        );

        let odds_underdog = DecimalOdds::new(2.9090909090909092).unwrap();
        assert_eq!(
            odds_underdog.to_american().unwrap(),
            AmericanOdds::new(191).unwrap()
        );
    }

    #[test]
    fn test_decimal_odds_to_probability() {
        let odds = DecimalOdds::new(1.9090909090909092).unwrap();
        assert_eq!(
            odds.to_probability().unwrap(),
            Probability::new(0.5238095238095238).unwrap()
        );
    }

    #[test]
    fn test_invalid_decimal_odds() {
        let result = DecimalOdds::new(0.99);
        assert_eq!(result, Err("Decimal odds must be greater than 1"));

        let result = DecimalOdds::new(-1.0);
        assert_eq!(result, Err("Decimal odds must be greater than 1"));
    }
}
