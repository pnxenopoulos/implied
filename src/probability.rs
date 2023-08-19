use crate::odds::american::AmericanOdds;
use crate::odds::decimal::DecimalOdds;
use crate::odds::format::OddsFormat;

/// Represents odds in probability format.
#[derive(Debug, PartialEq)]
pub struct ProbabilityOdds {
    value: f64,
}

impl ProbabilityOdds {
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
        if !(0.0..=1.0).contains(&value) {
            return Err("Probability odds must be between 0 and 1");
        }

        Ok(Self { value })
    }
}

impl OddsFormat for ProbabilityOdds {
    fn to_decimal(&self) -> Result<DecimalOdds, &'static str> {
        return DecimalOdds::new(1.0 / self.value);
    }

    fn to_american(&self) -> Result<AmericanOdds, &'static str> {
        // We just take an easy route of converting to decimal and then to American
        let decimal_odds = self.to_decimal().unwrap();
        return decimal_odds.to_american();
    }

    fn to_probability(&self) -> Result<ProbabilityOdds, &'static str> {
        return ProbabilityOdds::new(self.value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_probability_odds_to_decimal() {
        let odds = ProbabilityOdds::new(0.75).unwrap();
        assert_eq!(
            odds.to_decimal().unwrap(),
            DecimalOdds::new(1.3333333333333333).unwrap()
        );
    }

    #[test]
    fn test_probability_odds_to_american() {
        let odds = ProbabilityOdds::new(0.75).unwrap();
        assert_eq!(
            odds.to_american().unwrap(),
            AmericanOdds::new(-300).unwrap()
        );
    }

    #[test]
    fn test_probability_odds_to_probability() {
        let odds = ProbabilityOdds::new(0.5238095238095238).unwrap();
        assert_eq!(
            odds.to_probability().unwrap(),
            ProbabilityOdds::new(0.5238095238095238).unwrap()
        );
    }

    #[test]
    fn test_invalid_probability_odds() {
        let result = ProbabilityOdds::new(1.01);
        assert_eq!(result, Err("Probability odds must be between 0 and 1"));

        let result = ProbabilityOdds::new(-0.99);
        assert_eq!(result, Err("Probability odds must be between 0 and 1"));
    }
}
