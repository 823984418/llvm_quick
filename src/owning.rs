use std::fmt::{Debug, Formatter};
use std::mem::forget;
use std::ops::Deref;
use std::ptr::{null_mut, NonNull};

use crate::Opaque;

pub trait OpaqueDrop {
    unsafe fn drop_raw(ptr: *mut Self);
}

pub trait OpaqueClone {
    unsafe fn clone_raw(ptr: *mut Self) -> *mut Self;
}

pub struct Owning<T: Opaque<Inner: OpaqueDrop>> {
    ptr: NonNull<T>,
}

impl<T: Opaque<Inner: OpaqueDrop + OpaqueClone>> Clone for Owning<T> {
    fn clone(&self) -> Self {
        unsafe { Self::from_raw(T::Inner::clone_raw(self.as_raw())) }
    }
}

impl<T: Opaque<Inner: OpaqueDrop>> Drop for Owning<T> {
    fn drop(&mut self) {
        unsafe { T::Inner::drop_raw(self.as_raw()) }
    }
}

impl<T: Opaque<Inner: OpaqueDrop>> Deref for Owning<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T: Opaque<Inner: OpaqueDrop> + Debug> Debug for Owning<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self.deref(), f)
    }
}

impl<T: Opaque<Inner: OpaqueDrop>> Owning<T> {
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
