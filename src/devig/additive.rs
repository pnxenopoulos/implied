use crate::probability::{Probability, ToProbability};


pub fn devig_additive<T: ToProbability>(odds: &[T]) -> Vec<Probability> {
    let margin: f64 = (odds
        .iter()
        .map(|o| o.to_probability().unwrap().value)
        .sum::<f64>()
        - 1.0)
        / (odds.len() as f64);

    odds.iter()
        .map(|o| Probability::new(o.to_probability().unwrap().value - margin).unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{odds::american::AmericanOdds, DecimalOdds};
    use std::vec::Vec;

    #[test]
    fn test_additive_devigger_american() {
        let odds = [
            AmericanOdds::new(-110).unwrap(),
            AmericanOdds::new(-110).unwrap(),
        ];

        let devigged_probs = devig_additive(&odds);

        assert_eq!(devigged_probs[0].value, 0.5);
        assert_eq!(devigged_probs[1].value, 0.5);
    }

    #[test]
    #[should_panic(expected = "Probability odds must be between 0 and 1")]
    fn test_negative_devig_results() {
        let odds = [
            DecimalOdds::new(1.870).unwrap(),
            DecimalOdds::new(1.2).unwrap(),
            DecimalOdds::new(1.1).unwrap(),
            DecimalOdds::new(1.05).unwrap(),
            DecimalOdds::new(1.02).unwrap(),
            DecimalOdds::new(1.01).unwrap(),
        ];

        devig_additive(&odds);
    }

    #[test]
    fn test_empty_odds() {
        let odds: Vec<DecimalOdds> = vec!();

        assert_eq!(std::vec::Vec::<Probability>::new(), devig_additive(&odds));
    }
}
