use std::ffi::CStr;

use llvm_sys::target::*;

use crate::core::context::Context;
use crate::core::module::Module;
use crate::core::pass_manager::PassManager;
use crate::core::type_tag::integers::int_any;
use crate::core::type_tag::TypeTag;
use crate::core::types::Type;
use crate::core::values::Value;
use crate::core::Message;
use crate::opaque::{Opaque, PhantomOpaque};
use crate::owning::{OpaqueDrop, Owning};

#[repr(transparent)]
pub struct TargetData {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for TargetData {
    type Inner = LLVMOpaqueTargetData;
}

impl OpaqueDrop for TargetData {
    unsafe fn drop_raw(ptr: *mut Self::Inner) {
        unsafe { LLVMDisposeTargetData(ptr) };
    }
}

#[repr(transparent)]
pub struct TargetLibraryInfo {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for TargetLibraryInfo {
    type Inner = LLVMOpaqueTargetLibraryInfotData;
}

impl<'s> Module<'s> {
    pub fn get_data_layout(&self) -> &TargetData {
        unsafe { TargetData::from_ref(LLVMGetModuleDataLayout(self.as_raw())) }
    }

    pub fn set_data_layout(&self, v: &TargetData) {
        unsafe { LLVMSetModuleDataLayout(self.as_raw(), v.as_raw()) };
    }
}

impl TargetData {
    pub fn create(rep: &CStr) -> Owning<Self> {
        unsafe { Owning::from_raw(LLVMCreateTargetData(rep.as_ptr())) }
    }
}

impl PassManager {
    pub fn add_target_library_info(&self, info: &TargetLibraryInfo) {
        unsafe { LLVMAddTargetLibraryInfo(info.as_raw(), self.as_raw()) }
    }
}

impl TargetData {
    /// Converts target data to a target layout string.
    pub fn get_rep(&self) -> Message {
        unsafe { Message::from_raw(LLVMCopyStringRepOfTargetData(self.as_raw())) }
    }

    pub fn get_byte_order(&self) -> LLVMByteOrdering {
        unsafe { LLVMByteOrder(self.as_raw()) }
    }

    pub fn get_pointer_size(&self) -> u32 {
        unsafe { LLVMPointerSize(self.as_raw()) }
    }

    pub fn int_ptr_type<'s>(&self) -> &'s Type<int_any> {
        unsafe { Type::from_ref(LLVMIntPtrType(self.as_raw())) }
    }

    pub fn int_ptr_type_for_address_space<'s>(&self, address_space: u32) -> &'s Type<int_any> {
        unsafe { Type::from_ref(LLVMIntPtrTypeForAS(self.as_raw(), address_space)) }
    }

    pub fn int_ptr_type_in_context<'s>(&self, context: &'s Context) -> &'s Type<int_any> {
        unsafe { Type::from_ref(LLVMIntPtrTypeInContext(context.as_raw(), self.as_raw())) }
    }

    pub fn int_ptr_type_for_address_space_in_context<'s>(
        &self,
        context: &'s Context,
        space: u32,
    ) -> &'s Type<int_any> {
        unsafe {
            let ptr = LLVMIntPtrTypeForASInContext(context.as_raw(), self.as_raw(), space);
            Type::from_ref(ptr)
        }
    }

    pub fn get_size_of_type_in_bits<T: TypeTag>(&self, ty: &Type<T>) -> u64 {
        unsafe { LLVMSizeOfTypeInBits(self.as_raw(), ty.as_raw()) }
    }

    pub fn get_store_size_of_type<T: TypeTag>(&self, ty: &Type<T>) -> u64 {
        unsafe { LLVMStoreSizeOfType(self.as_raw(), ty.as_raw()) }
    }

    pub fn get_abi_size_of_type<T: TypeTag>(&self, ty: &Type<T>) -> u64 {
        unsafe { LLVMABISizeOfType(self.as_raw(), ty.as_raw()) }
    }

    pub fn get_abi_alignment_of_type<T: TypeTag>(&self, ty: &Type<T>) -> u32 {
        unsafe { LLVMABIAlignmentOfType(self.as_raw(), ty.as_raw()) }
    }

    pub fn get_call_frame_alignment_of_type<T: TypeTag>(&self, ty: &Type<T>) -> u32 {
        unsafe { LLVMCallFrameAlignmentOfType(self.as_raw(), ty.as_raw()) }
    }

    pub fn get_preferred_alignment_of_type<T: TypeTag>(&self, ty: &Type<T>) -> u32 {
        unsafe { LLVMPreferredAlignmentOfType(self.as_raw(), ty.as_raw()) }
    }

    pub fn get_preferred_alignment_of_global<T: TypeTag>(&self, ty: &Value<T>) -> u32 {
        unsafe { LLVMPreferredAlignmentOfGlobal(self.as_raw(), ty.as_raw()) }
    }
    // FIXME: LLVMElementAtOffset
    // FIXME: LLVMOffsetOfElement
}

pub fn initialize_all_target_infos() {
    unsafe { LLVM_InitializeAllTargetInfos() };
}
pub fn initialize_all_targets() {
    unsafe { LLVM_InitializeAllTargets() };
}
pub fn initialize_all_target_mcs() {
    unsafe { LLVM_InitializeAllTargetMCs() };
}
pub fn initialize_all_asm_printers() {
    unsafe { LLVM_InitializeAllAsmPrinters() };
}
pub fn initialize_all_asm_parsers() {
    unsafe { LLVM_InitializeAllAsmParsers() };
}
pub fn initialize_all_disassemblers() {
    unsafe { LLVM_InitializeAllDisassemblers() };
}

pub fn initialize_native_target() -> bool {
    unsafe { LLVM_InitializeNativeTarget() != 0 }
}
pub fn initialize_native_asm_parser() -> bool {
    unsafe { LLVM_InitializeNativeAsmParser() != 0 }
}
pub fn initialize_native_asm_printer() -> bool {
    unsafe { LLVM_InitializeNativeAsmPrinter() != 0 }
}
pub fn initialize_native_disassembler() -> bool {
    unsafe { LLVM_InitializeNativeDisassembler() != 0 }
}
