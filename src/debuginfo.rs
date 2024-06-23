use llvm_sys::debuginfo::*;
use llvm_sys::{LLVMOpaqueDIBuilder, LLVMOpaqueMetadata};

use crate::owning::{OpaqueDrop, Owning};
use crate::type_tag::TypeTag;
use crate::{DIBuilder, DILocation, DISubprogram, DIType, Instruction, Metadata, Module, Opaque};

#[inline(always)]
pub fn debug_metadata_version() -> u32 {
    unsafe { LLVMDebugMetadataVersion() }
}

impl<'c> Module<'c> {
    pub fn get_debug_metadata_version(&self) -> u32 {
        unsafe { LLVMGetModuleDebugMetadataVersion(self.as_raw()) }
    }

    pub fn strip_module_debug_info(&self) -> bool {
        unsafe { LLVMStripModuleDebugInfo(self.as_raw()) != 0 }
    }

    pub fn create_debug_info_builder_disallow_unresolved<'m>(
        &'m self,
    ) -> Owning<DIBuilder<'m, 'c>> {
        unsafe { Owning::from_raw(LLVMCreateDIBuilderDisallowUnresolved(self.as_raw())) }
    }

    pub fn create_debug_info_builder<'m>(&'m self) -> Owning<DIBuilder<'m, 'c>> {
        unsafe { Owning::from_raw(LLVMCreateDIBuilder(self.as_raw())) }
    }
}

impl OpaqueDrop for LLVMOpaqueDIBuilder {
    unsafe fn drop_raw(ptr: *mut Self) {
        unsafe { LLVMDisposeDIBuilder(ptr) }
    }
}

impl<'m, 'c> DIBuilder<'m, 'c> {
    pub fn finalize(&self) {
        unsafe { LLVMDIBuilderFinalize(self.as_raw()) }
    }

    pub fn finalize_subprogram(&self, subprogram: &DISubprogram) {
        unsafe { LLVMDIBuilderFinalizeSubprogram(self.as_raw(), subprogram.as_raw()) }
    }

    pub fn create_compile_unit(
        &self,
        lang: LLVMDWARFSourceLanguage,
        file_ref: &Metadata,
        producer: &[u8],
        is_optimized: bool,
        flags: &[u8],
        runtime_ver: u32,
        split_name: &[u8],
        kind: LLVMDWARFEmissionKind,
        dwo_id: u32,
        split_debug_inlining: bool,
        debug_info_for_profiling: bool,
        sys_root: &[u8],
        sdk: &[u8],
    ) -> &Metadata {
        unsafe {
            Metadata::from_raw(LLVMDIBuilderCreateCompileUnit(
                self.as_raw(),
                lang,
                file_ref.as_raw(),
                producer.as_ptr() as _,
                producer.len(),
                is_optimized as _,
                flags.as_ptr() as _,
                flags.len(),
                runtime_ver,
                split_name.as_ptr() as _,
                split_name.len(),
                kind,
                dwo_id,
                split_debug_inlining as _,
                debug_info_for_profiling as _,
                sys_root.as_ptr() as _,
                sys_root.len(),
                sdk.as_ptr() as _,
                sdk.len(),
            ))
        }
    }

    pub fn create_file(&self, filename: &[u8], directory: &[u8]) -> &Metadata {
        unsafe {
            Metadata::from_raw(LLVMDIBuilderCreateFile(
                self.as_raw(),
                filename.as_ptr() as _,
                filename.len(),
                directory.as_ptr() as _,
                directory.len(),
            ))
        }
    }
    // TODO
}

impl DIType {
    pub fn get_name(&self) -> &[u8] {
        unsafe {
            let mut len = 0;
            let ptr = LLVMDITypeGetName(self.as_raw(), &mut len);
            std::slice::from_raw_parts(ptr as _, len)
        }
    }

    pub fn get_size_in_bits(&self) -> u64 {
        unsafe { LLVMDITypeGetSizeInBits(self.as_raw()) }
    }
}

// TODO

impl OpaqueDrop for LLVMOpaqueMetadata {
    unsafe fn drop_raw(ptr: *mut Self) {
        unsafe { LLVMDisposeTemporaryMDNode(ptr) }
    }
}

// TODO

impl DISubprogram {
    pub fn get_line(&self) -> u32 {
        unsafe { LLVMDISubprogramGetLine(self.as_raw()) }
    }
}

impl<T: TypeTag> Instruction<T> {
    pub fn get_debug_loc(&self) -> &DILocation {
        unsafe { DILocation::from_raw(LLVMInstructionGetDebugLoc(self.as_raw())) }
    }

    pub fn set_debug_loc(&self, loc: &DILocation) {
        unsafe { LLVMInstructionSetDebugLoc(self.as_raw(), loc.as_raw()) }
    }
}

impl Metadata {
    pub fn get_kind(&self) -> LLVMMetadataKind {
        unsafe { LLVMGetMetadataKind(self.as_raw()) }
    }
}
