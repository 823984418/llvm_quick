use std::fmt::{Debug, Formatter};
use std::ops::Deref;

use llvm_sys::core::*;
use llvm_sys::*;

use crate::core::Message;
use crate::type_tag::*;
use crate::*;

pub mod constants;
pub mod function;
pub mod usage;
pub mod user_value;

impl<T: TypeTag> Debug for Value<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.print_to_string().to_str().unwrap())
    }
}

impl<T: TypeTag> Value<T> {
    pub fn to_any(&self) -> &Value<any> {
        self.cast()
    }
}

impl<T: TypeTag> Debug for Argument<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self.deref(), f)
    }
}

impl<T: TypeTag> Argument<T> {
    pub fn to_any(&self) -> &Argument<any> {
        self.cast()
    }
}

impl<T: TypeTag> Debug for Constant<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self.deref(), f)
    }
}

impl<T: TypeTag> Constant<T> {
    pub fn to_any(&self) -> &Constant<any> {
        self.cast()
    }
}

impl<T: TypeTag> Debug for GlobalValue<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self.deref(), f)
    }
}

impl<T: TypeTag> GlobalValue<T> {
    pub fn to_any(&self) -> &GlobalValue<any> {
        self.cast()
    }
}

impl<T: TypeTag> Debug for Instruction<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self.deref(), f)
    }
}

impl<T: TypeTag> Instruction<T> {
    pub fn to_any(&self) -> &Instruction<any> {
        self.cast()
    }
}

impl<T: TypeTag> Value<T> {
    pub fn get_kind(&self) -> LLVMValueKind {
        unsafe { LLVMGetValueKind(self.as_raw()) }
    }

    /// Obtain the type of a value.
    pub fn get_type(&self) -> &Type<T> {
        unsafe { Type::from_raw(LLVMTypeOf(self.as_raw())) }
    }

    /// Obtain the string name of a value.
    pub fn get_name(&self) -> &[u8] {
        unsafe {
            let mut len = 0;
            let ptr = LLVMGetValueName2(self.as_raw(), &mut len);
            std::slice::from_raw_parts(ptr.cast(), len)
        }
    }

    /// Set the string name of a value.
    pub fn set_name(&self, name: &[u8]) {
        unsafe { LLVMSetValueName2(self.as_raw(), name.as_ptr() as _, name.len()) }
    }

    pub fn dump(&self) {
        unsafe { LLVMDumpValue(self.as_raw()) }
    }

    /// Return a string representation of the value.
    pub fn print_to_string(&self) -> Message {
        unsafe { Message::from_raw(LLVMPrintValueToString(self.as_raw())) }
    }

    pub fn replace_all_uses_with(&self, new: &Self)
    where
        T: InstanceTypeTag,
    {
        unsafe { LLVMReplaceAllUsesWith(self.as_raw(), new.as_raw()) }
    }

    /// Determine whether the specified value instance is constant.
    pub fn is_constant(&self) -> bool {
        unsafe { LLVMIsConstant(self.as_raw()) != 0 }
    }

    /// Determine whether a value instance is undefined.
    pub fn is_undef(&self) -> bool {
        unsafe { LLVMIsUndef(self.as_raw()) != 0 }
    }

    /// Determine whether a value instance is poisonous.
    pub fn is_poison(&self) -> bool {
        unsafe { LLVMIsPoison(self.as_raw()) != 0 }
    }

    pub fn is_a_metadata_node(&self) -> Option<&Value<metadata>> {
        unsafe { Value::from_ptr(LLVMIsAMDNode(self.as_raw())) }
    }

    pub fn is_a_value_as_metadata(&self) -> Option<&Value<metadata>> {
        unsafe { Value::from_ptr(LLVMIsAValueAsMetadata(self.as_raw())) }
    }

    pub fn is_a_metadata_string(&self) -> Option<&Value<metadata>> {
        unsafe { Value::from_ptr(LLVMIsAMDString(self.as_raw())) }
    }
}

impl<T: TypeTag> Value<T> {
    pub fn is_a_argument(&self) -> Option<&Argument<T>> {
        unsafe { Argument::from_ptr(LLVMIsAArgument(self.as_raw())) }
    }

    pub fn is_a_basic_block(&self) -> Option<&Value<T>> {
        unsafe { Value::from_ptr(LLVMIsABasicBlock(self.as_raw())) }
    }

