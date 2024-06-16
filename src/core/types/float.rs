use llvm_sys::core::*;

use crate::type_tag::*;
use crate::{Context, Opaque, Type};

impl Context {
    pub fn half_type(&self) -> &Type<half> {
        unsafe { Type::from_raw(LLVMHalfTypeInContext(self.as_raw())) }
    }

    pub fn bfloat_type(&self) -> &Type<bfloat> {
        unsafe { Type::from_raw(LLVMBFloatTypeInContext(self.as_raw())) }
    }

    /// Create a float type in a context.
    pub fn float_type(&self) -> &Type<float> {
        unsafe { Type::from_raw(LLVMFloatTypeInContext(self.as_raw())) }
    }

    pub fn double_type(&self) -> &Type<double> {
        unsafe { Type::from_raw(LLVMDoubleTypeInContext(self.as_raw())) }
    }

    pub fn x86_fp80_type(&self) -> &Type<x86_fp80> {
        unsafe { Type::from_raw(LLVMX86FP80TypeInContext(self.as_raw())) }
    }

    pub fn fp128_type(&self) -> &Type<fp128> {
        unsafe { Type::from_raw(LLVMFP128TypeInContext(self.as_raw())) }
    }

    pub fn ppc_fp128_type(&self) -> &Type<ppc_fp128> {
        unsafe { Type::from_raw(LLVMPPCFP128TypeInContext(self.as_raw())) }
    }
}
