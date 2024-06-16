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

    unsafe fn try_from_raw<'a>(ptr: *mut Self::Inner) -> Option<&'a Self> {
        unsafe { Some(Self::from_raw(ptr)) }
    }

    unsafe fn from_raw<'a>(ptr: *mut Self::Inner) -> &'a Self {
        debug_assert!(!ptr.is_null());
        unsafe { &*(ptr as *const Self) }
    }

    unsafe fn from_ptr<'s>(ptr: *mut Self::Inner) -> Option<&'s Self> {
        if ptr.is_null() {
            None
        } else {
            unsafe { Some(Self::from_raw(ptr)) }
        }
    }

    fn as_raw(&self) -> *mut Self::Inner {
        self as *const Self as *mut Self::Inner
    }

    unsafe fn cast_unchecked<T: Opaque<Inner = Self::Inner>>(&self) -> &T {
        T::from_raw(self.as_raw())
    }

    fn cast<T: Opaque<Inner = Self::Inner>>(&self) -> Option<&T> {
        unsafe { T::try_from_raw(self.as_raw()) }
    }

    unsafe fn cast_check<T: Opaque<Inner = Self::Inner>, F: FnOnce(&Self) -> bool>(
        &self,
        f: F,
    ) -> Option<&T> {
        if f(self) {
            unsafe { Some(self.cast_unchecked()) }
        } else {
            None
        }
    }
}
