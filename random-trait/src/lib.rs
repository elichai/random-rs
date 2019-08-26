#![no_std]
#![recursion_limit = "130"]
#![deny(missing_docs)]
#![cfg_attr(test, deny(warnings))]

//! # Random Trait
//!
//! This crate provides a simple thin trait for producing generic random values based on any random source.
//! The crates assurances are based on the assurances of the RNG it is implemented on.<br>
//! if that RNG is cryptographically secure then this crate should provide a cryptographically secure numbers. (including floats)
//! if the RNG is biased (which is fine for tests and some other applications) then the results will also be bias.
//! This crate **does not** try to compensate for biases in the RNG source.
//!
//! please see the [`GenerateRand`](trait.GenerateRand.html) and [`Random`](trait.Random.html) for more information and examples.
//!

use core::{char, mem};

#[cfg(feature = "doc-comment")]
extern crate doc_comment;
#[cfg(feature = "doc-comment")]
doc_comment::doctest!("../README.md");

/// This trait is used by `Random::gen()` as a generic function to create a random value for any type which implements it.
/// I try to give by default implementations for all the types in libcore, including arrays and tuples, if anything is missing please raise the issue.
/// You can implement this for any of your types.
/// # Examples
/// ```rust
/// use random_trait::{Random, GenerateRand};
/// struct MyStuff{
///     a: u64,
///     b: char,
/// }
///
/// impl GenerateRand for MyStuff {
///     fn generate<R: Random + ?Sized>(rand: &mut R) -> Self {
///         MyStuff {a: rand.gen(), b: rand.gen() }
///     }
/// }
/// ```
///
pub trait GenerateRand {
    /// Generate a random value, using the `rand` as source of randomness.
    fn generate<R: Random + ?Sized>(rand: &mut R) -> Self;
}

///
/// This is the base trait of the crate. By implementing the required method on your random generator source
/// it will give you a long list of functions, the important of them is `Random::gen() -> T` which will produce a random value
/// for every type which implements `GenerateRand` (you can implement this for your own types).
///
/// Notice that some random sources can produce non byte-array values with more efficiency, so if you want to use that
/// you can just override a provided method and use the random source directly.
///
/// If your random source is fallable in a way that can be handled please also implement `fill_bytes` and handle the errors properly.
/// otherwise it will panic.
///
/// # Example
/// ```rust
/// use random_trait::{Random, GenerateRand};
///
/// #[derive(Default)]
/// struct MyRandomGenerator {
///     ctr: usize,
/// }
///
/// impl Random for MyRandomGenerator {
///     type Error = ();
///     fn try_fill_bytes(&mut self, buf: &mut [u8]) -> Result<(), Self::Error> {
///         for e in buf.iter_mut() {
///             *e = self.ctr as u8;
///             self.ctr += 1;
///         }
///         Ok(())
///     }
/// }
///
/// # fn main() {
/// let mut rand = MyRandomGenerator::default();
/// let rand_u32: u32 = rand.gen();
/// assert_eq!(rand_u32, 50462976);
/// let rand_u32: u32 = rand.gen();
/// assert_eq!(rand_u32, 117835012);
/// # }
/// ```
///
pub trait Random {
    /// The Error type, based on the source of randomness, non fallible sources can use `Error=()`
    type Error;

    /// This is the main method of the trait.
    /// You should implement this on your randomness source and will the buffer with random data.
    fn try_fill_bytes(&mut self, buf: &mut [u8]) -> Result<(), Self::Error>;

    /// Uses `try_fill_bytes` but panics if returns an error.
    /// Override if you can gracefully handle errors in the randomness source.
    fn fill_bytes(&mut self, buf: &mut [u8]) {
        if self.try_fill_bytes(buf).is_err() {
            panic!("Failed getting randmness");
        }
    }

    /// Returns a generic random value which implements `GenerateRand`
    fn gen<T: GenerateRand>(&mut self) -> T {
        T::generate(self)
    }

    /// Returns a random `u8` number.
    fn get_u8(&mut self) -> u8 {
        let mut buf = [0u8; 1];
        self.fill_bytes(&mut buf);
        buf[0]
    }

    /// Returns a random `u16` number.
    fn get_u16(&mut self) -> u16 {
        let mut buf = [0u8; 2];
        self.fill_bytes(&mut buf);
        unsafe { mem::transmute(buf) }
    }

