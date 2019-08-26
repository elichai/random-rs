#![no_std]

#[cfg(feature = "std")]
extern crate std;

pub extern crate random_trait;
pub use random_trait::Random;

use core::mem;

#[cfg(feature = "doc-comment")]
extern crate doc_comment;
#[cfg(feature = "doc-comment")]
doc_comment::doctest!("../README.md");

const PCG_DEFAULT_MULTIPLIER_64: u64 = 6_364_136_223_846_793_005;

// Pcg32
pub struct FastRng {
    state: u64,
    inc: u64,
}

impl FastRng {
    #[cfg(feature = "std")]
    pub fn new() -> Self {
        let (a, b) = time_seed();
        Self::seed(a, b)
    }

    pub fn seed(seed: u64, seq: u64) -> Self {
        let init_inc = (seq << 1) | 1;
        let init_state = seed + init_inc;
        let mut rng = FastRng { state: init_state, inc: init_inc };
        rng.state = rng.state * PCG_DEFAULT_MULTIPLIER_64 + rng.inc;
        rng
    }

    fn gen_u32(&mut self) -> u32 {
        let old_state = self.state;
        self.state = self.state * PCG_DEFAULT_MULTIPLIER_64 + self.inc;

        let xorshift = (((old_state >> 18) ^ old_state) >> 27) as u32;
        let rot = (old_state >> 59) as i32;
        (xorshift >> rot) | (xorshift << ((-rot) & 31))
    }
}

#[cfg(feature = "std")]
fn time_seed() -> (u64, u64) {
    use std::time;
    let now = time::SystemTime::now();
    let unix = now.duration_since(time::UNIX_EPOCH).unwrap();

    (unix.as_secs(), unix.subsec_nanos() as u64)
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
