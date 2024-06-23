use std::ffi::{CStr, CString};

use llvm_sys::lto::*;

use crate::owning::{OpaqueDrop, Owning};
use crate::{Opaque, PhantomOpaque};

#[repr(transparent)]
pub struct LTOModule {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for LTOModule {
    type Inner = LLVMOpaqueLTOModule;
}

#[repr(transparent)]
pub struct LTOCodeGenerator {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for LTOCodeGenerator {
    type Inner = LLVMOpaqueLTOCodeGenerator;
}

#[repr(transparent)]
pub struct ThinLTOCodeGenerator {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for ThinLTOCodeGenerator {
    type Inner = LLVMOpaqueThinLTOCodeGenerator;
}

#[repr(transparent)]
pub struct LTOInput {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for LTOInput {
    type Inner = LLVMOpaqueLTOInput;
}

pub fn get_version() -> &'static CStr {
    unsafe { CStr::from_ptr(lto_get_version()) }
}

pub fn get_error_message() -> CString {
    unsafe { CStr::from_ptr(lto_get_error_message()).into() }
}

pub fn module_is_object_file(path: &CStr) -> bool {
    unsafe { lto_module_is_object_file(path.as_ptr()) != 0 }
}

pub fn module_is_object_file_for_target(path: &CStr, target_triple_prefix: &CStr) -> bool {
    unsafe {
        lto_module_is_object_file_for_target(path.as_ptr(), target_triple_prefix.as_ptr()) != 0
    }
}

pub fn module_has_objc_category(mem: &[u8]) -> bool {
    unsafe { lto_module_has_objc_category(mem.as_ptr() as _, mem.len()) != 0 }
}

pub fn module_is_object_file_in_memory(mem: &[u8]) -> bool {
    unsafe { lto_module_is_object_file_in_memory(mem.as_ptr() as _, mem.len()) != 0 }
}

pub fn module_is_object_file_in_memory_for_target(mem: &[u8], target_triple_prefix: &CStr) -> bool {
    unsafe {
        lto_module_is_object_file_in_memory_for_target(
            mem.as_ptr() as _,
            mem.len(),
            target_triple_prefix.as_ptr(),
        ) != 0
    }
}

impl LTOModule {
    pub fn create(path: &CStr) -> Owning<LTOModule> {
        unsafe { Owning::from_raw(lto_module_create(path.as_ptr())) }
    }

    pub fn create_from_memory(mem: &[u8]) -> Owning<LTOModule> {
        unsafe { Owning::from_raw(lto_module_create_from_memory(mem.as_ptr() as _, mem.len())) }
    }

    pub fn create_from_memory_with_path(mem: &[u8], path: &CStr) -> Owning<LTOModule> {
        unsafe {
            Owning::from_raw(lto_module_create_from_memory_with_path(
                mem.as_ptr() as _,
                mem.len(),
                path.as_ptr(),
            ))
        }
    }

    pub fn create_in_local_context(mem: &[u8], path: &CStr) -> Owning<LTOModule> {
        unsafe {
            Owning::from_raw(lto_module_create_in_local_context(
                mem.as_ptr() as _,
                mem.len(),
                path.as_ptr(),
            ))
        }
    }

    pub fn lto_module_create_in_codegen_context(
        mem: &[u8],
        path: &CStr,
        cg: &LTOCodeGenerator,
    ) -> Owning<LTOModule> {
        unsafe {
            Owning::from_raw(lto_module_create_in_codegen_context(
                mem.as_ptr() as _,
                mem.len(),
                path.as_ptr(),
                cg.as_raw(),
            ))
        }
    }

    pub fn create_from_fd(fd: i32, path: &CStr, file_size: usize) -> Owning<LTOModule> {
        unsafe { Owning::from_raw(lto_module_create_from_fd(fd, path.as_ptr(), file_size)) }
    }

