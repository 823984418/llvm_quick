use std::ffi::{CStr, CString};
use std::ptr::null;

use llvm_sys::lto::*;

use crate::owning::{OpaqueDrop, Owning};
use crate::*;

#[inline(always)]
unsafe fn check_error(v: bool) -> Result<(), CString> {
    if v {
        Err(unsafe { get_error_message() })
    } else {
        Ok(())
    }
}

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

#[inline(always)]
pub fn get_version() -> &'static CStr {
    unsafe { CStr::from_ptr(lto_get_version()) }
}

#[inline(always)]
pub unsafe fn get_error_message() -> CString {
    unsafe { CStr::from_ptr(lto_get_error_message()).into() }
}

#[inline(always)]
pub fn module_is_object_file(path: &CStr) -> bool {
    unsafe { lto_module_is_object_file(path.as_ptr()) != 0 }
}

#[inline(always)]
pub fn module_is_object_file_for_target(path: &CStr, target_triple_prefix: &CStr) -> bool {
    unsafe {
        lto_module_is_object_file_for_target(path.as_ptr(), target_triple_prefix.as_ptr()) != 0
    }
}

#[inline(always)]
pub fn module_has_objc_category(mem: &[u8]) -> bool {
    unsafe { lto_module_has_objc_category(mem.as_ptr() as _, mem.len()) != 0 }
}

#[inline(always)]
pub fn module_is_object_file_in_memory(mem: &[u8]) -> bool {
    unsafe { lto_module_is_object_file_in_memory(mem.as_ptr() as _, mem.len()) != 0 }
}

#[inline(always)]
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
    #[inline(always)]
    pub fn create(path: &CStr) -> Owning<LTOModule> {
        unsafe { Owning::from_raw(lto_module_create(path.as_ptr())) }
    }

    #[inline(always)]
    pub fn create_from_memory(mem: &[u8]) -> Owning<LTOModule> {
        unsafe { Owning::from_raw(lto_module_create_from_memory(mem.as_ptr() as _, mem.len())) }
    }

    #[inline(always)]
    pub fn create_from_memory_with_path(mem: &[u8], path: &CStr) -> Owning<LTOModule> {
        unsafe {
            Owning::from_raw(lto_module_create_from_memory_with_path(
                mem.as_ptr() as _,
                mem.len(),
                path.as_ptr(),
            ))
        }
    }

    #[inline(always)]
    pub fn create_in_local_context(mem: &[u8], path: &CStr) -> Owning<LTOModule> {
        unsafe {
            Owning::from_raw(lto_module_create_in_local_context(
                mem.as_ptr() as _,
                mem.len(),
                path.as_ptr(),
            ))
        }
    }

    #[inline(always)]
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

    #[inline(always)]
    pub fn create_from_fd(fd: i32, path: &CStr, file_size: usize) -> Owning<LTOModule> {
        unsafe { Owning::from_raw(lto_module_create_from_fd(fd, path.as_ptr(), file_size)) }
    }

    #[inline(always)]
    pub fn create_from_fd_at_offset(
        fd: i32,
        path: &CStr,
        file_size: usize,
        map_size: usize,
        offset: isize,
    ) -> Owning<LTOModule> {
        unsafe {
            Owning::from_raw(lto_module_create_from_fd_at_offset(
                fd,
                path.as_ptr(),
                file_size,
                map_size,
                offset as _,
            ))
        }
    }
}

impl OpaqueDrop for LLVMOpaqueLTOModule {
    #[inline(always)]
    unsafe fn drop_raw(ptr: *mut Self) {
        unsafe { lto_module_dispose(ptr) }
    }
}

impl LTOModule {
    #[inline(always)]
    pub fn get_target_triple(&self) -> &CStr {
        unsafe { CStr::from_ptr(lto_module_get_target_triple(self.as_raw())) }
    }

    #[inline(always)]
    pub unsafe fn set_target_triple(&self, triple: &CStr) {
        unsafe { lto_module_set_target_triple(self.as_raw(), triple.as_ptr()) }
    }

    #[inline(always)]
    pub fn get_num_symbols(&self) -> u32 {
        unsafe { lto_module_get_num_symbols(self.as_raw()) }
    }

    #[inline(always)]
    pub fn get_symbol_name(&self, index: u32) -> &CStr {
        assert!(index < self.get_num_symbols());
        unsafe { CStr::from_ptr(lto_module_get_symbol_name(self.as_raw(), index)) }
    }

    #[inline(always)]
    pub fn get_symbol_attribute(&self, index: u32) -> lto_symbol_attributes {
        assert!(index < self.get_num_symbols());
        unsafe { lto_module_get_symbol_attribute(self.as_raw(), index) }
    }

