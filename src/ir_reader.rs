use std::ptr::null_mut;

use llvm_sys::ir_reader::*;

use crate::core::Message;
use crate::owning::Owning;
use crate::*;

impl Context {
    pub fn parse_ir(&self, mem_buf: &MemoryBuffer) -> Result<Owning<Module>, Message> {
        unsafe {
            let mut ptr = null_mut();
            let mut err = null_mut();
            if LLVMParseIRInContext(self.as_raw(), mem_buf.as_raw(), &mut ptr, &mut err) != 0 {
                Err(Message::from_raw(err))
            } else {
                Ok(Owning::from_raw(ptr))
            }
        }
    }
}
