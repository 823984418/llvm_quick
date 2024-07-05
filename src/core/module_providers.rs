use llvm_sys::core::*;
use llvm_sys::LLVMModuleProvider;

use crate::owning::{OpaqueDrop, Owning};
use crate::*;

impl<'c> Module<'c> {
    pub fn create_module_provider_for_existing_module(&self) -> Owning<ModuleProvider> {
        unsafe { Owning::from_raw(LLVMCreateModuleProviderForExistingModule(self.as_raw())) }
    }
}

impl OpaqueDrop for LLVMModuleProvider {
    unsafe fn drop_raw(ptr: *mut Self) {
        unsafe { LLVMDisposeModuleProvider(ptr) }
    }
}
