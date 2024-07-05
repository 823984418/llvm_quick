use llvm_sys::core::*;

use crate::type_tag::*;
use crate::*;

impl Context {
    /// Obtain an integer type from a context with specified bit width.
    pub fn i1_type(&self) -> &Type<int1> {
        unsafe { Type::from_raw(LLVMInt1TypeInContext(self.as_raw())) }
    }

    /// Obtain an integer type from a context with specified bit width.
    pub fn i8_type(&self) -> &Type<int8> {
        unsafe { Type::from_raw(LLVMInt8TypeInContext(self.as_raw())) }
    }

    /// Obtain an integer type from a context with specified bit width.
    pub fn i16_type(&self) -> &Type<int16> {
        unsafe { Type::from_raw(LLVMInt16TypeInContext(self.as_raw())) }
    }

    /// Obtain an integer type from a context with specified bit width.
    pub fn i32_type(&self) -> &Type<int32> {
        unsafe { Type::from_raw(LLVMInt32TypeInContext(self.as_raw())) }
    }

    /// Obtain an integer type from a context with specified bit width.
    pub fn i64_type(&self) -> &Type<int64> {
        unsafe { Type::from_raw(LLVMInt64TypeInContext(self.as_raw())) }
    }

    /// Obtain an integer type from a context with specified bit width.
    pub fn i128_type(&self) -> &Type<int128> {
        unsafe { Type::from_raw(LLVMInt128TypeInContext(self.as_raw())) }
    }

    /// Obtain an integer type from a context with specified bit width.
    pub fn int_type<const N: u32>(&self) -> &Type<int<N>> {
        unsafe { Type::from_raw(LLVMIntTypeInContext(self.as_raw(), N)) }
    }
}

impl<T: IntTypeTag> Type<T> {
    pub fn get_int_width(&self) -> u32 {
        unsafe { LLVMGetIntTypeWidth(self.as_raw()) }
    }
}