    #[inline(always)]
    pub fn get_linkerops(&self) -> &CStr {
        unsafe { CStr::from_ptr(lto_module_get_linkeropts(self.as_raw())) }
    }

    #[inline(always)]
    pub fn get_macho_cputype(&self) -> Result<(u32, u32), CString> {
        unsafe {
            let mut cputype = 0;
            let mut cpusubtype = 0;
            check_error(
                lto_module_get_macho_cputype(self.as_raw(), &mut cputype, &mut cpusubtype) != 0,
            )
            .map(|_| (cputype, cpusubtype))
        }
    }

    #[inline(always)]
    pub fn has_ctor_dtor(&self) -> bool {
        unsafe { lto_module_has_ctor_dtor(self.as_raw()) != 0 }
    }
}

impl LTOCodeGenerator {
    #[inline(always)]
    pub fn set_diagnostic_handler(&self, handler: lto_diagnostic_handler_t, ctx: *const ()) {
        unsafe { lto_codegen_set_diagnostic_handler(self.as_raw(), handler, ctx as _) }
    }

    #[inline(always)]
    pub fn create() -> Owning<LTOCodeGenerator> {
        unsafe { Owning::from_raw(lto_codegen_create()) }
    }

    #[inline(always)]
    pub fn create_in_local_context() -> Owning<LTOCodeGenerator> {
        unsafe { Owning::from_raw(lto_codegen_create_in_local_context()) }
    }
}

impl OpaqueDrop for LLVMOpaqueLTOCodeGenerator {
    #[inline(always)]
    unsafe fn drop_raw(ptr: *mut Self) {
        unsafe { lto_codegen_dispose(ptr) }
    }
}

impl LTOCodeGenerator {
    #[inline(always)]
    pub fn add_module(&self, module: &LTOModule) -> Result<(), CString> {
        unsafe { check_error(lto_codegen_add_module(self.as_raw(), module.as_raw()) != 0) }
    }

    #[inline(always)]
    pub fn set_module(&self, module: Owning<LTOModule>) {
        unsafe { lto_codegen_set_module(self.as_raw(), module.into_raw()) }
    }

    #[inline(always)]
    pub fn set_debug_model(&self, model: lto_debug_model) -> Result<(), CString> {
        unsafe { check_error(lto_codegen_set_debug_model(self.as_raw(), model) != 0) }
    }

    #[inline(always)]
    pub fn set_pic_model(&self, model: lto_codegen_model) -> Result<(), CString> {
        unsafe { check_error(lto_codegen_set_pic_model(self.as_raw(), model) != 0) }
    }

    #[inline(always)]
    pub fn set_cpu(&self, cpu: &CStr) {
        unsafe { lto_codegen_set_cpu(self.as_raw(), cpu.as_ptr()) }
    }

    #[deprecated]
    #[inline(always)]
    pub fn set_assembler_path(&self, path: &CStr) {
        unsafe { lto_codegen_set_assembler_path(self.as_raw(), path.as_ptr()) }
    }

    #[deprecated]
    #[inline(always)]
    pub fn set_assembler_args(&self, args: &[&CStr]) {
        let args = args.iter().map(|x| x.as_ptr()).collect::<Vec<_>>();
        unsafe {
            lto_codegen_set_assembler_args(self.as_raw(), args.as_ptr() as _, args.len() as _)
        }
    }

    #[inline(always)]
    pub fn add_must_preserve_symbol(&self, symbol: &CStr) {
        unsafe { lto_codegen_add_must_preserve_symbol(self.as_raw(), symbol.as_ptr()) }
    }

    #[inline(always)]
    pub fn write_merged_modules(&self, path: &CStr) -> Result<(), CString> {
        unsafe { check_error(lto_codegen_write_merged_modules(self.as_raw(), path.as_ptr()) != 0) }
    }

    #[inline(always)]
    pub fn compile(&self) -> Result<&[u8], CString> {
        unsafe {
            let mut len = 0;
            let ptr = lto_codegen_compile(self.as_raw(), &mut len);
            check_error(ptr.is_null())?;
            Ok(std::slice::from_raw_parts(ptr as _, len))
        }
    }

    #[inline(always)]
    pub fn compile_to_file(&self) -> Result<CString, CString> {
        unsafe {
            let mut name = null();
            check_error(lto_codegen_compile_to_file(self.as_raw(), &mut name) != 0)?;
            Ok(CStr::from_ptr(name).into())
        }
    }

    #[inline(always)]
    pub fn optimize(&self) -> Result<(), CString> {
        unsafe { check_error(lto_codegen_optimize(self.as_raw()) != 0) }
    }

