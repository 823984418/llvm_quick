use std::ptr::null_mut;

use llvm_sys::core::*;

use crate::core::ValueMetadataEntries;
use crate::opaque::Opaque;
use crate::type_tag::{metadata, TypeTag};
use crate::{BasicBlock, Value};

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
}
