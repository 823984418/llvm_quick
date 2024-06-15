use std::ffi::CStr;

use llvm_sys::core::*;
use llvm_sys::*;

use crate::owning::{OpaqueDrop, Owning};
use crate::type_tag::*;
use crate::{BasicBlock, Builder, Context, Metadata, Opaque, OperandBundle, Type, Value};

impl Context {
    pub fn create_builder(&self) -> Owning<Builder> {
        unsafe { Owning::from_raw(LLVMCreateBuilderInContext(self.as_raw())) }
    }
}

impl<'s> Builder<'s> {
    pub fn position<T: TypeTag>(&self, basic_block: &BasicBlock, inst: &Value<T>) {
        unsafe { LLVMPositionBuilder(self.as_raw(), basic_block.as_raw(), inst.as_raw()) }
    }

    pub fn position_at_end_before<T: TypeTag>(&self, inst: &Value<T>) {
        unsafe { LLVMPositionBuilderBefore(self.as_raw(), inst.as_raw()) }
    }

    pub fn position_at_end(&self, basic_block: &BasicBlock) {
        unsafe { LLVMPositionBuilderAtEnd(self.as_raw(), basic_block.as_raw()) }
    }

    pub fn get_insert_block(&self) -> &'s BasicBlock {
        unsafe { BasicBlock::from_ref(LLVMGetInsertBlock(self.as_raw())) }
    }

    pub fn clear_insertion_position(&self) {
        unsafe { LLVMClearInsertionPosition(self.as_raw()) }
    }

    pub fn insert<T: TypeTag>(&self, inst: &'s Value<T>) {
        unsafe { LLVMInsertIntoBuilder(self.as_raw(), inst.as_raw()) }
    }

    pub fn insert_with_name<T: TypeTag>(&self, inst: &Value<T>, name: &CStr) {
        unsafe { LLVMInsertIntoBuilderWithName(self.as_raw(), inst.as_raw(), name.as_ptr()) }
    }
}

impl OpaqueDrop for LLVMBuilder {
    unsafe fn drop_raw(ptr: *mut Self) {
        unsafe { LLVMDisposeBuilder(ptr) }
    }
}

