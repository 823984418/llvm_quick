use std::ffi::CStr;
use std::marker::PhantomData;

use llvm_sys::core::*;
use llvm_sys::*;

use crate::core::basic_block::BasicBlock;
use crate::core::context::Context;
use crate::core::metadata::{Metadata, OperandBundle};
use crate::core::type_tag::functions::FunTypeTag;
use crate::core::type_tag::integers::{int, IntTypeTag};
use crate::core::type_tag::{
    any, label, void, FloatMathTypeTag, InstanceTypeTag, IntMathTypeTag, TypeTag,
};
use crate::core::types::Type;
use crate::core::values::Value;
use crate::opaque::{Opaque, PhantomOpaque};
use crate::owning::{OpaqueDrop, Owning};

/// Return a constant that specifies that the result of a ShuffleVectorInst is undefined.
pub fn get_undef_mask_elem() -> i32 {
    unsafe { LLVMGetUndefMaskElem() }
}

#[repr(transparent)]
pub struct Builder<'s> {
    _opaque: PhantomOpaque,
    _marker: PhantomData<&'s Context>,
}

unsafe impl<'s> Opaque for Builder<'s> {
    type Inner = LLVMBuilder;
}

impl Context {
    pub fn create_builder(&self) -> Owning<Builder> {
        unsafe { Owning::from_raw(LLVMCreateBuilderInContext(self.as_raw())) }
    }
}

impl<'s> Builder<'s> {
    pub fn position<T: TypeTag>(&self, basic_block: &'s BasicBlock, inst: &'s Value<T>) {
        unsafe { LLVMPositionBuilder(self.as_raw(), basic_block.as_raw(), inst.as_raw()) };
    }

    pub fn position_at_end_before<T: TypeTag>(&self, inst: &'s Value<T>) {
        unsafe { LLVMPositionBuilderBefore(self.as_raw(), inst.as_raw()) };
    }

    pub fn position_at_end(&self, basic_block: &'s BasicBlock) {
        unsafe { LLVMPositionBuilderAtEnd(self.as_raw(), basic_block.as_raw()) };
    }

    pub fn get_insert_block(&self) -> &'s BasicBlock {
        unsafe { BasicBlock::from_ref(LLVMGetInsertBlock(self.as_raw())) }
    }

    pub fn clear_insertion_position(&self) {
        unsafe { LLVMClearInsertionPosition(self.as_raw()) };
    }

    pub fn insert<T: TypeTag>(&self, inst: &'s Value<T>) {
        unsafe { LLVMInsertIntoBuilder(self.as_raw(), inst.as_raw()) };
    }

    pub fn insert_with_name<'a, T: TypeTag>(&self, inst: &'s Value<T>, name: &'a CStr) {
        unsafe { LLVMInsertIntoBuilderWithName(self.as_raw(), inst.as_raw(), name.as_ptr()) };
    }
}

impl<'s> OpaqueDrop for Builder<'s> {
    unsafe fn drop_raw(ptr: *mut Self::Inner) {
        unsafe { LLVMDisposeBuilder(ptr) };
    }
}

