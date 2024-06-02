use llvm_sys::*;

use crate::opaque::{Opaque, PhantomOpaque};

#[repr(transparent)]
pub struct Metadata {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for Metadata {
    type Inner = LLVMOpaqueMetadata;
}

// TODO: more
