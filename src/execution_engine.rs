use std::ffi::CStr;
use std::marker::PhantomData;
use std::mem::size_of;
use std::ptr::null_mut;

use llvm_sys::execution_engine::*;
use llvm_sys::target_machine::*;

use crate::core::context::Context;
use crate::core::module::Module;
use crate::message::Message;
use crate::opaque::{Opaque, PhantomOpaque};
use crate::owning::{Dispose, Owning};

#[inline(always)]
pub fn link_in_mc_jit() {
    unsafe { LLVMLinkInMCJIT() };
}

#[inline(always)]
pub fn link_in_interpreter() {
    unsafe { LLVMLinkInInterpreter() };
}

#[repr(transparent)]
pub struct ExecutionEngine<'s> {
    _opaque: PhantomOpaque,
    _marker: PhantomData<&'s Context>,
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

    pub fn create_execution_engine_for_module(
        module: Owning<Module<'s>>,
    ) -> Result<Owning<Self>, Message> {
        unsafe {
            let mut ptr = null_mut();
            let mut err = null_mut();
            if LLVMCreateExecutionEngineForModule(&mut ptr, module.into_ptr(), &mut err) != 0 {
                return Err(Message::from_raw(err));
            }
            Ok(Owning::from_raw(ptr))
        }
    }

    pub fn create_jit_compiler_for_module(
        module: Owning<Module<'s>>,
        opt_level: u32,
    ) -> Result<Owning<Self>, Message> {
        unsafe {
            let mut ptr = null_mut();
            let mut err = null_mut();
            if LLVMCreateJITCompilerForModule(&mut ptr, module.into_ptr(), opt_level, &mut err) != 0
            {
                return Err(Message::from_raw(err));
            }
            Ok(Owning::from_raw(ptr))
        }
    }

    pub fn create_interpreter_for_module(
        module: Owning<Module<'s>>,
    ) -> Result<Owning<Self>, Message> {
        unsafe {
            let mut ptr = null_mut();
            let mut err = null_mut();
            if LLVMCreateInterpreterForModule(&mut ptr, module.into_ptr(), &mut err) != 0 {
                return Err(Message::from_raw(err));
            }
            Ok(Owning::from_raw(ptr))
        }
    }

    /// Create an MCJIT execution engine for a module, with the given options.
    pub fn create_mc_jit_compiler_for_module(
        module: Owning<Module<'s>>,
        option: MCJITCompilerOptions<'s>,
    ) -> Result<Owning<Self>, Message> {
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
            if LLVMCreateMCJITCompilerForModule(
                &mut ptr,
                module.into_ptr(),
                &mut o,
                size_of::<LLVMMCJITCompilerOptions>(),
                &mut err,
            ) != 0
            {
                return Err(Message::from_raw(err));
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
        unsafe {
            let mut o = LLVMMCJITCompilerOptions {
                OptLevel: 0,
                CodeModel: LLVMCodeModel::LLVMCodeModelJITDefault,
                NoFramePointerElim: 0,
                EnableFastISel: 0,
                MCJMM: null_mut(),
            };
            LLVMInitializeMCJITCompilerOptions(&mut o, size_of::<LLVMMCJITCompilerOptions>());
            debug_assert!(o.MCJMM.is_null());
            Self {
                opt_level: o.OptLevel,
                code_model: o.CodeModel,
                no_frame_pointer_elim: o.NoFramePointerElim != 0,
                enable_fast_instruction_select: o.EnableFastISel != 0,
                mc_jit_memory_manager: Owning::try_from_raw(o.MCJMM),
            }
        }
    }
}

pub struct McJitMemoryManager<'s> {
    _opaque: PhantomOpaque,
    _marker: PhantomData<&'s Context>,
}

unsafe impl<'s> Opaque for McJitMemoryManager<'s> {
    type Inner = LLVMOpaqueMCJITMemoryManager;
}

impl<'s> Dispose for McJitMemoryManager<'s> {
    unsafe fn dispose(ptr: *mut Self::Inner) {
        unsafe { LLVMDisposeMCJITMemoryManager(ptr) }
    }
}
