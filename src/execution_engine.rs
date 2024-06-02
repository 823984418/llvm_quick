use std::ffi::{c_char, c_void, CStr};
use std::marker::PhantomData;
use std::mem::size_of;
use std::ptr::{null, null_mut};

use llvm_sys::execution_engine::*;
use llvm_sys::prelude::*;
use llvm_sys::target_machine::*;
use llvm_sys::*;

use crate::core::context::Context;
use crate::core::module::Module;
use crate::core::types::Type;
use crate::core::values::Value;
use crate::core::Message;
use crate::opaque::{Opaque, PhantomOpaque};
use crate::owning::{Dispose, Owning};
use crate::target::TargetData;
use crate::target_machine::TargetMachine;
use crate::type_tag::float_tag::FloatTypeTag;
use crate::type_tag::function_tag::{fun, fun_any, FunTypeTag};
use crate::type_tag::integer_tag::{int32, IntTypeTag};
use crate::type_tag::pointer_tag::ptr;
use crate::type_tag::TypeTag;

#[repr(transparent)]
pub struct GenericValue {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for GenericValue {
    type Inner = LLVMOpaqueGenericValue;
}

impl Dispose for GenericValue {
    unsafe fn dispose(ptr: *mut Self::Inner) {
        unsafe { LLVMDisposeGenericValue(ptr) }
    }
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

#[repr(transparent)]
pub struct McJitMemoryManager {
    _opaque: PhantomOpaque,
}

unsafe impl<'s> Opaque for McJitMemoryManager {
    type Inner = LLVMOpaqueMCJITMemoryManager;
}

impl<'s> Dispose for McJitMemoryManager {
    unsafe fn dispose(ptr: *mut Self::Inner) {
        unsafe { LLVMDisposeMCJITMemoryManager(ptr) }
    }
}

#[repr(transparent)]
pub struct JitEventListener {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for JitEventListener {
    type Inner = LLVMOpaqueJITEventListener;
}

pub struct MCJITCompilerOptions {
    pub opt_level: u32,
    pub code_model: LLVMCodeModel,
    pub no_frame_pointer_elim: bool,
    pub enable_fast_instruction_select: bool,
    pub mc_jit_memory_manager: Option<Owning<McJitMemoryManager>>,
}

impl Default for MCJITCompilerOptions {
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

#[inline(always)]
pub fn link_in_mc_jit() {
    unsafe { LLVMLinkInMCJIT() };
}

#[inline(always)]
pub fn link_in_interpreter() {
    unsafe { LLVMLinkInInterpreter() };
}

impl GenericValue {
    pub fn create_int<T: IntTypeTag>(ty: &Type<T>, n: u64, is_signed: bool) -> Owning<Self> {
        unsafe { Owning::from_raw(LLVMCreateGenericValueOfInt(ty.as_ptr(), n, is_signed as _)) }
    }

    pub fn create_pointer<T>(p: *mut T) -> Owning<Self> {
        unsafe { Owning::from_raw(LLVMCreateGenericValueOfPointer(p as _)) }
    }

    pub fn create_float<T: FloatTypeTag>(ty: &Type<T>, n: f64) -> Owning<Self> {
        unsafe { Owning::from_raw(LLVMCreateGenericValueOfFloat(ty.as_ptr(), n)) }
    }

    pub fn int_width(&self) -> u32 {
        unsafe { LLVMGenericValueIntWidth(self.as_ptr()) }
    }

    pub fn to_int(&self, is_signed: bool) -> u64 {
        unsafe { LLVMGenericValueToInt(self.as_ptr(), is_signed as _) }
    }

    pub fn to_pointer(&self) -> *mut () {
        unsafe { LLVMGenericValueToPointer(self.as_ptr()) as _ }
    }

