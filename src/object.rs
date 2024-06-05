use llvm_sys::object::*;

use crate::{Opaque, PhantomOpaque};

#[repr(transparent)]
pub struct SectionIterator {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for SectionIterator {
    type Inner = LLVMOpaqueSectionIterator;
}

#[repr(transparent)]
pub struct SymbolIterator {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for SymbolIterator {
    type Inner = LLVMOpaqueSymbolIterator;
}

#[repr(transparent)]
pub struct RelocationIterator {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for RelocationIterator {
    type Inner = LLVMOpaqueRelocationIterator;
}

#[repr(transparent)]
pub struct Binary {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for Binary {
    type Inner = LLVMOpaqueBinary;
}

// TODO
