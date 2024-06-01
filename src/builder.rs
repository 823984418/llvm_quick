use std::ffi::CStr;
use std::marker::PhantomData;

use llvm_sys::core::*;
use llvm_sys::LLVMBuilder;

use crate::basic_block::BasicBlock;
use crate::context::Context;
use crate::metadata::Metadata;
use crate::opaque::{Opaque, PhantomOpaque};
use crate::owning::Dispose;
use crate::type_tag::{void, MathTypeTag, TypeTag};
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
    pub fn position<T: TypeTag>(&self, basic_block: &'s BasicBlock, inst: &'s Value<T>) {
        unsafe { LLVMPositionBuilder(self.as_ptr(), basic_block.as_ptr(), inst.as_ptr()) };
    }

    pub fn position_at_end(&self, basic_block: &'s BasicBlock) {
        unsafe { LLVMPositionBuilderAtEnd(self.as_ptr(), basic_block.as_ptr()) };
    }

    pub fn position_at_end_before<T: TypeTag>(&self, inst: &'s Value<T>) {
        unsafe { LLVMPositionBuilderBefore(self.as_ptr(), inst.as_ptr()) };
    }

    pub fn get_insert_block(&self) -> &'s BasicBlock {
        unsafe { BasicBlock::from_ref(LLVMGetInsertBlock(self.as_ptr())) }
    }

    pub fn clear_insertion_position(&self) {
        unsafe { LLVMClearInsertionPosition(self.as_ptr()) };
    }

    pub fn insert<T: TypeTag>(&self, inst: &'s Value<T>) {
        unsafe { LLVMInsertIntoBuilder(self.as_ptr(), inst.as_ptr()) };
    }

    pub fn insert_with_name<'a, T: TypeTag>(&self, inst: &'s Value<T>, name: &'a CStr) {
        unsafe { LLVMInsertIntoBuilderWithName(self.as_ptr(), inst.as_ptr(), name.as_ptr()) };
    }

    /// Get location information used by debugging information.
    pub fn get_current_debug_location(&self) -> &'s Metadata {
        unsafe { Metadata::from_ref(LLVMGetCurrentDebugLocation2(self.as_ptr())) }
    }

    /// Set location information used by debugging information.
    pub fn set_current_debug_location(&self, loc: &'s Metadata) {
        unsafe { LLVMSetCurrentDebugLocation2(self.as_ptr(), loc.as_ptr()) };
    }

    /// Adds the metadata registered with the given builder to the given instruction.
    pub fn add_metadata_to_inst<T: TypeTag>(&self, inst: &'s Value<T>) {
        unsafe { LLVMAddMetadataToInst(self.as_ptr(), inst.as_ptr()) };
    }

    /// Get the dafult floating-point math metadata for a given builder.
    pub fn get_default_fp_math_tag(&self) -> &'s Metadata {
        unsafe { Metadata::from_ref(LLVMBuilderGetDefaultFPMathTag(self.as_ptr())) }
    }

    /// Set the default floating-point math metadata for the given builder.
    pub fn set_default_fp_math_tag(&self, fp_math_tag: &'s Metadata) {
        unsafe { LLVMBuilderSetDefaultFPMathTag(self.as_ptr(), fp_math_tag.as_ptr()) };
    }

    pub fn build_return_void(&self) -> &'s Value<void> {
        unsafe { Value::from_ref(LLVMBuildRetVoid(self.as_ptr())) }
    }

    pub fn build_return<T: TypeTag>(&self, value: &'s Value<T>) -> &'s Value<void> {
        unsafe { Value::from_ref(LLVMBuildRet(self.as_ptr(), value.as_ptr())) }
    }

    pub fn build_aggregate_ret<'a, T: TypeTag>(
        &self,
        ret_vals: &'a [&'s Value<T>],
    ) -> &'s Value<void> {
        unsafe {
            Value::from_ref(LLVMBuildAggregateRet(
                self.as_ptr(),
                ret_vals.as_ptr() as _,
                ret_vals.len() as _,
            ))
        }
    }

    pub fn build_br(&self, dest: &'s BasicBlock) -> &'s Value<void> {
        unsafe { Value::from_ref(LLVMBuildBr(self.as_ptr(), dest.as_ptr())) }
    }
    pub fn build_cond_br<T: TypeTag>(
        &self,
        cond: &'s Value<T>,
        then: &'s BasicBlock,
        els: &'s BasicBlock,
    ) -> &'s Value<void> {
        unsafe {
            Value::from_ref(LLVMBuildCondBr(
                self.as_ptr(),
                cond.as_ptr(),
                then.as_ptr(),
                els.as_ptr(),
            ))
        }
    }

    pub fn build_switch<T>(
        &self,
        v: &'s Value<T>,
        els: &BasicBlock,
        num_cases: u32,
    ) -> &'s Value<void> {
        unsafe {
            Value::from_ref(LLVMBuildSwitch(
                self.as_ptr(),
                v.as_prt(),
                els.as_ptr(),
                num_cases,
            ))
        }
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
}