    pub fn to_float<T: FloatTypeTag>(&self, ty: &Type<T>) -> f64 {
        unsafe { LLVMGenericValueToFloat(ty.as_ptr(), self.as_ptr()) }
    }
}

impl<'s> ExecutionEngine<'s> {
    pub fn create_execution_engine_for_module(
        module: Owning<Module<'s>>,
    ) -> Result<Owning<Self>, Message> {
        unsafe {
            let mut ptr = null_mut();
            let mut err = null_mut();
            if LLVMCreateExecutionEngineForModule(&mut ptr, module.into_raw(), &mut err) != 0 {
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
            if LLVMCreateInterpreterForModule(&mut ptr, module.into_raw(), &mut err) != 0 {
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
            if LLVMCreateJITCompilerForModule(&mut ptr, module.into_raw(), opt_level, &mut err) != 0
            {
                return Err(Message::from_raw(err));
            }
            Ok(Owning::from_raw(ptr))
        }
    }

    /// Create an MCJIT execution engine for a module, with the given options.
    pub fn create_mc_jit_compiler_for_module(
        module: Owning<Module<'s>>,
        option: MCJITCompilerOptions,
    ) -> Result<Owning<Self>, Message> {
        unsafe {
            let mut ptr = null_mut();
            let mut err = null_mut();
            let mut o = LLVMMCJITCompilerOptions {
                OptLevel: option.opt_level,
                CodeModel: option.code_model,
                NoFramePointerElim: option.no_frame_pointer_elim as _,
                EnableFastISel: option.enable_fast_instruction_select as _,
                MCJMM: Owning::option_into_raw(option.mc_jit_memory_manager),
            };
            if LLVMCreateMCJITCompilerForModule(
                &mut ptr,
                module.into_raw(),
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

    pub fn run_static_constructors(&self) {
        unsafe { LLVMRunStaticConstructors(self.as_ptr()) };
    }

    pub fn run_static_destructors(&self) {
        unsafe { LLVMRunStaticDestructors(self.as_ptr()) };
    }

    pub fn run_function_as_main(
        &self,
        f: &'s Value<fun<(int32, ptr), int32>>,
        args: &[&CStr],
        envs: &[&CStr],
    ) -> i32 {
        let args = args.iter().map(|x| x.as_ptr()).collect::<Vec<_>>();
        let envs = envs
            .iter()
            .map(|x| x.as_ptr())
            .chain([null()])
            .collect::<Vec<_>>();
        unsafe {
            LLVMRunFunctionAsMain(
                self.as_ptr(),
                f.as_ptr(),
                args.len() as u32,
                args.as_ptr(),
                envs.as_ptr(),
            )
        }
    }

    pub fn run_function<T: FunTypeTag>(
        &self,
        f: &'s Value<T>,
        args: &[&GenericValue],
    ) -> Owning<GenericValue> {
        unsafe {
            Owning::from_raw(LLVMRunFunction(
                self.as_ptr(),
                f.as_ptr(),
                args.len() as u32,
                args.as_ptr() as _,
            ))
        }
    }

    pub fn free_machine_code_for_function<T: FunTypeTag>(&self, f: &'s Value<T>) {
        unsafe { LLVMFreeMachineCodeForFunction(self.as_ptr(), f.as_ptr()) };
    }

    pub fn add_module(&self, m: Owning<Module<'s>>) {
        unsafe { LLVMAddModule(self.as_ptr(), m.into_raw()) };
    }

    pub fn remove_module(&self, m: *const Module<'s>) -> Result<Owning<Module>, Message> {
        unsafe {
            let mut ptr = null_mut();
            let mut err = null_mut();
            if LLVMRemoveModule(self.as_ptr(), m as _, &mut ptr, &mut err) != 0 {
                return Err(Message::from_raw(err));
            }
            Ok(Owning::from_raw(ptr))
        }
    }

    pub fn find_function(&self, name: &CStr) -> Option<&'s Value<fun_any>> {
        unsafe {
            let mut ptr = null_mut();
            if LLVMFindFunction(self.as_ptr(), name.as_ptr(), &mut ptr) != 0 {
                return None;
            }
            Some(Value::from_ref(ptr))
        }
    }

    // TODO: LLVMRecompileAndRelinkFunction

    pub fn get_target_data(&self) -> &TargetData {
        unsafe { TargetData::from_ref(LLVMGetExecutionEngineTargetData(self.as_ptr())) }
    }

    pub fn get_target_machine(&self) -> &TargetMachine {
        unsafe { TargetMachine::from_ref(LLVMGetExecutionEngineTargetMachine(self.as_ptr())) }
    }

    pub fn add_global_mapping<T: TypeTag>(&self, global: &'s Value<T>, addr: *mut ()) {
        unsafe { LLVMAddGlobalMapping(self.as_ptr(), global.as_ptr(), addr as _) }
    }

    pub fn get_pointer_to_global<T: TypeTag>(&self, global: &'s Value<T>) -> *mut () {
        unsafe { LLVMGetPointerToGlobal(self.as_ptr(), global.as_ptr()) as _ }
    }

    pub fn get_global_value_address(&self, name: &CStr) -> u64 {
        unsafe { LLVMGetGlobalValueAddress(self.as_ptr(), name.as_ptr()) }
    }

    pub fn get_function_address(&self, name: &CStr) -> u64 {
        unsafe { LLVMGetFunctionAddress(self.as_ptr(), name.as_ptr()) }
    }

    pub fn get_err_msg(&self) -> Result<(), Message> {
        unsafe {
            let mut ptr = null_mut();
            if LLVMExecutionEngineGetErrMsg(self.as_ptr(), &mut ptr) != 0 {
                return Err(Message::from_raw(ptr));
            }
            Ok(())
        }
    }
}

pub trait SimpleMcJitMemoryManager {
    fn allocate_code_section(
        &self,
        size: usize,
        alignment: u32,
        section_id: u32,
        section_name: &CStr,
    ) -> *mut u8;

    fn allocate_data_section(
        &self,
        size: usize,
        alignment: u32,
        section_id: u32,
        section_name: &CStr,
        is_read_only: bool,
    ) -> *mut u8;

    fn finalize_memory(&self) -> Result<(), Message>;
}

impl McJitMemoryManager {
    pub fn create_simple<T: SimpleMcJitMemoryManager>(t: T) -> Owning<Self> {
        let opaque = Box::into_raw(Box::new(t));

        extern "C" fn allocate_code_section_raw<T: SimpleMcJitMemoryManager>(
            this: *mut c_void,
            size: usize,
            alignment: u32,
            section_id: u32,
            section_name: *const c_char,
        ) -> *mut u8 {
            unsafe {
                (*(this as *const T)).allocate_code_section(
                    size,
                    alignment,
                    section_id,
                    CStr::from_ptr(section_name),
                )
            }
        }
        extern "C" fn allocate_data_section_raw<T: SimpleMcJitMemoryManager>(
            this: *mut c_void,
            size: usize,
            alignment: u32,
            section_id: u32,
            section_name: *const c_char,
            is_read_only: LLVMBool,
        ) -> *mut u8 {
            unsafe {
                (*(this as *const T)).allocate_data_section(
                    size,
                    alignment,
                    section_id,
                    CStr::from_ptr(section_name),
                    is_read_only != 0,
                )
            }
        }
        extern "C" fn finalize_memory_raw<T: SimpleMcJitMemoryManager>(
            this: *mut c_void,
            err_msg: *mut *mut c_char,
        ) -> LLVMBool {
            unsafe {
                if let Err(e) = (*(this as *const T)).finalize_memory() {
                    *err_msg = e.into_raw();
                    1
                } else {
                    0
                }
            }
        }
        extern "C" fn destroy_raw<T: SimpleMcJitMemoryManager>(this: *mut c_void) {
            unsafe {
                let _ = Box::from_raw(this as *mut T);
            }
        }

        Self::create_simple_raw(
            opaque as _,
            allocate_code_section_raw::<T>,
            allocate_data_section_raw::<T>,
            finalize_memory_raw::<T>,
            Some(destroy_raw::<T>),
        )
    }

    pub fn create_simple_raw(
        opaque: *mut c_void,
        allocate_code_section: LLVMMemoryManagerAllocateCodeSectionCallback,
        allocate_data_section: LLVMMemoryManagerAllocateDataSectionCallback,
        finalize_memory: LLVMMemoryManagerFinalizeMemoryCallback,
        destroy: LLVMMemoryManagerDestroyCallback,
    ) -> Owning<Self> {
        unsafe {
            Owning::from_raw(LLVMCreateSimpleMCJITMemoryManager(
                opaque,
                allocate_code_section,
                allocate_data_section,
                finalize_memory,
                destroy,
            ))
        }
    }
}

impl JitEventListener {
    pub fn create_gdb_registration_listener() -> &'static Self {
        unsafe { Self::from_ref(LLVMCreateGDBRegistrationListener()) }
    }

    pub fn create_intel_jit_event_listener() -> &'static Self {
        unsafe { Self::from_ref(LLVMCreateIntelJITEventListener()) }
    }

    pub fn create_oprofile_jit_event_listener() -> &'static Self {
        unsafe { Self::from_ref(LLVMCreateOProfileJITEventListener()) }
    }

    pub fn create_perf_jit_event_listener() -> &'static Self {
        unsafe { Self::from_ref(LLVMCreatePerfJITEventListener()) }
    }
}
