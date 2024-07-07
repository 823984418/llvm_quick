use std::ffi::CStr;
use std::fmt::{Debug, Formatter};
use std::ptr::null_mut;

use llvm_sys::core::*;
use llvm_sys::*;

use crate::core::Message;
use crate::owning::{OpaqueClone, OpaqueDrop, Owning};
use crate::type_tag::*;
use crate::*;

impl<'c> Debug for Module<'c> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.print_to_string().to_str().unwrap())
    }
}

impl Context {
    /// Create a new, empty module in a specific context.
    pub fn create_module(&self, name: &CStr) -> Owning<Module> {
        unsafe {
            let ptr = LLVMModuleCreateWithNameInContext(name.as_ptr(), self.as_raw());
            Owning::from_raw(ptr)
        }
    }
}

impl OpaqueClone for LLVMModule {
    unsafe fn clone_raw(ptr: *mut Self) -> *mut Self {
        unsafe { LLVMCloneModule(ptr) }
    }
}

impl OpaqueDrop for LLVMModule {
    unsafe fn drop_raw(ptr: *mut Self) {
        unsafe { LLVMDisposeModule(ptr) }
    }
}

impl<'c> Module<'c> {
    pub fn get_identifier(&self) -> &[u8] {
        unsafe {
            let mut len = 0;
            let ptr = LLVMGetModuleIdentifier(self.as_raw(), &mut len);
            std::slice::from_raw_parts(ptr as _, len)
        }
    }

    pub fn set_identifier(&self, ident: &[u8]) {
        unsafe { LLVMSetModuleIdentifier(self.as_raw(), ident.as_ptr() as _, ident.len()) }
    }

    pub fn get_source_file_name(&self) -> &[u8] {
        unsafe {
            let mut len = 0;
            let ptr = LLVMGetSourceFileName(self.as_raw(), &mut len);
            std::slice::from_raw_parts(ptr as _, len)
        }
    }

    pub fn set_source_file_name(&self, ident: &[u8]) {
        unsafe { LLVMSetSourceFileName(self.as_raw(), ident.as_ptr() as _, ident.len()) }
    }

    pub fn get_data_layout_str(&self) -> &CStr {
        unsafe { CStr::from_ptr(LLVMGetDataLayoutStr(self.as_raw())) }
    }

    pub fn set_data_layout_str(&self, data_layout_str: &CStr) {
        unsafe { LLVMSetDataLayout(self.as_raw(), data_layout_str.as_ptr()) }
    }

    pub fn get_target(&self) -> &CStr {
        unsafe { CStr::from_ptr(LLVMGetTarget(self.as_raw())) }
    }

    pub fn set_target(&self, triple: &CStr) {
        unsafe { LLVMSetTarget(self.as_raw(), triple.as_ptr()) }
    }

    pub fn copy_flags_metadata(&self) -> ModuleFlagsMetadata {
        unsafe {
            let mut len = 0;
            let ptr = LLVMCopyModuleFlagsMetadata(self.as_raw(), &mut len);
            ModuleFlagsMetadata::from_raw(ptr, len)
        }
    }
}

impl<'m> Drop for ModuleFlagsMetadata<'m> {
    fn drop(&mut self) {
        unsafe { LLVMDisposeModuleFlagsMetadata(self.as_raw()) }
    }
}

impl<'m> ModuleFlagsMetadata<'m> {
    pub fn get_flag_behavior(&self, index: u32) -> LLVMModuleFlagBehavior {
        assert!((index as usize) < self.len);
        unsafe { LLVMModuleFlagEntriesGetFlagBehavior(self.as_raw() as _, index) }
    }

    pub fn get_key(&self, index: u32) -> &[u8] {
        assert!((index as usize) < self.len);
        unsafe {
            let mut len = 0;
            let ptr = LLVMModuleFlagEntriesGetKey(self.as_raw() as _, index, &mut len);
            std::slice::from_raw_parts(ptr as _, len)
        }
    }

    pub fn get_metadata(&self, index: u32) -> &Metadata {
        assert!((index as usize) < self.len);
        unsafe { Metadata::from_raw(LLVMModuleFlagEntriesGetMetadata(self.as_raw() as _, index)) }
    }
}

impl<'c> Module<'c> {
    pub fn get_flag(&self, key: &[u8]) -> &Metadata {
        unsafe {
            Metadata::from_raw(LLVMGetModuleFlag(
                self.as_raw(),
                key.as_ptr() as _,
                key.len(),
            ))
        }
    }