    pub fn create_from_fd_at_offset(
        fd: i32,
        path: &CStr,
        file_size: usize,
        map_size: usize,
        offset: i32,
    ) -> Owning<LTOModule> {
        unsafe {
            Owning::from_raw(lto_module_create_from_fd_at_offset(
                fd,
                path.as_ptr(),
                file_size,
                map_size,
                offset,
            ))
        }
    }
}

impl OpaqueDrop for LLVMOpaqueLTOModule {
    unsafe fn drop_raw(ptr: *mut Self) {
        unsafe { lto_module_dispose(ptr) }
    }
}

impl LTOModule {
    pub fn get_target_triple(&self) -> &CStr {
        unsafe { CStr::from_ptr(lto_module_get_target_triple(self.as_raw())) }
    }

    pub unsafe fn set_target_triple(&self, triple: &CStr) {
        unsafe { lto_module_set_target_triple(self.as_raw(), triple.as_ptr()) }
    }

    pub fn get_num_symbols(&self) -> u32 {
        unsafe { lto_module_get_num_symbols(self.as_raw()) }
    }

    pub fn get_symbol_name(&self, index: u32) -> &CStr {
        assert!(index < self.get_num_symbols());
        unsafe { CStr::from_ptr(lto_module_get_symbol_name(self.as_raw(), index)) }
    }

    pub fn get_symbol_attribute(&self, index: u32) -> lto_symbol_attributes {
        assert!(index < self.get_num_symbols());
        unsafe { lto_module_get_symbol_attribute(self.as_raw(), index) }
    }

    pub fn get_linkerops(&self) -> &CStr {
        unsafe { CStr::from_ptr(lto_module_get_linkeropts(self.as_raw())) }
    }

    pub fn get_macho_cputype(&self) -> Result<(u32, u32), CString> {
        unsafe {
            let mut cputype = 0;
            let mut cpusubtype = 0;
            if lto_module_get_macho_cputype(self.as_raw(), &mut cputype, &mut cpusubtype) != 0 {
                Err(get_error_message())
            } else {
                Ok((cputype, cpusubtype))
            }
        }
    }

    pub fn has_ctor_dtor(&self) -> bool {
        unsafe { lto_module_has_ctor_dtor(self.as_raw()) != 0 }
    }
}

impl LTOCodeGenerator {
    pub fn set_diagnostic_handler(&self, handler: lto_diagnostic_handler_t, ctx: *const ()) {
        unsafe { lto_codegen_set_diagnostic_handler(self.as_raw(), handler, ctx as _) }
    }

    pub fn create() -> Owning<LTOCodeGenerator> {
        unsafe { Owning::from_raw(lto_codegen_create()) }
    }

    pub fn create_in_local_context() -> Owning<LTOCodeGenerator> {
        unsafe { Owning::from_raw(lto_codegen_create_in_local_context()) }
    }
}

impl OpaqueDrop for LLVMOpaqueLTOCodeGenerator {
    unsafe fn drop_raw(ptr: *mut Self) {
        unsafe { lto_codegen_dispose(ptr) }
    }
}

impl LTOCodeGenerator {
    pub fn add_module(&self, module: &LTOModule) -> Result<(), CString> {
        unsafe {
            if lto_codegen_add_module(self.as_raw(), module.as_raw()) != 0 {
                Err(get_error_message())
            } else {
                Ok(())
            }
        }
    }

    pub fn set_module(&self, module: Owning<LTOModule>) {
        unsafe { lto_codegen_set_module(self.as_raw(), module.into_raw()) }
    }

    pub fn set_debug_model(&self, model: lto_debug_model) -> Result<(), CString> {
        unsafe {
            if lto_codegen_set_debug_model(self.as_raw(), model) != 0 {
                Err(get_error_message())
            } else {
                Ok(())
            }
        }
    }

    pub fn set_pic_model(&self, model: lto_codegen_model) -> Result<(), CString> {
        unsafe {
            if lto_codegen_set_pic_model(self.as_raw(), model) != 0 {
                Err(get_error_message())
            } else {
                Ok(())
            }
        }
    }

    pub fn set_cpu(&self, cpu: &CStr) {
        unsafe { lto_codegen_set_cpu(self.as_raw(), cpu.as_ptr()) }
    }

    pub fn set_assembler_path(&self, path: &CStr) {
        unsafe { lto_codegen_set_assembler_path(self.as_raw(), path.as_ptr()) }
    }
}

// TODO
