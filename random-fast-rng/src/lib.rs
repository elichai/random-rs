#![no_std]

#[cfg(feature = "std")]
#[macro_use] extern crate std;

#[cfg(feature = "std")] mod thread;

pub extern crate random_trait;
pub use random_trait::Random;

#[cfg(feature = "std")] use thread::FromRawPtr;
#[cfg(feature = "std")] pub use thread::ThreadFastRng;

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

    #[cfg(feature = "std")]
    pub fn thread_local() -> ThreadFastRng {
        use std::cell::RefCell;
        thread_local! {
            pub static THREAD_FAST_RNG: RefCell<FastRng> = RefCell::new(FastRng::new());
        }
        let ptr = THREAD_FAST_RNG.with(|r| r.as_ptr());
        ThreadFastRng::from_ptr(ptr)
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
        let mut local_rng = FastRng::thread_local();
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
