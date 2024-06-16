use std::ffi::CStr;

use llvm_sys::core::*;
use llvm_sys::*;

use crate::owning::{OpaqueDrop, Owning};
use crate::type_tag::*;
use crate::{
    BasicBlock, Builder, Context, Instruction, Metadata, Opaque, OperandBundle, Type, Value,
};

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
        unsafe { BasicBlock::from_raw(LLVMGetInsertBlock(self.as_raw())) }
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
        unsafe { Metadata::from_raw(LLVMGetCurrentDebugLocation2(self.as_raw())) }
    }

    /// Set location information used by debugging information.
    pub fn set_current_debug_location(&self, loc: &Metadata) {
        unsafe { LLVMSetCurrentDebugLocation2(self.as_raw(), loc.as_raw()) }
    }

    /// Adds the metadata registered with the given builder to the given instruction.
    pub fn add_metadata_to_inst<T: TypeTag>(&self, inst: &Instruction<T>) {
        unsafe { LLVMAddMetadataToInst(self.as_raw(), inst.as_raw()) }
    }

    /// Get the default floating-point math metadata for a given builder.
    pub fn get_default_fp_math_tag(&self) -> &'s Metadata {
        unsafe { Metadata::from_raw(LLVMBuilderGetDefaultFPMathTag(self.as_raw())) }
    }

    /// Set the default floating-point math metadata for the given builder.
    pub fn set_default_fp_math_tag(&self, fp_math_tag: &Metadata) {
        unsafe { LLVMBuilderSetDefaultFPMathTag(self.as_raw(), fp_math_tag.as_raw()) }
    }

    pub fn return_void(&self) -> &'s Instruction<void> {
        unsafe { Instruction::from_raw(LLVMBuildRetVoid(self.as_raw())) }
    }

    pub fn return_value<T: TypeTag>(&self, value: &Value<T>) -> &'s Instruction<void> {
        unsafe { Instruction::from_raw(LLVMBuildRet(self.as_raw(), value.as_raw())) }
    }

    pub fn return_aggregate<T: TypeTag>(&self, ret_vals: &[&Value<T>]) -> &'s Instruction<void> {
        unsafe {
            Instruction::from_raw(LLVMBuildAggregateRet(
                self.as_raw(),
                ret_vals.as_ptr() as _,
                ret_vals.len() as _,
            ))
        }
    }

    pub fn branch(&self, dest: &BasicBlock) -> &'s Instruction<void> {
        unsafe { Instruction::from_raw(LLVMBuildBr(self.as_raw(), dest.as_raw())) }
    }

    pub fn cond_branch<T: IntTypeTag>(
        &self,
        cond: &Value<T>,
        then: &BasicBlock,
        els: &BasicBlock,
    ) -> &'s Instruction<void> {
        unsafe {
            Instruction::from_raw(LLVMBuildCondBr(
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
    ) -> &'s Instruction<void> {
        unsafe {
            let ptr = LLVMBuildSwitch(self.as_raw(), v.as_raw(), els.as_raw(), cases.len() as u32);
            for &(i, j) in cases {
                LLVMAddCase(ptr, i.as_raw(), j.as_raw());
            }
            Instruction::from_raw(ptr)
        }
    }

    pub fn indirect_branch(
        &self,
        addr: &Value<label>,
        destinations: &[&BasicBlock],
    ) -> &'s Instruction<void> {
        unsafe {
            let ptr = LLVMBuildIndirectBr(self.as_raw(), addr.as_raw(), destinations.len() as u32);
            for &i in destinations {
                LLVMAddDestination(ptr, i.as_raw());
            }
            Instruction::from_raw(ptr)
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
    ) -> &'s Instruction<any> {
        unsafe {
            Instruction::from_raw(LLVMBuildInvoke2(
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
    ) -> &'s Instruction<any> {
        unsafe {
            Instruction::from_raw(LLVMBuildInvokeWithOperandBundles(
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

    pub fn unreachable(&self) -> &'s Instruction<void> {
        unsafe { Instruction::from_raw(LLVMBuildUnreachable(self.as_raw())) }
    }

    pub fn resume<T: TypeTag>(&self, exn: &Value<T>) -> &'s Instruction<any> {
        unsafe { Instruction::from_raw(LLVMBuildResume(self.as_raw(), exn.as_raw())) }
    }

    pub fn landing_pad<F: FunTypeTag>(
        &self,
        ty: &Type<any>,
        pers_fn: &Value<F>,
        clauses: &[&Value<any>],
        name: &CStr,
    ) -> &'s Instruction<any> {
        unsafe {
            let ptr = LLVMBuildLandingPad(
                self.as_raw(),
                ty.as_raw(),
                pers_fn.as_raw(),
                clauses.len() as _,
                name.as_ptr(),
            );
            for &i in clauses {
                LLVMAddClause(ptr, i.as_raw())
            }
            Instruction::from_raw(ptr)
        }
    }

    pub fn cleanup_return(&self, catch_pad: &Value<any>, bb: &BasicBlock) -> &'s Instruction<any> {
        unsafe {
            Instruction::from_raw(LLVMBuildCleanupRet(
                self.as_raw(),
                catch_pad.as_raw(),
                bb.as_raw(),
            ))
        }
    }

    pub fn catch_return(&self, catch_pad: &Value<any>, bb: &BasicBlock) -> &Instruction<void> {
        unsafe {
            Instruction::from_raw(LLVMBuildCatchRet(
                self.as_raw(),
                catch_pad.as_raw(),
                bb.as_raw(),
            ))
        }
    }

    pub fn catch_pad(
        &self,
        parent_pad: &Value<any>,
        args: &[&Value<any>],
        name: &CStr,
    ) -> &Value<any> {
        unsafe {
            Value::from_raw(LLVMBuildCatchPad(
                self.as_raw(),
                parent_pad.as_raw(),
                args.as_ptr() as _,
                args.len() as _,
                name.as_ptr(),
            ))
        }
    }

    pub fn cleanup_pad(
        &self,
        parent_pad: &Value<any>,
        args: &[&Value<any>],
        name: &CStr,
    ) -> &Value<any> {
        unsafe {
            Value::from_raw(LLVMBuildCleanupPad(
                self.as_raw(),
                parent_pad.as_raw(),
                args.as_ptr() as _,
                args.len() as _,
                name.as_ptr(),
            ))
        }
    }

    pub fn catch_switch(
        &self,
        parent_pad: &Value<any>,
        unwind_bb: &BasicBlock,
        handlers: &[&BasicBlock],
        name: &CStr,
    ) -> &Instruction<any> {
        unsafe {
            let ptr = LLVMBuildCatchSwitch(
                self.as_raw(),
                parent_pad.as_raw(),
                unwind_bb.as_raw(),
                handlers.len() as _,
                name.as_ptr(),
            );
            for &i in handlers {
                LLVMAddHandler(ptr, i.as_raw());
            }
            Instruction::from_raw(ptr)
        }
    }
}

impl<T: TypeTag> Instruction<T> {
    pub fn is_cleanup(&self) -> bool {
        unsafe { LLVMIsCleanup(self.as_raw()) != 0 }
    }

    pub fn set_cleanup(&self, val: bool) {
        unsafe { LLVMSetCleanup(self.as_raw(), val as _) }
    }
}

impl<T: TypeTag> Instruction<T> {
    pub fn get_num_handles(&self) -> u32 {
        unsafe { LLVMGetNumHandlers(self.as_raw()) }
    }

    pub fn get_handlers<'a, 's>(
        &'s self,
        slice: &'a mut [Option<&'s BasicBlock>],
    ) -> &'a mut [&'s BasicBlock] {
        assert_eq!(slice.len(), self.get_num_handles() as usize);
        unsafe {
            LLVMGetHandlers(self.as_raw(), slice.as_mut_ptr() as _);
            std::mem::transmute(slice)
        }
    }
}

