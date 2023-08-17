//! # implied
//!
//! A library to represent, convert and devig sports betting odds.
pub mod odds;

pub use odds::american::AmericanOdds;
pub use odds::decimal::DecimalOdds;
pub use odds::probability::ProbabilityOdds;
