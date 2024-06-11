use llvm_sys::core::*;

use crate::owning::{OpaqueDrop, Owning};
use crate::{Module, ModuleProvider, Opaque};

impl<'s> Module<'s> {
    pub fn create_module_provider_for_existing_module(&self) -> Owning<ModuleProvider> {
        unsafe { Owning::from_raw(LLVMCreateModuleProviderForExistingModule(self.as_raw())) }
    }
}

impl OpaqueDrop for ModuleProvider {
    unsafe fn drop_raw(ptr: *mut Self::Inner) {
        unsafe { LLVMDisposeModuleProvider(ptr) }
    }
}
