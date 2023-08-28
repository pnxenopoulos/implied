use crate::{probability::Probability, DecimalOdds};

pub fn kelly_criterion(win_prob: Probability, to_win: DecimalOdds) -> f64 {
    win_prob.value - (1.0 - win_prob.value) / to_win.value
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::odds::american::AmericanOdds;
    use crate::odds::decimal::DecimalOdds;

    #[test]
    fn test_kelly_criterion_zero() {
        let win_prob = Probability::new(0.5).unwrap();
        let to_win = DecimalOdds::new(2.0).unwrap();
        let amount_to_bet = kelly_criterion(win_prob, to_win);

        assert_eq!(amount_to_bet, 0.0)
    }
}