    pub fn add_flag(&self, behavior: LLVMModuleFlagBehavior, key: &[u8], val: &Metadata) {
        unsafe {
            LLVMAddModuleFlag(
                self.as_raw(),
                behavior,
                key.as_ptr() as _,
                key.len(),
                val.as_raw(),
            )
        }
    }

    pub fn dump(&self) {
        unsafe { LLVMDumpModule(self.as_raw()) }
    }

    pub fn print_to_file(&self, filename: &CStr) -> Result<(), Message> {
        unsafe {
            let mut error = null_mut();
            if LLVMPrintModuleToFile(self.as_raw(), filename.as_ptr(), &mut error) != 0 {
                Err(Message::from_raw(error))
            } else {
                Ok(())
            }
        }
    }

    pub fn print_to_string(&self) -> Message {
        unsafe { Message::from_raw(LLVMPrintModuleToString(self.as_raw())) }
    }

    pub fn get_inline_asm_raw(&self) -> *const [u8] {
        unsafe {
            let mut len = 0;
            let ptr = LLVMGetModuleInlineAsm(self.as_raw(), &mut len);
            std::ptr::slice_from_raw_parts(ptr as _, len)
        }
    }

    pub fn get_inline_asm(&self) -> Vec<u8> {
        unsafe { Vec::from(&*self.get_inline_asm_raw()) }
    }

    pub fn set_inline_asm(&self, asm: &[u8]) {
        unsafe { LLVMSetModuleInlineAsm2(self.as_raw(), asm.as_ptr() as _, asm.len()) }
    }

    pub fn append_inline_asm(&self, asm: &[u8]) {
        unsafe { LLVMAppendModuleInlineAsm(self.as_raw(), asm.as_ptr() as _, asm.len()) }
    }
}

impl<T: TypeTag> Type<T> {
    pub fn get_inline_asm(
        &self,
        asm_string: &[u8],
        constraints: &[u8],
        has_side_effects: bool,
        is_align_stack: bool,
        dialect: LLVMInlineAsmDialect,
        can_throw: bool,
    ) -> &Value<T> {
        unsafe {
            Value::from_raw(LLVMGetInlineAsm(
                self.as_raw(),
                asm_string.as_ptr() as _,
                asm_string.len(),
                constraints.as_ptr() as _,
                constraints.len(),
                has_side_effects as _,
                is_align_stack as _,
                dialect,
                can_throw as _,
            ))
        }
    }
}

impl<T: TypeTag> Value<T> {
    pub fn get_inline_asm_constrain_string(&self) -> &[u8] {
        unsafe {
            let mut len = 0;
            let ptr = LLVMGetInlineAsmConstraintString(self.as_raw(), &mut len);
            std::slice::from_raw_parts(ptr as _, len)
        }
    }

    pub fn get_inline_asm_dialect(&self) -> LLVMInlineAsmDialect {
        unsafe { LLVMGetInlineAsmDialect(self.as_raw()) }
    }

    pub fn get_inline_asm_function_type(&self) -> &Type<T> {
        unsafe { Type::from_raw(LLVMGetInlineAsmFunctionType(self.as_raw())) }
    }

    pub fn get_inline_asm_has_side_effects(&self) -> bool {
        unsafe { LLVMGetInlineAsmHasSideEffects(self.as_raw()) != 0 }
    }

    pub fn get_inline_asm_needs_aligned_stack(&self) -> bool {
        unsafe { LLVMGetInlineAsmNeedsAlignedStack(self.as_raw()) != 0 }
    }

    pub fn get_inline_asm_can_unwind(&self) -> bool {
        unsafe { LLVMGetInlineAsmCanUnwind(self.as_raw()) != 0 }
    }
}

impl<'c> Module<'c> {
    /// Obtain the context to which this module is associated.
    pub fn context(&self) -> &'c Context {
        unsafe { Context::from_raw(LLVMGetModuleContext(self.as_raw())) }
    }

    pub fn get_first_named_metadata(&self) -> &NamedMDNode {
        unsafe { NamedMDNode::from_raw(LLVMGetFirstNamedMetadata(self.as_raw())) }
    }

    pub fn get_last_named_metadata(&self) -> &NamedMDNode {
        unsafe { NamedMDNode::from_raw(LLVMGetLastNamedMetadata(self.as_raw())) }
    }
}

impl NamedMDNode {
    pub fn get_next(&self) -> &NamedMDNode {
        unsafe { NamedMDNode::from_raw(LLVMGetNextNamedMetadata(self.as_raw())) }
    }

