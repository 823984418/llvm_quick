use llvm_sys::core::*;

use crate::core::context::Context;
use crate::core::type_tag::integers::{int, int1, int128, int16, int32, int64, int8, IntTypeTag};
use crate::core::types::Type;
use crate::opaque::Opaque;

impl Context {
    /// Obtain an integer type from a context with specified bit width.
    pub fn i1_type(&self) -> &Type<int1> {
        unsafe { Type::from_ref(LLVMInt1TypeInContext(self.as_raw())) }
    }

    /// Obtain an integer type from a context with specified bit width.
    pub fn i8_type(&self) -> &Type<int8> {
        unsafe { Type::from_ref(LLVMInt8TypeInContext(self.as_raw())) }
    }

    /// Obtain an integer type from a context with specified bit width.
    pub fn i16_type(&self) -> &Type<int16> {
        unsafe { Type::from_ref(LLVMInt16TypeInContext(self.as_raw())) }
    }

    /// Obtain an integer type from a context with specified bit width.
    pub fn i32_type(&self) -> &Type<int32> {
        unsafe { Type::from_ref(LLVMInt32TypeInContext(self.as_raw())) }
    }

    /// Obtain an integer type from a context with specified bit width.
    pub fn i64_type(&self) -> &Type<int64> {
        unsafe { Type::from_ref(LLVMInt64TypeInContext(self.as_raw())) }
    }

    /// Obtain an integer type from a context with specified bit width.
    pub fn i128_type(&self) -> &Type<int128> {
        unsafe { Type::from_ref(LLVMInt128TypeInContext(self.as_raw())) }
    }

    /// Obtain an integer type from a context with specified bit width.
    pub fn int_type<const N: u32>(&self) -> &Type<int<N>> {
        unsafe { Type::from_ref(LLVMIntTypeInContext(self.as_raw(), N)) }
    }
}

impl<T: IntTypeTag> Type<T> {
    pub fn get_int_width(&self) -> u32 {
        T::type_get_int_width(self)
    }
}
