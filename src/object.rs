use std::ffi::CStr;
use std::marker::PhantomData;
use std::ptr::null_mut;

use llvm_sys::object::*;

use crate::core::Message;
use crate::owning::{OpaqueDrop, Owning};
use crate::*;

#[repr(transparent)]
pub struct SectionIterator<'b, 'm> {
    _opaque: PhantomOpaque,
    _marker: PhantomData<&'b Binary<'m>>,
}

unsafe impl<'b, 'm> Opaque for SectionIterator<'b, 'm> {
    type Inner = LLVMOpaqueSectionIterator;
}

#[repr(transparent)]
pub struct SymbolIterator<'b, 'm> {
    _opaque: PhantomOpaque,
    _marker: PhantomData<&'b Binary<'m>>,
}

unsafe impl<'b, 'm> Opaque for SymbolIterator<'b, 'm> {
    type Inner = LLVMOpaqueSymbolIterator;
}

#[repr(transparent)]
pub struct RelocationIterator<'b, 'm> {
    _opaque: PhantomOpaque,
    _marker: PhantomData<&'b Binary<'m>>,
}

unsafe impl<'b, 'm> Opaque for RelocationIterator<'b, 'm> {
    type Inner = LLVMOpaqueRelocationIterator;
}

#[repr(transparent)]
pub struct Binary<'m> {
    _opaque: PhantomOpaque,
    _marker: PhantomData<&'m MemoryBuffer>,
}

unsafe impl<'m> Opaque for Binary<'m> {
    type Inner = LLVMOpaqueBinary;
}

impl<'m> Binary<'m> {
    pub fn create(mem_buf: &'m MemoryBuffer, context: &Context) -> Result<Owning<Self>, Message> {
        unsafe {
            let mut error = null_mut();
            let ptr = LLVMCreateBinary(mem_buf.as_raw(), context.as_raw(), &mut error);
            if ptr.is_null() {
                Err(Message::from_raw(error))
            } else {
                Ok(Owning::from_raw(ptr))
            }
        }
    }
}

impl OpaqueDrop for LLVMOpaqueBinary {
    unsafe fn drop_raw(ptr: *mut Self) {
        unsafe { LLVMDisposeBinary(ptr) }
    }
}

impl<'m> Binary<'m> {
    pub fn copy_memory_buffer(&self) -> Owning<MemoryBuffer> {
        unsafe { Owning::from_raw(LLVMBinaryCopyMemoryBuffer(self.as_raw())) }
    }

    pub fn get_type(&self) -> LLVMBinaryType {
        unsafe { LLVMBinaryGetType(self.as_raw()) }
    }

    pub fn mach_o_universal_binary_copy_object_for_arch(
        &self,
        arch: &[u8],
    ) -> Result<Owning<Binary<'m>>, Message> {
        unsafe {
            let mut error = null_mut();
            let ptr = LLVMMachOUniversalBinaryCopyObjectForArch(
                self.as_raw(),
                arch.as_ptr() as _,
                arch.len(),
                &mut error,
            );
            if ptr.is_null() {
                Err(Message::from_raw(error))
            } else {
                Ok(Owning::from_raw(ptr))
            }
        }
    }

    pub fn copy_section_iterator(&self) -> Owning<SectionIterator<'_, 'm>> {
        unsafe { Owning::from_raw(LLVMObjectFileCopySectionIterator(self.as_raw())) }
    }

    pub fn is_section_iterator_at_end(&self, si: &SectionIterator) -> bool {
        unsafe { LLVMObjectFileIsSectionIteratorAtEnd(self.as_raw(), si.as_raw()) != 0 }
    }

    pub fn copy_symbol_iterator(&self) -> Owning<SymbolIterator> {
        unsafe { Owning::from_raw(LLVMObjectFileCopySymbolIterator(self.as_raw())) }
    }

    pub fn is_symbol_iterator_at_end(&self, si: &SymbolIterator) -> bool {
        unsafe { LLVMObjectFileIsSymbolIteratorAtEnd(self.as_raw(), si.as_raw()) != 0 }
    }
}

