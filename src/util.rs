use std::ffi::{c_char, CStr, CString};

use llvm_sys::LLVMCallConv;

pub unsafe fn c_str<'s>(s: *const c_char) -> Option<&'s CStr> {
    if s.is_null() {
        None
    } else {
        Some(unsafe { CStr::from_ptr(s) })
    }
}

pub unsafe fn c_string(s: *const c_char) -> Option<CString> {
    unsafe { c_str(s) }.map(CString::from)
}

pub const fn llvm_call_conv_from_u32(v: u32) -> LLVMCallConv {
    if v > LLVMCallConv::LLVMAMDGPUESCallConv as _ {
        panic!();
    }
    unsafe { std::mem::transmute(v) }
}
