use std::ffi::CStr;
use std::marker::PhantomData;

use llvm_sys::core::*;
use llvm_sys::*;

use crate::basic_block::BasicBlock;
use crate::context::Context;
use crate::metadata::Metadata;
use crate::opaque::{Opaque, PhantomOpaque};
use crate::owning::Dispose;
use crate::type_tag::integer_tag::{int, IntTypeTag};
use crate::type_tag::{label, void, FloatMathTypeTag, InstanceTypeTag, IntMathTypeTag, TypeTag};
use crate::values::Value;

#[repr(transparent)]
pub struct Builder<'s> {
    _opaque: PhantomOpaque,
    _marker: PhantomData<&'s Context>,
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

    /// Get the default floating-point math metadata for a given builder.
    pub fn get_default_fp_math_tag(&self) -> &'s Metadata {
        unsafe { Metadata::from_ref(LLVMBuilderGetDefaultFPMathTag(self.as_ptr())) }
    }

    /// Set the default floating-point math metadata for the given builder.
    pub fn set_default_fp_math_tag(&self, fp_math_tag: &'s Metadata) {
        unsafe { LLVMBuilderSetDefaultFPMathTag(self.as_ptr(), fp_math_tag.as_ptr()) };
    }

    pub fn return_void(&self) -> &'s Value<void> {
        unsafe { Value::from_ref(LLVMBuildRetVoid(self.as_ptr())) }
    }

    pub fn return_value<T: TypeTag>(&self, value: &'s Value<T>) -> &'s Value<void> {
        unsafe { Value::from_ref(LLVMBuildRet(self.as_ptr(), value.as_ptr())) }
    }

    pub fn return_aggregate<'a, T: TypeTag>(
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

    pub fn branch(&self, dest: &'s BasicBlock) -> &'s Value<void> {
        unsafe { Value::from_ref(LLVMBuildBr(self.as_ptr(), dest.as_ptr())) }
    }

    pub fn cond_branch<T: IntTypeTag>(
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

    pub fn switch<'a, const N: u32>(
        &self,
        v: &'s Value<int<N>>,
        els: &BasicBlock,
        cases: &'a [(&'s Value<int<N>>, &'s BasicBlock)],
    ) -> &'s Value<void> {
        unsafe {
            let value =
                LLVMBuildSwitch(self.as_ptr(), v.as_ptr(), els.as_ptr(), cases.len() as u32);
            for &(case, basic_block) in cases {
                LLVMAddCase(value, case.as_ptr(), basic_block.as_ptr());
            }
            Value::from_ref(value)
        }
    }

    pub fn indirect_branch<'a>(
        &self,
        addr: &'s Value<label>,
        destinations: &'a [&'s BasicBlock],
    ) -> &'s Value<void> {
        unsafe {
            let value =
                LLVMBuildIndirectBr(self.as_ptr(), addr.as_ptr(), destinations.len() as u32);
            for &destination in destinations {
                LLVMAddDestination(value, destination.as_ptr());
            }
            Value::from_ref(value)
        }
    }

    pub fn add<T: IntMathTypeTag>(
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

    pub fn nsw_add<T: IntMathTypeTag>(
        &self,
        lhs: &'s Value<T>,
        rhs: &'s Value<T>,
        name: &CStr,
    ) -> &'s Value<T> {
        unsafe {
            let ptr = LLVMBuildNSWAdd(self.as_ptr(), lhs.as_ptr(), rhs.as_ptr(), name.as_ptr());
            Value::from_ref(ptr)
        }
    }

    pub fn nuw_add<T: IntMathTypeTag>(
        &self,
        lhs: &'s Value<T>,
        rhs: &'s Value<T>,
        name: &CStr,
    ) -> &'s Value<T> {
        unsafe {
            let ptr = LLVMBuildNUWAdd(self.as_ptr(), lhs.as_ptr(), rhs.as_ptr(), name.as_ptr());
            Value::from_ref(ptr)
        }
    }

    pub fn float_add<T: FloatMathTypeTag>(
        &self,
        lhs: &'s Value<T>,
        rhs: &'s Value<T>,
        name: &CStr,
    ) -> &'s Value<T> {
        unsafe {
            let ptr = LLVMBuildFAdd(self.as_ptr(), lhs.as_ptr(), rhs.as_ptr(), name.as_ptr());
            Value::from_ref(ptr)
        }
    }

    pub fn sub<T: IntMathTypeTag>(
        &self,
        lhs: &'s Value<T>,
        rhs: &'s Value<T>,
        name: &CStr,
    ) -> &'s Value<T> {
        unsafe {
            let ptr = LLVMBuildSub(self.as_ptr(), lhs.as_ptr(), rhs.as_ptr(), name.as_ptr());
            Value::from_ref(ptr)
        }
    }

    pub fn nsw_sub<T: IntMathTypeTag>(
        &self,
        lhs: &'s Value<T>,
        rhs: &'s Value<T>,
        name: &CStr,
    ) -> &'s Value<T> {
        unsafe {
            let ptr = LLVMBuildNSWSub(self.as_ptr(), lhs.as_ptr(), rhs.as_ptr(), name.as_ptr());
            Value::from_ref(ptr)
        }
    }

    pub fn nuw_sub<T: IntMathTypeTag>(
        &self,
        lhs: &'s Value<T>,
        rhs: &'s Value<T>,
        name: &CStr,
    ) -> &'s Value<T> {
        unsafe {
            let ptr = LLVMBuildNUWSub(self.as_ptr(), lhs.as_ptr(), rhs.as_ptr(), name.as_ptr());
            Value::from_ref(ptr)
        }
    }

    pub fn float_sub<T: FloatMathTypeTag>(
        &self,
        lhs: &'s Value<T>,
        rhs: &'s Value<T>,
        name: &CStr,
    ) -> &'s Value<T> {
        unsafe {
            let ptr = LLVMBuildFSub(self.as_ptr(), lhs.as_ptr(), rhs.as_ptr(), name.as_ptr());
            Value::from_ref(ptr)
        }
    }

    pub fn mul<T: IntMathTypeTag>(
        &self,
        lhs: &'s Value<T>,
        rhs: &'s Value<T>,
        name: &CStr,
    ) -> &'s Value<T> {
        unsafe {
            let ptr = LLVMBuildMul(self.as_ptr(), lhs.as_ptr(), rhs.as_ptr(), name.as_ptr());
            Value::from_ref(ptr)
        }
    }

    pub fn nsw_mul<T: IntMathTypeTag>(
        &self,
        lhs: &'s Value<T>,
        rhs: &'s Value<T>,
        name: &CStr,
    ) -> &'s Value<T> {
        unsafe {
            let ptr = LLVMBuildNSWMul(self.as_ptr(), lhs.as_ptr(), rhs.as_ptr(), name.as_ptr());
            Value::from_ref(ptr)
        }
    }

    pub fn nuw_mul<T: IntMathTypeTag>(
        &self,
        lhs: &'s Value<T>,
        rhs: &'s Value<T>,
        name: &CStr,
    ) -> &'s Value<T> {
        unsafe {
            let ptr = LLVMBuildNUWMul(self.as_ptr(), lhs.as_ptr(), rhs.as_ptr(), name.as_ptr());
            Value::from_ref(ptr)
        }
    }

    pub fn float_mul<T: FloatMathTypeTag>(
        &self,
        lhs: &'s Value<T>,
        rhs: &'s Value<T>,
        name: &CStr,
    ) -> &'s Value<T> {
        unsafe {
            let ptr = LLVMBuildFMul(self.as_ptr(), lhs.as_ptr(), rhs.as_ptr(), name.as_ptr());
            Value::from_ref(ptr)
        }
    }

    pub fn unsigned_div<T: IntMathTypeTag>(
        &self,
        lhs: &'s Value<T>,
        rhs: &'s Value<T>,
        name: &CStr,
    ) -> &'s Value<T> {
        unsafe {
            let ptr = LLVMBuildUDiv(self.as_ptr(), lhs.as_ptr(), rhs.as_ptr(), name.as_ptr());
            Value::from_ref(ptr)
        }
    }

    pub fn signed_div<T: IntMathTypeTag>(
        &self,
        lhs: &'s Value<T>,
        rhs: &'s Value<T>,
        name: &CStr,
    ) -> &'s Value<T> {
        unsafe {
            let ptr = LLVMBuildSDiv(self.as_ptr(), lhs.as_ptr(), rhs.as_ptr(), name.as_ptr());
            Value::from_ref(ptr)
        }
    }

    pub fn exact_unsigned_div<T: IntMathTypeTag>(
        &self,
        lhs: &'s Value<T>,
        rhs: &'s Value<T>,
        name: &CStr,
    ) -> &'s Value<T> {
        unsafe {
            let ptr = LLVMBuildExactUDiv(self.as_ptr(), lhs.as_ptr(), rhs.as_ptr(), name.as_ptr());
            Value::from_ref(ptr)
        }
    }

    pub fn exact_signed_div<T: IntMathTypeTag>(
        &self,
        lhs: &'s Value<T>,
        rhs: &'s Value<T>,
        name: &CStr,
    ) -> &'s Value<T> {
        unsafe {
            let ptr = LLVMBuildExactSDiv(self.as_ptr(), lhs.as_ptr(), rhs.as_ptr(), name.as_ptr());
            Value::from_ref(ptr)
        }
    }

    pub fn float_div<T: FloatMathTypeTag>(
        &self,
        lhs: &'s Value<T>,
        rhs: &'s Value<T>,
        name: &CStr,
    ) -> &'s Value<T> {
        unsafe {
            let ptr = LLVMBuildFDiv(self.as_ptr(), lhs.as_ptr(), rhs.as_ptr(), name.as_ptr());
            Value::from_ref(ptr)
        }
    }

    pub fn unsigned_rem<T: IntMathTypeTag>(
        &self,
        lhs: &'s Value<T>,
        rhs: &'s Value<T>,
        name: &CStr,
    ) -> &'s Value<T> {
        unsafe {
            let ptr = LLVMBuildURem(self.as_ptr(), lhs.as_ptr(), rhs.as_ptr(), name.as_ptr());
            Value::from_ref(ptr)
        }
    }

    pub fn signed_rem<T: IntMathTypeTag>(
        &self,
        lhs: &'s Value<T>,
        rhs: &'s Value<T>,
        name: &CStr,
    ) -> &'s Value<T> {
        unsafe {
            let ptr = LLVMBuildSRem(self.as_ptr(), lhs.as_ptr(), rhs.as_ptr(), name.as_ptr());
            Value::from_ref(ptr)
        }
    }

    pub fn float_rem<T: FloatMathTypeTag>(
        &self,
        lhs: &'s Value<T>,
        rhs: &'s Value<T>,
        name: &CStr,
    ) -> &'s Value<T> {
        unsafe {
            let ptr = LLVMBuildFRem(self.as_ptr(), lhs.as_ptr(), rhs.as_ptr(), name.as_ptr());
            Value::from_ref(ptr)
        }
    }

    pub fn shl<T: IntMathTypeTag>(
        &self,
        lhs: &'s Value<T>,
        rhs: &'s Value<T>,
        name: &CStr,
    ) -> &'s Value<T> {
        unsafe {
            let ptr = LLVMBuildShl(self.as_ptr(), lhs.as_ptr(), rhs.as_ptr(), name.as_ptr());
            Value::from_ref(ptr)
        }
    }

    pub fn logic_shr<T: IntMathTypeTag>(
        &self,
        lhs: &'s Value<T>,
        rhs: &'s Value<T>,
        name: &CStr,
    ) -> &'s Value<T> {
        unsafe {
            let ptr = LLVMBuildLShr(self.as_ptr(), lhs.as_ptr(), rhs.as_ptr(), name.as_ptr());
            Value::from_ref(ptr)
        }
    }

    pub fn arith_shr<T: IntMathTypeTag>(
        &self,
        lhs: &'s Value<T>,
        rhs: &'s Value<T>,
        name: &CStr,
    ) -> &'s Value<T> {
        unsafe {
            let ptr = LLVMBuildAShr(self.as_ptr(), lhs.as_ptr(), rhs.as_ptr(), name.as_ptr());
            Value::from_ref(ptr)
        }
    }

    pub fn and<T: IntMathTypeTag>(
        &self,
        lhs: &'s Value<T>,
        rhs: &'s Value<T>,
        name: &CStr,
    ) -> &'s Value<T> {
        unsafe {
            let ptr = LLVMBuildAnd(self.as_ptr(), lhs.as_ptr(), rhs.as_ptr(), name.as_ptr());
            Value::from_ref(ptr)
        }
    }

    pub fn or<T: IntMathTypeTag>(
        &self,
        lhs: &'s Value<T>,
        rhs: &'s Value<T>,
        name: &CStr,
    ) -> &'s Value<T> {
        unsafe {
            let ptr = LLVMBuildOr(self.as_ptr(), lhs.as_ptr(), rhs.as_ptr(), name.as_ptr());
            Value::from_ref(ptr)
        }
    }

    pub fn xor<T: IntMathTypeTag>(
        &self,
        lhs: &'s Value<T>,
        rhs: &'s Value<T>,
        name: &CStr,
    ) -> &'s Value<T> {
        unsafe {
            let ptr = LLVMBuildXor(self.as_ptr(), lhs.as_ptr(), rhs.as_ptr(), name.as_ptr());
            Value::from_ref(ptr)
        }
    }

    pub unsafe fn binary_op<T: InstanceTypeTag>(
        &self,
        op: LLVMOpcode,
        lhs: &'s Value<T>,
        rhs: &'s Value<T>,
        name: &CStr,
    ) -> &'s Value<T> {
        unsafe {
            let ptr = LLVMBuildBinOp(self.as_ptr(), op, lhs.as_ptr(), rhs.as_ptr(), name.as_ptr());
            Value::from_ref(ptr)
        }
    }

    pub fn neg<T: IntMathTypeTag>(&self, v: &'s Value<T>, name: &CStr) -> &'s Value<T> {
        unsafe { Value::from_ref(LLVMBuildNeg(self.as_ptr(), v.as_ptr(), name.as_ptr())) }
    }

    pub fn nsw_neg<T: IntMathTypeTag>(&self, v: &'s Value<T>, name: &CStr) -> &'s Value<T> {
        unsafe { Value::from_ref(LLVMBuildNSWNeg(self.as_ptr(), v.as_ptr(), name.as_ptr())) }
    }

    pub fn nuw_neg<T: IntMathTypeTag>(&self, v: &'s Value<T>, name: &CStr) -> &'s Value<T> {
        unsafe { Value::from_ref(LLVMBuildNUWNeg(self.as_ptr(), v.as_ptr(), name.as_ptr())) }
    }

    pub fn float_neg<T: FloatMathTypeTag>(&self, v: &'s Value<T>, name: &CStr) -> &'s Value<T> {
        unsafe { Value::from_ref(LLVMBuildFNeg(self.as_ptr(), v.as_ptr(), name.as_ptr())) }
    }

    pub fn not<T: IntMathTypeTag>(&self, v: &'s Value<T>, name: &CStr) -> &'s Value<T> {
        unsafe { Value::from_ref(LLVMBuildNot(self.as_ptr(), v.as_ptr(), name.as_ptr())) }
    }
}
