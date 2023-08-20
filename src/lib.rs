//! # implied
//!
//! A library to represent, convert and devig sports betting odds.
pub mod devig;
pub mod odds;
pub mod probability;

pub use devig::basic;
pub use odds::american::AmericanOdds;
pub use odds::decimal::DecimalOdds;
pub use probability::Probability;
