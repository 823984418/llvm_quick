use std::ptr::null_mut;

use llvm_sys::core::*;
use llvm_sys::{LLVMIntPredicate, LLVMOpcode, LLVMRealPredicate};

use crate::opaque::Opaque;
use crate::type_tag::{any, metadata, TypeTag};
use crate::{BasicBlock, Value, ValueMetadataEntries};

impl<T: TypeTag> Value<T> {
    pub fn has_metadata(&self) -> bool {
        unsafe { LLVMHasMetadata(self.as_raw()) != 0 }
    }

    pub fn get_metadata(&self, kind_id: u32) -> Option<&Value<metadata>> {
        unsafe { Value::try_from_ref(LLVMGetMetadata(self.as_raw(), kind_id)) }
    }

    pub fn set_metadata(&self, kind_id: u32, node: Option<&Value<metadata>>) {
        unsafe {
            LLVMSetMetadata(
                self.as_raw(),
                kind_id,
                node.map(Value::as_raw).unwrap_or(null_mut()),
            )
        }
    }

    pub fn instruction_get_all_metadata_other_than_debug_loc(&self) -> ValueMetadataEntries {
        unsafe {
            let mut len = 0;
            let ptr = LLVMInstructionGetAllMetadataOtherThanDebugLoc(self.as_raw(), &mut len);
            ValueMetadataEntries::from_raw(std::ptr::slice_from_raw_parts_mut(ptr as _, len))
        }
    }

    pub fn get_instruction_parent(&self) -> &BasicBlock {
        unsafe { BasicBlock::from_ref(LLVMGetInstructionParent(self.as_raw())) }
    }

    pub fn get_next_instruction(&self) -> Option<&Value<any>> {
        unsafe { Value::try_from_ref(LLVMGetNextInstruction(self.as_raw())) }
    }

    pub fn get_previous_instruction(&self) -> Option<&Value<any>> {
        unsafe { Value::try_from_ref(LLVMGetPreviousInstruction(self.as_raw())) }
    }

    pub fn instruction_remove_from_parent(&self) {
        unsafe { LLVMInstructionRemoveFromParent(self.as_raw()) }
    }

    pub unsafe fn instruction_erase_from_parent(&self) {
        unsafe { LLVMInstructionEraseFromParent(self.as_raw()) }
    }

    pub unsafe fn delete_instruction(&self) {
        unsafe { LLVMDeleteInstruction(self.as_raw()) }
    }

    pub fn get_instruction_opcode(&self) -> LLVMOpcode {
        unsafe { LLVMGetInstructionOpcode(self.as_raw()) }
    }

    pub fn get_i_cmp_predicate(&self) -> LLVMIntPredicate {
        unsafe { LLVMGetICmpPredicate(self.as_raw()) }
    }

    pub fn get_f_cmp_predicate(&self) -> LLVMRealPredicate {
        unsafe { LLVMGetFCmpPredicate(self.as_raw()) }
    }

    pub fn instruction_clone(&self) -> &Value<T> {
        unsafe { Value::from_ref(LLVMInstructionClone(self.as_raw())) }
    }

    pub fn is_a_terminator_inst(&self) -> Option<&Value<T>> {
        unsafe { Value::try_from_ref(LLVMIsATerminatorInst(self.as_raw())) }
    }

    pub fn get_num_arg_operands(&self) -> u32 {
        unsafe { LLVMGetNumArgOperands(self.as_raw()) }
    }

    pub fn set_instruction_call_conv(&self, cc: u32) {
        unsafe { LLVMSetInstructionCallConv(self.as_raw(), cc) }
    }

    pub fn get_instruction_call_conv(&self) -> u32 {
        unsafe { LLVMGetInstructionCallConv(self.as_raw()) }
    }
}

// TODO
