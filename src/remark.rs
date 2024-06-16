use std::ffi::CStr;
use std::marker::PhantomData;

use llvm_sys::remarks::*;

use crate::owning::{OpaqueDrop, Owning};
use crate::{Opaque, PhantomOpaque};

#[repr(transparent)]
pub struct RemarkString {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for RemarkString {
    type Inner = LLVMRemarkOpaqueString;
}

impl RemarkString {
    pub fn get_data_raw(&self) -> *const u8 {
        unsafe { LLVMRemarkStringGetData(self.as_raw()) as _ }
    }

    pub fn get_len(&self) -> u32 {
        unsafe { LLVMRemarkStringGetLen(self.as_raw()) }
    }

    pub fn get_data(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.get_data_raw(), self.get_len() as _) }
    }
}

#[repr(transparent)]
pub struct RemarkDebugLoc {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for RemarkDebugLoc {
    type Inner = LLVMRemarkOpaqueDebugLoc;
}

impl RemarkDebugLoc {
    pub fn get_source_file_path(&self) -> &RemarkString {
        unsafe { RemarkString::from_raw(LLVMRemarkDebugLocGetSourceFilePath(self.as_raw())) }
    }

    pub fn get_source_line(&self) -> u32 {
        unsafe { LLVMRemarkDebugLocGetSourceLine(self.as_raw()) }
    }

    pub fn get_source_column(&self) -> u32 {
        unsafe { LLVMRemarkDebugLocGetSourceColumn(self.as_raw()) }
    }
}

#[repr(transparent)]
pub struct RemarkArg {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for RemarkArg {
    type Inner = LLVMRemarkOpaqueArg;
}

impl RemarkArg {
    pub fn get_key(&self) -> &RemarkString {
        unsafe { RemarkString::from_raw(LLVMRemarkArgGetKey(self.as_raw())) }
    }

    pub fn get_value(&self) -> &RemarkString {
        unsafe { RemarkString::from_raw(LLVMRemarkArgGetValue(self.as_raw())) }
    }

    pub fn get_debug_loc(&self) -> &RemarkDebugLoc {
        unsafe { RemarkDebugLoc::from_raw(LLVMRemarkArgGetDebugLoc(self.as_raw())) }
    }
}

#[repr(transparent)]
pub struct RemarkEntry {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for RemarkEntry {
    type Inner = LLVMRemarkOpaqueEntry;
}

impl OpaqueDrop for LLVMRemarkOpaqueEntry {
    unsafe fn drop_raw(ptr: *mut Self) {
        unsafe { LLVMRemarkEntryDispose(ptr) }
    }
}

impl RemarkEntry {
    pub fn get_type(&self) -> LLVMRemarkType {
        unsafe { LLVMRemarkEntryGetType(self.as_raw()) }
    }

    pub fn get_pass_name(&self) -> &RemarkString {
        unsafe { RemarkString::from_raw(LLVMRemarkEntryGetPassName(self.as_raw())) }
    }

    pub fn get_remark_name(&self) -> &RemarkString {
        unsafe { RemarkString::from_raw(LLVMRemarkEntryGetRemarkName(self.as_raw())) }
    }

    pub fn get_file_name(&self) -> &RemarkString {
        unsafe { RemarkString::from_raw(LLVMRemarkEntryGetFunctionName(self.as_raw())) }
    }

    pub fn get_debug_loc(&self) -> &RemarkDebugLoc {
        unsafe { RemarkDebugLoc::from_raw(LLVMRemarkEntryGetDebugLoc(self.as_raw())) }
    }

    pub fn get_hotness(&self) -> u64 {
        unsafe { LLVMRemarkEntryGetHotness(self.as_raw()) }
    }

    pub fn get_num_args(&self) -> u32 {
        unsafe { LLVMRemarkEntryGetNumArgs(self.as_raw()) }
    }

    pub fn get_first_arg(&self) -> &RemarkArg {
        unsafe { RemarkArg::from_raw(LLVMRemarkEntryGetFirstArg(self.as_raw())) }
    }

    pub fn get_next_arg(&self, it: &RemarkArg) -> &RemarkArg {
        unsafe { RemarkArg::from_raw(LLVMRemarkEntryGetNextArg(it.as_raw(), self.as_raw())) }
    }
}

#[repr(transparent)]
pub struct RemarkParser<'s> {
    _opaque: PhantomOpaque,
    _marker: PhantomData<&'s [u8]>,
}

unsafe impl<'s> Opaque for RemarkParser<'s> {
    type Inner = LLVMRemarkOpaqueParser;
}

impl<'s> RemarkParser<'s> {
    pub fn create_yaml(buf: &'s [u8]) -> Owning<Self> {
        unsafe {
            Owning::from_raw(LLVMRemarkParserCreateYAML(
                buf.as_ptr() as _,
                buf.len() as _,
            ))
        }
    }

    pub fn create_bitstream(buf: &'s [u8]) -> Owning<Self> {
        unsafe {
            Owning::from_raw(LLVMRemarkParserCreateBitstream(
                buf.as_ptr() as _,
                buf.len() as _,
            ))
        }
    }

    pub fn get_next(&self) -> &'s RemarkEntry {
        unsafe { RemarkEntry::from_raw(LLVMRemarkParserGetNext(self.as_raw())) }
    }

    pub fn has_error(&self) -> bool {
        unsafe { LLVMRemarkParserHasError(self.as_raw()) != 0 }
    }

    pub fn get_error_message(&self) -> Option<&'s CStr> {
        unsafe {
            let ptr = LLVMRemarkParserGetErrorMessage(self.as_raw());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr))
            }
        }
    }
}

impl OpaqueDrop for LLVMRemarkOpaqueParser {
    unsafe fn drop_raw(ptr: *mut Self) {
        unsafe { LLVMRemarkParserDispose(ptr) }
    }
}

pub fn remark_version() -> u32 {
    unsafe { LLVMRemarkVersion() }
}
