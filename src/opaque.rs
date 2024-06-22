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

    /// Try to create ref from nonnull raw pointer and check cond.
    unsafe fn try_from_raw<'r>(ptr: *mut Self::Inner) -> Option<&'r Self> {
        unsafe { Some(Self::from_raw(ptr)) }
    }

    /// Try to create ref from nonnull raw pointer but don't check cond.
    unsafe fn from_raw<'r>(ptr: *mut Self::Inner) -> &'r Self {
        debug_assert!(!ptr.is_null());
        unsafe { &*(ptr as *const Self) }
    }

    /// Try to create ref from nullable raw pointer.
    unsafe fn from_ptr<'r>(ptr: *mut Self::Inner) -> Option<&'r Self> {
        if ptr.is_null() {
            None
        } else {
            unsafe { Some(Self::from_raw(ptr)) }
        }
    }

    /// Get raw pointer.
    fn as_raw(&self) -> *mut Self::Inner {
        self as *const Self as *mut Self::Inner
    }

    /// Cast to target but don't check cond.
    unsafe fn cast_unchecked<T: Opaque<Inner = Self::Inner>>(&self) -> &T {
        T::from_raw(self.as_raw())
    }

    fn cast<T: Opaque<Inner = Self::Inner>>(&self) -> &T {
        Self::try_cast(self).unwrap()
    }

    /// Cast to target and check cond.
    fn try_cast<T: Opaque<Inner = Self::Inner>>(&self) -> Option<&T> {
        unsafe { T::try_from_raw(self.as_raw()) }
    }
}