    pub fn is_a_inline_asm(&self) -> Option<&Value<T>> {
        unsafe { Value::from_ptr(LLVMIsAInlineAsm(self.as_raw())) }
    }

    pub fn is_a_user(&self) -> Option<&Value<T>> {
        unsafe { Value::from_ptr(LLVMIsAUser(self.as_raw())) }
    }

    pub fn is_a_constant(&self) -> Option<&Constant<T>> {
        unsafe { Constant::from_ptr(LLVMIsAConstant(self.as_raw())) }
    }

    pub fn is_a_block_address(&self) -> Option<&Value<T>> {
        unsafe { Value::from_ptr(LLVMIsABlockAddress(self.as_raw())) }
    }

    pub fn is_a_constant_aggregate_zero(&self) -> Option<&Value<T>> {
        unsafe { Value::from_ptr(LLVMIsAConstantAggregateZero(self.as_raw())) }
    }

    pub fn is_a_constant_array(&self) -> Option<&Value<T>> {
        unsafe { Value::from_ptr(LLVMIsAConstantArray(self.as_raw())) }
    }

    pub fn is_a_constant_data_sequential(&self) -> Option<&Value<T>> {
        unsafe { Value::from_ptr(LLVMIsAConstantDataSequential(self.as_raw())) }
    }

    pub fn is_a_constant_data_array(&self) -> Option<&Value<T>> {
        unsafe { Value::from_ptr(LLVMIsAConstantDataArray(self.as_raw())) }
    }

    pub fn is_a_constant_data_vector(&self) -> Option<&Value<T>> {
        unsafe { Value::from_ptr(LLVMIsAConstantDataVector(self.as_raw())) }
    }

    pub fn is_a_constant_expr(&self) -> Option<&Value<T>> {
        unsafe { Value::from_ptr(LLVMIsAConstantExpr(self.as_raw())) }
    }

    pub fn is_a_constant_fp(&self) -> Option<&Value<T>> {
        unsafe { Value::from_ptr(LLVMIsAConstantFP(self.as_raw())) }
    }

    pub fn is_a_constant_int(&self) -> Option<&Value<T>> {
        unsafe { Value::from_ptr(LLVMIsAConstantInt(self.as_raw())) }
    }

    pub fn is_a_constant_pointer_null(&self) -> Option<&Value<T>> {
        unsafe { Value::from_ptr(LLVMIsAConstantPointerNull(self.as_raw())) }
    }

    pub fn is_a_constant_struct(&self) -> Option<&Value<T>> {
        unsafe { Value::from_ptr(LLVMIsAConstantStruct(self.as_raw())) }
    }

    pub fn is_a_constant_token_none(&self) -> Option<&Value<T>> {
        unsafe { Value::from_ptr(LLVMIsAConstantTokenNone(self.as_raw())) }
    }

    pub fn is_a_constant_vector(&self) -> Option<&Value<T>> {
        unsafe { Value::from_ptr(LLVMIsAConstantVector(self.as_raw())) }
    }

    pub fn is_a_global_value(&self) -> Option<&GlobalValue<any>> {
        unsafe { GlobalValue::from_ptr(LLVMIsAGlobalValue(self.as_raw())) }
    }

    pub fn is_a_global_alias(&self) -> Option<&GlobalAlias<any>> {
        unsafe { GlobalAlias::from_ptr(LLVMIsAGlobalAlias(self.as_raw())) }
    }

    pub fn is_a_global_i_func(&self) -> Option<&Value<T>> {
        unsafe { Value::from_ptr(LLVMIsAGlobalIFunc(self.as_raw())) }
    }

    pub fn is_a_global_object(&self) -> Option<&Value<T>> {
        unsafe { Value::from_ptr(LLVMIsAGlobalObject(self.as_raw())) }
    }

    pub fn is_a_function(&self) -> Option<&Value<T>> {
        unsafe { Value::from_ptr(LLVMIsAFunction(self.as_raw())) }
    }

    pub fn is_a_global_variable(&self) -> Option<&Value<T>> {
        unsafe { Value::from_ptr(LLVMIsAGlobalVariable(self.as_raw())) }
    }

    pub fn is_a_undef_value(&self) -> Option<&Value<T>> {
        unsafe { Value::from_ptr(LLVMIsAUndefValue(self.as_raw())) }
    }

