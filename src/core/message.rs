use std::borrow::Borrow;
use std::ffi::{c_char, CStr};
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::ptr::NonNull;

use llvm_sys::core::*;

pub struct Message {
    ptr: NonNull<CStr>,
}

impl Drop for Message {
    fn drop(&mut self) {
        unsafe { LLVMDisposeMessage(self.as_ptr()) }
    }
}

impl Debug for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.deref(), f)
    }
}

impl AsRef<CStr> for Message {
    fn as_ref(&self) -> &CStr {
        self.deref()
    }
}

impl Borrow<CStr> for Message {
    fn borrow(&self) -> &CStr {
        self.deref()
    }
}

impl Deref for Message {
    type Target = CStr;

    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}

impl Message {
    pub unsafe fn try_from_raw(ptr: *mut c_char) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            let ptr = unsafe { CStr::from_ptr(ptr).into() };
            Some(Self { ptr })
        }
    }

    pub unsafe fn from_raw(ptr: *mut c_char) -> Self {
        debug_assert!(!ptr.is_null());
        let ptr = unsafe { CStr::from_ptr(ptr).into() };
        Self { ptr }
    }

    pub fn create(s: &CStr) -> Self {
        unsafe { Self::from_raw(LLVMCreateMessage(s.as_ptr())) }
    }

    pub fn as_ptr(&self) -> *mut c_char {
        self.ptr.as_ptr() as _
    }
}