impl<T: FunTypeTag> Value<T> {
    pub fn get_arg_operand(&self, i: u32) -> &Value<any> {
        unsafe { Value::from_raw(LLVMGetArgOperand(self.as_raw(), i)) }
    }

    pub fn set_arg_operand(&self, i: u32, value: &Value<any>) {
        unsafe { LLVMSetArgOperand(self.as_raw(), i, value.as_raw()) }
    }
}

impl<T: TypeTag> Value<T> {
    pub fn get_parent_catch_switch(&self) -> &Value<any> {
        unsafe { Value::from_raw(LLVMGetParentCatchSwitch(self.as_raw())) }
    }

    pub fn set_parent_catch_switch(&self, catch_switch: &Value<any>) {
        unsafe { LLVMSetParentCatchSwitch(self.as_raw(), catch_switch.as_raw()) }
    }
}

impl<'s> Builder<'s> {
    pub fn add<T: IntMathTypeTag>(
        &self,
        lhs: &Value<T>,
        rhs: &Value<T>,
        name: &CStr,
    ) -> &'s Instruction<T> {
        unsafe {
            let ptr = LLVMBuildAdd(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
            Instruction::from_raw(ptr)
        }
    }

    pub fn nsw_add<T: IntMathTypeTag>(
        &self,
        lhs: &Value<T>,
        rhs: &Value<T>,
        name: &CStr,
    ) -> &'s Instruction<T> {
        unsafe {
            let ptr = LLVMBuildNSWAdd(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
            Instruction::from_raw(ptr)
        }
    }