    /// Returns a random `u32` number.
    fn get_u32(&mut self) -> u32 {
        let mut buf = [0u8; 4];
        self.fill_bytes(&mut buf);
        unsafe { mem::transmute(buf) }
    }

    /// Returns a random `u64` number.
    fn get_u64(&mut self) -> u64 {
        let mut buf = [0u8; 8];
        self.fill_bytes(&mut buf);
        unsafe { mem::transmute(buf) }
    }

    /// Returns a random `usize` number.
    #[cfg(target_pointer_width = "64")]
    fn get_usize(&mut self) -> usize {
        self.get_u64() as usize
    }

    /// Returns a random `usize` number.
    #[cfg(target_pointer_width = "32")]
    fn get_usize(&mut self) -> usize {
        self.get_u32() as usize
    }

    /// Returns a random `usize` number.
    #[cfg(target_pointer_width = "16")]
    fn get_usize(&mut self) -> usize {
        self.get_u16() as usize
    }

    /// Returns a random `u128` number.
    #[cfg(feature = "u128")]
    fn get_u128(&mut self) -> u128 {
        let mut buf = [0u8; 16];
        self.fill_bytes(&mut buf);
        unsafe { mem::transmute(buf) }
    }

    /// Returns a random `bool` with 50/50 probability.
    fn get_bool(&mut self) -> bool {
        // TODO: More research, least/most significant bit?
        let bit = self.get_u8() & 0b1000_0000;
        debug_assert!(bit < 2);
        bit == 1
    }
}

impl GenerateRand for u8 {
    #[inline]
    fn generate<R: Random + ?Sized>(rand: &mut R) -> Self {
        rand.get_u8()
    }
}

impl GenerateRand for u16 {
    #[inline]
    fn generate<R: Random + ?Sized>(rand: &mut R) -> Self {
        rand.get_u16()
    }
}

impl GenerateRand for u32 {
    #[inline]
    fn generate<R: Random + ?Sized>(rand: &mut R) -> Self {
        rand.get_u32()
    }
}

impl GenerateRand for u64 {
    #[inline]
    fn generate<R: Random + ?Sized>(rand: &mut R) -> Self {
        rand.get_u64()
    }
}

impl GenerateRand for usize {
    #[inline]
    fn generate<R: Random + ?Sized>(rand: &mut R) -> Self {
        rand.get_usize()
    }
}

#[cfg(feature = "u128")]
impl GenerateRand for u128 {
    #[inline]
    fn generate<R: Random + ?Sized>(rand: &mut R) -> Self {
        rand.get_u128()
    }
}

impl GenerateRand for char {
    #[inline]
    fn generate<R: Random + ?Sized>(rand: &mut R) -> Self {
        loop {
            if let Some(c) = char::from_u32(rand.get_u32()) {
                return c;
            }
        }
    }
}

impl GenerateRand for bool {
    #[inline]
    fn generate<R: Random + ?Sized>(rand: &mut R) -> Self {
        rand.get_bool()
    }
}

// Source: https://mumble.net/~campbell/2014/04/28/uniform-random-float
// https://mumble.net/~campbell/2014/04/28/random_real.c
impl GenerateRand for f64 {
    fn generate<R: Random + ?Sized>(rand: &mut R) -> Self {
        let mut exponent: i32 = -64;
        let mut significand = rand.get_u64();
        while significand == 0 {
            exponent -= 64;
            if exponent < -1074i32 {
                // E min(-1022)-p(53)+1  (https://en.wikipedia.org/wiki/IEEE_754)
                // In reallity this should probably never happen. prob of ~1/(2^1024) unless randomness is broken.
                unreachable!("The randomness is broken, got 0 16 times. (prob of 1/2^1024)");
            }
            significand = rand.get_u64();
        }

        // Shift the leading zeros into the exponent
        let shift = significand.leading_zeros() as i32;
        if shift > 0 {
            exponent -= shift;
            significand <<= shift;
            significand |= rand.get_u64() >> (64 - shift);
        }
        // Set the sticky bit.
        significand |= 1;

        // Convert to float and scale by 2^exponent.
        significand as f64 * exp2(exponent)
    }
}

