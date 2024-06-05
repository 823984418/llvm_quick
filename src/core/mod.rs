use std::borrow::Borrow;
use std::ffi::{c_char, CStr};
use std::fmt::{Debug, Formatter};
use std::mem::forget;
use std::ops::Deref;
use std::ptr::NonNull;

use llvm_sys::core::*;

pub mod attribute;
pub mod basic_block;
pub mod builder;
pub mod context;
pub mod memory_buffer;
pub mod metadata;
pub mod module;
pub mod module_provider;
pub mod pass_manager;
pub mod thread;
pub mod type_tag;
pub mod types;
pub mod values;

/// Return the major, minor, and patch version of LLVM.
pub fn get_version() -> (u32, u32, u32) {
    let mut r = (0, 0, 0);
    unsafe { LLVMGetVersion(&mut r.0, &mut r.1, &mut r.2) };
    r
}

/// Deallocate and destroy all ManagedStatic variables.
pub unsafe fn shutdown() {
    unsafe { LLVMShutdown() }
}

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

    pub fn into_raw(self) -> *mut c_char {
        let ptr = self.as_ptr();
        forget(self);
        ptr
    }
}