impl<'s> Builder<'s> {
    /// Get location information used by debugging information.
    pub fn get_current_debug_location(&self) -> &'s Metadata {
        unsafe { Metadata::from_ref(LLVMGetCurrentDebugLocation2(self.as_raw())) }
    }

    /// Set location information used by debugging information.
    pub fn set_current_debug_location(&self, loc: &Metadata) {
        unsafe { LLVMSetCurrentDebugLocation2(self.as_raw(), loc.as_raw()) }
    }

    /// Adds the metadata registered with the given builder to the given instruction.
    pub fn add_metadata_to_inst<T: TypeTag>(&self, inst: &'s Value<T>) {
        unsafe { LLVMAddMetadataToInst(self.as_raw(), inst.as_raw()) }
    }

    /// Get the default floating-point math metadata for a given builder.
    pub fn get_default_fp_math_tag(&self) -> &'s Metadata {
        unsafe { Metadata::from_ref(LLVMBuilderGetDefaultFPMathTag(self.as_raw())) }
    }

    /// Set the default floating-point math metadata for the given builder.
    pub fn set_default_fp_math_tag(&self, fp_math_tag: &'s Metadata) {
        unsafe { LLVMBuilderSetDefaultFPMathTag(self.as_raw(), fp_math_tag.as_raw()) }
    }

    pub fn return_void(&self) -> &'s Value<void> {
        unsafe { Value::from_ref(LLVMBuildRetVoid(self.as_raw())) }
    }

    pub fn return_value<T: TypeTag>(&self, value: &Value<T>) -> &'s Value<void> {
        unsafe { Value::from_ref(LLVMBuildRet(self.as_raw(), value.as_raw())) }
    }

    pub fn return_aggregate<T: TypeTag>(&self, ret_vals: &[&Value<T>]) -> &'s Value<void> {
        unsafe {
            Value::from_ref(LLVMBuildAggregateRet(
                self.as_raw(),
                ret_vals.as_ptr() as _,
                ret_vals.len() as _,
            ))
        }
    }

    pub fn branch(&self, dest: &BasicBlock) -> &'s Value<void> {
        unsafe { Value::from_ref(LLVMBuildBr(self.as_raw(), dest.as_raw())) }
    }

    pub fn cond_branch<T: IntTypeTag>(
        &self,
        cond: &Value<T>,
        then: &BasicBlock,
        els: &BasicBlock,
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

    pub fn switch<const N: u32>(
        &self,
        v: &Value<int<N>>,
        els: &BasicBlock,
        cases: &[(&Value<int<N>>, &BasicBlock)],
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

    pub fn indirect_branch(
        &self,
        addr: &Value<label>,
        destinations: &[&BasicBlock],
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
    ) -> &'s Value<any> {
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
    ) -> &'s Value<any> {
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

    pub fn unreachable(&self) -> &'s Value<void> {
        unsafe { Value::from_ref(LLVMBuildUnreachable(self.as_raw())) }
    }

    pub fn resume<T: TypeTag>(&self, exn: &Value<T>) -> &'s Value<any> {
        unsafe { Value::from_ref(LLVMBuildResume(self.as_raw(), exn.as_raw())) }
    }

    pub fn landing_pad<F: FunTypeTag>(
        &self,
        ty: &Type<any>,
        pers_fn: &Value<F>,
        num_clauses: u32,
        name: &CStr,
    ) -> &'s Value<any> {
        unsafe {
            Value::from_ref(LLVMBuildLandingPad(
                self.as_raw(),
                ty.as_raw(),
                pers_fn.as_raw(),
                num_clauses,
                name.as_ptr(),
            ))
        }
    }

    pub fn cleanup_return(&self, catch_pad: &Value<any>, bb: &BasicBlock) -> &'s Value<any> {
        unsafe {
            Value::from_ref(LLVMBuildCleanupRet(
                self.as_raw(),
                catch_pad.as_raw(),
                bb.as_raw(),
            ))
        }
    }

    // TODO

    pub fn add<T: IntMathTypeTag>(
        &self,
        lhs: &Value<T>,
        rhs: &Value<T>,
        name: &CStr,
    ) -> &'s Value<T> {
        unsafe {
            let ptr = LLVMBuildAdd(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
            Value::from_ref(ptr)
        }
    }

    pub fn nsw_add<T: IntMathTypeTag>(
        &self,
        lhs: &Value<T>,
        rhs: &Value<T>,
        name: &CStr,
    ) -> &'s Value<T> {
        unsafe {
            let ptr = LLVMBuildNSWAdd(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
            Value::from_ref(ptr)
        }
    }

    pub fn nuw_add<T: IntMathTypeTag>(
        &self,
        lhs: &Value<T>,
        rhs: &Value<T>,
        name: &CStr,
    ) -> &'s Value<T> {
        unsafe {
            let ptr = LLVMBuildNUWAdd(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
            Value::from_ref(ptr)
        }
    }

    pub fn float_add<T: FloatMathTypeTag>(
        &self,
        lhs: &Value<T>,
        rhs: &Value<T>,
        name: &CStr,
    ) -> &'s Value<T> {
        unsafe {
            let ptr = LLVMBuildFAdd(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
            Value::from_ref(ptr)
        }
    }

    pub fn sub<T: IntMathTypeTag>(
        &self,
        lhs: &Value<T>,
        rhs: &Value<T>,
        name: &CStr,
    ) -> &'s Value<T> {
        unsafe {
            let ptr = LLVMBuildSub(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
            Value::from_ref(ptr)
        }
    }

    pub fn nsw_sub<T: IntMathTypeTag>(
        &self,
        lhs: &Value<T>,
        rhs: &Value<T>,
        name: &CStr,
    ) -> &'s Value<T> {
        unsafe {
            let ptr = LLVMBuildNSWSub(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
            Value::from_ref(ptr)
        }
    }

    pub fn nuw_sub<T: IntMathTypeTag>(
        &self,
        lhs: &Value<T>,
        rhs: &Value<T>,
        name: &CStr,
    ) -> &'s Value<T> {
        unsafe {
            let ptr = LLVMBuildNUWSub(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
            Value::from_ref(ptr)
        }
    }

    pub fn float_sub<T: FloatMathTypeTag>(
        &self,
        lhs: &Value<T>,
        rhs: &Value<T>,
        name: &CStr,
    ) -> &'s Value<T> {
        unsafe {
            let ptr = LLVMBuildFSub(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
            Value::from_ref(ptr)
        }
    }

    pub fn mul<T: IntMathTypeTag>(
        &self,
        lhs: &Value<T>,
        rhs: &Value<T>,
        name: &CStr,
    ) -> &'s Value<T> {
        unsafe {
            let ptr = LLVMBuildMul(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
            Value::from_ref(ptr)
        }
    }

    pub fn nsw_mul<T: IntMathTypeTag>(
        &self,
        lhs: &Value<T>,
        rhs: &Value<T>,
        name: &CStr,
    ) -> &'s Value<T> {
        unsafe {
            let ptr = LLVMBuildNSWMul(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
            Value::from_ref(ptr)
        }
    }

    pub fn nuw_mul<T: IntMathTypeTag>(
        &self,
        lhs: &Value<T>,
        rhs: &Value<T>,
        name: &CStr,
    ) -> &'s Value<T> {
        unsafe {
            let ptr = LLVMBuildNUWMul(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
            Value::from_ref(ptr)
        }
    }

    pub fn float_mul<T: FloatMathTypeTag>(
        &self,
        lhs: &Value<T>,
        rhs: &Value<T>,
        name: &CStr,
    ) -> &'s Value<T> {
        unsafe {
            let ptr = LLVMBuildFMul(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
            Value::from_ref(ptr)
        }
    }

    pub fn unsigned_div<T: IntMathTypeTag>(
        &self,
        lhs: &Value<T>,
        rhs: &Value<T>,
        name: &CStr,
    ) -> &'s Value<T> {
        unsafe {
            let ptr = LLVMBuildUDiv(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
            Value::from_ref(ptr)
        }
    }

    pub fn exact_unsigned_div<T: IntMathTypeTag>(
        &self,
        lhs: &Value<T>,
        rhs: &Value<T>,
        name: &CStr,
    ) -> &'s Value<T> {
        unsafe {
            let ptr = LLVMBuildExactUDiv(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
            Value::from_ref(ptr)
        }
    }

    pub fn signed_div<T: IntMathTypeTag>(
        &self,
        lhs: &Value<T>,
        rhs: &Value<T>,
        name: &CStr,
    ) -> &'s Value<T> {
        unsafe {
            let ptr = LLVMBuildSDiv(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
            Value::from_ref(ptr)
        }
    }

    pub fn exact_signed_div<T: IntMathTypeTag>(
        &self,
        lhs: &Value<T>,
        rhs: &Value<T>,
        name: &CStr,
    ) -> &'s Value<T> {
        unsafe {
            let ptr = LLVMBuildExactSDiv(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
            Value::from_ref(ptr)
        }
    }

    pub fn float_div<T: FloatMathTypeTag>(
        &self,
        lhs: &Value<T>,
        rhs: &Value<T>,
        name: &CStr,
    ) -> &'s Value<T> {
        unsafe {
            let ptr = LLVMBuildFDiv(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
            Value::from_ref(ptr)
        }
    }

    pub fn unsigned_rem<T: IntMathTypeTag>(
        &self,
        lhs: &Value<T>,
        rhs: &Value<T>,
        name: &CStr,
    ) -> &'s Value<T> {
        unsafe {
            let ptr = LLVMBuildURem(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
            Value::from_ref(ptr)
        }
    }

    pub fn signed_rem<T: IntMathTypeTag>(
        &self,
        lhs: &Value<T>,
        rhs: &Value<T>,
        name: &CStr,
    ) -> &'s Value<T> {
        unsafe {
            let ptr = LLVMBuildSRem(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
            Value::from_ref(ptr)
        }
    }

    pub fn float_rem<T: FloatMathTypeTag>(
        &self,
        lhs: &Value<T>,
        rhs: &Value<T>,
        name: &CStr,
    ) -> &'s Value<T> {
        unsafe {
            let ptr = LLVMBuildFRem(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
            Value::from_ref(ptr)
        }
    }

    pub fn shl<T: IntMathTypeTag>(
        &self,
        lhs: &Value<T>,
        rhs: &Value<T>,
        name: &CStr,
    ) -> &'s Value<T> {
        unsafe {
            let ptr = LLVMBuildShl(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
            Value::from_ref(ptr)
        }
    }

    pub fn logic_shr<T: IntMathTypeTag>(
        &self,
        lhs: &Value<T>,
        rhs: &Value<T>,
        name: &CStr,
    ) -> &'s Value<T> {
        unsafe {
            let ptr = LLVMBuildLShr(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
            Value::from_ref(ptr)
        }
    }

    pub fn arith_shr<T: IntMathTypeTag>(
        &self,
        lhs: &Value<T>,
        rhs: &Value<T>,
        name: &CStr,
    ) -> &'s Value<T> {
        unsafe {
            let ptr = LLVMBuildAShr(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
            Value::from_ref(ptr)
        }
    }

    pub fn and<T: IntMathTypeTag>(
        &self,
        lhs: &Value<T>,
        rhs: &Value<T>,
        name: &CStr,
    ) -> &'s Value<T> {
        unsafe {
            let ptr = LLVMBuildAnd(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
            Value::from_ref(ptr)
        }
    }

    pub fn or<T: IntMathTypeTag>(
        &self,
        lhs: &Value<T>,
        rhs: &Value<T>,
        name: &CStr,
    ) -> &'s Value<T> {
        unsafe {
            let ptr = LLVMBuildOr(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
            Value::from_ref(ptr)
        }
    }

    pub fn xor<T: IntMathTypeTag>(
        &self,
        lhs: &Value<T>,
        rhs: &Value<T>,
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
        lhs: &Value<T>,
        rhs: &Value<T>,
        name: &CStr,
    ) -> &'s Value<T> {
        unsafe {
            let ptr = LLVMBuildBinOp(self.as_raw(), op, lhs.as_raw(), rhs.as_raw(), name.as_ptr());
            Value::from_ref(ptr)
        }
    }

    pub fn neg<T: IntMathTypeTag>(&self, v: &Value<T>, name: &CStr) -> &'s Value<T> {
        unsafe { Value::from_ref(LLVMBuildNeg(self.as_raw(), v.as_raw(), name.as_ptr())) }
    }

    pub fn nsw_neg<T: IntMathTypeTag>(&self, v: &Value<T>, name: &CStr) -> &'s Value<T> {
        unsafe { Value::from_ref(LLVMBuildNSWNeg(self.as_raw(), v.as_raw(), name.as_ptr())) }
    }

    pub fn nuw_neg<T: IntMathTypeTag>(&self, v: &Value<T>, name: &CStr) -> &'s Value<T> {
        unsafe { Value::from_ref(LLVMBuildNUWNeg(self.as_raw(), v.as_raw(), name.as_ptr())) }
    }

    pub fn float_neg<T: FloatMathTypeTag>(&self, v: &Value<T>, name: &CStr) -> &'s Value<T> {
        unsafe { Value::from_ref(LLVMBuildFNeg(self.as_raw(), v.as_raw(), name.as_ptr())) }
    }

    pub fn not<T: IntMathTypeTag>(&self, v: &Value<T>, name: &CStr) -> &'s Value<T> {
        unsafe { Value::from_ref(LLVMBuildNot(self.as_raw(), v.as_raw(), name.as_ptr())) }
    }
}

