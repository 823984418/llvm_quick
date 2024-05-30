use std::cell::UnsafeCell;
use std::marker::PhantomPinned;

#[repr(transparent)]
pub struct PhantomOpaque {
    pinned: PhantomPinned,
    unfreeze: UnsafeCell<()>,
}

pub unsafe trait Opaque: Sized {
    type Inner;

    unsafe fn from_ref<'s>(ptr: *mut Self::Inner) -> &'s Self {
        unsafe { &*(ptr as *mut Self) }
    }

    fn as_ptr(&self) -> *mut Self::Inner {
        self as *const Self as *mut Self::Inner
    }
}
