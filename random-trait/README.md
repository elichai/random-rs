# random-trait
[![Latest version](https://img.shields.io/crates/v/random-trait.svg)](https://crates.io/crates/random-trait)
[![Documentation](https://docs.rs/random-trait/badge.svg)](https://docs.rs/random-trait)
![License](https://img.shields.io/crates/l/random-trait.svg)

A Rust library [random-trait](https://crates.io/crates/random-trait) that helps generating random values in an easy and convinient way 
while still being a very thin library without dependencies.

This crate is inspired by the [rand](http://crates.io/crates/rand) crate, 
but with the purpose of providing a very thin library, and support old compilers.

* [Documentation](https://docs.rs/random-trait)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
random-trait = "0.1"
```

and for Rust Edition 2015 add this to your crate root:

```rust
extern crate random_trait;
use random_trait::{GenerateRand, Random};
```
In Rust Edition 2018 you can simply do:
```rust
use random_trait::{GenerateRand, Random};
```

After that you'll need to implement `Random` for your source of randomness,  <br>
And optionally also add `GenerateRand` implementations for your custom types.

# Examples

```rust
use random_trait::{Random, GenerateRand};
 #[derive(Default)]
 struct MyRandomGenerator {
     ctr: usize,
 }

 impl Random for MyRandomGenerator {
     type Error = ();
     fn try_fill_bytes(&mut self, buf: &mut [u8]) -> Result<(), Self::Error> {
         for e in buf.iter_mut() {
             *e = self.ctr as u8;
             self.ctr += 1;
         }
         Ok(())
     }
 }

struct MyStuff {
    a: u64,
    b: char,
}

impl GenerateRand for MyStuff {
    fn generate<R: Random + ?Sized>(rand: &mut R) -> Self {
        MyStuff {a: rand.gen(), b: rand.gen() }
    }
}

fn get_random_stuff() -> MyStuff {
    let mut rand = MyRandomGenerator::default();
    rand.gen()
}

fn get_random_u128() -> u128 {
    let mut rand = MyRandomGenerator::default();
    rand.gen()
}
```