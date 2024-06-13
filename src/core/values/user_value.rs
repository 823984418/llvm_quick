use llvm_sys::core::*;

use crate::opaque::Opaque;
use crate::type_tag::{any, TypeTag};
use crate::{Use, Value};

impl<T: TypeTag> Value<T> {
    pub fn get_operand(&self, index: u32) -> Option<&Value<any>> {
        unsafe { Value::try_from_ref(LLVMGetOperand(self.as_raw(), index)) }
    }

    pub fn get_operand_use(&self, index: u32) -> Option<&Use> {
        unsafe { Use::try_from_ref(LLVMGetOperandUse(self.as_raw(), index)) }
    }

    pub fn set_operand<O: TypeTag>(&self, index: u32, val: &Value<O>) {
        unsafe { LLVMSetOperand(self.as_raw(), index, val.as_raw()) }
    }

    pub fn get_num_operands(&self) -> i32 {
        unsafe { LLVMGetNumOperands(self.as_raw()) }
    }
}
