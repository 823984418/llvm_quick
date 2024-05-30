use llvm_sys::core::LLVMBasicBlockAsValue;
use llvm_sys::LLVMBasicBlock;

use crate::opaque::{Opaque, PhantomOpaque};
use crate::type_tag::any;
use crate::values::Value;

#[repr(transparent)]
pub struct BasicBlock {
    opaque: PhantomOpaque,
}

unsafe impl Opaque for BasicBlock {
    type Inner = LLVMBasicBlock;
}

impl BasicBlock {
    // TODO: any?
    pub fn as_value(&self) -> &Value<any> {
        unsafe { Value::from_ref(LLVMBasicBlockAsValue(self.as_ptr())) }
    }
}