    pub fn is_a_poison_value(&self) -> Option<&Value<T>> {
        unsafe { Value::from_ptr(LLVMIsAPoisonValue(self.as_raw())) }
    }

    pub fn is_a_instruction(&self) -> Option<&Instruction<T>> {
        unsafe { Instruction::from_ptr(LLVMIsAInstruction(self.as_raw())) }
    }

    pub fn is_a_unary_operator(&self) -> Option<&Value<T>> {
        unsafe { Value::from_ptr(LLVMIsAUnaryOperator(self.as_raw())) }
    }

    pub fn is_a_binary_operator(&self) -> Option<&Value<T>> {
        unsafe { Value::from_ptr(LLVMIsABinaryOperator(self.as_raw())) }
    }

    pub fn is_a_call_inst(&self) -> Option<&Instruction<T>> {
        unsafe { Instruction::from_ptr(LLVMIsACallInst(self.as_raw())) }
    }

    pub fn is_a_intrinsic_inst(&self) -> Option<&Instruction<T>> {
        unsafe { Instruction::from_ptr(LLVMIsAIntrinsicInst(self.as_raw())) }
    }

    pub fn is_a_dbg_info_intrinsic(&self) -> Option<&Value<T>> {
        unsafe { Value::from_ptr(LLVMIsADbgInfoIntrinsic(self.as_raw())) }
    }

    pub fn is_a_dbg_variable_intrinsic(&self) -> Option<&Value<T>> {
        unsafe { Value::from_ptr(LLVMIsADbgVariableIntrinsic(self.as_raw())) }
    }

    pub fn is_a_dbg_declare_inst(&self) -> Option<&Value<T>> {
        unsafe { Value::from_ptr(LLVMIsADbgDeclareInst(self.as_raw())) }
    }

    pub fn is_a_dbg_label_inst(&self) -> Option<&Value<T>> {
        unsafe { Value::from_ptr(LLVMIsADbgLabelInst(self.as_raw())) }
    }

    pub fn is_a_mem_intrinsic(&self) -> Option<&Value<T>> {
        unsafe { Value::from_ptr(LLVMIsAMemIntrinsic(self.as_raw())) }
    }

    pub fn is_a_mem_cpy_inst(&self) -> Option<&Instruction<T>> {
        unsafe { Instruction::from_ptr(LLVMIsAMemCpyInst(self.as_raw())) }
    }

    pub fn is_a_mem_move_inst(&self) -> Option<&Instruction<T>> {
        unsafe { Instruction::from_ptr(LLVMIsAMemMoveInst(self.as_raw())) }
    }

    pub fn is_a_mem_set_inst(&self) -> Option<&Instruction<T>> {
        unsafe { Instruction::from_ptr(LLVMIsAMemSetInst(self.as_raw())) }
    }

    pub fn is_a_cmp_inst(&self) -> Option<&Instruction<T>> {
        unsafe { Instruction::from_ptr(LLVMIsACmpInst(self.as_raw())) }
    }

    pub fn is_a_f_cmp_inst(&self) -> Option<&Instruction<T>> {
        unsafe { Instruction::from_ptr(LLVMIsAFCmpInst(self.as_raw())) }
    }

    pub fn is_a_i_cmp_inst(&self) -> Option<&Instruction<T>> {
        unsafe { Instruction::from_ptr(LLVMIsAICmpInst(self.as_raw())) }
    }

    pub fn is_a_extract_element_inst(&self) -> Option<&Instruction<T>> {
        unsafe { Instruction::from_ptr(LLVMIsAExtractElementInst(self.as_raw())) }
    }

    pub fn is_a_get_element_ptr_inst(&self) -> Option<&Instruction<T>> {
        unsafe { Instruction::from_ptr(LLVMIsAGetElementPtrInst(self.as_raw())) }
    }

    pub fn is_a_insert_element_inst(&self) -> Option<&Instruction<T>> {
        unsafe { Instruction::from_ptr(LLVMIsAInsertElementInst(self.as_raw())) }
    }

    pub fn is_a_insert_value_inst(&self) -> Option<&Instruction<T>> {
        unsafe { Instruction::from_ptr(LLVMIsAInsertValueInst(self.as_raw())) }
    }

    pub fn is_a_landing_pad_inst(&self) -> Option<&Instruction<T>> {
        unsafe { Instruction::from_ptr(LLVMIsALandingPadInst(self.as_raw())) }
    }

