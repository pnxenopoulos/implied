use crate::odds::american::{AmericanOdds, ToAmericanOdds};
use crate::odds::decimal::{DecimalOdds, ToDecimalOdds};

/// Represents odds in probability format.
#[derive(Debug, PartialEq)]
pub struct Probability {
    value: f64,
}

impl Probability {
    /// Creates a new instance of `ProbabilityOdds`.
    ///
    /// # Arguments
    ///
    /// * `value` - The probability odds value.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `ProbabilityOdds` instance if the value is between 0 and 1,
    /// otherwise returns an error message.
    pub fn new(value: f64) -> Result<Self, &'static str> {
        if value < 0.0 || 1.0 < value {
            return Err("Probability odds must be between 0 and 1");
        }

        Ok(Self { value })
    }
}

impl ToAmericanOdds for Probability {
    fn to_american(&self) -> Result<AmericanOdds, &'static str> {
        // We just take an easy route of converting to decimal and then to American
        let decimal_odds = self.to_decimal().unwrap();
        return decimal_odds.to_american();
    }
}

impl ToDecimalOdds for Probability {
    fn to_decimal(&self) -> Result<DecimalOdds, &'static str> {
        return DecimalOdds::new(1.0 / self.value);
    }
 }

pub trait ToProbability {
    fn to_probability(&self) -> Result<Probability, &'static str>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_probability_odds_to_decimal() {
        let odds = Probability::new(0.75).unwrap();
        assert_eq!(
            odds.to_decimal().unwrap(),
            DecimalOdds::new(1.3333333333333333).unwrap()
        );
    }

    #[test]
    fn test_probability_odds_to_american() {
        let odds = Probability::new(0.75).unwrap();
        assert_eq!(
            odds.to_american().unwrap(),
            AmericanOdds::new(-300).unwrap()
        );
    }

    #[test]
    fn test_invalid_probability_odds() {
        let result = Probability::new(1.01);
        assert_eq!(result, Err("Probability odds must be between 0 and 1"));

        let result = Probability::new(-0.99);
        assert_eq!(result, Err("Probability odds must be between 0 and 1"));
    }
}
