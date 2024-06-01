use std::cell::UnsafeCell;
use std::marker::PhantomPinned;

/// Zero-sized type used to mark things that "act like" an opaque type.
/// Adding a PhantomOpaque field to your type tells the compiler that don't optimize pointer access.
#[repr(transparent)]
pub struct PhantomOpaque {
    pinned: PhantomPinned,
    unfreeze: UnsafeCell<()>,
}

/// Mark an opaque type that associates with a corresponding inner type.
pub unsafe trait Opaque: Sized {
    type Inner;

    unsafe fn from_ref<'s>(ptr: *mut Self::Inner) -> &'s Self {
        unsafe { &*(ptr as *mut Self) }
    }

    fn as_ptr(&self) -> *mut Self::Inner {
        self as *const Self as *mut Self::Inner
    }
}
