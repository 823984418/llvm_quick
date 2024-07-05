use llvm_sys::core::*;

use crate::opaque::Opaque;
use crate::type_tag::*;
use crate::*;

impl<T: TypeTag> Value<T> {
    pub fn get_first_use(&self) -> Option<&Use> {
        unsafe { Use::from_ptr(LLVMGetFirstUse(self.as_raw())) }
    }
}

impl Use {
    pub fn get_next_use(&self) -> Option<&Use> {
        unsafe { Use::from_ptr(LLVMGetNextUse(self.as_raw())) }
    }

    pub fn get_user(&self) -> &Value<any> {
        unsafe { Value::from_raw(LLVMGetUser(self.as_raw())) }
    }

    pub fn get_used_value(&self) -> &Value<any> {
        unsafe { Value::from_raw(LLVMGetUsedValue(self.as_raw())) }
    }
}
