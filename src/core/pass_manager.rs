use llvm_sys::core::*;

use crate::core::type_tag::functions::FunTypeTag;
use crate::owning::{OpaqueDrop, Owning};
use crate::ModuleProvider;
use crate::Opaque;
use crate::Value;
use crate::{Module, PassManager};

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
    pub fn run(&self, module: &Module) -> bool {
        unsafe { LLVMRunPassManager(self.as_raw(), module.as_raw()) != 0 }
    }

    pub fn initialize_function_pass_manager(&self) -> bool {
        unsafe { LLVMInitializeFunctionPassManager(self.as_raw()) != 0 }
    }

    pub fn run_function_pass_manager<T: FunTypeTag>(&self, f: &Value<T>) -> bool {
        unsafe { LLVMRunFunctionPassManager(self.as_raw(), f.as_raw()) != 0 }
    }

    pub fn finalize_function_pass_manager(&self) -> bool {
        unsafe { LLVMFinalizeFunctionPassManager(self.as_raw()) != 0 }
    }
}

impl OpaqueDrop for PassManager {
    fn drop_raw(ptr: *mut Self::Inner) {
        unsafe { LLVMDisposePassManager(ptr) };
    }
}
