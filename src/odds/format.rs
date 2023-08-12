use crate::odds::american::AmericanOdds;
use crate::odds::decimal::DecimalOdds;
use crate::odds::probability::ProbabilityOdds;

pub trait OddsFormat {
    fn to_decimal(&self) -> Result<DecimalOdds, &'static str>;
    fn to_american(&self) -> Result<AmericanOdds, &'static str>;
    fn to_probability(&self) -> Result<ProbabilityOdds, &'static str>;
}
