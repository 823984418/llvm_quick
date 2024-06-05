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

// TODO
