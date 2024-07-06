use std::ffi::{c_char, c_void, CStr};
use std::ptr::null_mut;

use llvm_sys::orc2::lljit::*;
use llvm_sys::orc2::*;

use crate::error::Error;
use crate::orc2::*;
use crate::owning::{OpaqueDrop, Owning};
use crate::*;

#[repr(transparent)]
pub struct OrcLLJITBuilder {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for OrcLLJITBuilder {
    type Inner = LLVMOrcOpaqueLLJITBuilder;
}

#[repr(transparent)]
pub struct OrcLLJIT {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for OrcLLJIT {
    type Inner = LLVMOrcOpaqueLLJIT;
}

impl OrcLLJITBuilder {
    pub fn create() -> Owning<Self> {
        unsafe { Owning::from_raw(LLVMOrcCreateLLJITBuilder()) }
    }
}

impl OpaqueDrop for LLVMOrcOpaqueLLJITBuilder {
    unsafe fn drop_raw(ptr: *mut Self) {
        unsafe { LLVMOrcDisposeLLJITBuilder(ptr) }
    }
}

impl OrcLLJITBuilder {
    pub fn set_jit_target_machine_builder(&self, j: Owning<OrcJitTargetMachineBuilder>) {
        unsafe { LLVMOrcLLJITBuilderSetJITTargetMachineBuilder(self.as_raw(), j.into_raw()) }
    }

    pub fn set_object_linking_layer_creator_raw(
        &self,
        f: LLVMOrcLLJITBuilderObjectLinkingLayerCreatorFunction,
        ctx: *mut (),
    ) {
        unsafe { LLVMOrcLLJITBuilderSetObjectLinkingLayerCreator(self.as_raw(), f, ctx as _) }
    }

    pub fn set_object_linking_layer_creator<
        F: Fn(&OrcExecutionSession, &CStr) -> Owning<OrcObjectLayer>,
    >(
        &self,
        f: F,
    ) {
        extern "C" fn creator_raw<F: Fn(&OrcExecutionSession, &CStr) -> Owning<OrcObjectLayer>>(
            ctx: *mut c_void,
            es: *mut LLVMOrcOpaqueExecutionSession,
            triple: *const c_char,
        ) -> *mut LLVMOrcOpaqueObjectLayer {
            unsafe {
                (*(ctx as *const F))(OrcExecutionSession::from_raw(es), CStr::from_ptr(triple))
                    .into_raw()
            }
        }
        self.set_object_linking_layer_creator_raw(creator_raw::<F>, Box::into_raw(Box::new(f)) as _)
    }
}

impl OrcLLJIT {
    pub fn create(builder: Option<Owning<OrcLLJITBuilder>>) -> Result<Owning<Self>, Owning<Error>> {
        unsafe {
            let mut ptr = null_mut();
            Error::check(LLVMOrcCreateLLJIT(
                &mut ptr,
                builder.map(Owning::into_raw).unwrap_or(null_mut()),
            ))?;
            Ok(Owning::from_raw(ptr))
        }
    }
}

impl OpaqueDrop for LLVMOrcOpaqueLLJIT {
    unsafe fn drop_raw(ptr: *mut Self) {
        // In fact, currently it always returns success
        unsafe { Error::check(LLVMOrcDisposeLLJIT(ptr)).unwrap() }
    }
}

impl OrcLLJIT {
    pub fn get_execution_session(&self) -> &OrcExecutionSession {
        unsafe { OrcExecutionSession::from_raw(LLVMOrcLLJITGetExecutionSession(self.as_raw())) }
    }

    pub fn get_main_jit_dylib(&self) -> &OrcJitDylib {
        unsafe { OrcJitDylib::from_raw(LLVMOrcLLJITGetMainJITDylib(self.as_raw())) }
    }

    pub fn get_triple(&self) -> &CStr {
        unsafe { CStr::from_ptr(LLVMOrcLLJITGetTripleString(self.as_raw())) }
    }

    pub fn get_global_prefix(&self) -> c_char {
        unsafe { LLVMOrcLLJITGetGlobalPrefix(self.as_raw()) }
    }

    pub fn mangle_and_intern(&self, unmangled_name: &CStr) -> Owning<OrcSymbolStringPoolEntry> {
        unsafe {
            Owning::from_raw(LLVMOrcLLJITMangleAndIntern(
                self.as_raw(),
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
                self.as_raw(),
                jd.as_raw(),
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
                self.as_raw(),
                rt.as_raw(),
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
                self.as_raw(),
                jd.as_raw(),
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
                self.as_raw(),
                rt.as_raw(),
                tsm.into_raw(),
            ))
        }
    }

    pub fn lookup(&self, name: &CStr) -> Result<LLVMOrcExecutorAddress, Owning<Error>> {
        unsafe {
            let mut result = 0;
            Error::check(LLVMOrcLLJITLookup(
                self.as_raw(),
                &mut result,
                name.as_ptr(),
            ))?;
            Ok(result)
        }
    }

    pub fn get_obj_linking_layer(&self) -> &OrcObjectLayer {
        unsafe { OrcObjectLayer::from_raw(LLVMOrcLLJITGetObjLinkingLayer(self.as_raw())) }
    }

    pub fn get_obj_transform_layer(&self) -> &OrcObjectTransformLayer {
        unsafe {
            OrcObjectTransformLayer::from_raw(LLVMOrcLLJITGetObjTransformLayer(self.as_raw()))
        }
    }

    pub fn get_ir_transform_layer(&self) -> &OrcIrTransformLayer {
        unsafe { OrcIrTransformLayer::from_raw(LLVMOrcLLJITGetIRTransformLayer(self.as_raw())) }
    }

    pub fn get_data_layout_str(&self) -> &CStr {
        unsafe { CStr::from_ptr(LLVMOrcLLJITGetDataLayoutStr(self.as_raw())) }
    }

    pub fn enable_debug_support(&self) -> Result<(), Owning<Error>> {
        unsafe { Error::check(LLVMOrcLLJITEnableDebugSupport(self.as_raw())) }
    }
}