    #[inline(always)]
    pub fn compile_optimized(&self) -> Result<&[u8], CString> {
        unsafe {
            let mut len = 0;
            let ptr = lto_codegen_compile_optimized(self.as_raw(), &mut len);
            check_error(ptr.is_null())?;
            Ok(std::slice::from_raw_parts(ptr as _, len))
        }
    }
}

#[inline(always)]
pub fn api_version() -> u32 {
    unsafe { lto_api_version() }
}

#[inline(always)]
pub fn set_debug_options(options: &[&CStr]) {
    let options = options.iter().map(|x| x.as_ptr()).collect::<Vec<_>>();
    unsafe { lto_set_debug_options(options.as_ptr() as _, options.len() as _) }
}

impl LTOCodeGenerator {
    #[inline(always)]
    pub fn debug_options(&self, options: &CStr) {
        unsafe { lto_codegen_debug_options(self.as_raw(), options.as_ptr()) }
    }

    #[inline(always)]
    pub fn debug_options_array(&self, options: &[&CStr]) {
        let options = options.iter().map(|x| x.as_ptr()).collect::<Vec<_>>();
        unsafe {
            lto_codegen_debug_options_array(self.as_raw(), options.as_ptr(), options.len() as _)
        }
    }
}

#[inline(always)]
pub fn initialize_disassembler() {
    unsafe { lto_initialize_disassembler() }
}

impl LTOCodeGenerator {
    #[inline(always)]
    pub fn set_should_internalize(&self, should_internalize: bool) {
        unsafe { lto_codegen_set_should_internalize(self.as_raw(), should_internalize as _) }
    }

    #[inline(always)]
    pub fn set_should_embed_uselists(&self, should_embed_uselists: bool) {
        unsafe { lto_codegen_set_should_embed_uselists(self.as_raw(), should_embed_uselists as _) }
    }
}

impl ThinLTOCodeGenerator {
    #[inline(always)]
    pub fn create() -> Owning<ThinLTOCodeGenerator> {
        unsafe { Owning::from_raw(thinlto_create_codegen()) }
    }
}

impl OpaqueDrop for LLVMOpaqueThinLTOCodeGenerator {
    #[inline(always)]
    unsafe fn drop_raw(ptr: *mut Self) {
        unsafe { thinlto_codegen_dispose(ptr) }
    }
}

impl ThinLTOCodeGenerator {
    #[inline(always)]
    pub fn add_module(&self, identifier: &CStr, data: &[u8]) {
        unsafe {
            thinlto_codegen_add_module(
                self.as_raw(),
                identifier.as_ptr(),
                data.as_ptr() as _,
                data.len() as _,
            )
        }
    }

    #[inline(always)]
    pub fn process(&self) {
        unsafe { thinlto_codegen_process(self.as_raw()) }
    }

    #[inline(always)]
    pub fn get_num_objects(&self) -> u32 {
        unsafe { thinlto_module_get_num_objects(self.as_raw()) as _ }
    }

    #[inline(always)]
    pub fn get_object(&self, index: u32) -> &[u8] {
        assert!(index < self.get_num_objects());
        unsafe {
            let buffer = thinlto_module_get_object(self.as_raw(), index);
            #[repr(C)]
            pub struct Buffer {
                buffer: *const u8,
                size: usize,
            }
            let buffer: Buffer = std::mem::transmute(buffer);
            std::slice::from_raw_parts(buffer.buffer, buffer.size)
        }
    }

    #[inline(always)]
    pub fn get_num_object_files(&self) -> u32 {
        unsafe { thinlto_module_get_num_object_files(self.as_raw()) }
    }

    #[inline(always)]
    pub fn get_object_file(&self, index: u32) -> &CStr {
        assert!(index < self.get_num_object_files());
        unsafe { CStr::from_ptr(thinlto_module_get_object_file(self.as_raw(), index)) }
    }

    #[inline(always)]
    pub fn set_pic_model(&self, model: lto_codegen_model) -> Result<(), CString> {
        unsafe { check_error(thinlto_codegen_set_pic_model(self.as_raw(), model) != 0) }
    }

    #[inline(always)]
    pub fn set_cache_dir(&self, cache_dir: &CStr) {
        unsafe { thinlto_codegen_set_cache_dir(self.as_raw(), cache_dir.as_ptr()) }
    }

    #[inline(always)]
    pub fn set_cache_pruning_interval(&self, interval: i32) {
        unsafe { thinlto_codegen_set_cache_pruning_interval(self.as_raw(), interval) }
    }

    #[inline(always)]
    pub fn set_final_cache_size_relative_to_available_space(&self, percentage: u32) {
        unsafe {
            thinlto_codegen_set_final_cache_size_relative_to_available_space(
                self.as_raw(),
                percentage,
            )
        }
    }

