use llvm_sys::core::*;
use llvm_sys::*;

use crate::core::type_tag::TypeTag;
use crate::core::values::Value;
use crate::opaque::{Opaque, PhantomOpaque};

#[repr(transparent)]
pub struct OperandBundle {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for OperandBundle {
    type Inner = LLVMOpaqueOperandBundle;
}

#[repr(transparent)]
pub struct Metadata {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for Metadata {
    type Inner = LLVMOpaqueMetadata;
}

// TODO: more

impl<T: TypeTag> Value<T> {
    pub fn as_metadata(&self) -> &Metadata {
        unsafe { Metadata::from_ref(LLVMValueAsMetadata(self.as_raw())) }
    }
}
