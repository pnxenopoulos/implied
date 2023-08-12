# implied
Represent and devig odds for sports betting in Rust

![Crates.io](https://img.shields.io/crates/d/implied) ![Build](https://img.shields.io/github/actions/workflow/status/pnxenopoulos/implied/build.yaml)

`implied` is a Rust crate used to represent odds formats commonly used in betting (American, Decimal, Probability) and to devig them (i.e., find the implied odds) using methods such as the additive method or Shin's method. You can use `implied` in your own Rust project by including it as a dependency in your `Cargo.toml`:

```
[dependencies]
implied = "0.0.1"
```