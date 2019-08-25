#![no_std]

use core::{char, mem};

pub enum Error {
    Something,
}

pub trait GenerateRand {
    fn generate<R: Random + ?Sized>(rand: &mut R) -> Self;
}

pub trait Random {
    fn try_fill_bytes(&mut self, buf: &mut [u8]) -> Result<(), Error>;

    fn fill_bytes(&mut self, buf: &mut [u8]) {
        if self.try_fill_bytes(buf).is_err() {
            panic!("Failed getting randmness");
        }
    }

    fn gen<T: GenerateRand>(&mut self) -> T {
        T::generate(self)
    }

    fn get_u8(&mut self) -> u8 {
        let mut buf = [0u8; 1];
        self.fill_bytes(&mut buf);
        buf[0]
    }

    fn get_u16(&mut self) -> u16 {
        let mut buf = [0u8; 2];
        self.fill_bytes(&mut buf);
        unsafe { mem::transmute(buf) }
    }

    fn get_u32(&mut self) -> u32 {
        let mut buf = [0u8; 4];
        self.fill_bytes(&mut buf);
        unsafe { mem::transmute(buf) }
    }

    fn get_u64(&mut self) -> u64 {
        let mut buf = [0u8; 8];
        self.fill_bytes(&mut buf);
        unsafe { mem::transmute(buf) }
    }

    #[cfg(feature = "u128")]
    fn get_u128(&mut self) -> u128 {
        let mut buf = [0u8; 16];
        self.fill_bytes(&mut buf);
        unsafe { mem::transmute(buf) }
    }

    // TODO: More research, least/most significant bit?
    fn get_bool(&mut self) -> bool {
        let bit = self.get_u8() & 0b1000_0000;
        debug_assert!(bit < 2);
        bit == 1
    }
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
    }
}


impl GenerateRand for u8 {
    fn generate<R: Random + ?Sized>(rand: &mut R) -> Self {
        rand.get_u8()
    }
}

impl GenerateRand for u16 {
    fn generate<R: Random + ?Sized>(rand: &mut R) -> Self {
        rand.get_u16()
    }
}

impl GenerateRand for u32 {
    fn generate<R: Random + ?Sized>(rand: &mut R) -> Self {
        rand.get_u32()
    }
}

impl GenerateRand for u64 {
    fn generate<R: Random + ?Sized>(rand: &mut R) -> Self {
        rand.get_u64()
    }
}

#[cfg(feature = "u128")]
impl GenerateRand for u128 {
    fn generate<R: Random + ?Sized>(rand: &mut R) -> Self {
        rand.get_u128()
    }
}

impl GenerateRand for char {
    fn generate<R: Random + ?Sized>(rand: &mut R) -> Self {
        loop {
            if let Some(c) = char::from_u32(rand.get_u32()) {
                return c;
            }
        }
    }
}

impl GenerateRand for bool {
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
            if exponent < -1074i32 { // emin(-1022)-p(53)+1  (https://en.wikipedia.org/wiki/IEEE_754)
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
        significand as f64 * f64::from(1 << exponent)
    }
}

// Source: https://mumble.net/~campbell/2014/04/28/uniform-random-float
// https://mumble.net/~campbell/2014/04/28/random_real.c
impl GenerateRand for f32 {
    fn generate<R: Random + ?Sized>(rand: &mut R) -> Self {
        let mut exponent: i16  = -32;
        let mut significand = rand.get_u32();
        while significand == 0 {
            exponent -= 32;
            if exponent < -149i16 { // emin(-126)-p(24)+1  (https://en.wikipedia.org/wiki/IEEE_754)
                // In reallity this should probably never happen. prob of ~1/(2^1024) unless randomness is broken.
                unreachable!("The randomness is broken, got 0 5 times. (prob of 1/2^160)");
                // TODO: Should this stay unreachable or change to return 0?
            }
            significand = rand.get_u32();
        }

        // Shift the leading zeros into the exponent
        let shift = significand.leading_zeros() as i16;
        if shift > 0 {
            exponent -= shift;
            significand <<= shift;
            significand |= rand.get_u32() >> (32 - shift);
        }
        // Set the sticky bit, almost definitely another 1 in the random stream.
        significand |= 1;

        // Convert to float and scale by 2^exponent.
        significand as f32 * f32::from(1i16 << exponent)
    }
}


impl_generate_rand_ifromu!{i8, u8}
impl_generate_rand_ifromu!{i16, u16}
impl_generate_rand_ifromu!{i32, u32}
impl_generate_rand_ifromu!{i64, u64}
#[cfg(feature = "u128")]
impl_generate_rand_ifromu!{i128, u128}
