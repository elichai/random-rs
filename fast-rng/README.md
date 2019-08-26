# Fast-Rng
[![Build Status](https://travis-ci.org/elichai/fast-rng.svg?branch=master)](https://travis-ci.org/elichai/fast-rng)
[![Latest version](https://img.shields.io/crates/v/fast-rng.svg)](https://crates.io/crates/fast-rng)
[![Documentation](https://docs.rs/fast-rng/badge.svg)](https://docs.rs/fast-rng)
![License](https://img.shields.io/crates/l/fast-rng.svg)
[![dependency status](https://deps.rs/repo/github/elichai/fast-rng/status.svg)](https://deps.rs/repo/github/elichai/fast-rng)

A Rust library [fast-rng](https://crates.io/crates/fast-rng) That helps generate **non-cryptographic** blazing fast randomness.

The randomness provided here implements the `random_trait::Random` trait.
* [Documentation](https://docs.rs/fast-rng)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
fast-rng = "0.1"
```

# Examples

```rust
use fast_rng::{FastRng, Random};

fn do_something() {
    let mut rng = FastRng::new();
    let i: u64 = rng.gen();
    let b: [u8; 12] = rng.gen();
}
```