use llvm_sys::*;
use llvm_sys::core::*;

use crate::core::module::Module;
use crate::core::module_provider::ModuleProvider;
use crate::opaque::{Opaque, PhantomOpaque};
use crate::owning::{OpaqueDrop, Owning};

#[repr(transparent)]
pub struct PassManager {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for PassManager {
    type Inner = LLVMPassManager;
}

impl OpaqueDrop for PassManager {
    unsafe fn drop_raw(ptr: *mut Self::Inner) {
        unsafe { LLVMDisposePassManager(ptr) };
    }
}

impl PassManager {
    pub fn create() -> Owning<Self> {
        unsafe { Owning::from_raw(LLVMCreatePassManager()) }
    }
}

impl<'s> Module<'s> {
    pub fn create_function_pass_manager(&self) -> Owning<PassManager> {
        unsafe { Owning::from_raw(LLVMCreateFunctionPassManagerForModule(self.as_raw())) }
    }
}

impl ModuleProvider {
    pub fn create_function_pass_manager(&self) -> Owning<PassManager> {
        unsafe { Owning::from_raw(LLVMCreateFunctionPassManager(self.as_raw())) }
    }
}

impl PassManager {
    pub fn initialize_function_pass_manager(&self) -> bool {
        unsafe { LLVMInitializeFunctionPassManager(self.as_raw()) != 0 }
    }

    pub fn run(&self, module: &Module) -> bool {
        unsafe { LLVMRunPassManager(self.as_raw(), module.as_raw()) != 0 }
    }

    pub fn finalize_function_pass_manager(&self) -> bool {
        unsafe { LLVMFinalizeFunctionPassManager(self.as_raw()) != 0 }
    }
}
