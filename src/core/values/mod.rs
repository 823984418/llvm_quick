use std::ffi::CString;
use std::fmt::{Debug, Formatter};

use llvm_sys::core::*;
use llvm_sys::*;

use crate::core::Message;
use crate::type_tag::*;
use crate::{Opaque, Type, Value};

pub mod functions;

impl<T: TypeTag> Debug for Value<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.print_to_string().to_str().unwrap())
    }
}

impl<T: TypeTag> Value<T> {
    pub unsafe fn cast_unchecked<N: TypeTag>(&self) -> &Value<N> {
        unsafe { Value::from_ref(self.as_raw()) }
    }

    pub fn try_cast<N: TypeTag>(&self) -> Option<&Value<N>> {
        if self.get_type().try_cast::<N>().is_some() {
            Some(unsafe { self.cast_unchecked() })
        } else {
            None
        }
    }

    pub fn cast<N: TypeTag>(&self) -> &Value<N> {
        self.try_cast().unwrap()
    }

    pub fn to_any(&self) -> &Value<any> {
        unsafe { self.cast_unchecked() }
    }

    pub fn get_kind(&self) -> LLVMValueKind {
        unsafe { LLVMGetValueKind(self.as_raw()) }
    }

    /// Obtain the type of a value.
    pub fn get_type(&self) -> &Type<T> {
        unsafe { Type::from_ref(LLVMTypeOf(self.as_raw())) }
    }

    /// Obtain the string name of a value.
    pub fn get_name(&self) -> *const [u8] {
        unsafe {
            let mut len = 0;
            let s = LLVMGetValueName2(self.as_raw(), &mut len);
            std::ptr::slice_from_raw_parts(s.cast(), len)
        }
    }

    /// Obtain the string name of a value.
    pub fn get_name_string(&self) -> CString {
        unsafe { CString::new(&*self.get_name()).unwrap() }
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
        unsafe { Value::try_from_ref(LLVMIsAMDNode(self.as_raw())) }
    }

    pub fn is_a_value_as_metadata(&self) -> Option<&Value<metadata>> {
        unsafe { Value::try_from_ref(LLVMIsAValueAsMetadata(self.as_raw())) }
    }

    pub fn is_a_metadata_string(&self) -> Option<&Value<metadata>> {
        unsafe { Value::try_from_ref(LLVMIsAMDString(self.as_raw())) }
    }
}

impl<T: TypeTag> Value<T> {
    pub fn is_a_argument(&self) -> Option<&Value<T>> {
        unsafe { Value::try_from_ref(LLVMIsAArgument(self.as_raw())) }
    }

