use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::mem::size_of;
use std::ptr::null_mut;

use llvm_sys::execution_engine::*;
use llvm_sys::target_machine::LLVMCodeModel;

use crate::context::Context;
use crate::module::Module;
use crate::opaque::{Opaque, PhantomOpaque};
use crate::owning::{Dispose, Owning};
use crate::util::c_string;

#[repr(transparent)]
pub struct ExecutionEngine<'s> {
    opaque: PhantomOpaque,
    marker: PhantomData<&'s Context>,
}

unsafe impl<'s> Opaque for ExecutionEngine<'s> {
    type Inner = LLVMOpaqueExecutionEngine;
}

impl<'s> Dispose for ExecutionEngine<'s> {
    unsafe fn dispose(ptr: *mut Self::Inner) {
        unsafe { LLVMDisposeExecutionEngine(ptr) };
    }
}

impl<'s> ExecutionEngine<'s> {
    pub fn get_function_address(&self, name: &CStr) -> u64 {
        unsafe { LLVMGetFunctionAddress(self.as_ptr(), name.as_ptr()) }
    }

    pub fn create_execution_engine(module: Owning<Module<'s>>) -> Result<Owning<Self>, CString> {
        unsafe {
            let mut ptr = null_mut();
            let mut err = null_mut();
            let code = LLVMCreateExecutionEngineForModule(&mut ptr, module.into_ptr(), &mut err);
            if code != 0 {
                return Err(c_string(err).unwrap());
            }
            Ok(Owning::from_raw(ptr))
        }
    }

    pub fn create_jit_compiler(
        module: Owning<Module<'s>>,
        opt_level: u32,
    ) -> Result<Owning<Self>, CString> {
        unsafe {
            let mut ptr = null_mut();
            let mut err = null_mut();
            let code =
                LLVMCreateJITCompilerForModule(&mut ptr, module.into_ptr(), opt_level, &mut err);
            if code != 0 {
                return Err(c_string(err).unwrap());
            }
            Ok(Owning::from_raw(ptr))
        }
    }

    pub fn create_interpreter(module: Owning<Module<'s>>) -> Result<Owning<Self>, CString> {
        unsafe {
            let mut ptr = null_mut();
            let mut err = null_mut();
            let code = LLVMCreateInterpreterForModule(&mut ptr, module.into_ptr(), &mut err);
            if code != 0 {
                return Err(c_string(err).unwrap());
            }
            Ok(Owning::from_raw(ptr))
        }
    }

    /// Create an MCJIT execution engine for a module, with the given options.
    pub fn create_mc_jit_compiler(
        module: Owning<Module<'s>>,
        option: MCJITCompilerOptions<'s>,
    ) -> Result<Owning<Self>, CString> {
        unsafe {
            let mut ptr = null_mut();
            let mut err = null_mut();
            let mut o = LLVMMCJITCompilerOptions {
                OptLevel: option.opt_level,
                CodeModel: option.code_model,
                NoFramePointerElim: option.no_frame_pointer_elim as _,
                EnableFastISel: option.enable_fast_instruction_select as _,
                MCJMM: Owning::option_into_ptr(option.mc_jit_memory_manager),
            };
            let code = LLVMCreateMCJITCompilerForModule(
                &mut ptr,
                module.into_ptr(),
                &mut o,
                size_of::<LLVMMCJITCompilerOptions>(),
                &mut err,
            );
            if code != 0 {
                return Err(c_string(err).unwrap());
            }
            Ok(Owning::from_raw(ptr))
        }
    }
}

pub struct MCJITCompilerOptions<'s> {
    pub opt_level: u32,
    pub code_model: LLVMCodeModel,
    pub no_frame_pointer_elim: bool,
    pub enable_fast_instruction_select: bool,
    pub mc_jit_memory_manager: Option<Owning<McJitMemoryManager<'s>>>,
}

impl<'s> Default for MCJITCompilerOptions<'s> {
    fn default() -> Self {
        Self {
            opt_level: 0,
            code_model: LLVMCodeModel::LLVMCodeModelJITDefault,
            no_frame_pointer_elim: false,
            enable_fast_instruction_select: false,
            mc_jit_memory_manager: None,
        }
    }
}

pub struct McJitMemoryManager<'s> {
    opaque: PhantomOpaque,
    marker: PhantomData<&'s Context>,
}

unsafe impl<'s> Opaque for McJitMemoryManager<'s> {
    type Inner = LLVMOpaqueMCJITMemoryManager;
}

impl<'s> Dispose for McJitMemoryManager<'s> {
    unsafe fn dispose(ptr: *mut Self::Inner) {
        unsafe { LLVMDisposeMCJITMemoryManager(ptr) }
    }
}
