#![no_std]
#![deny(missing_docs)]
#![cfg_attr(test, deny(warnings))]

//! # Random-fast-rng
//!
//! This crate provides a fast **non cryptographic** random number generator that implements the [`Random`](trait.Random.html) trait. <br>
//! Currently it's implemented using the `Pcg32` algorithm, that generates 32 bit of random data for every state change. <br>
//! the exact algorithm might change in the future, but the properties should stay the same (Blazing fast, non cryptographic, and minimal I/O)
//! The crate is part of the `random-rs` facade, and as such supports older rust compilers(currently 1.13+) and should have only thin amount of dependencies.
//!
//! This Random generator is good for testing uses, and use cases that require some non-determinism. it shouldn't be used to generate keys/passwords. <br>
//!
//! By enabling the `std` feature this crate exposes a [`new()`](struct.FastRng.html#method.new) function that uses [`SystemTime::now()`](https://doc.rust-lang.org/std/time/struct.SystemTime.html) to seed the RNG.<br>
//! It also exposes a [`local_rng()`](fn.local_rng.html) function to give a persistent Rng that is seeded only once and is unique per thread (so there's no need to worry about dropping and reinitializing the Rng)
//!

#[cfg(feature = "std")]
#[macro_use]
extern crate std;

#[cfg(feature = "std")]
mod thread;

pub extern crate random_trait;
pub use random_trait::Random;

#[cfg(feature = "std")]
use thread::FromRawPtr;
#[cfg(feature = "std")]
pub use thread::ThreadFastRng;

use core::mem;

#[cfg(feature = "doc-comment")]
extern crate doc_comment;
#[cfg(feature = "doc-comment")]
doc_comment::doctest!("../README.md");

const PCG_DEFAULT_MULTIPLIER_64: u64 = 6_364_136_223_846_793_005;

/// A FastRng struct implementing [`Random`](trait.Random.html). you can initialize it with your own seed using [`FastRng::seed()`](struct.FastRng.html#method.seed)
/// Or if the `std` feature is enabled call [`FastRng::new()`](struct.FastRng.html#method.seed) which will seed it with the system time. <br>
/// For ergonomics and ease of usability the Rng is also provided as a global thread local variable using [`local_rng()`](fn.local_rng.html)
pub struct FastRng {
    // Pcg32
    state: u64,
    inc: u64,
}

impl FastRng {
    /// Creates a new instance of `FastRng` seeded with the system time.
    ///
    /// # Examples
    /// ```rust
    /// use random_fast_rng::{FastRng, Random};
    ///
    /// let mut rng = FastRng::new();
    /// let random_u8 = rng.get_u8();
    /// let arr: [u8; 32] = rng.gen();
    /// ```
    ///
    #[cfg(feature = "std")]
    pub fn new() -> Self {
        let (a, b) = time_seed();
        Self::seed(a, b)
    }

    /// A function to manually seed the Rng in `no-std` cases.
    /// Ideally both the `seed` and the `seq` should be randcom numbers.
    /// the `seed` represents the starting state of the algorithm,
    /// and  the `seq` represents a constant random sequence that will be used to increment and re-randomize the state.
    /// The exact usage of this numbers is not promised by the API, and might change.
    /// as this is firstly a fast random generator, not a PCG random generator, and not a deterministic random generator.
    pub fn seed(seed: u64, seq: u64) -> Self {
        let init_inc = (seq << 1) | 1;
        let init_state = seed + init_inc;
        let mut rng = FastRng { state: init_state, inc: init_inc };
        rng.state = rng.state.wrapping_mul(PCG_DEFAULT_MULTIPLIER_64).wrapping_add(rng.inc);
        rng
    }

    fn gen_u32(&mut self) -> u32 {
        let old_state = self.state;
        self.state = self.state.wrapping_mul(PCG_DEFAULT_MULTIPLIER_64).wrapping_add(self.inc);

        let xorshift = (((old_state >> 18) ^ old_state) >> 27) as u32;
        let rot = (old_state >> 59) as i32;
        (xorshift >> rot) | (xorshift << ((-rot) & 31))
    }
}

/// Returns a thread local instance which is seeded only once per thread (no need to worry about dropping and reinitializing)
///
/// # Examples
/// ```rust
/// use random_fast_rng::{Random, local_rng};
///
/// let random_u8 = local_rng().get_u8();
/// let arr: [u8; 32] = local_rng().gen();
/// ```
///
#[cfg(feature = "std")]
pub fn local_rng() -> ThreadFastRng {
    use std::cell::RefCell;
    thread_local! {
        pub static THREAD_FAST_RNG: RefCell<FastRng> = RefCell::new(FastRng::new());
    }
    let ptr = THREAD_FAST_RNG.with(|r| r.as_ptr());
    ThreadFastRng::from_ptr(ptr)
}

#[cfg(feature = "std")]
fn time_seed() -> (u64, u64) {
    use std::time;
    let now = time::SystemTime::now();
    let unix = now.duration_since(time::UNIX_EPOCH).unwrap();

    (unix.as_secs(), u64::from(unix.subsec_nanos()))
}

impl Random for FastRng {
    type Error = ();

    fn try_fill_bytes(&mut self, buf: &mut [u8]) -> Result<(), Self::Error> {
        for chunk in buf.chunks_mut(4) {
            let rand: [u8; 4] = unsafe { mem::transmute(self.gen_u32()) };
            let len = chunk.len();
            chunk.copy_from_slice(&rand[..len]);
        }
        Ok(())
    }
    fn get_u32(&mut self) -> u32 {
        self.gen_u32()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_local() {
        let mut local_rng = local_rng();
        let a: u64 = local_rng.gen();
        let b: u32 = local_rng.gen();
        let c: [u8; 64] = local_rng.gen();
        assert_ne!(a, 0);
        assert_ne!(b, 0);
        assert_ne!(&c[..], &[0u8; 64][..]);
    }

    #[test]
    fn test_float() {
        let mut rng = FastRng::new();
        let f: f32 = rng.gen();
        assert!(f > 0.0 && f < 1.0);
        let f: f64 = rng.gen();
        assert!(f > 0.0 && f < 1.0);
    }
}
