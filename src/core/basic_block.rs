use llvm_sys::core::*;
use llvm_sys::*;

use crate::core::values::Value;
use crate::opaque::{Opaque, PhantomOpaque};
use crate::type_tag::label;

#[repr(transparent)]
pub struct BasicBlock {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for BasicBlock {
    type Inner = LLVMBasicBlock;
}

impl BasicBlock {
    pub fn as_value(&self) -> &Value<label> {
        unsafe { Value::from_ref(LLVMBasicBlockAsValue(self.as_ptr())) }
    }
}