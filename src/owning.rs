use std::fmt::{Debug, Formatter};
use std::mem::forget;
use std::ops::Deref;
use std::ptr::{null_mut, NonNull};

use crate::Opaque;

pub trait OpaqueDrop: Opaque {
    fn drop_raw(ptr: *mut Self::Inner);
}

pub trait OpaqueClone: OpaqueDrop {
    fn clone_raw(ptr: *mut Self::Inner) -> *mut Self::Inner;
}

pub struct Owning<T: OpaqueDrop> {
    ptr: NonNull<T>,
}

impl<T: OpaqueClone> Clone for Owning<T> {
    fn clone(&self) -> Self {
        unsafe { Self::from_raw(T::clone_raw(self.as_raw())) }
    }
}

impl<T: OpaqueDrop> Drop for Owning<T> {
    fn drop(&mut self) {
        T::drop_raw(self.as_raw());
    }
}

impl<T: OpaqueDrop> Deref for Owning<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T: OpaqueDrop + Debug> Debug for Owning<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self.deref(), f)
    }
}

impl<T: OpaqueDrop> Owning<T> {
    pub unsafe fn try_from_raw(ptr: *mut T::Inner) -> Option<Self> {
        NonNull::new(ptr as *mut T).map(|ptr| Self { ptr })
    }

    pub unsafe fn from_raw(ptr: *mut T::Inner) -> Self {
        unsafe { Self::try_from_raw(ptr).unwrap() }
    }

    pub unsafe fn into_raw(self) -> *mut T::Inner {
        let ptr = self.as_raw();
        forget(self);
        ptr
    }

    pub unsafe fn option_into_raw(this: Option<Self>) -> *mut T::Inner {
        this.map(|x| unsafe { x.into_raw() }).unwrap_or(null_mut())
    }
}
