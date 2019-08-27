# random-rs
[![Latest version](https://img.shields.io/crates/v/random-rs.svg)](https://crates.io/crates/random-rs)
[![Documentation](https://docs.rs/random-rs/badge.svg)](https://docs.rs/random-rs)
![License](https://img.shields.io/crates/l/random-rs.svg)

A Rust library [random-rs](https://crates.io/crates/random-rs) that helps generating random values in an easy and convinient way 
while still being a very thin library without dependencies.

This crate is inspired by the [rand](http://crates.io/crates/rand) crate, 
but with the purpose of providing a very thin library, and support old compilers.

* [Documentation](https://docs.rs/random-rs)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
random-rs = "0.1"
```

After that the crate gives you a couple of options, you can Implement the `Random` trait for your source of randomness. <br>
Or you can use use one of the provided RNGs which already implemenmt the trait. <br>
Optionally you can also implement `GenerateRand` for your custom types so that the RNGs can create those for you.

# Examples

```rust
use random_rs::{Random, GenerateRand};
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

Or use a provided RNG:
```rust
use random_rs::{Random, fast::FastRng};
impl GenerateRand for MyStuff {
    fn generate<R: Random + ?Sized>(rand: &mut R) -> Self {
        MyStuff {a: rand.gen(), b: rand.gen() }
    }
}

fn get_random_stuff() -> MyStuff {
    let mut rand = FastRng::new();
    rand.gen()
}

```