impl<T: TypeTag> Value<T> {
    pub fn get_nuw(&self) -> bool {
        unsafe { LLVMGetNUW(self.as_raw()) != 0 }
    }

    pub fn set_nuw(&self, set: bool) {
        unsafe { LLVMSetNUW(self.as_raw(), set as _) }
    }

    pub fn get_nsw(&self) -> bool {
        unsafe { LLVMGetNSW(self.as_raw()) != 0 }
    }

    pub fn set_nsw(&self, set: bool) {
        unsafe { LLVMSetNSW(self.as_raw(), set as _) }
    }

    pub fn get_exact(&self) -> bool {
        unsafe { LLVMGetExact(self.as_raw()) != 0 }
    }

    pub fn set_exact(&self, set: bool) {
        unsafe { LLVMSetExact(self.as_raw(), set as _) }
    }

    /// Gets if the instruction has the non-negative flag set.
    pub fn get_non_neg(&self) -> bool {
        unsafe { LLVMGetNNeg(self.as_raw()) != 0 }
    }

    /// Sets the non-negative flag for the instruction.
    pub fn set_non_neg(&self, set: bool) {
        unsafe { LLVMSetNNeg(self.as_raw(), set as _) }
    }

    /// Get the flags for which fast-math-style optimizations are allowed for this value.
    pub fn get_fast_math_flags(&self) -> LLVMFastMathFlags {
        unsafe { LLVMGetFastMathFlags(self.as_raw()) }
    }

