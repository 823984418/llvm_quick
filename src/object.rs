use llvm_sys::object::*;

use crate::opaque::{Opaque, PhantomOpaque};

#[repr(transparent)]
pub struct SectionIterator {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for SectionIterator {
    type Inner = LLVMOpaqueSectionIterator;
}

// TODO
