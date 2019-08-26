use FastRng;

use core::ops::{Deref, DerefMut};

pub struct ThreadFastRng(*mut FastRng);

impl Deref for ThreadFastRng {
    type Target = FastRng;

    fn deref(&self) -> &Self::Target {
        unsafe {&*self.0}
    }
}

impl DerefMut for ThreadFastRng {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {&mut *self.0}
    }
}


pub trait FromRawPtr<T> {
    fn from_ptr(ptr: *mut T) -> Self;
}


impl FromRawPtr<FastRng> for ThreadFastRng {
    fn from_ptr(ptr: *mut FastRng) -> ThreadFastRng {
        ThreadFastRng(ptr)
    }
}