    pub fn nuw_add<T: IntMathTypeTag>(
        &self,
        lhs: &Value<T>,
        rhs: &Value<T>,
        name: &CStr,
    ) -> &'s Instruction<T> {
        unsafe {
            let ptr = LLVMBuildNUWAdd(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
            Instruction::from_raw(ptr)
        }
    }

    pub fn float_add<T: FloatMathTypeTag>(
        &self,
        lhs: &Value<T>,
        rhs: &Value<T>,
        name: &CStr,
    ) -> &'s Instruction<T> {
        unsafe {
            let ptr = LLVMBuildFAdd(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
            Instruction::from_raw(ptr)
        }
    }

    pub fn sub<T: IntMathTypeTag>(
        &self,
        lhs: &Value<T>,
        rhs: &Value<T>,
        name: &CStr,
    ) -> &'s Instruction<T> {
        unsafe {
            let ptr = LLVMBuildSub(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
            Instruction::from_raw(ptr)
        }
    }

    pub fn nsw_sub<T: IntMathTypeTag>(
        &self,
        lhs: &Value<T>,
        rhs: &Value<T>,
        name: &CStr,
    ) -> &'s Instruction<T> {
        unsafe {
            let ptr = LLVMBuildNSWSub(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
            Instruction::from_raw(ptr)
        }
    }

    pub fn nuw_sub<T: IntMathTypeTag>(
        &self,
        lhs: &Value<T>,
        rhs: &Value<T>,
        name: &CStr,
    ) -> &'s Instruction<T> {
        unsafe {
            let ptr = LLVMBuildNUWSub(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
            Instruction::from_raw(ptr)
        }
    }

    pub fn float_sub<T: FloatMathTypeTag>(
        &self,
        lhs: &Value<T>,
        rhs: &Value<T>,
        name: &CStr,
    ) -> &'s Instruction<T> {
        unsafe {
            let ptr = LLVMBuildFSub(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
            Instruction::from_raw(ptr)
        }
    }

    pub fn mul<T: IntMathTypeTag>(
        &self,
        lhs: &Value<T>,
        rhs: &Value<T>,
        name: &CStr,
    ) -> &'s Instruction<T> {
        unsafe {
            let ptr = LLVMBuildMul(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
            Instruction::from_raw(ptr)
        }
    }

    pub fn nsw_mul<T: IntMathTypeTag>(
        &self,
        lhs: &Value<T>,
        rhs: &Value<T>,
        name: &CStr,
    ) -> &'s Instruction<T> {
        unsafe {
            let ptr = LLVMBuildNSWMul(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
            Instruction::from_raw(ptr)
        }
    }

    pub fn nuw_mul<T: IntMathTypeTag>(
        &self,
        lhs: &Value<T>,
        rhs: &Value<T>,
        name: &CStr,
    ) -> &'s Instruction<T> {
        unsafe {
            let ptr = LLVMBuildNUWMul(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
            Instruction::from_raw(ptr)
        }
    }

