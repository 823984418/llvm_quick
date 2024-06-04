use llvm_sys::linker::LLVMLinkModules2;

use crate::core::module::Module;
use crate::opaque::Opaque;
use crate::owning::Owning;

impl<'s> Module<'s> {
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
