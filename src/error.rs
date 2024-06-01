use std::borrow::Borrow;
use std::ffi::{c_char, CStr};
use std::fmt::{Debug, Formatter};
use std::mem::{forget, size_of, MaybeUninit};
use std::ops::Deref;
use std::ptr::NonNull;

use llvm_sys::error::*;
use llvm_sys::error_handling::*;

use crate::opaque::{Opaque, PhantomOpaque};
use crate::owning::Dispose;

pub fn install_fatal_error_handler<T: Fn(&CStr) + 'static>(handle: T) {
    assert_eq!(size_of::<T>(), 0, "Fatal error handler can't capture anything.");
    forget(handle);
    extern "C" fn handler_raw<T: Fn(&CStr) + 'static>(reason: *const c_char) {
        unsafe {
            let handle = MaybeUninit::<T>::uninit().assume_init();
            handle(CStr::from_ptr(reason));
            forget(handle);
        }
    }
    install_fatal_error_handler_raw(Some(handler_raw::<T>));
}

pub fn install_fatal_error_handler_none() {
    install_fatal_error_handler_raw(None);
}

/// Install a fatal error handler.
///
/// LLVM will call `exit(1)` if it detects a fatal error. A callback
/// registered with this function will be invoked before the program is
/// exited.
pub fn install_fatal_error_handler_raw(handler: LLVMFatalErrorHandler) {
    unsafe { LLVMInstallFatalErrorHandler(handler) };
}

/// Reset fatal error handling to the default.
pub fn reset_fatal_error_handler() {
    unsafe { LLVMResetFatalErrorHandler() };
}

/// Enable LLVM's build-in stack trace code.
///
/// This intercepts the OS's crash signals and prints which component
/// of LLVM you were in at the time of the crash.
pub fn enable_pretty_stack_trace() {
    unsafe { LLVMEnablePrettyStackTrace() };
}

pub fn get_string_error_type_id() -> LLVMErrorTypeId {
    unsafe { LLVMGetStringErrorTypeId() }
}

pub struct Error {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for Error {
    type Inner = LLVMOpaqueError;
}

impl Dispose for Error {
    unsafe fn dispose(ptr: *mut Self::Inner) {
        unsafe { LLVMConsumeError(ptr) };
    }
}

impl Error {
    pub fn get_type_id(&self) -> LLVMErrorTypeId {
        unsafe { LLVMGetErrorTypeId(self.as_ptr()) }
    }

    pub fn get_message(&self) -> ErrorMessage {
        unsafe { ErrorMessage::from_raw(LLVMGetErrorMessage(self.as_ptr())) }
    }
}

pub struct ErrorMessage {
    ptr: NonNull<CStr>,
}

impl Drop for ErrorMessage {
    fn drop(&mut self) {
        unsafe { LLVMDisposeErrorMessage(self.as_ptr()) }
    }
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
