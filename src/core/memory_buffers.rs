use std::ffi::CStr;
use std::ptr::null_mut;

use llvm_sys::core::*;
use llvm_sys::LLVMMemoryBuffer;

use crate::core::Message;
use crate::owning::{OpaqueDrop, Owning};
use crate::*;

impl MemoryBuffer {
    pub fn create_with_contents_of_file(path: &CStr) -> Result<Owning<Self>, Message> {
        unsafe {
            let mut ptr = null_mut();
            let mut err = null_mut();
            if LLVMCreateMemoryBufferWithContentsOfFile(path.as_ptr(), &mut ptr, &mut err) != 0 {
                return Err(Message::from_raw(err));
            }
            Ok(Owning::from_raw(ptr))
        }
    }

    pub fn create_with_stdin() -> Result<Owning<Self>, Message> {
        unsafe {
            let mut ptr = null_mut();
            let mut err = null_mut();
            if LLVMCreateMemoryBufferWithSTDIN(&mut ptr, &mut err) != 0 {
                return Err(Message::from_raw(err));
            }
            Ok(Owning::from_raw(ptr))
        }
    }

    pub unsafe fn create_with_memory_range(
        data: *const [u8],
        name: &CStr,
        requires_null_terminator: bool,
    ) -> Owning<Self> {
        unsafe {
            let ptr = LLVMCreateMemoryBufferWithMemoryRange(
                data.cast(),
                data.len(),
                name.as_ptr(),
                requires_null_terminator as _,
            );
            Owning::from_raw(ptr)
        }
    }

    pub fn create_with_memory_range_copy(data: &[u8], name: &CStr) -> Owning<Self> {
        unsafe {
            let ptr = LLVMCreateMemoryBufferWithMemoryRangeCopy(
                data.as_ptr().cast(),
                data.len(),
                name.as_ptr(),
            );
            Owning::from_raw(ptr)
        }
    }

    pub fn get_start(&self) -> *const u8 {
        unsafe { LLVMGetBufferStart(self.as_raw()) as *const u8 }
    }

    pub fn get_size(&self) -> usize {
        unsafe { LLVMGetBufferSize(self.as_raw()) }
    }
}

impl OpaqueDrop for LLVMMemoryBuffer {
    unsafe fn drop_raw(ptr: *mut Self) {
        unsafe { LLVMDisposeMemoryBuffer(ptr) }
    }
}
