use std::ffi::CString;
use std::fmt::{Debug, Formatter};

use llvm_sys::core::*;
use llvm_sys::*;

use crate::core::type_tag::{any, InstanceTypeTag, TypeTag};
use crate::core::Message;
use crate::Opaque;
use crate::{Type, Value};

impl<T: TypeTag> Debug for Value<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.print_to_string().to_str().unwrap())
    }
}

impl<T: TypeTag> Value<T> {
    pub unsafe fn cast_unchecked<N: TypeTag>(&self) -> &Value<N> {
        unsafe { Value::from_ref(self.as_raw()) }
    }

    pub fn try_cast<N: TypeTag>(&self) -> Option<&Value<N>> {
        if self.get_type().try_cast::<N>().is_some() {
            Some(unsafe { self.cast_unchecked() })
        } else {
            None
        }
    }

    pub fn cast<N: TypeTag>(&self) -> &Value<N> {
        self.try_cast().unwrap()
    }

    pub fn to_any(&self) -> &Value<any> {
        unsafe { self.cast_unchecked() }
    }

    pub fn get_kind(&self) -> LLVMValueKind {
        unsafe { LLVMGetValueKind(self.as_raw()) }
    }

    /// Obtain the type of a value.
    pub fn get_type(&self) -> &Type<T> {
        unsafe { Type::from_ref(LLVMTypeOf(self.as_raw())) }
    }

    /// Obtain the string name of a value.
    pub fn get_name(&self) -> *const [u8] {
        unsafe {
            let mut len = 0;
            let s = LLVMGetValueName2(self.as_raw(), &mut len);
            std::ptr::slice_from_raw_parts(s.cast(), len)
        }
    }

    /// Obtain the string name of a value.
    pub fn get_name_string(&self) -> CString {
        unsafe { CString::new(&*self.get_name()).unwrap() }
    }

    /// Set the string name of a value.
    pub fn set_name(&self, name: &[u8]) {
        unsafe { LLVMSetValueName2(self.as_raw(), name.as_ptr() as _, name.len()) }
    }

    pub fn dump(&self) {
        unsafe { LLVMDumpValue(self.as_raw()) };
    }

    /// Return a string representation of the value.
    pub fn print_to_string(&self) -> Message {
        unsafe { Message::from_raw(LLVMPrintValueToString(self.as_raw())) }
    }

    pub fn replace_all_uses_with(&self, new: &Self)
    where
        T: InstanceTypeTag,
    {
        unsafe { LLVMReplaceAllUsesWith(self.as_raw(), new.as_raw()) };
    }

    /// Determine whether the specified value instance is constant.
    pub fn is_constant(&self) -> bool {
        unsafe { LLVMIsConstant(self.as_raw()) != 0 }
    }

    /// Determine whether a value instance is undefined.
    pub fn is_undef(&self) -> bool {
        unsafe { LLVMIsUndef(self.as_raw()) != 0 }
    }

    /// Determine whether a value instance is poisonous.
    pub fn is_poison(&self) -> bool {
        unsafe { LLVMIsPoison(self.as_raw()) != 0 }
    }

    pub fn is_a_metadata_node(&self) -> &Self {
        unsafe { Value::from_ref(LLVMIsAMDNode(self.as_raw())) }
    }

    pub fn is_a_value_as_metadata(&self) -> &Self {
        unsafe { Value::from_ref(LLVMIsAValueAsMetadata(self.as_raw())) }
    }

    pub fn is_a_metadata_string(&self) -> &Self {
        unsafe { Value::from_ref(LLVMIsAMDString(self.as_raw())) }
    }
}
