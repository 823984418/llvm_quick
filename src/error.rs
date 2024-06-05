use std::borrow::Borrow;
use std::ffi::{c_char, CStr};
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::ptr::NonNull;

use llvm_sys::error::*;

use crate::owning::{OpaqueDrop, Owning};
use crate::{Opaque, PhantomOpaque};

pub fn string_error_type_id() -> LLVMErrorTypeId {
    unsafe { LLVMGetStringErrorTypeId() }
}

#[repr(transparent)]
pub struct Error {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for Error {
    type Inner = LLVMOpaqueError;
}

impl Error {
    pub unsafe fn check(ptr: *mut LLVMOpaqueError) -> Result<(), Owning<Self>> {
        if let Some(e) = unsafe { Owning::try_from_raw(ptr) } {
            Err(e)
        } else {
            Ok(())
        }
    }
}

impl Error {
    pub fn get_type_id(&self) -> LLVMErrorTypeId {
        unsafe { LLVMGetErrorTypeId(self.as_raw()) }
    }
}

impl OpaqueDrop for Error {
    fn drop_raw(ptr: *mut Self::Inner) {
        unsafe { LLVMConsumeError(ptr) };
    }
}

impl Error {
    pub fn get_message(&self) -> ErrorMessage {
        unsafe { ErrorMessage::from_raw(LLVMGetErrorMessage(self.as_raw())) }
    }
}

impl Drop for ErrorMessage {
    fn drop(&mut self) {
        unsafe { LLVMDisposeErrorMessage(self.as_ptr()) }
    }
}

impl Error {
    pub fn create_string_error(err_msg: &CStr) -> Owning<Self> {
        unsafe { Owning::from_raw(LLVMCreateStringError(err_msg.as_ptr())) }
    }
}

pub struct ErrorMessage {
    ptr: NonNull<CStr>,
}

impl Debug for ErrorMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.deref(), f)
    }
}

impl AsRef<CStr> for ErrorMessage {
    fn as_ref(&self) -> &CStr {
        self.deref()
    }
}

impl Borrow<CStr> for ErrorMessage {
    fn borrow(&self) -> &CStr {
        self.deref()
    }
}

impl Deref for ErrorMessage {
    type Target = CStr;

    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}

impl ErrorMessage {
    pub unsafe fn from_raw(ptr: *mut c_char) -> Self {
        let ptr = unsafe { CStr::from_ptr(ptr).into() };
        Self { ptr }
    }

    pub fn as_ptr(&self) -> *mut c_char {
        self.ptr.as_ptr() as _
    }
}
