use std::ffi::CStr;

use llvm_sys::target_machine::*;

use crate::message::Message;
use crate::opaque::{Opaque, PhantomOpaque};
use crate::owning::{Dispose, Owning};
use crate::target::PassManager;

// TODO

pub struct TargetMachine {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for TargetMachine {
    type Inner = LLVMOpaqueTargetMachine;
}

impl Dispose for TargetMachine {
    unsafe fn dispose(ptr: *mut Self::Inner) {
        unsafe { LLVMDisposeTargetMachine(ptr) };
    }
}

pub struct TargetMachineOptions {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for TargetMachineOptions {
    type Inner = LLVMOpaqueTargetMachineOptions;
}

impl Dispose for TargetMachineOptions {
    unsafe fn dispose(ptr: *mut Self::Inner) {
        unsafe { LLVMDisposeTargetMachineOptions(ptr) };
    }
}

impl TargetMachineOptions {
    pub fn create() -> Owning<Self> {
        unsafe { Owning::from_raw(LLVMCreateTargetMachineOptions()) }
    }

    pub fn set_cpu(&self, cpu: &CStr) {
        unsafe { LLVMTargetMachineOptionsSetCPU(self.as_ptr(), cpu.as_ptr()) }
    }
}

pub struct Target {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for Target {
    type Inner = LLVMTarget;
}

impl Target {
    pub fn get_first_target() -> Option<&'static Self> {
        unsafe { Target::try_from_ref(LLVMGetFirstTarget()) }
    }

    pub fn get_next_target(&self) -> Option<&Self> {
        unsafe { Target::try_from_ref(LLVMGetNextTarget(self.as_ptr())) }
    }

    pub fn has_jit(&self) -> bool {
        unsafe { LLVMTargetHasJIT(self.as_ptr()) != 0 }
    }

    pub fn has_target_machine(&self) -> bool {
        unsafe { LLVMTargetHasTargetMachine(self.as_ptr()) != 0 }
    }

    pub fn has_asm_backend(&self) -> bool {
        unsafe { LLVMTargetHasAsmBackend(self.as_ptr()) != 0 }
    }
}

pub fn get_default_target_triple() -> Message {
    unsafe { Message::from_raw(LLVMGetDefaultTargetTriple()) }
}

pub fn normalize_target_triple(triple: &CStr) -> Message {
    unsafe { Message::from_raw(LLVMNormalizeTargetTriple(triple.as_ptr())) }
}

pub fn get_host_cpuname() -> Message {
    unsafe { Message::from_raw(LLVMGetHostCPUName()) }
}

pub fn get_host_cpu_features() -> Message {
    unsafe { Message::from_raw(LLVMGetHostCPUFeatures()) }
}

impl PassManager {
    pub fn add_analysis_passes(&self, v: &TargetMachine) {
        unsafe { LLVMAddAnalysisPasses(v.as_ptr(), self.as_ptr()) }
    }
}
