#![no_std]

use core::mem;
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

    // TODO: check compiler version.
    //    fn get_u128(&mut self) -> u128 {
    //        let mut buf = [0u8; 16];
    //        self.fill_bytes(&mut buf);
    //        unsafe { mem::transmute(buf) }
    //    }
}