    pub fn is_a_phi_node(&self) -> Option<&Value<T>> {
        unsafe { Value::from_ptr(LLVMIsAPHINode(self.as_raw())) }
    }

    pub fn is_a_select_inst(&self) -> Option<&Instruction<T>> {
        unsafe { Instruction::from_ptr(LLVMIsASelectInst(self.as_raw())) }
    }

    pub fn is_a_shuffle_vector_inst(&self) -> Option<&Instruction<T>> {
        unsafe { Instruction::from_ptr(LLVMIsAShuffleVectorInst(self.as_raw())) }
    }

    pub fn is_a_store_inst(&self) -> Option<&Instruction<T>> {
        unsafe { Instruction::from_ptr(LLVMIsAStoreInst(self.as_raw())) }
    }

    pub fn is_a_branch_inst(&self) -> Option<&Instruction<T>> {
        unsafe { Instruction::from_ptr(LLVMIsABranchInst(self.as_raw())) }
    }

    pub fn is_a_indirect_br_inst(&self) -> Option<&Instruction<T>> {
        unsafe { Instruction::from_ptr(LLVMIsAIndirectBrInst(self.as_raw())) }
    }

    pub fn is_a_invoke_inst(&self) -> Option<&Instruction<T>> {
        unsafe { Instruction::from_ptr(LLVMIsAInvokeInst(self.as_raw())) }
    }

    pub fn is_a_return_inst(&self) -> Option<&Instruction<T>> {
        unsafe { Instruction::from_ptr(LLVMIsAReturnInst(self.as_raw())) }
    }

    pub fn is_a_switch_inst(&self) -> Option<&Instruction<T>> {
        unsafe { Instruction::from_ptr(LLVMIsASwitchInst(self.as_raw())) }
    }

    pub fn is_a_unreachable_inst(&self) -> Option<&Instruction<T>> {
        unsafe { Instruction::from_ptr(LLVMIsAUnreachableInst(self.as_raw())) }
    }

    pub fn is_a_resume_inst(&self) -> Option<&Instruction<T>> {
        unsafe { Instruction::from_ptr(LLVMIsAResumeInst(self.as_raw())) }
    }

    pub fn is_a_cleanup_return_inst(&self) -> Option<&Instruction<T>> {
        unsafe { Instruction::from_ptr(LLVMIsACleanupReturnInst(self.as_raw())) }
    }

    pub fn is_a_catch_return_inst(&self) -> Option<&Instruction<T>> {
        unsafe { Instruction::from_ptr(LLVMIsACatchReturnInst(self.as_raw())) }
    }

    pub fn is_a_catch_switch_inst(&self) -> Option<&Instruction<T>> {
        unsafe { Instruction::from_ptr(LLVMIsACatchSwitchInst(self.as_raw())) }
    }

    pub fn is_a_call_br_inst(&self) -> Option<&Instruction<T>> {
        unsafe { Instruction::from_ptr(LLVMIsACallBrInst(self.as_raw())) }
    }

    pub fn is_a_funclet_pad_inst(&self) -> Option<&Instruction<T>> {
        unsafe { Instruction::from_ptr(LLVMIsAFuncletPadInst(self.as_raw())) }
    }

    pub fn is_a_catch_pad_inst(&self) -> Option<&Instruction<T>> {
        unsafe { Instruction::from_ptr(LLVMIsACatchPadInst(self.as_raw())) }
    }

    pub fn is_a_cleanup_pad_inst(&self) -> Option<&Instruction<T>> {
        unsafe { Instruction::from_ptr(LLVMIsACleanupPadInst(self.as_raw())) }
    }

    pub fn is_a_unary_instruction(&self) -> Option<&Instruction<T>> {
        unsafe { Instruction::from_ptr(LLVMIsAUnaryInstruction(self.as_raw())) }
    }

    pub fn is_a_alloca_inst(&self) -> Option<&Instruction<T>> {
        unsafe { Instruction::from_ptr(LLVMIsAAllocaInst(self.as_raw())) }
    }

    pub fn is_a_cast_inst(&self) -> Option<&Instruction<T>> {
        unsafe { Instruction::from_ptr(LLVMIsACastInst(self.as_raw())) }
    }

    pub fn is_a_addr_space_cast_inst(&self) -> Option<&Instruction<T>> {
        unsafe { Instruction::from_ptr(LLVMIsAAddrSpaceCastInst(self.as_raw())) }
    }

