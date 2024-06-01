use llvm_sys::LLVMOpaqueMetadata;

use crate::opaque::{Opaque, PhantomOpaque};

pub struct Metadata {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for Metadata {
    type Inner = LLVMOpaqueMetadata;
}
