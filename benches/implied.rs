use criterion::{criterion_group, criterion_main, Criterion};

use implied::{odds::format::OddsFormat, AmericanOdds, DecimalOdds, ProbabilityOdds};

pub fn benchmark_american(c: &mut Criterion) {
    c.bench_function("american to decimal", |b| {
        b.iter(|| AmericanOdds::new(-110).unwrap().to_decimal())
    });
    c.bench_function("american to probability", |b| {
        b.iter(|| AmericanOdds::new(-110).unwrap().to_probability())
    });
}

pub fn benchmark_decimal(c: &mut Criterion) {
    c.bench_function("decimal to american", |b| {
        b.iter(|| DecimalOdds::new(1.91).unwrap().to_american())
    });
    c.bench_function("decimal to probability", |b| {
        b.iter(|| DecimalOdds::new(1.91).unwrap().to_probability())
    });
}

pub fn benchmark_probability(c: &mut Criterion) {
    c.bench_function("probability to american", |b| {
        b.iter(|| ProbabilityOdds::new(0.5).unwrap().to_american())
    });
    c.bench_function("probability to decimal", |b| {
        b.iter(|| ProbabilityOdds::new(0.5).unwrap().to_decimal())
    });
}

criterion_group!(
    benches_odds_formats,
    benchmark_american,
    benchmark_decimal,
    benchmark_probability
);
criterion_main!(benches_odds_formats);
