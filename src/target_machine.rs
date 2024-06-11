use std::ffi::CStr;
use std::ptr::null_mut;

use llvm_sys::target_machine::*;

use crate::core::Message;
use crate::owning::{OpaqueDrop, Owning};
use crate::target::TargetData;
use crate::{MemoryBuffer, Module, Opaque, PassManager, PhantomOpaque};

#[repr(transparent)]
pub struct TargetMachine {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for TargetMachine {
    type Inner = LLVMOpaqueTargetMachine;
}

#[repr(transparent)]
pub struct TargetMachineOptions {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for TargetMachineOptions {
    type Inner = LLVMOpaqueTargetMachineOptions;
}

#[repr(transparent)]
pub struct Target {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for Target {
    type Inner = LLVMTarget;
}

impl Target {
    pub fn iter_all() -> impl Iterator<Item = &'static Self> {
        unsafe {
            let mut ptr = Self::try_from_ref(LLVMGetFirstTarget());
            std::iter::from_fn(move || {
                let ret = ptr;
                if let Some(v) = ptr {
                    ptr = Self::try_from_ref(LLVMGetNextTarget(v.as_raw()));
                }
                ret
            })
        }
    }

    pub fn from_name(name: &CStr) -> Option<&'static Self> {
        unsafe { Self::try_from_ref(LLVMGetTargetFromName(name.as_ptr())) }
    }

    pub fn from_triple(triple: &CStr) -> Result<&'static Self, Message> {
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
        unsafe { CStr::from_ptr(LLVMGetTargetName(self.as_raw())) }
    }

    pub fn get_description(&self) -> &CStr {
        unsafe { CStr::from_ptr(LLVMGetTargetDescription(self.as_raw())) }
    }

    pub fn has_jit(&self) -> bool {
        unsafe { LLVMTargetHasJIT(self.as_raw()) != 0 }
    }

    pub fn has_target_machine(&self) -> bool {
        unsafe { LLVMTargetHasTargetMachine(self.as_raw()) != 0 }
    }

    pub fn has_asm_backend(&self) -> bool {
        unsafe { LLVMTargetHasAsmBackend(self.as_raw()) != 0 }
    }
}

impl TargetMachineOptions {
    pub fn create() -> Owning<Self> {
        unsafe { Owning::from_raw(LLVMCreateTargetMachineOptions()) }
    }
}

impl OpaqueDrop for TargetMachineOptions {
    fn drop_raw(ptr: *mut Self::Inner) {
        unsafe { LLVMDisposeTargetMachineOptions(ptr) }
    }
}

impl TargetMachineOptions {
    pub fn set_cpu(&self, v: &CStr) {
        unsafe { LLVMTargetMachineOptionsSetCPU(self.as_raw(), v.as_ptr()) }
    }

    pub fn set_features(&self, v: &CStr) {
        unsafe { LLVMTargetMachineOptionsSetFeatures(self.as_raw(), v.as_ptr()) }
    }

    pub fn set_abi(&self, v: &CStr) {
        unsafe { LLVMTargetMachineOptionsSetABI(self.as_raw(), v.as_ptr()) }
    }

    pub fn set_code_gen_opt_level(&self, v: LLVMCodeGenOptLevel) {
        unsafe { LLVMTargetMachineOptionsSetCodeGenOptLevel(self.as_raw(), v) }
    }

    pub fn set_reloc_mode(&self, v: LLVMRelocMode) {
        unsafe { LLVMTargetMachineOptionsSetRelocMode(self.as_raw(), v) }
    }

    pub fn set_code_mode(&self, v: LLVMCodeModel) {
        unsafe { LLVMTargetMachineOptionsSetCodeModel(self.as_raw(), v) }
    }
}

impl Target {
    pub fn create_target_machine_with_options(
        &self,
        triple: &CStr,
        options: &TargetMachineOptions,
    ) -> Owning<TargetMachine> {
        unsafe {
            Owning::from_raw(LLVMCreateTargetMachineWithOptions(
                self.as_raw(),
                triple.as_ptr(),
                options.as_raw(),
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
                self.as_raw(),
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

impl OpaqueDrop for TargetMachine {
    fn drop_raw(ptr: *mut Self::Inner) {
        unsafe { LLVMDisposeTargetMachine(ptr) }
    }
}

impl TargetMachine {
    pub fn get_target(&self) -> &Target {
        unsafe { Target::from_ref(LLVMGetTargetMachineTarget(self.as_raw())) }
    }

    pub fn get_triple(&self) -> Message {
        unsafe { Message::from_raw(LLVMGetTargetMachineTriple(self.as_raw())) }
    }

    pub fn get_cpu(&self) -> Message {
        unsafe { Message::from_raw(LLVMGetTargetMachineCPU(self.as_raw())) }
    }

    pub fn get_features_string(&self) -> Message {
        unsafe { Message::from_raw(LLVMGetTargetMachineFeatureString(self.as_raw())) }
    }

    pub fn create_data_layout(&self) -> Owning<TargetData> {
        unsafe { Owning::from_raw(LLVMCreateTargetDataLayout(self.as_raw())) }
    }

    pub fn set_asm_verbosity(&self, v: bool) {
        unsafe { LLVMSetTargetMachineAsmVerbosity(self.as_raw(), v as _) }
    }

    pub fn set_fast_instruction_select(&self, v: bool) {
        unsafe { LLVMSetTargetMachineFastISel(self.as_raw(), v as _) }
    }

    pub fn set_global_i_sel(&self, v: bool) {
        unsafe { LLVMSetTargetMachineGlobalISel(self.as_raw(), v as _) }
    }

    pub fn set_global_i_sel_abort(&self, v: LLVMGlobalISelAbortMode) {
        unsafe { LLVMSetTargetMachineGlobalISelAbort(self.as_raw(), v) }
    }

    pub fn set_outliner(&self, v: bool) {
        unsafe { LLVMSetTargetMachineMachineOutliner(self.as_raw(), v as _) }
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
                self.as_raw(),
                module.as_raw(),
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
                self.as_raw(),
                module.as_raw(),
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

pub fn get_default_target_triple() -> Message {
    unsafe { Message::from_raw(LLVMGetDefaultTargetTriple()) }
}

pub fn normalize_target_triple(triple: &CStr) -> Message {
    unsafe { Message::from_raw(LLVMNormalizeTargetTriple(triple.as_ptr())) }
}

pub fn get_host_cpu_name() -> Message {
    unsafe { Message::from_raw(LLVMGetHostCPUName()) }
}

pub fn get_host_cpu_features() -> Message {
    unsafe { Message::from_raw(LLVMGetHostCPUFeatures()) }
}

impl PassManager {
    pub fn add_analysis_passes(&self, v: &TargetMachine) {
        unsafe { LLVMAddAnalysisPasses(v.as_raw(), self.as_raw()) }
    }
}
