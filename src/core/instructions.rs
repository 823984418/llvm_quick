use std::ptr::null_mut;

use llvm_sys::core::*;
use llvm_sys::*;

use crate::opaque::Opaque;
use crate::type_tag::{any, fun_any, metadata, TypeTag};
use crate::{Attribute, BasicBlock, Type, Value, ValueMetadataEntries};

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

    pub fn set_instr_param_alignment(&self, idx: LLVMAttributeIndex, align: u32) {
        unsafe { LLVMSetInstrParamAlignment(self.as_raw(), idx, align) }
    }

    pub fn add_call_site_attribute(&self, idx: LLVMAttributeIndex, a: &Attribute) {
        unsafe { LLVMAddCallSiteAttribute(self.as_raw(), idx, a.as_raw()) }
    }

    pub fn get_call_site_attribute_count(&self, idx: LLVMAttributeIndex) -> u32 {
        unsafe { LLVMGetCallSiteAttributeCount(self.as_raw(), idx) }
    }

    pub fn get_call_site_attributes<'a, 's>(
        &'s self,
        idx: LLVMAttributeIndex,
        slice: &'a mut [Option<&'s Attribute>],
    ) -> &'a mut [&'s Attribute] {
        unsafe {
            LLVMGetCallSiteAttributes(self.as_raw(), idx, slice.as_mut_ptr() as _);
            std::mem::transmute(slice)
        }
    }

    pub fn get_call_site_enum_attribute(
        &self,
        idx: LLVMAttributeIndex,
        kind_id: u32,
    ) -> &Attribute {
        unsafe { Attribute::from_ref(LLVMGetCallSiteEnumAttribute(self.as_raw(), idx, kind_id)) }
    }

    pub fn get_call_site_string_attribute(&self, idx: LLVMAttributeIndex, k: &[u8]) -> &Attribute {
        unsafe {
            Attribute::from_ref(LLVMGetCallSiteStringAttribute(
                self.as_raw(),
                idx,
                k.as_ptr() as _,
                k.len() as _,
            ))
        }
    }

    pub fn remove_call_site_enum_attribute(&self, idx: LLVMAttributeIndex, kind_id: u32) {
        unsafe { LLVMRemoveCallSiteEnumAttribute(self.as_raw(), idx, kind_id) }
    }

    pub fn remove_call_site_string_attribute(&self, idx: LLVMAttributeIndex, k: &[u8]) {
        unsafe {
            LLVMRemoveCallSiteStringAttribute(self.as_raw(), idx, k.as_ptr() as _, k.len() as _)
        }
    }

    pub fn get_called_function_type(&self) -> &Type<fun_any> {
        unsafe { Type::from_ref(LLVMGetCalledFunctionType(self.as_raw())) }
    }

    pub fn get_called_value(&self) -> &Value<fun_any> {
        unsafe { Value::from_ref(LLVMGetCalledValue(self.as_raw())) }
    }

    pub fn get_num_operand_bundles(&self) -> u32 {
        unsafe { LLVMGetNumOperandBundles(self.as_raw()) }
    }
}

// TODO
