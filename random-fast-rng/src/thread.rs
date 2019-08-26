use FastRng;

use core::ops::{Deref, DerefMut};

/// A shim that points to the global `FastRng` instance. isn't safe for multi-threading.
///
/// This struct is created by [`thread_local()`](../struct.FastRng.html#method.thread_local)
pub struct ThreadFastRng(*mut FastRng);

impl Deref for ThreadFastRng {
    type Target = FastRng;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}

impl DerefMut for ThreadFastRng {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.0 }
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