    pub fn float_mul<T: FloatMathTypeTag>(
        &self,
        lhs: &Value<T>,
        rhs: &Value<T>,
        name: &CStr,
    ) -> &'s Instruction<T> {
        unsafe {
            let ptr = LLVMBuildFMul(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
            Instruction::from_raw(ptr)
        }
    }

    pub fn unsigned_div<T: IntMathTypeTag>(
        &self,
        lhs: &Value<T>,
        rhs: &Value<T>,
        name: &CStr,
    ) -> &'s Instruction<T> {
        unsafe {
            let ptr = LLVMBuildUDiv(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
            Instruction::from_raw(ptr)
        }
    }

    pub fn exact_unsigned_div<T: IntMathTypeTag>(
        &self,
        lhs: &Value<T>,
        rhs: &Value<T>,
        name: &CStr,
    ) -> &'s Instruction<T> {
        unsafe {
            let ptr = LLVMBuildExactUDiv(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
            Instruction::from_raw(ptr)
        }
    }

    pub fn signed_div<T: IntMathTypeTag>(
        &self,
        lhs: &Value<T>,
        rhs: &Value<T>,
        name: &CStr,
    ) -> &'s Instruction<T> {
        unsafe {
            let ptr = LLVMBuildSDiv(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
            Instruction::from_raw(ptr)
        }
    }

    pub fn exact_signed_div<T: IntMathTypeTag>(
        &self,
        lhs: &Value<T>,
        rhs: &Value<T>,
        name: &CStr,
    ) -> &'s Instruction<T> {
        unsafe {
            let ptr = LLVMBuildExactSDiv(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
            Instruction::from_raw(ptr)
        }
    }

    pub fn float_div<T: FloatMathTypeTag>(
        &self,
        lhs: &Value<T>,
        rhs: &Value<T>,
        name: &CStr,
    ) -> &'s Instruction<T> {
        unsafe {
            let ptr = LLVMBuildFDiv(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
            Instruction::from_raw(ptr)
        }
    }

    pub fn unsigned_rem<T: IntMathTypeTag>(
        &self,
        lhs: &Value<T>,
        rhs: &Value<T>,
        name: &CStr,
    ) -> &'s Instruction<T> {
        unsafe {
            let ptr = LLVMBuildURem(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
            Instruction::from_raw(ptr)
        }
    }

    pub fn signed_rem<T: IntMathTypeTag>(
        &self,
        lhs: &Value<T>,
        rhs: &Value<T>,
        name: &CStr,
    ) -> &'s Instruction<T> {
        unsafe {
            let ptr = LLVMBuildSRem(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
            Instruction::from_raw(ptr)
        }
    }

    pub fn float_rem<T: FloatMathTypeTag>(
        &self,
        lhs: &Value<T>,
        rhs: &Value<T>,
        name: &CStr,
    ) -> &'s Instruction<T> {
        unsafe {
            let ptr = LLVMBuildFRem(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
            Instruction::from_raw(ptr)
        }
    }

    pub fn shl<T: IntMathTypeTag>(
        &self,
        lhs: &Value<T>,
        rhs: &Value<T>,
        name: &CStr,
    ) -> &'s Instruction<T> {
        unsafe {
            let ptr = LLVMBuildShl(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
            Instruction::from_raw(ptr)
        }
    }

    pub fn logic_shr<T: IntMathTypeTag>(
        &self,
        lhs: &Value<T>,
        rhs: &Value<T>,
        name: &CStr,
    ) -> &'s Instruction<T> {
        unsafe {
            let ptr = LLVMBuildLShr(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
            Instruction::from_raw(ptr)
        }
    }

    pub fn arith_shr<T: IntMathTypeTag>(
        &self,
        lhs: &Value<T>,
        rhs: &Value<T>,
        name: &CStr,
    ) -> &'s Instruction<T> {
        unsafe {
            let ptr = LLVMBuildAShr(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
            Instruction::from_raw(ptr)
        }
    }

    pub fn and<T: IntMathTypeTag>(
        &self,
        lhs: &Value<T>,
        rhs: &Value<T>,
        name: &CStr,
    ) -> &'s Instruction<T> {
        unsafe {
            let ptr = LLVMBuildAnd(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
            Instruction::from_raw(ptr)
        }
    }

