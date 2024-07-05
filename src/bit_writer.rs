use std::ffi::CStr;

use llvm_sys::bit_writer::*;

use crate::owning::Owning;
use crate::*;

impl<'c> Module<'c> {
    pub fn write_bitcode_to_file(&self, path: &CStr) -> Result<(), ()> {
        unsafe {
            if LLVMWriteBitcodeToFile(self.as_raw(), path.as_ptr()) != 0 {
                Err(())
            } else {
                Ok(())
            }
        }
    }

    pub fn write_bitcode_to_file_descriptor(
        &self,
        file_descriptor: i32,
        should_close: i32,
        unbuffer: i32,
    ) -> Result<(), ()> {
        unsafe {
            if LLVMWriteBitcodeToFD(self.as_raw(), file_descriptor, should_close, unbuffer) != 0 {
                Err(())
            } else {
                Ok(())
            }
        }
    }

    pub fn write_bitcode_to_file_handle(&self, handle: i32) -> Result<(), ()> {
        unsafe {
            if LLVMWriteBitcodeToFileHandle(self.as_raw(), handle) != 0 {
                Err(())
            } else {
                Ok(())
            }
        }
    }

    pub fn write_bitcode_to_memory_buffer(&self) -> Owning<MemoryBuffer> {
        unsafe { Owning::from_raw(LLVMWriteBitcodeToMemoryBuffer(self.as_raw())) }
    }
}