    /// Sets the flags for which fast-math-style optimizations are allowed for this value.
    pub fn set_fast_math_flags(&self, set: LLVMFastMathFlags) {
        unsafe { LLVMSetFastMathFlags(self.as_raw(), set) }
    }

    pub fn can_use_fast_math_flags(&self) -> bool {
        unsafe { LLVMCanValueUseFastMathFlags(self.as_raw()) != 0 }
    }

    pub fn get_is_disjoint(&self) -> bool {
        unsafe { LLVMGetIsDisjoint(self.as_raw()) != 0 }
    }

    pub fn set_is_disjoint(&self, is_disjoint: bool) {
        unsafe { LLVMSetIsDisjoint(self.as_raw(), is_disjoint as _) }
    }
}

impl<'s> Builder<'s> {
    pub fn malloc<T: TypeTag>(&self, ty: &Type<T>, name: &CStr) -> &'s Value<T> {
        unsafe { Value::from_ref(LLVMBuildMalloc(self.as_raw(), ty.as_raw(), name.as_ptr())) }
    }

    pub fn array_malloc<T: TypeTag, L: IntTypeTag>(
        &self,
        ty: &Type<T>,
        val: &Value<L>,
        name: &CStr,
    ) -> &'s Value<T> {
        unsafe {
            Value::from_ref(LLVMBuildArrayMalloc(
                self.as_raw(),
                ty.as_raw(),
                val.as_raw(),
                name.as_ptr(),
            ))
        }
    }

    pub fn mem_set<P: PtrTypeTag, T: TypeTag, L: IntTypeTag>(
        &self,
        ptr: &Value<P>,
        val: &Value<T>,
        len: &Value<L>,
        align: u32,
    ) -> &'s Value<void> {
        unsafe {
            Value::from_ref(LLVMBuildMemSet(
                self.as_raw(),
                ptr.as_raw(),
                val.as_raw(),
                len.as_raw(),
                align,
            ))
        }
    }

    pub fn mem_cpy<P: TypeTag, Q: TypeTag, L: IntTypeTag>(
        &self,
        dst: &Value<P>,
        dst_align: u32,
        src: &Value<Q>,
        src_align: u32,
        size: &Value<L>,
    ) -> &'s Value<void> {
        unsafe {
            Value::from_ref(LLVMBuildMemCpy(
                self.as_raw(),
                dst.as_raw(),
                dst_align,
                src.as_raw(),
                src_align,
                size.as_raw(),
            ))
        }
    }

    pub fn mem_move<P: TypeTag, Q: TypeTag, L: IntTypeTag>(
        &self,
        dst: &Value<P>,
        dst_align: u32,
        src: &Value<Q>,
        src_align: u32,
        size: &Value<L>,
    ) -> &'s Value<void> {
        unsafe {
            Value::from_ref(LLVMBuildMemMove(
                self.as_raw(),
                dst.as_raw(),
                dst_align,
                src.as_raw(),
                src_align,
                size.as_raw(),
            ))
        }
    }

    pub fn alloc<T: TypeTag>(&self, ty: &Type<T>, name: &CStr) -> &'s Value<T> {
        unsafe { Value::from_ref(LLVMBuildAlloca(self.as_raw(), ty.as_raw(), name.as_ptr())) }
    }

    pub fn array_alloc<T: TypeTag, L: IntTypeTag>(
        &self,
        ty: &Type<T>,
        val: &Value<L>,
        name: &CStr,
    ) -> &'s Value<T> {
        unsafe {
            Value::from_ref(LLVMBuildArrayAlloca(
                self.as_raw(),
                ty.as_raw(),
                val.as_raw(),
                name.as_ptr(),
            ))
        }
    }

    pub fn free<P: PtrTypeTag>(&self, pointer_val: &Value<P>) -> &'s Value<void> {
        unsafe { Value::from_ref(LLVMBuildFree(self.as_raw(), pointer_val.as_raw())) }
    }

    pub fn load<T: TypeTag, P: PtrTypeTag>(
        &self,
        ty: &Type<T>,
        pointer_val: &Value<P>,
        name: &CStr,
    ) -> &'s Value<T> {
        unsafe {
            Value::from_ref(LLVMBuildLoad2(
                self.as_raw(),
                ty.as_raw(),
                pointer_val.as_raw(),
                name.as_ptr(),
            ))
        }
    }

    pub fn store<T: TypeTag, P: PtrTypeTag>(
        &self,
        val: &Value<T>,
        ptr: &Value<P>,
    ) -> &'s Value<void> {
        unsafe { Value::from_ref(LLVMBuildStore(self.as_raw(), val.as_raw(), ptr.as_raw())) }
    }

    pub fn get_element_ptr<T: ElementTypeTag, P: PtrTypeTag, I: IntTypeTag>(
        &self,
        ty: &Type<T>,
        pointer: &Value<P>,
        indices: &[&Value<I>],
        name: &CStr,
    ) -> &'s Value<any> {
        unsafe {
            Value::from_ref(LLVMBuildGEP2(
                self.as_raw(),
                ty.as_raw(),
                pointer.as_raw(),
                indices.as_ptr() as _,
                indices.len() as _,
                name.as_ptr(),
            ))
        }
    }

    pub fn in_bounds_get_element_ptr<T: TypeTag, P: PtrTypeTag, I: IntTypeTag>(
        &self,
        ty: &Type<T>,
        pointer: &Value<P>,
        indices: &[&Value<I>],
        name: &CStr,
    ) -> &'s Value<any> {
        unsafe {
            Value::from_ref(LLVMBuildInBoundsGEP2(
                self.as_raw(),
                ty.as_raw(),
                pointer.as_raw(),
                indices.as_ptr() as _,
                indices.len() as _,
                name.as_ptr(),
            ))
        }
    }

    pub fn struct_get_element_ptr<P: PtrTypeTag>(
        &self,
        ty: &Type<struct_any>,
        pointer: &Value<P>,
        idx: u32,
        name: &CStr,
    ) -> &'s Value<any> {
        unsafe {
            Value::from_ref(LLVMBuildStructGEP2(
                self.as_raw(),
                ty.as_raw(),
                pointer.as_raw(),
                idx,
                name.as_ptr(),
            ))
        }
    }

    pub fn global_string(&self, str: &CStr, name: &CStr) -> &'s Value<any> {
        unsafe {
            Value::from_ref(LLVMBuildGlobalString(
                self.as_raw(),
                str.as_ptr(),
                name.as_ptr(),
            ))
        }
    }

    pub fn global_string_ptr(&self, str: &CStr, name: &CStr) -> &'s Value<any> {
        unsafe {
            Value::from_ref(LLVMBuildGlobalStringPtr(
                self.as_raw(),
                str.as_ptr(),
                name.as_ptr(),
            ))
        }
    }
}