    pub fn or<T: IntMathTypeTag>(
        &self,
        lhs: &Value<T>,
        rhs: &Value<T>,
        name: &CStr,
    ) -> &'s Instruction<T> {
        unsafe {
            let ptr = LLVMBuildOr(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
            Instruction::from_raw(ptr)
        }
    }

    pub fn xor<T: IntMathTypeTag>(
        &self,
        lhs: &Value<T>,
        rhs: &Value<T>,
        name: &CStr,
    ) -> &'s Instruction<T> {
        unsafe {
            let ptr = LLVMBuildXor(self.as_raw(), lhs.as_raw(), rhs.as_raw(), name.as_ptr());
            Instruction::from_raw(ptr)
        }
    }

    pub unsafe fn binary_op<T: InstanceTypeTag>(
        &self,
        op: LLVMOpcode,
        lhs: &Value<T>,
        rhs: &Value<T>,
        name: &CStr,
    ) -> &'s Instruction<T> {
        unsafe {
            let ptr = LLVMBuildBinOp(self.as_raw(), op, lhs.as_raw(), rhs.as_raw(), name.as_ptr());
            Instruction::from_raw(ptr)
        }
    }

    pub fn neg<T: IntMathTypeTag>(&self, v: &Value<T>, name: &CStr) -> &'s Instruction<T> {
        unsafe { Instruction::from_raw(LLVMBuildNeg(self.as_raw(), v.as_raw(), name.as_ptr())) }
    }

    pub fn nsw_neg<T: IntMathTypeTag>(&self, v: &Value<T>, name: &CStr) -> &'s Instruction<T> {
        unsafe { Instruction::from_raw(LLVMBuildNSWNeg(self.as_raw(), v.as_raw(), name.as_ptr())) }
    }

    pub fn nuw_neg<T: IntMathTypeTag>(&self, v: &Value<T>, name: &CStr) -> &'s Instruction<T> {
        unsafe { Instruction::from_raw(LLVMBuildNUWNeg(self.as_raw(), v.as_raw(), name.as_ptr())) }
    }

    pub fn float_neg<T: FloatMathTypeTag>(&self, v: &Value<T>, name: &CStr) -> &'s Instruction<T> {
        unsafe { Instruction::from_raw(LLVMBuildFNeg(self.as_raw(), v.as_raw(), name.as_ptr())) }
    }

    pub fn not<T: IntMathTypeTag>(&self, v: &Value<T>, name: &CStr) -> &'s Instruction<T> {
        unsafe { Instruction::from_raw(LLVMBuildNot(self.as_raw(), v.as_raw(), name.as_ptr())) }
    }
}