impl<'s> Builder<'s> {
    /// Get location information used by debugging information.
    pub fn get_current_debug_location(&self) -> &'s Metadata {
        unsafe { Metadata::from_ref(LLVMGetCurrentDebugLocation2(self.as_raw())) }
    }

    /// Set location information used by debugging information.
    pub fn set_current_debug_location(&self, loc: &'s Metadata) {
        unsafe { LLVMSetCurrentDebugLocation2(self.as_raw(), loc.as_raw()) };
    }

    /// Adds the metadata registered with the given builder to the given instruction.
    pub fn add_metadata_to_inst<T: TypeTag>(&self, inst: &'s Value<T>) {
        unsafe { LLVMAddMetadataToInst(self.as_raw(), inst.as_raw()) };
    }

    /// Get the default floating-point math metadata for a given builder.
    pub fn get_default_fp_math_tag(&self) -> &'s Metadata {
        unsafe { Metadata::from_ref(LLVMBuilderGetDefaultFPMathTag(self.as_raw())) }
    }

    /// Set the default floating-point math metadata for the given builder.
    pub fn set_default_fp_math_tag(&self, fp_math_tag: &'s Metadata) {
        unsafe { LLVMBuilderSetDefaultFPMathTag(self.as_raw(), fp_math_tag.as_raw()) };
    }

    pub fn return_void(&self) -> &'s Value<void> {
        unsafe { Value::from_ref(LLVMBuildRetVoid(self.as_raw())) }
    }

    pub fn return_value<T: TypeTag>(&self, value: &'s Value<T>) -> &'s Value<void> {
        unsafe { Value::from_ref(LLVMBuildRet(self.as_raw(), value.as_raw())) }
    }

    pub fn return_aggregate<'a, T: TypeTag>(
        &self,
        ret_vals: &'a [&'s Value<T>],
    ) -> &'s Value<void> {
        unsafe {
            Value::from_ref(LLVMBuildAggregateRet(
                self.as_raw(),
                ret_vals.as_ptr() as _,
                ret_vals.len() as _,
            ))
        }
    }

    pub fn branch(&self, dest: &'s BasicBlock) -> &'s Value<void> {
        unsafe { Value::from_ref(LLVMBuildBr(self.as_raw(), dest.as_raw())) }
    }

    pub fn cond_branch<T: IntTypeTag>(
        &self,
        cond: &'s Value<T>,
        then: &'s BasicBlock,
        els: &'s BasicBlock,
    ) -> &'s Value<void> {
        unsafe {
            Value::from_ref(LLVMBuildCondBr(
                self.as_raw(),
                cond.as_raw(),
                then.as_raw(),
                els.as_raw(),
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
                LLVMBuildSwitch(self.as_raw(), v.as_raw(), els.as_raw(), cases.len() as u32);
            for &(case, basic_block) in cases {
                LLVMAddCase(value, case.as_raw(), basic_block.as_raw());
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
                LLVMBuildIndirectBr(self.as_raw(), addr.as_raw(), destinations.len() as u32);
            for &destination in destinations {
                LLVMAddDestination(value, destination.as_raw());
            }
            Value::from_ref(value)
        }
    }

    pub fn invoke<T: FunTypeTag>(
        &self,
        ty: &Type<T>,
        f: &Value<T>,
        args: &[&Value<any>],
        then: &BasicBlock,
        catch: &BasicBlock,
        name: &CStr,
    ) -> &Value<any> {
        unsafe {
            Value::from_ref(LLVMBuildInvoke2(
                self.as_raw(),
                ty.as_raw(),
                f.as_raw(),
                args.as_ptr() as _,
                args.len() as _,
                then.as_raw(),
                catch.as_raw(),
                name.as_ptr(),
            ))
        }
    }

    pub fn invoke_with_operand_bundles<T: FunTypeTag>(
        &self,
        ty: &Type<T>,
        f: &Value<T>,
        args: &[&Value<any>],
        then: &BasicBlock,
        catch: &BasicBlock,
        bundles: &[&OperandBundle],
        name: &CStr,
    ) -> &Value<any> {
        unsafe {
            Value::from_ref(LLVMBuildInvokeWithOperandBundles(
                self.as_raw(),
                ty.as_raw(),
                f.as_raw(),
                args.as_ptr() as _,
                args.len() as _,
                then.as_raw(),
                catch.as_raw(),
                bundles.as_ptr() as _,
                bundles.len() as _,
                name.as_ptr(),
            ))
        }
    }

    pub fn unreachable(&self) -> &Value<void> {
        unsafe { Value::from_ref(LLVMBuildUnreachable(self.as_raw())) }
    }

    pub fn resume<T: TypeTag>(&self, exn: &Value<T>) -> &Value<any> {
        // TODO
        unsafe { Value::from_ref(LLVMBuildResume(self.as_raw(), exn.as_raw())) }
    }

    // TODO

    pub fn add<T: IntMathTypeTag>(
        &self,
        lhs: &'s Value<T>,
        rhs: &'s Value<T>,
        name: &CStr,
    ) -> &'s Value<T> {
        unsafe {
            let ptr = LLVMBuildAdd(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
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
            let ptr = LLVMBuildNSWAdd(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
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
            let ptr = LLVMBuildNUWAdd(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
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
            let ptr = LLVMBuildFAdd(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
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
            let ptr = LLVMBuildSub(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
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
            let ptr = LLVMBuildNSWSub(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
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
            let ptr = LLVMBuildNUWSub(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
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
            let ptr = LLVMBuildFSub(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
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
            let ptr = LLVMBuildMul(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
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
            let ptr = LLVMBuildNSWMul(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
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
            let ptr = LLVMBuildNUWMul(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
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
            let ptr = LLVMBuildFMul(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
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
            let ptr = LLVMBuildUDiv(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
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
            let ptr = LLVMBuildSDiv(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
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
            let ptr = LLVMBuildExactUDiv(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
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
            let ptr = LLVMBuildExactSDiv(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
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
            let ptr = LLVMBuildFDiv(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
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
            let ptr = LLVMBuildURem(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
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
            let ptr = LLVMBuildSRem(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
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
            let ptr = LLVMBuildFRem(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
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
            let ptr = LLVMBuildShl(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
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
            let ptr = LLVMBuildLShr(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
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
            let ptr = LLVMBuildAShr(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
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
            let ptr = LLVMBuildAnd(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
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
            let ptr = LLVMBuildOr(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
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
            let ptr = LLVMBuildXor(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
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
            let ptr = LLVMBuildBinOp(self.as_raw(), op, lhs.as_raw(), rhs.as_raw(), name.as_ptr());
            Value::from_ref(ptr)
        }
    }

    pub fn neg<T: IntMathTypeTag>(&self, v: &'s Value<T>, name: &CStr) -> &'s Value<T> {
        unsafe { Value::from_ref(LLVMBuildNeg(self.as_raw(), v.as_raw(), name.as_ptr())) }
    }

    pub fn nsw_neg<T: IntMathTypeTag>(&self, v: &'s Value<T>, name: &CStr) -> &'s Value<T> {
        unsafe { Value::from_ref(LLVMBuildNSWNeg(self.as_raw(), v.as_raw(), name.as_ptr())) }
    }

    pub fn nuw_neg<T: IntMathTypeTag>(&self, v: &'s Value<T>, name: &CStr) -> &'s Value<T> {
        unsafe { Value::from_ref(LLVMBuildNUWNeg(self.as_raw(), v.as_raw(), name.as_ptr())) }
    }

    pub fn float_neg<T: FloatMathTypeTag>(&self, v: &'s Value<T>, name: &CStr) -> &'s Value<T> {
        unsafe { Value::from_ref(LLVMBuildFNeg(self.as_raw(), v.as_raw(), name.as_ptr())) }
    }

    pub fn not<T: IntMathTypeTag>(&self, v: &'s Value<T>, name: &CStr) -> &'s Value<T> {
        unsafe { Value::from_ref(LLVMBuildNot(self.as_raw(), v.as_raw(), name.as_ptr())) }
    }

    // TODO: more
}

impl<T: TypeTag> Value<T> {
    pub fn get_nuw(&self) -> bool {
        unsafe { LLVMGetNUW(self.as_raw()) != 0 }
    }

    pub fn set_nuw(&self, set: bool) {
        unsafe { LLVMSetNUW(self.as_raw(), set as _) };
    }

    pub fn get_nsw(&self) -> bool {
        unsafe { LLVMGetNSW(self.as_raw()) != 0 }
    }

    pub fn set_nsw(&self, set: bool) {
        unsafe { LLVMSetNSW(self.as_raw(), set as _) };
    }

    pub fn get_exact(&self) -> bool {
        unsafe { LLVMGetExact(self.as_raw()) != 0 }
    }

    pub fn set_exact(&self, set: bool) {
        unsafe { LLVMSetExact(self.as_raw(), set as _) };
    }

    /// Gets if the instruction has the non-negative flag set.
    pub fn get_non_neg(&self) -> bool {
        unsafe { LLVMGetNNeg(self.as_raw()) != 0 }
    }

    /// Sets the non-negative flag for the instruction.
    pub fn set_non_neg(&self, set: bool) {
        unsafe { LLVMSetNNeg(self.as_raw(), set as _) };
    }

    /// Get the flags for which fast-math-style optimizations are allowed for this value.
    pub fn get_fast_math_flags(&self) -> LLVMFastMathFlags {
        unsafe { LLVMGetFastMathFlags(self.as_raw()) }
    }

    /// Sets the flags for which fast-math-style optimizations are allowed for this value.
    pub fn set_fast_math_flags(&self, set: LLVMFastMathFlags) {
        unsafe { LLVMSetFastMathFlags(self.as_raw(), set) };
    }
}
