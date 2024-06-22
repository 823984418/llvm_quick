use std::ptr::null_mut;

use llvm_sys::core::*;
use llvm_sys::*;

use crate::opaque::Opaque;
use crate::type_tag::{any, fun_any, metadata, TypeTag};
use crate::{Attribute, BasicBlock, Instruction, OperandBundle, Type, Value, ValueMetadataEntries};

impl<T: TypeTag> Instruction<T> {
    pub fn has_metadata(&self) -> bool {
        unsafe { LLVMHasMetadata(self.as_raw()) != 0 }
    }

    pub fn get_metadata(&self, kind_id: u32) -> Option<&Value<metadata>> {
        unsafe { Value::from_ptr(LLVMGetMetadata(self.as_raw(), kind_id)) }
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
            ValueMetadataEntries::from_raw(ptr, len)
        }
    }

    pub fn get_instruction_parent(&self) -> &BasicBlock {
        unsafe { BasicBlock::from_raw(LLVMGetInstructionParent(self.as_raw())) }
    }

    pub fn get_next_instruction(&self) -> Option<&Value<any>> {
        unsafe { Value::from_ptr(LLVMGetNextInstruction(self.as_raw())) }
    }

    pub fn get_previous_instruction(&self) -> Option<&Value<any>> {
        unsafe { Value::from_ptr(LLVMGetPreviousInstruction(self.as_raw())) }
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
        unsafe { Value::from_raw(LLVMInstructionClone(self.as_raw())) }
    }

    pub fn is_a_terminator_inst(&self) -> Option<&Value<T>> {
        unsafe { Value::from_ptr(LLVMIsATerminatorInst(self.as_raw())) }
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

    pub fn get_call_site_attributes<'s, 'c>(
        &'c self,
        idx: LLVMAttributeIndex,
        slice: &'s mut [Option<&'c Attribute>],
    ) -> &'s mut [&'c Attribute] {
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
        unsafe { Attribute::from_raw(LLVMGetCallSiteEnumAttribute(self.as_raw(), idx, kind_id)) }
    }

    pub fn get_call_site_string_attribute(&self, idx: LLVMAttributeIndex, k: &[u8]) -> &Attribute {
        unsafe {
            Attribute::from_raw(LLVMGetCallSiteStringAttribute(
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

    pub fn get_called_function_type(&self) -> Option<&Type<fun_any>> {
        unsafe { Type::from_ptr(LLVMGetCalledFunctionType(self.as_raw())) }
    }

    pub fn get_called_value(&self) -> &Value<fun_any> {
        unsafe { Value::from_raw(LLVMGetCalledValue(self.as_raw())) }
    }

    pub fn get_num_operand_bundles(&self) -> u32 {
        unsafe { LLVMGetNumOperandBundles(self.as_raw()) }
    }

    pub fn get_operand_bundle_as_index(&self, index: u32) -> &OperandBundle {
        unsafe { OperandBundle::from_raw(LLVMGetOperandBundleAtIndex(self.as_raw(), index)) }
    }

    pub fn is_tail_call(&self) -> bool {
        unsafe { LLVMIsTailCall(self.as_raw()) != 0 }
    }

    pub fn set_tail_call(&self, is_tail_call: bool) {
        unsafe { LLVMSetTailCall(self.as_raw(), is_tail_call as _) }
    }

    pub fn get_tail_call_kind(&self) -> LLVMTailCallKind {
        unsafe { LLVMGetTailCallKind(self.as_raw()) }
    }

    pub fn set_tail_call_kind(&self, kind: LLVMTailCallKind) {
        unsafe { LLVMSetTailCallKind(self.as_raw(), kind) }
    }

    pub fn get_normal_dest(&self) -> &BasicBlock {
        unsafe { BasicBlock::from_raw(LLVMGetNormalDest(self.as_raw())) }
    }

    pub fn get_unwind_dest(&self) -> &BasicBlock {
        unsafe { BasicBlock::from_raw(LLVMGetUnwindDest(self.as_raw())) }
    }

    pub fn set_normal_dest(&self, b: &BasicBlock) {
        unsafe { LLVMSetNormalDest(self.as_raw(), b.as_raw()) }
    }

    pub fn set_unwind_dest(&self, b: &BasicBlock) {
        unsafe { LLVMSetUnwindDest(self.as_raw(), b.as_raw()) }
    }
}

impl<T: TypeTag> Instruction<T> {
    pub fn get_num_successors(&self) -> u32 {
        unsafe { LLVMGetNumSuccessors(self.as_raw()) }
    }

    pub fn get_successor(&self, i: u32) -> &BasicBlock {
        unsafe { BasicBlock::from_raw(LLVMGetSuccessor(self.as_raw(), i)) }
    }

    pub fn set_successor(&self, i: u32, block: &BasicBlock) {
        unsafe { LLVMSetSuccessor(self.as_raw(), i, block.as_raw()) }
    }

    pub fn is_conditional(&self) -> bool {
        unsafe { LLVMIsConditional(self.as_raw()) != 0 }
    }

    pub fn get_conditional(&self) -> &Value<any> {
        unsafe { Value::from_raw(LLVMGetCondition(self.as_raw())) }
    }

    pub fn set_conditional(&self, cond: &Value<any>) {
        unsafe { LLVMSetCondition(self.as_raw(), cond.as_raw()) }
    }

    pub fn get_switch_default_dest(&self) -> &BasicBlock {
        unsafe { BasicBlock::from_raw(LLVMGetSwitchDefaultDest(self.as_raw())) }
    }
}

impl<T: TypeTag> Instruction<T> {
    pub fn get_allocated_type(&self) -> &Type<any> {
        unsafe { Type::from_raw(LLVMGetAllocatedType(self.as_raw())) }
    }
}

impl<T: TypeTag> Instruction<T> {
    pub fn is_in_bounds(&self) -> bool {
        unsafe { LLVMIsInBounds(self.as_raw()) != 0 }
    }

    pub fn set_is_in_bounds(&self, in_bounds: bool) {
        unsafe { LLVMSetIsInBounds(self.as_raw(), in_bounds as _) }
    }

    pub fn get_gep_source_element_type(&self) -> &Type<any> {
        unsafe { Type::from_raw(LLVMGetGEPSourceElementType(self.as_raw())) }
    }
}

impl<T: TypeTag> Instruction<T> {
    pub fn add_incoming(&self, v: &[&Value<any>], b: &[&BasicBlock]) {
        assert_eq!(v.len(), b.len());
        unsafe {
            LLVMAddIncoming(
                self.as_raw(),
                v.as_ptr() as _,
                b.as_ptr() as _,
                v.len() as _,
            )
        }
    }

    pub fn count_incoming(&self) -> u32 {
        unsafe { LLVMCountIncoming(self.as_raw()) }
    }

    pub fn get_incoming_value(&self, index: u32) -> &Value<any> {
        unsafe { Value::from_raw(LLVMGetIncomingValue(self.as_raw(), index)) }
    }

    pub fn get_incoming_block(&self, index: u32) -> &BasicBlock {
        unsafe { BasicBlock::from_raw(LLVMGetIncomingBlock(self.as_raw(), index)) }
    }
}
