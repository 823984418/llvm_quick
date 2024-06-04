use llvm_sys::remarks::*;

use crate::opaque::{Opaque, PhantomOpaque};

#[repr(transparent)]
pub struct RemarkString {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for RemarkString {
    type Inner = LLVMRemarkOpaqueString;
}

// TODO