    #[inline(always)]
    pub fn set_cache_entry_expiration(&self, expiration: u32) {
        unsafe {
            thinlto_codegen_set_final_cache_size_relative_to_available_space(
                self.as_raw(),
                expiration,
            )
        }
    }

    #[inline(always)]
    pub fn set_cache_size_bytes(&self, max_size_bytes: u32) {
        unsafe { thinlto_codegen_set_cache_size_bytes(self.as_raw(), max_size_bytes) }
    }

    #[inline(always)]
    pub fn set_cache_size_megabytes(&self, max_size_megabytes: u32) {
        unsafe { thinlto_codegen_set_cache_size_megabytes(self.as_raw(), max_size_megabytes) }
    }

    #[inline(always)]
    pub fn set_cache_size_files(&self, max_size_files: u32) {
        unsafe { thinlto_codegen_set_cache_size_files(self.as_raw(), max_size_files) }
    }
}

impl LTOInput {
    #[inline(always)]
    pub fn create(buffer: &[u8], path: &CStr) -> Owning<LTOInput> {
        unsafe {
            Owning::from_raw(lto_input_create(
                buffer.as_ptr() as _,
                buffer.len(),
                path.as_ptr(),
            ))
        }
    }
}

impl OpaqueDrop for LLVMOpaqueLTOInput {
    #[inline(always)]
    unsafe fn drop_raw(ptr: *mut Self) {
        unsafe { lto_input_dispose(ptr) }
    }
}

impl LTOInput {
    #[inline(always)]
    pub fn get_num_dependent_libraries(&self) -> u32 {
        unsafe { lto_input_get_num_dependent_libraries(self.as_raw()) }
    }

    #[inline(always)]
    pub fn get_dependent_library(&self, index: u32) -> &[u8] {
        assert!(index < self.get_num_dependent_libraries());
        unsafe {
            let mut size = 0;
            let ptr = lto_input_get_dependent_library(self.as_raw(), index as usize, &mut size);
            std::slice::from_raw_parts(ptr as _, size)
        }
    }
}

#[inline(always)]
pub fn runtime_lib_symbols_list() -> Vec<&'static CStr> {
    unsafe {
        let mut size = 0;
        let ptr = lto_runtime_lib_symbols_list(&mut size);
        std::slice::from_raw_parts(ptr, size)
            .iter()
            .copied()
            .map(|x| CStr::from_ptr(x))
            .collect()
    }
}

impl ThinLTOCodeGenerator {
    #[inline(always)]
    pub fn set_savetemps_dir(&self, save_temps_dir: &CStr) {
        unsafe { thinlto_codegen_set_savetemps_dir(self.as_raw(), save_temps_dir.as_ptr()) }
    }

    #[inline(always)]
    pub fn set_generated_objects_dir(&self, save_temps_dir: &CStr) {
        unsafe { thinlto_set_generated_objects_dir(self.as_raw(), save_temps_dir.as_ptr()) }
    }

    #[inline(always)]
    pub fn set_cpu(&self, cpu: &CStr) {
        unsafe { thinlto_codegen_set_cpu(self.as_raw(), cpu.as_ptr()) }
    }

    #[inline(always)]
    pub fn disable_codegen(&self, disable: bool) {
        unsafe { thinlto_codegen_disable_codegen(self.as_raw(), disable as _) }
    }

    #[inline(always)]
    pub fn set_codegen_only(&self, codegen_only: bool) {
        unsafe { thinlto_codegen_set_codegen_only(self.as_raw(), codegen_only as _) }
    }
}

#[inline(always)]
pub fn debug_options(options: &[&CStr]) {
    let options = options
        .iter()
        .copied()
        .map(CStr::as_ptr)
        .collect::<Vec<_>>();
    unsafe { thinlto_debug_options(options.as_ptr(), options.len() as _) }
}

impl LTOModule {
    #[inline(always)]
    pub fn is_thinlto(&self) -> bool {
        unsafe { lto_module_is_thinlto(self.as_raw()) != 0 }
    }
}

impl ThinLTOCodeGenerator {
    #[inline(always)]
    pub fn add_must_preserve_symbol(&self, name: &[u8]) {
        unsafe {
            thinlto_codegen_add_must_preserve_symbol(
                self.as_raw(),
                name.as_ptr() as _,
                name.len() as _,
            )
        }
    }

    #[inline(always)]
    pub fn add_cross_referenced_symbol(&self, name: &[u8]) {
        unsafe {
            thinlto_codegen_add_cross_referenced_symbol(
                self.as_raw(),
                name.as_ptr() as _,
                name.len() as _,
            )
        }
    }
}
