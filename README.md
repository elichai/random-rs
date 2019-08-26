# random-rs

This repository is highly inspired by the [rand](http://crates.io/crates/rand) crate.
But it plans to give something a bit different. this is meant to give a very minimalistic random generation with as little crates + code as possible.
And keep backwards compatibily for as long as possible.

## Rust version requirements
`random-rs` is currently tested against rust 1.13.0. it might also support older compilers, but no promises.
The plan is to keep supporting 1.13 as long as possible, in a case where a bump will be needed it will be accompanied by a major version bump.

## Crates
 | name                                                 | version | purpose                                                    | algorithm                                                                                                                                        |
 |------------------------------------------------------|---------|------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------|
 | [random-trait](http://crates.io/crates/random-trait) | 0.1     | The main trait, let's you randomly generate generic types  | The only part that requires a specific algorithm here is the floating points, using https://mumble.net/~campbell/2014/04/28/uniform-random-float |
 | [random-fast-rng](https://crates.io/crates/random-fast-rng)        | 0.1     | Blazing fast **non cryptographic** random number generator | Uses Pcg32 and seeds it from system time                                                                                                         |
