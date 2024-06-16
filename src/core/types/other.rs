use std::ffi::CStr;

use llvm_sys::core::*;

use crate::type_tag::*;
use crate::{Context, Opaque, Type};

impl Context {
    /// Create a void type in a context.
    pub fn void_type(&self) -> &Type<void> {
        unsafe { Type::from_raw(LLVMVoidTypeInContext(self.as_raw())) }
    }

    pub fn label_type(&self) -> &Type<label> {
        unsafe { Type::from_raw(LLVMLabelTypeInContext(self.as_raw())) }
    }

    pub fn x86_mmx_type(&self) -> &Type<x86_mmx> {
        unsafe { Type::from_raw(LLVMX86MMXTypeInContext(self.as_raw())) }
    }

    pub fn x86_amx_type(&self) -> &Type<x86_amx> {
        unsafe { Type::from_raw(LLVMX86AMXTypeInContext(self.as_raw())) }
    }

    pub fn token_type(&self) -> &Type<token> {
        unsafe { Type::from_raw(LLVMTokenTypeInContext(self.as_raw())) }
    }

    pub fn metadata_type(&self) -> &Type<metadata> {
        unsafe { Type::from_raw(LLVMMetadataTypeInContext(self.as_raw())) }
    }

    pub fn target_ext_type(
        &self,
        name: &CStr,
        type_params: &[&Type<any>],
        int_params: &[u32],
    ) -> &Type<target_ext_any> {
        unsafe {
            Type::from_raw(LLVMTargetExtTypeInContext(
                self.as_raw(),
                name.as_ptr(),
                type_params.as_ptr() as _,
                type_params.len() as _,
                int_params.as_ptr() as _,
                int_params.len() as _,
            ))
        }
    }
}
