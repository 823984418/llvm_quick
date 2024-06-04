use llvm_sys::lto::*;

use crate::opaque::{Opaque, PhantomOpaque};

#[repr(transparent)]
pub struct LTOModule {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for LTOModule {
    type Inner = LLVMOpaqueLTOModule;
}

// TODO
