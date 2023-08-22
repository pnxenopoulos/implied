//! # Odds
//!
//! Different odds formats and conversion methods.

use crate::probability::ToProbability;

// Odds types
pub mod american;
pub mod decimal;

pub fn calculate_hold<T: ToProbability>(odds: &[T]) -> f64 {
    odds.iter()
        .map(|o| o.to_probability().unwrap().value)
        .sum::<f64>()
        - 1.0
}

#[cfg(test)]
mod tests {
    use crate::odds::american::AmericanOdds;
    use crate::odds::calculate_hold;
    use crate::odds::decimal::DecimalOdds;

    #[test]
    fn test_hold_no_hold() {
        let odds = vec![
            AmericanOdds::new(-102).unwrap(),
            AmericanOdds::new(102).unwrap(),
        ];

        assert_eq!(calculate_hold(&odds), 0.0);
    }

    #[test]
    fn test_hold_standard_hold_american() {
        let odds = vec![
            AmericanOdds::new(-110).unwrap(),
            AmericanOdds::new(-110).unwrap(),
        ];

        assert_eq!(calculate_hold(&odds), 0.04761904761904767);
    }

    #[test]
    fn test_hold_standard_hold_decimal() {
        let odds = vec![
            DecimalOdds::new(1.0).unwrap(),
            DecimalOdds::new(2.0).unwrap(),
        ];

        assert_eq!(calculate_hold(&odds), 0.5);
    }
}
