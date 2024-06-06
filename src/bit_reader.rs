use std::ptr::null_mut;

use llvm_sys::bit_reader::*;

use crate::owning::Owning;
use crate::{Context, MemoryBuffer, Module, Opaque};

impl Context {
    pub fn parse_bitcode(&self, mem_buf: &MemoryBuffer) -> Result<Owning<Module>, ()> {
        unsafe {
            let mut ptr = null_mut();
            if LLVMParseBitcodeInContext2(self.as_raw(), mem_buf.as_raw(), &mut ptr) != 0 {
                return Err(());
            }
            Ok(Owning::from_raw(ptr))
        }
    }

    pub fn get_bitcode_module(&self, mem_buf: Owning<MemoryBuffer>) -> Result<Owning<Module>, ()> {
        unsafe {
            let mut ptr = null_mut();
            if LLVMGetBitcodeModuleInContext2(self.as_raw(), mem_buf.as_raw(), &mut ptr) != 0 {
                return Err(());
            }
            Ok(Owning::from_raw(ptr))
        }
    }
}
