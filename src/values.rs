use std::ffi::{CStr, CString};
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;

use llvm_sys::core::*;
use llvm_sys::*;

use crate::message::Message;
use crate::opaque::{Opaque, PhantomOpaque};
use crate::type_tag::{any, TypeTag};
use crate::types::Type;

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

    /// Return a string representation of the value.
    pub fn print_to_string(&self) -> Message {
        unsafe { Message::from_raw(LLVMPrintValueToString(self.as_ptr())) }
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

    /// Set the string name of a value.
    pub fn set_name(&self, name: &CStr) {
        unsafe { LLVMSetValueName2(self.as_ptr(), name.as_ptr(), name.count_bytes()) };
    }

    /// Obtain the type of a value.
    pub fn get_type(&self) -> &Type<T> {
        unsafe { Type::from_ref(LLVMTypeOf(self.as_ptr())) }
    }

    pub fn get_nuw(&self) -> bool {
        unsafe { LLVMGetNUW(self.as_ptr()) != 0 }
    }

    pub fn set_nuw(&self, set: bool) {
        unsafe { LLVMSetNUW(self.as_ptr(), set as _) };
    }

    pub fn get_nsw(&self) -> bool {
        unsafe { LLVMGetNSW(self.as_ptr()) != 0 }
    }

    pub fn set_nsw(&self, set: bool) {
        unsafe { LLVMSetNSW(self.as_ptr(), set as _) };
    }

    pub fn get_exact(&self) -> bool {
        unsafe { LLVMGetExact(self.as_ptr()) != 0 }
    }

    pub fn set_exact(&self, set: bool) {
        unsafe { LLVMSetExact(self.as_ptr(), set as _) };
    }

    /// Gets if the instruction has the non-negative flag set.
    pub fn get_non_neg(&self) -> bool {
        unsafe { LLVMGetNNeg(self.as_ptr()) != 0 }
    }

    /// Sets the non-negative flag for the instruction.
    pub fn set_non_neg(&self, set: bool) {
        unsafe { LLVMSetNNeg(self.as_ptr(), set as _) };
    }

    /// Get the flags for which fast-math-style optimizations are allowed for this value.
    pub fn get_fast_math_flags(&self) -> LLVMFastMathFlags {
        unsafe { LLVMGetFastMathFlags(self.as_ptr()) }
    }

    /// Sets the flags for which fast-math-style optimizations are allowed for this value.
    pub fn set_fast_math_flags(&self, set: LLVMFastMathFlags) {
        unsafe { LLVMSetFastMathFlags(self.as_ptr(), set) };
    }
}
