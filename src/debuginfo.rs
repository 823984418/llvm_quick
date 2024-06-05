use llvm_sys::debuginfo::*;

use crate::core::module::Module;
use crate::opaque::Opaque;

pub fn debug_metadata_version() -> u32 {
    unsafe { LLVMDebugMetadataVersion() }
}

impl<'s> Module<'s> {
    pub fn get_debug_metadata_version(&self) -> u32 {
        unsafe { LLVMGetModuleDebugMetadataVersion(self.as_raw()) }
    }

    // TODO
}
