use llvm_sys::core::*;

use crate::opaque::Opaque;
use crate::owning::OpaqueDrop;
use crate::type_tag::{PtrTypeTag, TypeTag};
use crate::{Type, Value, ValueMetadataEntry};

impl<T: TypeTag> Type<T> {
    pub fn const_null(&self) -> &Value<T> {
        unsafe { Value::from_ref(LLVMConstNull(self.as_raw())) }
    }

    pub fn const_all_ones(&self) -> &Value<T> {
        unsafe { Value::from_ref(LLVMConstAllOnes(self.as_raw())) }
    }

    pub fn get_undef(&self) -> &Value<T> {
        unsafe { Value::from_ref(LLVMGetUndef(self.as_raw())) }
    }

    pub fn get_poison(&self) -> &Value<T> {
        unsafe { Value::from_ref(LLVMGetPoison(self.as_raw())) }
    }
}

impl<T: TypeTag> Value<T> {
    pub fn is_null(&self) -> bool {
        unsafe { LLVMIsNull(self.as_raw()) != 0 }
    }
}

impl<T: PtrTypeTag> Type<T> {
    pub fn const_pointer_null(&self) -> &Value<T> {
        unsafe { Value::from_ref(LLVMConstPointerNull(self.as_raw())) }
    }
}

// TODO

impl<'s> OpaqueDrop for [&'s ValueMetadataEntry] {
    unsafe fn drop_raw(ptr: *mut Self) {
        unsafe { LLVMDisposeValueMetadataEntries(ptr as _) }
    }
}

// TODO