    pub fn is_a_bit_cast_inst(&self) -> Option<&Instruction<T>> {
        unsafe { Instruction::from_ptr(LLVMIsABitCastInst(self.as_raw())) }
    }

    pub fn is_a_fp_ext_inst(&self) -> Option<&Instruction<T>> {
        unsafe { Instruction::from_ptr(LLVMIsAFPExtInst(self.as_raw())) }
    }

    pub fn is_a_fp_to_si_inst(&self) -> Option<&Instruction<T>> {
        unsafe { Instruction::from_ptr(LLVMIsAFPToSIInst(self.as_raw())) }
    }

    pub fn is_a_fp_to_ui_inst(&self) -> Option<&Instruction<T>> {
        unsafe { Instruction::from_ptr(LLVMIsAFPToUIInst(self.as_raw())) }
    }

    pub fn is_a_fp_trunc_inst(&self) -> Option<&Instruction<T>> {
        unsafe { Instruction::from_ptr(LLVMIsAFPTruncInst(self.as_raw())) }
    }

    pub fn is_a_int_to_ptr_inst(&self) -> Option<&Instruction<T>> {
        unsafe { Instruction::from_ptr(LLVMIsAIntToPtrInst(self.as_raw())) }
    }

    pub fn is_a_ptr_to_int_inst(&self) -> Option<&Instruction<T>> {
        unsafe { Instruction::from_ptr(LLVMIsAPtrToIntInst(self.as_raw())) }
    }

    pub fn is_a_s_ext_inst(&self) -> Option<&Instruction<T>> {
        unsafe { Instruction::from_ptr(LLVMIsASExtInst(self.as_raw())) }
    }

    pub fn is_a_si_to_fp_inst(&self) -> Option<&Instruction<T>> {
        unsafe { Instruction::from_ptr(LLVMIsASIToFPInst(self.as_raw())) }
    }

    pub fn is_a_trunc_inst(&self) -> Option<&Instruction<T>> {
        unsafe { Instruction::from_ptr(LLVMIsATruncInst(self.as_raw())) }
    }

    pub fn is_a_ui_to_fp_inst(&self) -> Option<&Instruction<T>> {
        unsafe { Instruction::from_ptr(LLVMIsAUIToFPInst(self.as_raw())) }
    }

    pub fn is_a_z_ext_inst(&self) -> Option<&Instruction<T>> {
        unsafe { Instruction::from_ptr(LLVMIsAZExtInst(self.as_raw())) }
    }

    pub fn is_a_extract_value_inst(&self) -> Option<&Instruction<T>> {
        unsafe { Instruction::from_ptr(LLVMIsAExtractValueInst(self.as_raw())) }
    }

    pub fn is_a_load_inst(&self) -> Option<&Instruction<T>> {
        unsafe { Instruction::from_ptr(LLVMIsALoadInst(self.as_raw())) }
    }

    pub fn is_a_va_arg_inst(&self) -> Option<&Instruction<T>> {
        unsafe { Instruction::from_ptr(LLVMIsAVAArgInst(self.as_raw())) }
    }

    pub fn is_a_freeze_inst(&self) -> Option<&Instruction<T>> {
        unsafe { Instruction::from_ptr(LLVMIsAFreezeInst(self.as_raw())) }
    }

    pub fn is_a_atomic_cmp_xchg_inst(&self) -> Option<&Instruction<T>> {
        unsafe { Instruction::from_ptr(LLVMIsAAtomicCmpXchgInst(self.as_raw())) }
    }

    pub fn is_a_atomic_rmw_inst(&self) -> Option<&Instruction<T>> {
        unsafe { Instruction::from_ptr(LLVMIsAAtomicRMWInst(self.as_raw())) }
    }

    pub fn is_a_fence_inst(&self) -> Option<&Instruction<T>> {
        unsafe { Instruction::from_ptr(LLVMIsAFenceInst(self.as_raw())) }
    }
}

impl<T: TypeTag> Value<T> {
    pub fn get_num_indices(&self) -> u32 {
        unsafe { LLVMGetNumIndices(self.as_raw()) }
    }

    pub fn get_indices(&self) -> &[u32] {
        unsafe {
            let ptr = LLVMGetIndices(self.as_raw());
            std::slice::from_raw_parts(ptr, self.get_num_indices() as _)
        }
    }
}
