use std::ffi::CStr;
use std::ptr::null_mut;

use llvm_sys::orc2::lljit::*;

use crate::error::Error;
use crate::opaque::{Opaque, PhantomOpaque};
use crate::orc2::{OrcExecutionSession, OrcJitDylib, OrcJitTargetMachineBuilder};
use crate::owning::{Dispose, Owning};

#[repr(transparent)]
pub struct OrcLLJitBuilder {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for OrcLLJitBuilder {
    type Inner = LLVMOrcOpaqueLLJITBuilder;
}

impl Dispose for OrcLLJitBuilder {
    unsafe fn dispose(ptr: *mut Self::Inner) {
        unsafe { LLVMOrcDisposeLLJITBuilder(ptr) };
    }
}

#[repr(transparent)]
pub struct OrcLLJit {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for OrcLLJit {
    type Inner = LLVMOrcOpaqueLLJIT;
}

impl Dispose for OrcLLJit {
    unsafe fn dispose(ptr: *mut Self::Inner) {
        unsafe { LLVMOrcDisposeLLJIT(ptr) };
    }
}

impl OrcLLJitBuilder {
    pub fn create() -> Owning<Self> {
        unsafe { Owning::from_raw(LLVMOrcCreateLLJITBuilder()) }
    }

    pub fn set_jit_target_machine_builder(&self, j: Owning<OrcJitTargetMachineBuilder>) {
        unsafe { LLVMOrcLLJITBuilderSetJITTargetMachineBuilder(self.as_ptr(), j.into_raw()) };
    }

    pub fn set_object_linking_layer_creator_raw(
        &self,
        f: LLVMOrcLLJITBuilderObjectLinkingLayerCreatorFunction,
        ctx: *mut (),
    ) {
        unsafe { LLVMOrcLLJITBuilderSetObjectLinkingLayerCreator(self.as_ptr(), f, ctx as _) };
    }
}

impl OrcLLJit {
    pub fn create(builder: Option<Owning<OrcLLJitBuilder>>) -> Result<Owning<Self>, Owning<Error>> {
        unsafe {
            let mut ptr = null_mut();
            Error::check(LLVMOrcCreateLLJIT(
                &mut ptr,
                Owning::option_into_raw(builder),
            ))?;
            Ok(Owning::from_raw(ptr))
        }
    }

    pub fn get_execution_session(&self) -> &OrcExecutionSession {
        unsafe { OrcExecutionSession::from_ref(LLVMOrcLLJITGetExecutionSession(self.as_ptr())) }
    }

    pub fn get_main_jit_dylib(&self) -> &OrcJitDylib {
        unsafe { OrcJitDylib::from_ref(LLVMOrcLLJITGetMainJITDylib(self.as_ptr())) }
    }

    pub fn get_triple(&self) -> &CStr {
        unsafe { CStr::from_ptr(LLVMOrcLLJITGetTripleString(self.as_ptr())) }
    }
    // TODO

    pub fn enable_debug_support(&self) -> Result<(), Owning<Error>> {
        unsafe { Error::check(LLVMOrcLLJITEnableDebugSupport(self.as_ptr())) }
    }
}
