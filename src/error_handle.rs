use std::ffi::{c_char, CStr};

use llvm_sys::error_handling::*;

pub trait FatalErrorHandler {
    fn on_fatal_error(reason: &CStr);
}

pub fn install_fatal_error_handler<T: FatalErrorHandler>() {
    extern "C" fn handler_raw<T: FatalErrorHandler>(reason: *const c_char) {
        unsafe { T::on_fatal_error(CStr::from_ptr(reason)) };
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
