use llvm_sys::core::*;
use llvm_sys::*;

use crate::core::module::Module;
use crate::opaque::{Opaque, PhantomOpaque};
use crate::owning::{OpaqueDrop, Owning};

#[repr(transparent)]
pub struct ModuleProvider {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for ModuleProvider {
    type Inner = LLVMModuleProvider;
}

impl<'s> Module<'s> {
    pub fn create_module_provider_for_existing_module(&self) -> Owning<ModuleProvider> {
        unsafe { Owning::from_raw(LLVMCreateModuleProviderForExistingModule(self.as_raw())) }
    }
}

impl OpaqueDrop for ModuleProvider {
    unsafe fn drop_raw(ptr: *mut Self::Inner) {
        unsafe { LLVMDisposeModuleProvider(ptr) };
    }
}
