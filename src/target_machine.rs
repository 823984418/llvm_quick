use std::ffi::CStr;
use std::ptr::null_mut;

use llvm_sys::target_machine::*;

use crate::memory_buffer::MemoryBuffer;
use crate::message::Message;
use crate::module::Module;
use crate::opaque::{Opaque, PhantomOpaque};
use crate::owning::{Dispose, Owning};
use crate::target::{PassManager, TargetData};

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

impl TargetMachine {
    pub fn get_target(&self) -> &Target {
        unsafe { Target::from_ref(LLVMGetTargetMachineTarget(self.as_ptr())) }
    }

    pub fn get_triple(&self) -> Message {
        unsafe { Message::from_raw(LLVMGetTargetMachineTriple(self.as_ptr())) }
    }

    pub fn get_cpu(&self) -> Message {
        unsafe { Message::from_raw(LLVMGetTargetMachineCPU(self.as_ptr())) }
    }

    pub fn get_features_string(&self) -> Message {
        unsafe { Message::from_raw(LLVMGetTargetMachineFeatureString(self.as_ptr())) }
    }

    pub fn create_data_layout(&self) -> Owning<TargetData> {
        unsafe { Owning::from_raw(LLVMCreateTargetDataLayout(self.as_ptr())) }
    }

    pub fn set_asm_verbosity(&self, v: bool) {
        unsafe { LLVMSetTargetMachineAsmVerbosity(self.as_ptr(), v as _) }
    }

    pub fn set_fast_instruction_select(&self, v: bool) {
        unsafe { LLVMSetTargetMachineFastISel(self.as_ptr(), v as _) }
    }

    pub fn set_global_instruction_select(&self, v: bool) {
        unsafe { LLVMSetTargetMachineGlobalISel(self.as_ptr(), v as _) }
    }

    pub fn set_global_instruction_select_abort(&self, v: LLVMGlobalISelAbortMode) {
        unsafe { LLVMSetTargetMachineGlobalISelAbort(self.as_ptr(), v) }
    }

    pub fn set_outliner(&self, v: bool) {
        unsafe { LLVMSetTargetMachineMachineOutliner(self.as_ptr(), v as _) }
    }

    pub fn emit_to_file(
        &self,
        module: &Module,
        filename: &CStr,
        codegen: LLVMCodeGenFileType,
    ) -> Result<(), Message> {
        unsafe {
            let mut err = null_mut();
            if LLVMTargetMachineEmitToFile(
                self.as_ptr(),
                module.as_ptr(),
                filename.as_ptr() as _,
                codegen,
                &mut err,
            ) != 0
            {
                return Err(Message::from_raw(err));
            }
            Ok(())
        }
    }

    pub fn emit_to_memory_buffer(
        &self,
        module: &Module,
        codegen: LLVMCodeGenFileType,
    ) -> Result<Owning<MemoryBuffer>, Message> {
        unsafe {
            let mut err = null_mut();
            let mut mem = null_mut();
            if LLVMTargetMachineEmitToMemoryBuffer(
                self.as_ptr(),
                module.as_ptr(),
                codegen,
                &mut err,
                &mut mem,
            ) != 0
            {
                return Err(Message::from_raw(err));
            }
            Ok(Owning::from_raw(mem))
        }
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

    pub fn set_cpu(&self, v: &CStr) {
        unsafe { LLVMTargetMachineOptionsSetCPU(self.as_ptr(), v.as_ptr()) }
    }

    pub fn set_features(&self, v: &CStr) {
        unsafe { LLVMTargetMachineOptionsSetFeatures(self.as_ptr(), v.as_ptr()) }
    }

    pub fn set_abi(&self, v: &CStr) {
        unsafe { LLVMTargetMachineOptionsSetABI(self.as_ptr(), v.as_ptr()) }
    }

    pub fn set_code_gen_opt_level(&self, v: LLVMCodeGenOptLevel) {
        unsafe { LLVMTargetMachineOptionsSetCodeGenOptLevel(self.as_ptr(), v) }
    }

    pub fn set_reloc_mode(&self, v: LLVMRelocMode) {
        unsafe { LLVMTargetMachineOptionsSetRelocMode(self.as_ptr(), v) }
    }

    pub fn set_code_mode(&self, v: LLVMCodeModel) {
        unsafe { LLVMTargetMachineOptionsSetCodeModel(self.as_ptr(), v) }
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
        unsafe { Self::try_from_ref(LLVMGetFirstTarget()) }
    }

    pub fn get_next_target(&self) -> Option<&Self> {
        unsafe { Self::try_from_ref(LLVMGetNextTarget(self.as_ptr())) }
    }

    pub fn get_from_name(name: &CStr) -> Option<&'static Self> {
        unsafe { Self::try_from_ref(LLVMGetTargetFromName(name.as_ptr())) }
    }

    pub fn get_from_triple(triple: &CStr) -> Result<&'static Self, Message> {
        unsafe {
            let mut ptr = null_mut();
            let mut err = null_mut();
            if LLVMGetTargetFromTriple(triple.as_ptr(), &mut ptr, &mut err) != 0 {
                return Err(Message::from_raw(err));
            }
            Ok(Self::from_ref(ptr))
        }
    }

    pub fn get_name(&self) -> &CStr {
        unsafe { CStr::from_ptr(LLVMGetTargetName(self.as_ptr())) }
    }

    pub fn get_description(&self) -> &CStr {
        unsafe { CStr::from_ptr(LLVMGetTargetDescription(self.as_ptr())) }
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

    pub fn create_target_machine_with_options(
        &self,
        triple: &CStr,
        options: &TargetMachineOptions,
    ) -> Owning<TargetMachine> {
        unsafe {
            Owning::from_raw(LLVMCreateTargetMachineWithOptions(
                self.as_ptr(),
                triple.as_ptr(),
                options.as_ptr(),
            ))
        }
    }

    pub fn create_target_machine(
        &self,
        triple: &CStr,
        cpu: &CStr,
        features: &CStr,
        level: LLVMCodeGenOptLevel,
        reloc: LLVMRelocMode,
        code_model: LLVMCodeModel,
    ) -> Owning<TargetMachine> {
        unsafe {
            Owning::from_raw(LLVMCreateTargetMachine(
                self.as_ptr(),
                triple.as_ptr(),
                cpu.as_ptr(),
                features.as_ptr(),
                level,
                reloc,
                code_model,
            ))
        }
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