impl<T: TypeTag> Value<T> {
    pub fn get_volatile(&self) -> bool {
        unsafe { LLVMGetVolatile(self.as_raw()) != 0 }
    }

    pub fn set_volatile(&self, is_volatile: bool) {
        unsafe { LLVMSetVolatile(self.as_raw(), is_volatile as _) }
    }

    pub fn get_weak(&self) -> bool {
        unsafe { LLVMGetWeak(self.as_raw()) != 0 }
    }

    pub fn set_weak(&self, is_weak: bool) {
        unsafe { LLVMSetWeak(self.as_raw(), is_weak as _) }
    }

    pub fn get_ordering(&self) -> LLVMAtomicOrdering {
        unsafe { LLVMGetOrdering(self.as_raw()) }
    }

    pub fn set_ordering(&self, ordering: LLVMAtomicOrdering) {
        unsafe { LLVMSetOrdering(self.as_raw(), ordering) }
    }

    pub fn get_atomic_rmw_bin_op(&self) -> LLVMAtomicRMWBinOp {
        unsafe { LLVMGetAtomicRMWBinOp(self.as_raw()) }
    }

    pub fn set_atomic_rmw_bin_op(&self, bin_op: LLVMAtomicRMWBinOp) {
        unsafe { LLVMSetAtomicRMWBinOp(self.as_raw(), bin_op) }
    }
}

impl<'s> Builder<'s> {
    pub fn trunc<T: TypeTag, D: TypeTag>(
        &self,
        val: &Value<T>,
        dest_ty: &Type<D>,
        name: &CStr,
    ) -> &'s Value<D> {
        unsafe {
            Value::from_ref(LLVMBuildTrunc(
                self.as_raw(),
                val.as_raw(),
                dest_ty.as_raw(),
                name.as_ptr(),
            ))
        }
    }

    // TODO
}

/// Return a constant that specifies that the result of a ShuffleVectorInst is undefined.
pub fn get_undef_mask_elem() -> i32 {
    unsafe { LLVMGetUndefMaskElem() }
}
