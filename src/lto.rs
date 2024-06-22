use std::ffi::CStr;

use llvm_sys::lto::*;

use crate::{Opaque, PhantomOpaque};

#[repr(transparent)]
pub struct LTOModule {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for LTOModule {
    type Inner = LLVMOpaqueLTOModule;
}

#[repr(transparent)]
pub struct LTOCodeGenerator {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for LTOCodeGenerator {
    type Inner = LLVMOpaqueLTOCodeGenerator;
}

#[repr(transparent)]
pub struct ThinLTOCodeGenerator {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for ThinLTOCodeGenerator {
    type Inner = LLVMOpaqueThinLTOCodeGenerator;
}

#[repr(transparent)]
pub struct LTOInput {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for LTOInput {
    type Inner = LLVMOpaqueLTOInput;
}

pub fn get_version() -> &'static CStr {
    unsafe { CStr::from_ptr(lto_get_version()) }
}

pub fn get_error_message() -> &'static CStr {
    unsafe { CStr::from_ptr(lto_get_error_message()) }
}

pub fn module_is_object_file(path: &CStr) -> bool {
    unsafe { lto_module_is_object_file(path.as_ptr()) != 0 }
}

// TODO
