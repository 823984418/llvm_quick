use llvm_sys::debuginfo::*;
use llvm_sys::LLVMOpaqueDIBuilder;

use crate::owning::{OpaqueDrop, Owning};
use crate::{DIBuilder, Module, Opaque};

#[inline(always)]
pub fn debug_metadata_version() -> u32 {
    unsafe { LLVMDebugMetadataVersion() }
}

impl<'c> Module<'c> {
    pub fn get_debug_metadata_version(&self) -> u32 {
        unsafe { LLVMGetModuleDebugMetadataVersion(self.as_raw()) }
    }

    pub fn strip_module_debug_info(&self) -> bool {
        unsafe { LLVMStripModuleDebugInfo(self.as_raw()) != 0 }
    }

    pub fn create_debug_info_builder_disallow_unresolved<'m>(
        &'m self,
    ) -> Owning<DIBuilder<'m, 'c>> {
        unsafe { Owning::from_raw(LLVMCreateDIBuilderDisallowUnresolved(self.as_raw())) }
    }

    pub fn create_debug_info_builder<'m>(&'m self) -> Owning<DIBuilder<'m, 'c>> {
        unsafe { Owning::from_raw(LLVMCreateDIBuilder(self.as_raw())) }
    }
}

impl OpaqueDrop for LLVMOpaqueDIBuilder {
    unsafe fn drop_raw(ptr: *mut Self) {
        unsafe { LLVMDisposeDIBuilder(ptr) }
    }
}

impl<'m, 'c> DIBuilder<'m, 'c> {}
