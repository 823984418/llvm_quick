use llvm_sys::core::*;

use crate::opaque::Opaque;
use crate::type_tag::{any, TypeTag};
use crate::{Use, Value};

impl<T: TypeTag> Value<T> {
    pub fn get_first_use(&self) -> Option<&Use> {
        unsafe { Use::try_from_ref(LLVMGetFirstUse(self.as_raw())) }
    }
}

impl Use {
    pub fn get_next_use(&self) -> Option<&Use> {
        unsafe { Use::try_from_ref(LLVMGetNextUse(self.as_raw())) }
    }

    pub fn get_user(&self) -> &Value<any> {
        unsafe { Value::from_ref(LLVMGetUser(self.as_raw())) }
    }

    pub fn get_used_value(&self) -> &Value<any> {
        unsafe { Value::from_ref(LLVMGetUsedValue(self.as_raw())) }
    }
}