// Source: https://mumble.net/~campbell/2014/04/28/uniform-random-float
// https://mumble.net/~campbell/2014/04/28/random_real.c
impl GenerateRand for f32 {
    fn generate<R: Random + ?Sized>(rand: &mut R) -> Self {
        let mut exponent: i32 = -32;
        let mut significand = rand.get_u32();
        while significand == 0 {
            exponent -= 32;
            if exponent < -149i32 {
                // E min(-126)-p(24)+1  (https://en.wikipedia.org/wiki/IEEE_754)
                // In reallity this should probably never happen. prob of ~1/(2^1024) unless randomness is broken.
                unreachable!("The randomness is broken, got 0 5 times. (prob of 1/2^160)");
                // TODO: Should this stay unreachable or change to return 0?
            }
            significand = rand.get_u32();
        }

        // Shift the leading zeros into the exponent
        let shift = significand.leading_zeros() as i32;
        if shift != 0 {
            exponent -= shift;
            significand <<= shift;
            significand |= rand.get_u32() >> (32 - shift);
        }
        // Set the sticky bit, almost definitely another 1 in the random stream.
        significand |= 1;

        // Convert to float and scale by 2^exponent.
        significand as f32 * exp2f(exponent)
    }
}

/// This is from IEEE-754.
/// you take the E max, subtract the exponent from it, and shift it according to the precision-1
fn exp2f(exp: i32) -> f32 {
    debug_assert!(exp > -127);
    let bits = ((127i32 + exp) as u32) << 23u32;
    unsafe { mem::transmute(bits) } // this is the same as `f32::from_bits`
}
fn exp2(exp: i32) -> f64 {
    debug_assert!(exp > -1023);
    let bits = ((1023i32 + exp) as u64) << 52u64;
    unsafe { mem::transmute(bits) } // this is the same as `f64::from_bits`
}

// Will overflow(i.e. sign extend) correctly https://doc.rust-lang.org/nomicon/casts.html.
// should only be used with the same type.
macro_rules! impl_generate_rand_ifromu {
    ($ity:ty, $uty: ty) => {
        impl GenerateRand for $ity {
            #[inline]
            fn generate<R: Random + ?Sized>(rand: &mut R) -> Self {
                debug_assert_eq!(mem::size_of::<$ity>(), mem::size_of::<$uty>());
                <$uty>::generate(rand) as $ity
            }
        }
    };
}

impl_generate_rand_ifromu! {i8, u8}
impl_generate_rand_ifromu! {i16, u16}
impl_generate_rand_ifromu! {i32, u32}
impl_generate_rand_ifromu! {i64, u64}
impl_generate_rand_ifromu! {isize, usize}
#[cfg(feature = "u128")]
impl_generate_rand_ifromu! {i128, u128}

// the reason for both $t and $ts is that this way each iteration it's reducing the amount of variables by one
macro_rules! array_impls {
    {$N:expr, $t:ident $($ts:ident)*} => {
            impl<T: GenerateRand> GenerateRand for [T; $N] {
                #[inline]
                fn generate<R: Random + ?Sized>(rand: &mut R) -> Self {
                    [rand.gen::<$t>(), $(rand.gen::<$ts>()),*]
                }
            }
            array_impls!{($N - 1), $($ts)*}
    };
    {$N:expr,} => {
        impl<T: GenerateRand> GenerateRand for [T; $N] {
            #[inline]
            fn generate<R: Random + ?Sized>(_: &mut R) -> Self { [] }
        }
    };
}

array_impls! {128, T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T
T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T}

macro_rules! tuple_impls {
    ($(
        ($($T:ident),+),
    )+) => {
        $(
            impl<$($T: GenerateRand),+> GenerateRand for ($($T,)+) {
                #[inline]
                fn generate<R: Random + ?Sized>(rand: &mut R) -> Self {
                    ($({ let x: $T = rand.gen(); x},)+)
                }
            }
        )+
    }
}

tuple_impls! {
    (A),
    (A, B),
    (A, B, C),
    (A, B, C, D),
    (A, B, C, D, E),
    (A, B, C, D, E, F),
    (A, B, C, D, E, F, G),
    (A, B, C, D, E, F, G, H),
    (A, B, C, D, E, F, G, H, I),
    (A, B, C, D, E, F, G, H, I, J),
    (A, B, C, D, E, F, G, H, I, J, K),
    (A, B, C, D, E, F, G, H, I, J, K, L),
    (A, B, C, D, E, F, G, H, I, J, K, L, M),
    (A, B, C, D, E, F, G, H, I, J, K, L, M, N),
    (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O),
    (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P),
}
