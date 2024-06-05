use crate::{Opaque, PhantomOpaque};
use llvm_sys::remarks::*;

#[repr(transparent)]
pub struct RemarkString {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for RemarkString {
    type Inner = LLVMRemarkOpaqueString;
}

#[repr(transparent)]
pub struct RemarkDebugLoc {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for RemarkDebugLoc {
    type Inner = LLVMRemarkOpaqueDebugLoc;
}

#[repr(transparent)]
pub struct RemarkArg {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for RemarkArg {
    type Inner = LLVMRemarkOpaqueArg;
}

#[repr(transparent)]
pub struct RemarkEntry {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for RemarkEntry {
    type Inner = LLVMRemarkOpaqueEntry;
}

#[repr(transparent)]
pub struct RemarkParser {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for RemarkParser {
    type Inner = LLVMRemarkOpaqueParser;
}

// TODO
