use std::ffi::{CStr, CString};
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;

use llvm_sys::core::*;
use llvm_sys::*;

use crate::core::types::Type;
use crate::core::Message;
use crate::opaque::{Opaque, PhantomOpaque};
use crate::type_tag::{any, TypeTag};

#[repr(transparent)]
pub struct Value<T: TypeTag> {
    _opaque: PhantomOpaque,
    _marker: PhantomData<fn(T) -> T>,
}

unsafe impl<T: TypeTag> Opaque for Value<T> {
    type Inner = LLVMValue;
}

impl<T: TypeTag> Debug for Value<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        T::value_debug_fmt(self, f)
    }
}

impl<T: TypeTag> Value<T> {
    pub unsafe fn cast_unchecked<N: TypeTag>(&self) -> &Value<N> {
        unsafe { Value::from_ref(self.as_ptr()) }
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

    // TODO: ValueTag
    pub fn get_kind(&self) -> LLVMValueKind {
        unsafe { LLVMGetValueKind(self.as_ptr()) }
    }

    /// Obtain the type of a value.
    pub fn get_type(&self) -> &Type<T> {
        unsafe { Type::from_ref(LLVMTypeOf(self.as_ptr())) }
    }

    /// Obtain the string name of a value.
    pub fn get_name_raw(&self) -> *const [u8] {
        unsafe {
            let mut len = 0;
            let s = LLVMGetValueName2(self.as_ptr(), &mut len);
            std::ptr::slice_from_raw_parts(s.cast(), len)
        }
    }

    /// Obtain the string name of a value.
    pub fn get_name(&self) -> CString {
        unsafe { CString::new(&*self.get_name_raw()).unwrap() }
    }

    pub fn set_name_raw(&self, name: &[u8]) {
        unsafe { LLVMSetValueName2(self.as_ptr(), name.as_ptr().cast(), name.len()) }
    }

    /// Set the string name of a value.
    pub fn set_name(&self, name: &CStr) {
        self.set_name_raw(name.to_bytes());
    }

    pub fn dump(&self) {
        unsafe { LLVMDumpValue(self.as_ptr()) };
    }

    /// Return a string representation of the value.
    pub fn print_to_string(&self) -> Message {
        unsafe { Message::from_raw(LLVMPrintValueToString(self.as_ptr())) }
    }

    // FIXME
    pub fn replace_all_uses_with(&self, new: &Self) {
        unsafe { LLVMReplaceAllUsesWith(self.as_ptr(), new.as_ptr()) };
    }

    /// Determine whether the specified value instance is constant.
    pub fn is_constant(&self) -> bool {
        unsafe { LLVMIsConstant(self.as_ptr()) != 0 }
    }

    /// Determine whether a value instance is undefined.
    pub fn is_undef(&self) -> bool {
        unsafe { LLVMIsUndef(self.as_ptr()) != 0 }
    }

    /// Determine whether a value instance is poisonous.
    pub fn is_poison(&self) -> bool {
        unsafe { LLVMIsPoison(self.as_ptr()) != 0 }
    }

    // FIXME
    pub fn is_a_metadata_node(&self) -> &Self {
        unsafe { Value::from_ref(LLVMIsAMDNode(self.as_ptr())) }
    }

    // FIXME
    pub fn is_a_value_as_metadata(&self) -> &Self {
        unsafe { Value::from_ref(LLVMIsAValueAsMetadata(self.as_ptr())) }
    }

    // FIXME
    pub fn is_a_metadata_string(&self) -> &Self {
        unsafe { Value::from_ref(LLVMIsAMDString(self.as_ptr())) }
    }
}
