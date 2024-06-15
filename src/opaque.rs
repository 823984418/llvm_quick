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
pub unsafe trait Opaque {
    type Inner: ?Sized;

    unsafe fn try_from_ref<'s>(ptr: *mut Self::Inner) -> Option<&'s Self> {
        unsafe { std::mem::transmute_copy::<*mut Self::Inner, *const Self>(&ptr).as_ref() }
    }

    unsafe fn from_ref<'a>(ptr: *mut Self::Inner) -> &'a Self {
        unsafe { Self::try_from_ref(ptr).unwrap_unchecked() }
    }

    fn as_raw(&self) -> *mut Self::Inner {
        unsafe { std::mem::transmute_copy::<*const Self, *mut Self::Inner>(&(self as *const Self)) }
    }
}
