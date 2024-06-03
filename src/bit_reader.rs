use std::ptr::null_mut;

use llvm_sys::bit_reader::*;

use crate::core::context::Context;
use crate::core::memory_buffer::MemoryBuffer;
use crate::core::module::Module;
use crate::opaque::Opaque;
use crate::owning::Owning;

impl Context {
    pub fn parse_bitcode(&self, mem_buf: &MemoryBuffer) -> Result<Owning<Module>, ()> {
        unsafe {
            let mut ptr = null_mut();
            if LLVMParseBitcodeInContext2(self.as_ptr(), mem_buf.as_ptr(), &mut ptr) != 0 {
                return Err(());
            }
            Ok(Owning::from_raw(ptr))
        }
    }

    pub fn get_bitcode_module(&self, mem_buf: Owning<MemoryBuffer>) -> Result<Owning<Module>, ()> {
        unsafe {
            let mut ptr = null_mut();
            if LLVMGetBitcodeModuleInContext2(self.as_ptr(), mem_buf.as_ptr(), &mut ptr) != 0 {
                return Err(());
            }
            Ok(Owning::from_raw(ptr))
        }
    }
}
