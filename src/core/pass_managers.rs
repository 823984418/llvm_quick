use llvm_sys::core::*;
use llvm_sys::LLVMPassManager;

use crate::owning::{OpaqueDrop, Owning};
use crate::type_tag::*;
use crate::*;

impl PassManager {
    pub fn create() -> Owning<Self> {
        unsafe { Owning::from_raw(LLVMCreatePassManager()) }
    }
}

impl<'c> Module<'c> {
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
    pub fn run(&self, module: &Module) -> bool {
        unsafe { LLVMRunPassManager(self.as_raw(), module.as_raw()) != 0 }
    }

    pub fn initialize_function_pass_manager(&self) -> bool {
        unsafe { LLVMInitializeFunctionPassManager(self.as_raw()) != 0 }
    }

    pub fn run_function_pass_manager<T: FunTypeTag>(&self, f: &Function<T>) -> bool {
        unsafe { LLVMRunFunctionPassManager(self.as_raw(), f.as_raw()) != 0 }
    }

    pub fn finalize_function_pass_manager(&self) -> bool {
        unsafe { LLVMFinalizeFunctionPassManager(self.as_raw()) != 0 }
    }
}

impl OpaqueDrop for LLVMPassManager {
    unsafe fn drop_raw(ptr: *mut Self) {
        unsafe { LLVMDisposePassManager(ptr) }
    }
}
