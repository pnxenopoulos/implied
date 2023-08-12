use crate::odds::decimal::DecimalOdds;
use crate::odds::format::OddsFormat;
use crate::odds::probability::ProbabilityOdds;

/// Represents odds in American format, which range from negative infinity to -100 and 100 to
/// positive infinity.
#[derive(Debug)]
pub struct AmericanOdds {
    value: i32,
}

impl AmericanOdds {
    /// Creates a new instance of `AmericanOdds`.
    ///
    /// # Arguments
    ///
    /// * `value` - The American odds value.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `AmericanOdds` instance if the value is within the valid range,
    /// otherwise returns an error message. The valid range for American odds is below -100 and above 100.
    pub fn new(value: i32) -> Result<Self, &'static str> {
        if value > -100 && value < 100 {
            return Err("American odds must be < -100 and > 100");
        }

        Ok(Self { value })
    }
}

impl PartialEq for AmericanOdds {
    fn eq(&self, other: &Self) -> bool {
        self.to_decimal() == other.to_decimal()
    }
}

impl OddsFormat for AmericanOdds {
    fn to_decimal(&self) -> Result<DecimalOdds, &'static str> {
        if self.value < 0 {
            return DecimalOdds::new(1.0 + (100.0 / (-1.0 * self.value as f64)));
        } else {
            return DecimalOdds::new((self.value as f64 / 100.0) + 1.0);
        }
    }

    fn to_american(&self) -> Result<Self, &'static str> {
        AmericanOdds::new(self.value)
    }

    fn to_probability(&self) -> Result<ProbabilityOdds, &'static str> {
        if self.value < 0 {
            let negative_american_odds = -1.0 * self.value as f64;
            return ProbabilityOdds::new(negative_american_odds / (negative_american_odds + 100.0));
        } else {
            return ProbabilityOdds::new(100.0 / (100.0 + self.value as f64));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_american_odds_to_decimal() {
        let negative_even_odds = AmericanOdds::new(-100).unwrap();
        let positive_even_odds = AmericanOdds::new(100).unwrap();
        assert_eq!(
            negative_even_odds.to_decimal().unwrap(),
            positive_even_odds.to_decimal().unwrap()
        );

        let odds = AmericanOdds::new(-110).unwrap();
        assert_eq!(
            odds.to_decimal().unwrap(),
            DecimalOdds::new(1.9090909090909092).unwrap()
        );
    }

    #[test]
    fn test_american_odds_to_american() {
        let negative_even_odds = AmericanOdds::new(-100).unwrap();
        assert_eq!(
            negative_even_odds.to_american().unwrap(),
            AmericanOdds::new(-100).unwrap()
        );

        let positive_even_odds = AmericanOdds::new(100).unwrap();
        assert_eq!(
            positive_even_odds.to_american().unwrap(),
            AmericanOdds::new(100).unwrap()
        );
    }

    #[test]
    fn test_american_odds_to_probability() {
        let negative_even_odds = AmericanOdds::new(-100).unwrap();
        let positive_even_odds = AmericanOdds::new(100).unwrap();
        assert_eq!(
            negative_even_odds.to_probability().unwrap(),
            positive_even_odds.to_probability().unwrap()
        );

        let odds = AmericanOdds::new(-110).unwrap();
        assert_eq!(
            odds.to_probability().unwrap(),
            ProbabilityOdds::new(0.5238095238095238).unwrap()
        );
    }

    #[test]
    fn test_invalid_american_odds() {
        let result = AmericanOdds::new(99);
        assert_eq!(result, Err("American odds must be < -100 and > 100"));

        let result = AmericanOdds::new(-99);
        assert_eq!(result, Err("American odds must be < -100 and > 100"));
    }
}