    pub fn get_previous(&self) -> &NamedMDNode {
        unsafe { NamedMDNode::from_raw(LLVMGetPreviousNamedMetadata(self.as_raw())) }
    }
}

impl<'c> Module<'c> {
    pub fn get_named_metadata(&self, name: &[u8]) -> &NamedMDNode {
        unsafe {
            NamedMDNode::from_raw(LLVMGetNamedMetadata(
                self.as_raw(),
                name.as_ptr() as _,
                name.len(),
            ))
        }
    }

    pub fn get_or_insert_named_metadata(&self, name: &[u8]) -> &NamedMDNode {
        unsafe {
            NamedMDNode::from_raw(LLVMGetOrInsertNamedMetadata(
                self.as_raw(),
                name.as_ptr() as _,
                name.len(),
            ))
        }
    }
}

impl NamedMDNode {
    pub fn get_name(&self) -> &[u8] {
        unsafe {
            let mut len = 0;
            let ptr = LLVMGetNamedMetadataName(self.as_raw(), &mut len);
            std::slice::from_raw_parts(ptr as _, len)
        }
    }
}

impl<'c> Module<'c> {
    pub fn get_named_metadata_num_operands(&self, name: &CStr) -> u32 {
        unsafe { LLVMGetNamedMetadataNumOperands(self.as_raw(), name.as_ptr()) }
    }

    pub fn get_named_metadata_operands<'s>(
        &self,
        name: &CStr,
        slice: &'s mut [Option<&'c Value<metadata>>],
    ) -> &'s mut [&'c Value<metadata>] {
        assert_eq!(
            slice.len(),
            self.get_named_metadata_num_operands(name) as usize
        );
        unsafe {
            LLVMGetNamedMetadataOperands(self.as_raw(), name.as_ptr(), slice.as_mut_ptr() as _);
            std::mem::transmute(slice)
        }
    }

    pub fn add_named_metadata_operand(&self, name: &CStr, val: &Value<metadata>) {
        unsafe { LLVMAddNamedMetadataOperand(self.as_raw(), name.as_ptr(), val.as_raw()) }
    }
}

impl<T: TypeTag> Value<T> {
    pub fn get_debug_loc_director(&self) -> &[u8] {
        unsafe {
            let mut len = 0;
            let ptr = LLVMGetDebugLocDirectory(self.as_raw(), &mut len);
            std::slice::from_raw_parts(ptr as _, len as _)
        }
    }

    pub fn get_debug_loc_filename(&self) -> &[u8] {
        unsafe {
            let mut len = 0;
            let ptr = LLVMGetDebugLocFilename(self.as_raw(), &mut len);
            std::slice::from_raw_parts(ptr as _, len as _)
        }
    }

    pub fn get_debug_loc_line(&self) -> u32 {
        unsafe { LLVMGetDebugLocLine(self.as_raw()) }
    }

    pub fn get_debug_loc_column(&self) -> u32 {
        unsafe { LLVMGetDebugLocColumn(self.as_raw()) }
    }
}

impl<'c> Module<'c> {
    /// Add a function to a module under a specified name.
    pub fn add_function<T: FunTypeTag>(&self, name: &CStr, ty: &'c Type<T>) -> &'c Function<T> {
        unsafe { Function::from_raw(LLVMAddFunction(self.as_raw(), name.as_ptr(), ty.as_raw())) }
    }

    /// Obtain a Function value from a Module by its name.
    pub fn get_named_function<T: FunTypeTag>(&self, name: &CStr) -> &'c Function<T> {
        unsafe {
            Function::<fun_any>::from_raw(LLVMGetNamedFunction(self.as_raw(), name.as_ptr())).cast()
        }
    }

    pub fn get_first_function(&self) -> &'c Function<fun_any> {
        unsafe { Function::from_raw(LLVMGetFirstFunction(self.as_raw())) }
    }

    pub fn get_last_function(&self) -> &'c Function<fun_any> {
        unsafe { Function::from_raw(LLVMGetLastFunction(self.as_raw())) }
    }
}

impl<T: FunTypeTag> Function<T> {
    pub fn get_next_function(&self) -> &Function<fun_any> {
        unsafe { Function::from_raw(LLVMGetNextFunction(self.as_raw())) }
    }

    pub fn get_previous_function(&self) -> &Function<fun_any> {
        unsafe { Function::from_raw(LLVMGetPreviousFunction(self.as_raw())) }
    }
}
