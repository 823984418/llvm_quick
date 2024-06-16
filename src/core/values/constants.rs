use std::ffi::CStr;

use llvm_sys::core::*;

use crate::opaque::Opaque;
use crate::type_tag::{IntTypeTag, PtrTypeTag, TypeTag};
use crate::{Constant, Module, Type, Value, ValueMetadataEntries};

impl<T: TypeTag> Type<T> {
    pub fn const_null(&self) -> &Constant<T> {
        unsafe { Constant::from_raw(LLVMConstNull(self.as_raw())) }
    }

    pub fn const_all_ones(&self) -> &Constant<T> {
        unsafe { Constant::from_raw(LLVMConstAllOnes(self.as_raw())) }
    }

    pub fn get_undef(&self) -> &Constant<T> {
        unsafe { Constant::from_raw(LLVMGetUndef(self.as_raw())) }
    }

    pub fn get_poison(&self) -> &Constant<T> {
        unsafe { Constant::from_raw(LLVMGetPoison(self.as_raw())) }
    }
}

impl<T: TypeTag> Value<T> {
    pub fn is_null(&self) -> bool {
        unsafe { LLVMIsNull(self.as_raw()) != 0 }
    }
}

impl<T: PtrTypeTag> Type<T> {
    pub fn const_pointer_null(&self) -> &Constant<T> {
        unsafe { Constant::from_raw(LLVMConstPointerNull(self.as_raw())) }
    }
}

impl<T: IntTypeTag> Type<T> {
    pub fn const_int(&self, n: u64, sign_extend: bool) -> &Constant<T> {
        unsafe { Constant::from_raw(LLVMConstInt(self.as_raw(), n, sign_extend as _)) }
    }
}

// TODO

impl<'s> Drop for ValueMetadataEntries<'s> {
    fn drop(&mut self) {
        unsafe { LLVMDisposeValueMetadataEntries(self.as_raw()) }
    }
}

// TODO

impl<'s> Module<'s> {
    pub fn add_alias<T: TypeTag>(
        &self,
        value_type: &Type<T>,
        addr_space: u32,
        aliasee: &Value<T>,
        name: &CStr,
    ) -> &Value<T> {
        unsafe {
            Value::from_raw(LLVMAddAlias2(
                self.as_raw(),
                value_type.as_raw(),
                addr_space,
                aliasee.as_raw(),
                name.as_ptr(),
            ))
        }
    }
}
