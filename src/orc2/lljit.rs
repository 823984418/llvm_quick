use std::ffi::{c_char, CStr};
use std::ptr::null_mut;

use llvm_sys::orc2::lljit::*;
use llvm_sys::orc2::LLVMOrcExecutorAddress;

use crate::core::memory_buffer::MemoryBuffer;
use crate::error::Error;
use crate::opaque::{Opaque, PhantomOpaque};
use crate::orc2::{
    OrcExecutionSession, OrcIrTransformLayer, OrcJitDylib, OrcJitTargetMachineBuilder,
    OrcObjectLayer, OrcObjectTransformLayer, OrcResourceTracker, OrcSymbolStringPoolEntry,
    OrcThreadSafeModule,
};
use crate::owning::{OpaqueDrop, Owning};

#[repr(transparent)]
pub struct OrcLLJitBuilder {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for OrcLLJitBuilder {
    type Inner = LLVMOrcOpaqueLLJITBuilder;
}

impl OpaqueDrop for OrcLLJitBuilder {
    unsafe fn drop_raw(ptr: *mut Self::Inner) {
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

impl OpaqueDrop for OrcLLJit {
    unsafe fn drop_raw(ptr: *mut Self::Inner) {
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

    pub fn get_global_prefix(&self) -> c_char {
        unsafe { LLVMOrcLLJITGetGlobalPrefix(self.as_ptr()) }
    }

    pub fn mangle_and_intern(&self, unmangled_name: &CStr) -> Owning<OrcSymbolStringPoolEntry> {
        unsafe {
            Owning::from_raw(LLVMOrcLLJITMangleAndIntern(
                self.as_ptr(),
                unmangled_name.as_ptr(),
            ))
        }
    }

    pub fn add_object_file(
        &self,
        jd: &OrcJitDylib,
        obj_buffer: Owning<MemoryBuffer>,
    ) -> Result<(), Owning<Error>> {
        unsafe {
            Error::check(LLVMOrcLLJITAddObjectFile(
                self.as_ptr(),
                jd.as_ptr(),
                obj_buffer.into_raw(),
            ))
        }
    }

    pub fn add_object_file_with_rt(
        &self,
        rt: &OrcResourceTracker,
        obj_buffer: Owning<MemoryBuffer>,
    ) -> Result<(), Owning<Error>> {
        unsafe {
            Error::check(LLVMOrcLLJITAddObjectFileWithRT(
                self.as_ptr(),
                rt.as_ptr(),
                obj_buffer.into_raw(),
            ))
        }
    }

    pub fn add_llvm_ir_module(
        &self,
        jd: &OrcJitDylib,
        tsm: Owning<OrcThreadSafeModule>,
    ) -> Result<(), Owning<Error>> {
        unsafe {
            Error::check(LLVMOrcLLJITAddLLVMIRModule(
                self.as_ptr(),
                jd.as_ptr(),
                tsm.into_raw(),
            ))
        }
    }
    pub fn add_llvm_ir_module_with_rt(
        &self,
        rt: &OrcResourceTracker,
        tsm: Owning<OrcThreadSafeModule>,
    ) -> Result<(), Owning<Error>> {
        unsafe {
            Error::check(LLVMOrcLLJITAddLLVMIRModuleWithRT(
                self.as_ptr(),
                rt.as_ptr(),
                tsm.into_raw(),
            ))
        }
    }

    pub fn lookup(&self, name: &CStr) -> Result<LLVMOrcExecutorAddress, Owning<Error>> {
        unsafe {
            let mut result = 0;
            Error::check(LLVMOrcLLJITLookup(
                self.as_ptr(),
                &mut result,
                name.as_ptr(),
            ))?;
            Ok(result)
        }
    }

    pub fn get_obj_linking_layer(&self) -> &OrcObjectLayer {
        unsafe { OrcObjectLayer::from_ref(LLVMOrcLLJITGetObjLinkingLayer(self.as_ptr())) }
    }

    pub fn get_obj_transform_layer(&self) -> &OrcObjectTransformLayer {
        unsafe {
            OrcObjectTransformLayer::from_ref(LLVMOrcLLJITGetObjTransformLayer(self.as_ptr()))
        }
    }

    pub fn get_ir_transform_layer(&self) -> &OrcIrTransformLayer {
        unsafe { OrcIrTransformLayer::from_ref(LLVMOrcLLJITGetIRTransformLayer(self.as_ptr())) }
    }

    pub fn get_data_layout_str(&self) -> &CStr {
        unsafe { CStr::from_ptr(LLVMOrcLLJITGetDataLayoutStr(self.as_ptr())) }
    }

    pub fn enable_debug_support(&self) -> Result<(), Owning<Error>> {
        unsafe { Error::check(LLVMOrcLLJITEnableDebugSupport(self.as_ptr())) }
    }
}
