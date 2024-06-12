use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::ptr::null_mut;

use llvm_sys::object::*;

use crate::core::Message;
use crate::owning::{OpaqueDrop, Owning};
use crate::{Context, MemoryBuffer, Opaque, PhantomOpaque};

#[repr(transparent)]
pub struct SectionIterator<'a, 's> {
    _opaque: PhantomOpaque,
    _marker: PhantomData<&'a Binary<'s>>,
}

unsafe impl<'a, 's> Opaque for SectionIterator<'a, 's> {
    type Inner = LLVMOpaqueSectionIterator;
}

#[repr(transparent)]
pub struct SymbolIterator<'a, 's> {
    _opaque: PhantomOpaque,
    _marker: PhantomData<&'a Binary<'s>>,
}

unsafe impl<'a, 's> Opaque for SymbolIterator<'a, 's> {
    type Inner = LLVMOpaqueSymbolIterator;
}

#[repr(transparent)]
pub struct RelocationIterator<'a, 's> {
    _opaque: PhantomOpaque,
    _marker: PhantomData<&'a Binary<'s>>,
}

unsafe impl<'a, 's> Opaque for RelocationIterator<'a, 's> {
    type Inner = LLVMOpaqueRelocationIterator;
}

#[repr(transparent)]
pub struct Binary<'s> {
    _opaque: PhantomOpaque,
    _marker: PhantomData<&'s MemoryBuffer>,
}

unsafe impl<'s> Opaque for Binary<'s> {
    type Inner = LLVMOpaqueBinary;
}

impl<'s> Binary<'s> {
    pub fn create(mem_buf: &'s MemoryBuffer, context: &Context) -> Result<Owning<Self>, Message> {
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

impl<'s> OpaqueDrop for Binary<'s> {
    unsafe fn drop_raw(ptr: *mut Self::Inner) {
        unsafe { LLVMDisposeBinary(ptr) }
    }
}

impl<'s> Binary<'s> {
    pub fn copy_memory_buffer(&self) -> Owning<MemoryBuffer> {
        unsafe { Owning::from_raw(LLVMBinaryCopyMemoryBuffer(self.as_raw())) }
    }

    pub fn get_type(&self) -> LLVMBinaryType {
        unsafe { LLVMBinaryGetType(self.as_raw()) }
    }

    pub fn mach_o_universal_binary_copy_object_for_arch(
        &self,
        arch: &[u8],
    ) -> Result<Owning<Binary<'s>>, Message> {
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

    pub fn copy_section_iterator(&self) -> Owning<SectionIterator<'_, 's>> {
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

impl<'a, 's> OpaqueDrop for SectionIterator<'a, 's> {
    unsafe fn drop_raw(ptr: *mut Self::Inner) {
        unsafe { LLVMDisposeSectionIterator(ptr) }
    }
}

impl<'a, 's> SectionIterator<'a, 's> {
    pub fn move_to_next_section(&self) {
        unsafe { LLVMMoveToNextSection(self.as_raw()) }
    }

    pub fn move_to_containing_section(&self, sym: &SymbolIterator) {
        unsafe { LLVMMoveToContainingSection(self.as_raw(), sym.as_raw()) }
    }
}

impl<'a, 's> OpaqueDrop for SymbolIterator<'a, 's> {
    unsafe fn drop_raw(ptr: *mut Self::Inner) {
        unsafe { LLVMDisposeSymbolIterator(ptr) }
    }
}

impl<'a, 's> SymbolIterator<'a, 's> {
    pub fn move_to_next_symbol(&self) {
        unsafe { LLVMMoveToNextSymbol(self.as_raw()) }
    }
}

impl<'a, 's> SectionIterator<'a, 's> {
    pub fn get_section_name_raw(&self) -> *const CStr {
        unsafe { CStr::from_ptr(LLVMGetSectionName(self.as_raw())) }
    }

    pub fn get_section_name(&self) -> CString {
        unsafe { CString::from(&*self.get_section_name_raw()) }
    }

    pub fn get_section_size(&self) -> u64 {
        unsafe { LLVMGetSectionSize(self.as_raw()) }
    }

    pub fn get_section_contents_raw(&self) -> *const CStr {
        unsafe { CStr::from_ptr(LLVMGetSectionContents(self.as_raw())) }
    }

    pub fn get_section_contents(&self) -> CString {
        unsafe { CString::from(&*self.get_section_contents_raw()) }
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

impl<'a, 's> OpaqueDrop for RelocationIterator<'a, 's> {
    unsafe fn drop_raw(ptr: *mut Self::Inner) {
        unsafe { LLVMDisposeRelocationIterator(ptr) }
    }
}

impl<'a, 's> SectionIterator<'a, 's> {
    pub fn is_relocation_iterator_at_end(&self, ri: &RelocationIterator) -> bool {
        unsafe { LLVMIsRelocationIteratorAtEnd(self.as_raw(), ri.as_raw()) != 0 }
    }
}

impl<'a, 's> RelocationIterator<'a, 's> {
    pub fn move_to_next_relocation(&self) {
        unsafe { LLVMMoveToNextRelocation(self.as_raw()) }
    }
}

impl<'a, 's> SymbolIterator<'a, 's> {
    pub fn get_symbol_name_raw(&self) -> *const CStr {
        unsafe { CStr::from_ptr(LLVMGetSymbolName(self.as_raw())) }
    }

    pub fn get_symbol_name(&self) -> CString {
        unsafe { CString::from(&*self.get_symbol_name_raw()) }
    }

    pub fn get_symbol_address(&self) -> u64 {
        unsafe { LLVMGetSymbolAddress(self.as_raw()) }
    }

    pub fn get_symbol_size(&self) -> u64 {
        unsafe { LLVMGetSymbolSize(self.as_raw()) }
    }
}

impl<'a, 's> RelocationIterator<'a, 's> {
    pub fn get_relocation_offset(&self) -> u64 {
        unsafe { LLVMGetRelocationOffset(self.as_raw()) }
    }

    pub fn get_relocation_symbol(&self) -> &SymbolIterator<'a, 's> {
        unsafe { SymbolIterator::from_ref(LLVMGetRelocationSymbol(self.as_raw())) }
    }

    pub fn get_relocation_type(&self) -> u64 {
        unsafe { LLVMGetRelocationType(self.as_raw()) }
    }

    pub fn get_relocation_type_name_raw(&self) -> *const CStr {
        unsafe { CStr::from_ptr(LLVMGetRelocationTypeName(self.as_raw())) }
    }

    pub fn get_relocation_type_name(&self) -> CString {
        unsafe { CString::from(&*self.get_relocation_type_name_raw()) }
    }

    pub fn get_relocation_value_string_raw(&self) -> *const CStr {
        unsafe { CStr::from_ptr(LLVMGetRelocationValueString(self.as_raw())) }
    }

    pub fn get_relocation_value_string(&self) -> CString {
        unsafe { CString::from(&*self.get_relocation_value_string_raw()) }
    }
}
