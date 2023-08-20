use crate::probability::{Probability, ToProbability};

pub fn devig_basic<T: ToProbability>(odds: &[T]) -> Vec<Probability> {
    // First, calculate the total sum of the odds converted to probabilities
    let mut sum = 0.0;
    for outcome in odds {
        let outcome_vigged_prob = outcome.to_probability().unwrap();
        sum += outcome_vigged_prob.value;
    }

    // Calculate the values and store them in a Vec
    let mut devigged_probs = Vec::with_capacity(odds.len());
    for outcome in odds {
        let outcome_vigged_prob = outcome.to_probability().unwrap();
        let implied_prob = Probability::new(outcome_vigged_prob.value / sum).unwrap();
        devigged_probs.push(implied_prob);
    }

    devigged_probs
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::odds::american::AmericanOdds;

    #[test]
    fn test_basic_devigger() {
        let odds = [
            AmericanOdds::new(-110).unwrap(),
            AmericanOdds::new(-110).unwrap(),
        ];

        let devigged_probs = devig_basic(&odds);

        assert_eq!(devigged_probs[0].value, 0.5);
        assert_eq!(devigged_probs[1].value, 0.5);
    }
}
