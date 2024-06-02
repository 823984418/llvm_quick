use std::ffi::{c_char, CStr};
use std::mem::{forget, size_of, MaybeUninit};

use llvm_sys::error_handling::*;

pub fn install_fatal_error_handler<T: Fn(&CStr) + 'static>(handle: T) {
    assert_eq!(
        size_of::<T>(),
        0,
        "Fatal error handler can't capture anything."
    );
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
