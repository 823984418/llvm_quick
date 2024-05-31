use std::ffi::CStr;
use std::marker::PhantomData;

use llvm_sys::core::*;
use llvm_sys::LLVMBuilder;

use crate::basic_block::BasicBlock;
use crate::context::Context;
use crate::opaque::{Opaque, PhantomOpaque};
use crate::owning::Dispose;
use crate::type_tag::{any, void, MathTypeTag, TypeTag};
use crate::values::Value;

#[repr(transparent)]
pub struct Builder<'s> {
    opaque: PhantomOpaque,
    marker: PhantomData<&'s Context>,
}

unsafe impl<'s> Opaque for Builder<'s> {
    type Inner = LLVMBuilder;
}

impl<'s> Dispose for Builder<'s> {
    unsafe fn dispose(ptr: *mut Self::Inner) {
        unsafe { LLVMDisposeBuilder(ptr) };
    }
}

impl<'s> Builder<'s> {
    pub fn position_at_end(&self, basic_block: &'s BasicBlock) {
        unsafe { LLVMPositionBuilderAtEnd(self.as_ptr(), basic_block.as_ptr()) };
    }

    pub fn build_add<T: MathTypeTag>(
        &self,
        lhs: &'s Value<T>,
        rhs: &'s Value<T>,
        name: &CStr,
    ) -> &'s Value<T> {
        unsafe {
            let ptr = LLVMBuildAdd(self.as_ptr(), lhs.as_ptr(), rhs.as_ptr(), name.as_ptr());
            Value::from_ref(ptr)
        }
    }

    pub fn build_return<T: TypeTag>(&self, value: &'s Value<T>) -> &'s Value<void> {
        unsafe { Value::from_ref(LLVMBuildRet(self.as_ptr(), value.as_ptr())) }
    }

    pub fn build_return_void(&self) -> &'s Value<any> {
        unsafe { Value::from_ref(LLVMBuildRetVoid(self.as_ptr())) }
    }
}
