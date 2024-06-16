use llvm_sys::core::*;

use crate::opaque::Opaque;
use crate::type_tag::{PtrTypeTag, TypeTag};
use crate::{Constant, Type, Value, ValueMetadataEntries};

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

// TODO

impl<'s> Drop for ValueMetadataEntries<'s> {
    fn drop(&mut self) {
        unsafe { LLVMDisposeValueMetadataEntries(self.as_raw()) }
    }
}

// TODO
