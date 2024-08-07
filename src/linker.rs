use llvm_sys::linker::LLVMLinkModules2;

use crate::owning::Owning;
use crate::*;

impl<'c> Module<'c> {
    pub fn link_modules(&self, src: Owning<Self>) -> Result<(), ()> {
        unsafe {
            if LLVMLinkModules2(self.as_raw(), src.into_raw()) != 0 {
                Err(())
            } else {
                Ok(())
            }
        }
    }
}