impl OpaqueDrop for LLVMOpaqueSectionIterator {
    unsafe fn drop_raw(ptr: *mut Self) {
        unsafe { LLVMDisposeSectionIterator(ptr) }
    }
}

impl<'b, 'm> SectionIterator<'b, 'm> {
    pub fn move_to_next_section(&self) {
        unsafe { LLVMMoveToNextSection(self.as_raw()) }
    }

    pub fn move_to_containing_section(&self, sym: &SymbolIterator) {
        unsafe { LLVMMoveToContainingSection(self.as_raw(), sym.as_raw()) }
    }
}

impl OpaqueDrop for LLVMOpaqueSymbolIterator {
    unsafe fn drop_raw(ptr: *mut Self) {
        unsafe { LLVMDisposeSymbolIterator(ptr) }
    }
}

impl<'b, 'm> SymbolIterator<'b, 'm> {
    pub fn move_to_next_symbol(&self) {
        unsafe { LLVMMoveToNextSymbol(self.as_raw()) }
    }
}

impl<'b, 'm> SectionIterator<'b, 'm> {
    pub fn get_section_name(&self) -> &CStr {
        unsafe { CStr::from_ptr(LLVMGetSectionName(self.as_raw())) }
    }

    pub fn get_section_size(&self) -> u64 {
        unsafe { LLVMGetSectionSize(self.as_raw()) }
    }

    pub fn get_section_contents(&self) -> &CStr {
        unsafe { CStr::from_ptr(LLVMGetSectionContents(self.as_raw())) }
    }

    pub fn get_section_address(&self) -> u64 {
        unsafe { LLVMGetSectionAddress(self.as_raw()) }
    }

    pub fn get_contains_symbol(&self, sym: &SymbolIterator) -> bool {
        unsafe { LLVMGetSectionContainsSymbol(self.as_raw(), sym.as_raw()) != 0 }
    }

    pub fn get_relocations(&self) -> Owning<RelocationIterator> {
        unsafe { Owning::from_raw(LLVMGetRelocations(self.as_raw())) }
    }
}

impl OpaqueDrop for LLVMOpaqueRelocationIterator {
    unsafe fn drop_raw(ptr: *mut Self) {
        unsafe { LLVMDisposeRelocationIterator(ptr) }
    }
}

impl<'b, 'm> SectionIterator<'b, 'm> {
    pub fn is_relocation_iterator_at_end(&self, ri: &RelocationIterator) -> bool {
        unsafe { LLVMIsRelocationIteratorAtEnd(self.as_raw(), ri.as_raw()) != 0 }
    }
}

impl<'b, 'm> RelocationIterator<'b, 'm> {
    pub fn move_to_next_relocation(&self) {
        unsafe { LLVMMoveToNextRelocation(self.as_raw()) }
    }
}

impl<'b, 'm> SymbolIterator<'b, 'm> {
    pub fn get_symbol_name(&self) -> &CStr {
        unsafe { CStr::from_ptr(LLVMGetSymbolName(self.as_raw())) }
    }

    pub fn get_symbol_address(&self) -> u64 {
        unsafe { LLVMGetSymbolAddress(self.as_raw()) }
    }

    pub fn get_symbol_size(&self) -> u64 {
        unsafe { LLVMGetSymbolSize(self.as_raw()) }
    }
}

impl<'b, 'm> RelocationIterator<'b, 'm> {
    pub fn get_relocation_offset(&self) -> u64 {
        unsafe { LLVMGetRelocationOffset(self.as_raw()) }
    }

    pub fn get_relocation_symbol(&self) -> &SymbolIterator<'b, 'm> {
        unsafe { SymbolIterator::from_raw(LLVMGetRelocationSymbol(self.as_raw())) }
    }

    pub fn get_relocation_type(&self) -> u64 {
        unsafe { LLVMGetRelocationType(self.as_raw()) }
    }

    pub fn get_relocation_type_name(&self) -> &CStr {
        unsafe { CStr::from_ptr(LLVMGetRelocationTypeName(self.as_raw())) }
    }

    pub fn get_relocation_value_string(&self) -> &CStr {
        unsafe { CStr::from_ptr(LLVMGetRelocationValueString(self.as_raw())) }
    }
}
