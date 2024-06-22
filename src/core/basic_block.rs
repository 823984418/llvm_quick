use std::ffi::CStr;
use std::fmt::{Debug, Formatter};

use llvm_sys::core::*;

use crate::type_tag::*;
use crate::{BasicBlock, Builder, Context, Opaque, Value};

impl Debug for BasicBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.get_name().to_str().unwrap())
    }
}

impl BasicBlock {
    pub fn as_value(&self) -> &Value<label> {
        unsafe { Value::from_raw(LLVMBasicBlockAsValue(self.as_raw())) }
    }
}

impl<T: TypeTag> Value<T> {
    pub fn is_basic_block(&self) -> bool {
        unsafe { LLVMValueIsBasicBlock(self.as_raw()) != 0 }
    }
}

impl Value<label> {
    pub fn as_basic_block(&self) -> &BasicBlock {
        unsafe { BasicBlock::from_raw(LLVMValueAsBasicBlock(self.as_raw())) }
    }
}

impl BasicBlock {
    pub fn get_name(&self) -> &CStr {
        unsafe { CStr::from_ptr(LLVMGetBasicBlockName(self.as_raw())) }
    }

    pub fn get_parent(&self) -> &Value<label> {
        unsafe { Value::from_raw(LLVMGetBasicBlockParent(self.as_raw())) }
    }

    pub fn get_terminator(&self) -> &Value<any> {
        unsafe { Value::from_raw(LLVMGetBasicBlockTerminator(self.as_raw())) }
    }
}

impl<T: FunTypeTag> Value<T> {
    pub fn count_basic_block(&self) -> u32 {
        unsafe { LLVMCountBasicBlocks(self.as_raw()) }
    }

    pub fn get_basic_blocks<'a, 'c>(
        &'c self,
        basic_blocks: &'a mut [Option<&'c BasicBlock>],
    ) -> &'a mut [&'c BasicBlock] {
        assert_eq!(basic_blocks.len(), self.count_basic_block() as usize);
        unsafe {
            LLVMGetBasicBlocks(self.as_raw(), basic_blocks.as_mut_ptr() as _);
            &mut *(basic_blocks as *mut [Option<&BasicBlock>] as *mut [&BasicBlock])
        }
    }

    pub fn get_first_basic_block(&self) -> &BasicBlock {
        unsafe { BasicBlock::from_raw(LLVMGetFirstBasicBlock(self.as_raw())) }
    }

    pub fn get_last_basic_block(&self) -> &BasicBlock {
        unsafe { BasicBlock::from_raw(LLVMGetLastBasicBlock(self.as_raw())) }
    }
}

impl BasicBlock {
    pub fn get_next(&self) -> &Self {
        unsafe { Self::from_raw(LLVMGetNextBasicBlock(self.as_raw())) }
    }

    pub fn get_previous(&self) -> &Self {
        unsafe { Self::from_raw(LLVMGetPreviousBasicBlock(self.as_raw())) }
    }
}

impl<T: FunTypeTag> Value<T> {
    pub fn get_entry_basic_block(&self) -> &BasicBlock {
        unsafe { BasicBlock::from_raw(LLVMGetEntryBasicBlock(self.as_raw())) }
    }
}

impl<'c> Builder<'c> {
    pub fn insert_existing_basic_block_after_insert_block(&self, bb: &BasicBlock) {
        unsafe { LLVMInsertExistingBasicBlockAfterInsertBlock(self.as_raw(), bb.as_raw()) }
    }
}

impl<T: FunTypeTag> Value<T> {
    pub fn append_existing_basic_block(&self, bb: &BasicBlock) {
        unsafe { LLVMAppendExistingBasicBlock(self.as_raw(), bb.as_raw()) }
    }
}

impl Context {
    pub fn create_basic_block(&self, name: &CStr) -> &BasicBlock {
        unsafe { BasicBlock::from_raw(LLVMCreateBasicBlockInContext(self.as_raw(), name.as_ptr())) }
    }

    pub fn append_basic_block<T: FunTypeTag>(&self, f: &Value<T>, name: &CStr) -> &BasicBlock {
        unsafe {
            let ptr = LLVMAppendBasicBlockInContext(self.as_raw(), f.as_raw(), name.as_ptr());
            BasicBlock::from_raw(ptr)
        }
    }

    pub fn insert_basic_block(&self, bb: &BasicBlock, name: &CStr) -> &BasicBlock {
        unsafe {
            BasicBlock::from_raw(LLVMInsertBasicBlockInContext(
                self.as_raw(),
                bb.as_raw(),
                name.as_ptr(),
            ))
        }
    }
}

impl BasicBlock {
    pub fn delete(&self) {
        unsafe { LLVMDeleteBasicBlock(self.as_raw()) }
    }

    pub fn remove_from_parent(&self) {
        unsafe { LLVMRemoveBasicBlockFromParent(self.as_raw()) }
    }

    pub fn move_basic_block_before(&self, move_pos: &BasicBlock) {
        unsafe { LLVMMoveBasicBlockBefore(self.as_raw(), move_pos.as_raw()) }
    }

    pub fn move_basic_block_after(&self, move_pos: &BasicBlock) {
        unsafe { LLVMMoveBasicBlockAfter(self.as_raw(), move_pos.as_raw()) }
    }

    pub fn get_first_instruction(&self) -> &Value<any> {
        unsafe { Value::from_raw(LLVMGetFirstInstruction(self.as_raw())) }
    }

    pub fn get_last_instruction(&self) -> &Value<any> {
        unsafe { Value::from_raw(LLVMGetLastInstruction(self.as_raw())) }
    }
}
