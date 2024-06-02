use std::fmt::{Debug, Formatter};
use std::mem::forget;
use std::ops::Deref;
use std::ptr::{null_mut, NonNull};

use crate::opaque::Opaque;

pub trait Dispose: Opaque {
    unsafe fn dispose(ptr: *mut Self::Inner);
}

pub struct Owning<T: Dispose> {
    ptr: NonNull<T>,
}

impl<T: Dispose> Drop for Owning<T> {
    fn drop(&mut self) {
        unsafe { T::dispose(self.as_ptr()) };
    }
}

impl<T: Dispose> Deref for Owning<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T: Dispose + Debug> Debug for Owning<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self.deref(), f)
    }
}

impl<T: Dispose> Owning<T> {
    pub unsafe fn try_from_raw(ptr: *mut T::Inner) -> Option<Self> {
        NonNull::new(ptr as *mut T).map(|ptr| Self { ptr })
    }

    pub unsafe fn from_raw(ptr: *mut T::Inner) -> Self {
        unsafe { Self::try_from_raw(ptr).unwrap() }
    }

    pub unsafe fn into_raw(self) -> *mut T::Inner {
        let ptr = self.as_ptr();
        forget(self);
        ptr
    }

    pub unsafe fn option_into_raw(this: Option<Self>) -> *mut T::Inner {
        this.map(|x| unsafe { x.into_raw() }).unwrap_or(null_mut())
    }
}
