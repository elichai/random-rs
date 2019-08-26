# random-random-fast-rng
[![Build Status](https://travis-ci.org/elichai/random-fast-rng.svg?branch=master)](https://travis-ci.org/elichai/random-fast-rng)
[![Latest version](https://img.shields.io/crates/v/random-fast-rng.svg)](https://crates.io/crates/random-fast-rng)
[![Documentation](https://docs.rs/random-fast-rng/badge.svg)](https://docs.rs/random-fast-rng)
![License](https://img.shields.io/crates/l/random-fast-rng.svg)
[![dependency status](https://deps.rs/repo/github/elichai/random-fast-rng/status.svg)](https://deps.rs/repo/github/elichai/random-fast-rng)

A Rust library [random-fast-rng](https://crates.io/crates/random-fast-rng) That helps generate **non-cryptographic** blazing fast randomness.

The randomness provided here implements the `random_trait::Random` trait.
* [Documentation](https://docs.rs/random-fast-rng)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
random-fast-rng = "0.1"
```

# Examples

```rust
use random_fast_rng::{FastRng, Random};

fn do_something() {
    let mut rng = FastRng::new();
    let i: u64 = rng.gen();
    let b: [u8; 12] = rng.gen();
}
```