impl<T: TypeTag> Instruction<T> {
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
    pub fn malloc<T: TypeTag>(&self, ty: &Type<T>, name: &CStr) -> &'s Instruction<T> {
        unsafe { Instruction::from_raw(LLVMBuildMalloc(self.as_raw(), ty.as_raw(), name.as_ptr())) }
    }

    pub fn array_malloc<T: TypeTag, L: IntTypeTag>(
        &self,
        ty: &Type<T>,
        val: &Value<L>,
        name: &CStr,
    ) -> &'s Instruction<T> {
        unsafe {
            Instruction::from_raw(LLVMBuildArrayMalloc(
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
    ) -> &'s Instruction<void> {
        unsafe {
            Instruction::from_raw(LLVMBuildMemSet(
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
    ) -> &'s Instruction<void> {
        unsafe {
            Instruction::from_raw(LLVMBuildMemCpy(
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
    ) -> &'s Instruction<void> {
        unsafe {
            Instruction::from_raw(LLVMBuildMemMove(
                self.as_raw(),
                dst.as_raw(),
                dst_align,
                src.as_raw(),
                src_align,
                size.as_raw(),
            ))
        }
    }

    pub fn alloc<T: TypeTag>(&self, ty: &Type<T>, name: &CStr) -> &'s Instruction<T> {
        unsafe { Instruction::from_raw(LLVMBuildAlloca(self.as_raw(), ty.as_raw(), name.as_ptr())) }
    }

    pub fn array_alloc<T: TypeTag, L: IntTypeTag>(
        &self,
        ty: &Type<T>,
        val: &Value<L>,
        name: &CStr,
    ) -> &'s Instruction<T> {
        unsafe {
            Instruction::from_raw(LLVMBuildArrayAlloca(
                self.as_raw(),
                ty.as_raw(),
                val.as_raw(),
                name.as_ptr(),
            ))
        }
    }

    pub fn free<P: PtrTypeTag>(&self, pointer_val: &Value<P>) -> &'s Instruction<void> {
        unsafe { Instruction::from_raw(LLVMBuildFree(self.as_raw(), pointer_val.as_raw())) }
    }

    pub fn load<T: TypeTag, P: PtrTypeTag>(
        &self,
        ty: &Type<T>,
        pointer_val: &Value<P>,
        name: &CStr,
    ) -> &'s Instruction<T> {
        unsafe {
            Instruction::from_raw(LLVMBuildLoad2(
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
    ) -> &'s Instruction<void> {
        unsafe { Instruction::from_raw(LLVMBuildStore(self.as_raw(), val.as_raw(), ptr.as_raw())) }
    }

    pub fn get_element_ptr<T: ElementTypeTag, P: PtrTypeTag, I: IntTypeTag>(
        &self,
        ty: &Type<T>,
        pointer: &Value<P>,
        indices: &[&Value<I>],
        name: &CStr,
    ) -> &'s Instruction<any> {
        unsafe {
            Instruction::from_raw(LLVMBuildGEP2(
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
    ) -> &'s Instruction<any> {
        unsafe {
            Instruction::from_raw(LLVMBuildInBoundsGEP2(
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
    ) -> &'s Instruction<any> {
        unsafe {
            Instruction::from_raw(LLVMBuildStructGEP2(
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
            Value::from_raw(LLVMBuildGlobalString(
                self.as_raw(),
                str.as_ptr(),
                name.as_ptr(),
            ))
        }
    }

    pub fn global_string_ptr(&self, str: &CStr, name: &CStr) -> &'s Value<any> {
        unsafe {
            Value::from_raw(LLVMBuildGlobalStringPtr(
                self.as_raw(),
                str.as_ptr(),
                name.as_ptr(),
            ))
        }
    }
}

impl<T: TypeTag> Instruction<T> {
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
    ) -> &'s Instruction<D> {
        unsafe {
            Instruction::from_raw(LLVMBuildTrunc(
                self.as_raw(),
                val.as_raw(),
                dest_ty.as_raw(),
                name.as_ptr(),
            ))
        }
    }

    // TODO
}

impl<T: TypeTag> Instruction<T> {
    pub fn get_num_mask_elements(&self) -> u32 {
        unsafe { LLVMGetNumMaskElements(self.as_raw()) }
    }
}

/// Return a constant that specifies that the result of a ShuffleVectorInst is undefined.
pub fn get_undef_mask_elem() -> i32 {
    unsafe { LLVMGetUndefMaskElem() }
}

impl<T: TypeTag> Instruction<T> {
    pub fn get_mask_value(&self, elt: u32) -> i32 {
        unsafe { LLVMGetMaskValue(self.as_raw(), elt) }
    }

    pub fn is_atomic_single_thread(&self) -> bool {
        unsafe { LLVMIsAtomicSingleThread(self.as_raw()) != 0 }
    }

    pub fn set_atomic_single_thread(&self, single_thread: bool) {
        unsafe { LLVMSetAtomicSingleThread(self.as_raw(), single_thread as _) }
    }

    pub fn get_cmp_xchg_success_ordering(&self) -> LLVMAtomicOrdering {
        unsafe { LLVMGetCmpXchgSuccessOrdering(self.as_raw()) }
    }

    pub fn set_cmp_xchg_success_ordering(&self, ordering: LLVMAtomicOrdering) {
        unsafe { LLVMSetCmpXchgSuccessOrdering(self.as_raw(), ordering) }
    }

    pub fn get_cmp_xchg_failure_ordering(&self) -> LLVMAtomicOrdering {
        unsafe { LLVMGetCmpXchgFailureOrdering(self.as_raw()) }
    }

    pub fn set_cmp_xchg_failure_ordering(&self, ordering: LLVMAtomicOrdering) {
        unsafe { LLVMSetCmpXchgFailureOrdering(self.as_raw(), ordering) }
    }
}