    pub fn is_a_basic_block(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsABasicBlock(self.as_raw())) }
    }

    pub fn is_a_inline_asm(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsAInlineAsm(self.as_raw())) }
    }

    pub fn is_a_user(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsAUser(self.as_raw())) }
    }

    pub fn is_a_constant(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsAConstant(self.as_raw())) }
    }

    pub fn is_a_block_address(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsABlockAddress(self.as_raw())) }
    }

    pub fn is_a_constant_aggregate_zero(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsAConstantAggregateZero(self.as_raw())) }
    }

    pub fn is_a_constant_array(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsAConstantArray(self.as_raw())) }
    }

    pub fn is_a_constant_data_sequential(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsAConstantDataSequential(self.as_raw())) }
    }

    pub fn is_a_constant_data_array(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsAConstantDataArray(self.as_raw())) }
    }

    pub fn is_a_constant_data_vector(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsAConstantDataVector(self.as_raw())) }
    }

    pub fn is_a_constant_expr(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsAConstantExpr(self.as_raw())) }
    }

    pub fn is_a_constant_fp(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsAConstantFP(self.as_raw())) }
    }

    pub fn is_a_constant_int(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsAConstantInt(self.as_raw())) }
    }

    pub fn is_a_constant_pointer_null(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsAConstantPointerNull(self.as_raw())) }
    }

    pub fn is_a_constant_struct(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsAConstantStruct(self.as_raw())) }
    }

    pub fn is_a_constant_token_none(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsAConstantTokenNone(self.as_raw())) }
    }

    pub fn is_a_constant_vector(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsAConstantVector(self.as_raw())) }
    }

    pub fn is_a_global_value(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsAGlobalValue(self.as_raw())) }
    }

    pub fn is_a_global_alias(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsAGlobalAlias(self.as_raw())) }
    }

    pub fn is_a_global_i_func(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsAGlobalIFunc(self.as_raw())) }
    }

    pub fn is_a_global_object(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsAGlobalObject(self.as_raw())) }
    }

    pub fn is_a_function(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsAFunction(self.as_raw())) }
    }

    pub fn is_a_global_variable(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsAGlobalVariable(self.as_raw())) }
    }

    pub fn is_a_undef_value(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsAUndefValue(self.as_raw())) }
    }

    pub fn is_a_poison_value(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsAPoisonValue(self.as_raw())) }
    }

    pub fn is_a_instruction(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsAInstruction(self.as_raw())) }
    }

    pub fn is_a_unary_operator(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsAUnaryOperator(self.as_raw())) }
    }

    pub fn is_a_binary_operator(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsABinaryOperator(self.as_raw())) }
    }

    pub fn is_a_call_inst(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsACallInst(self.as_raw())) }
    }

    pub fn is_a_intrinsic_inst(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsAIntrinsicInst(self.as_raw())) }
    }

    pub fn is_a_dbg_info_intrinsic(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsADbgInfoIntrinsic(self.as_raw())) }
    }

    pub fn is_a_dbg_variable_intrinsic(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsADbgVariableIntrinsic(self.as_raw())) }
    }

    pub fn is_a_dbg_declare_inst(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsADbgDeclareInst(self.as_raw())) }
    }

    pub fn is_a_dbg_label_inst(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsADbgLabelInst(self.as_raw())) }
    }

    pub fn is_a_mem_intrinsic(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsAMemIntrinsic(self.as_raw())) }
    }

    pub fn is_a_mem_cpy_inst(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsAMemCpyInst(self.as_raw())) }
    }

    pub fn is_a_mem_move_inst(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsAMemMoveInst(self.as_raw())) }
    }

    pub fn is_a_mem_set_inst(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsAMemSetInst(self.as_raw())) }
    }

    pub fn is_a_cmp_inst(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsACmpInst(self.as_raw())) }
    }

    pub fn is_a_f_cmp_inst(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsAFCmpInst(self.as_raw())) }
    }

    pub fn is_a_icmp_inst(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsAICmpInst(self.as_raw())) }
    }

    pub fn is_a_extract_element_inst(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsAExtractElementInst(self.as_raw())) }
    }

    pub fn is_a_get_element_ptr_inst(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsAGetElementPtrInst(self.as_raw())) }
    }

    pub fn is_a_insert_element_inst(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsAInsertElementInst(self.as_raw())) }
    }

    pub fn is_a_insert_value_inst(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsAInsertValueInst(self.as_raw())) }
    }

    pub fn is_a_landing_pad_inst(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsALandingPadInst(self.as_raw())) }
    }

    pub fn is_a_phi_node(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsAPHINode(self.as_raw())) }
    }

    pub fn is_a_select_inst(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsASelectInst(self.as_raw())) }
    }

    pub fn is_a_shuffle_vector_inst(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsAShuffleVectorInst(self.as_raw())) }
    }

    pub fn is_a_store_inst(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsAStoreInst(self.as_raw())) }
    }

    pub fn is_a_branch_inst(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsABranchInst(self.as_raw())) }
    }

    pub fn is_a_indirect_br_inst(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsAIndirectBrInst(self.as_raw())) }
    }

    pub fn is_a_invoke_inst(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsAInvokeInst(self.as_raw())) }
    }

    pub fn is_a_return_inst(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsAReturnInst(self.as_raw())) }
    }

    pub fn is_a_switch_inst(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsASwitchInst(self.as_raw())) }
    }

    pub fn is_a_unreachable_inst(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsAUnreachableInst(self.as_raw())) }
    }

    pub fn is_a_resume_inst(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsAResumeInst(self.as_raw())) }
    }

    pub fn is_a_cleanup_return_inst(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsACleanupReturnInst(self.as_raw())) }
    }

    pub fn is_a_catch_return_inst(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsACatchReturnInst(self.as_raw())) }
    }

    pub fn is_a_catch_switch_inst(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsACatchSwitchInst(self.as_raw())) }
    }

    pub fn is_a_call_br_inst(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsACallBrInst(self.as_raw())) }
    }

    pub fn is_a_funclet_pad_inst(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsAFuncletPadInst(self.as_raw())) }
    }

    pub fn is_a_catch_pad_inst(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsACatchPadInst(self.as_raw())) }
    }

    pub fn is_a_cleanup_pad_inst(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsACleanupPadInst(self.as_raw())) }
    }

    pub fn is_a_unary_instruction(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsAUnaryInstruction(self.as_raw())) }
    }

    pub fn is_a_alloca_inst(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsAAllocaInst(self.as_raw())) }
    }

    pub fn is_a_cast_inst(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsACastInst(self.as_raw())) }
    }

    pub fn is_a_addr_space_cast_inst(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsAAddrSpaceCastInst(self.as_raw())) }
    }

    pub fn is_a_bit_cast_inst(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsABitCastInst(self.as_raw())) }
    }

    pub fn is_a_fp_ext_inst(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsAFPExtInst(self.as_raw())) }
    }

    pub fn is_a_fp_to_si_inst(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsAFPToSIInst(self.as_raw())) }
    }

    pub fn is_a_fp_to_ui_inst(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsAFPToUIInst(self.as_raw())) }
    }

    pub fn is_a_fp_trunc_inst(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsAFPTruncInst(self.as_raw())) }
    }

    pub fn is_a_int_to_ptr_inst(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsAIntToPtrInst(self.as_raw())) }
    }

    pub fn is_a_ptr_to_int_inst(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsAPtrToIntInst(self.as_raw())) }
    }

    pub fn is_a_s_ext_inst(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsASExtInst(self.as_raw())) }
    }

    pub fn is_a_si_to_fp_inst(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsASIToFPInst(self.as_raw())) }
    }

    pub fn is_a_trunc_inst(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsATruncInst(self.as_raw())) }
    }

    pub fn is_a_ui_to_fp_inst(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsAUIToFPInst(self.as_raw())) }
    }

    pub fn is_a_z_ext_inst(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsAZExtInst(self.as_raw())) }
    }

    pub fn is_a_extract_value_inst(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsAExtractValueInst(self.as_raw())) }
    }

    pub fn is_a_load_inst(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsALoadInst(self.as_raw())) }
    }

    pub fn is_a_va_arg_inst(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsAVAArgInst(self.as_raw())) }
    }

    pub fn is_a_freeze_inst(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsAFreezeInst(self.as_raw())) }
    }

    pub fn is_a_atomic_cmp_xchg_inst(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsAAtomicCmpXchgInst(self.as_raw())) }
    }

    pub fn is_a_atomic_rmw_inst(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsAAtomicRMWInst(self.as_raw())) }
    }

    pub fn is_a_fence_inst(&self) -> Option<&Value<label>> {
        unsafe { Value::try_from_ref(LLVMIsAFenceInst(self.as_raw())) }
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
