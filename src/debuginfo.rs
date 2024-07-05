use llvm_sys::debuginfo::*;
use llvm_sys::LLVMOpaqueDIBuilder;

use crate::owning::{OpaqueDrop, Owning};
use crate::type_tag::*;
use crate::*;

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
    ) -> &'c DICompileUnit {
        unsafe {
            DICompileUnit::from_raw(LLVMDIBuilderCreateCompileUnit(
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

    pub fn create_file(&self, filename: &[u8], directory: &[u8]) -> &'c DIFile {
        unsafe {
            DIFile::from_raw(LLVMDIBuilderCreateFile(
                self.as_raw(),
                filename.as_ptr() as _,
                filename.len(),
                directory.as_ptr() as _,
                directory.len(),
            ))
        }
    }

    pub fn create_module(
        &self,
        parent_scope: &Metadata,
        name: &[u8],
        config_macros: &[u8],
        include_path: &[u8],
        api_notes_file: &[u8],
    ) -> &'c DIModule {
        unsafe {
            DIModule::from_raw(LLVMDIBuilderCreateModule(
                self.as_raw(),
                parent_scope.as_raw(),
                name.as_ptr() as _,
                name.len(),
                config_macros.as_ptr() as _,
                config_macros.len(),
                include_path.as_ptr() as _,
                include_path.len(),
                api_notes_file.as_ptr() as _,
                api_notes_file.len(),
            ))
        }
    }

    pub fn create_name_space(
        &self,
        parent_scope: &Metadata,
        name: &[u8],
        export_symbols: bool,
    ) -> &'c DINamespace {
        unsafe {
            DINamespace::from_raw(LLVMDIBuilderCreateNameSpace(
                self.as_raw(),
                parent_scope.as_raw(),
                name.as_ptr() as _,
                name.len(),
                export_symbols as _,
            ))
        }
    }

    pub fn create_function(
        &self,
        scope: &Metadata,
        name: &[u8],
        linkage_name: &[u8],
        file: &Metadata,
        line_no: u32,
        ty: &Metadata,
        is_local_to_unit: bool,
        is_definition: bool,
        scope_line: u32,
        flags: LLVMDIFlags,
        is_optimized: bool,
    ) -> &'c Metadata {
        unsafe {
            Metadata::from_raw(LLVMDIBuilderCreateFunction(
                self.as_raw(),
                scope.as_raw(),
                name.as_ptr() as _,
                name.len(),
                linkage_name.as_ptr() as _,
                linkage_name.len(),
                file.as_raw(),
                line_no,
                ty.as_raw(),
                is_local_to_unit as _,
                is_definition as _,
                scope_line,
                flags,
                is_optimized as _,
            ))
        }
    }

    pub fn create_lexical_block(
        &self,
        scope: &Metadata,
        file: &Metadata,
        line: u32,
        column: u32,
    ) -> &'c DILexicalBlock {
        unsafe {
            DILexicalBlock::from_raw(LLVMDIBuilderCreateLexicalBlock(
                self.as_raw(),
                scope.as_raw(),
                file.as_raw(),
                line,
                column,
            ))
        }
    }

    pub fn create_lexical_block_file(
        &self,
        scope: &DIScope,
        file: &Metadata,
        discriminator: u32,
    ) -> &'c DILexicalBlockFile {
        unsafe {
            DILexicalBlockFile::from_raw(LLVMDIBuilderCreateLexicalBlockFile(
                self.as_raw(),
                scope.as_raw(),
                file.as_raw(),
                discriminator,
            ))
        }
    }

    pub fn create_imported_module_from_namespace(
        &self,
        scope: &DIScope,
        ns: &Metadata,
        file: &Metadata,
        line: u32,
    ) -> &'c Metadata {
        unsafe {
            Metadata::from_raw(LLVMDIBuilderCreateImportedModuleFromNamespace(
                self.as_raw(),
                scope.as_raw(),
                ns.as_raw(),
                file.as_raw(),
                line,
            ))
        }
    }

    pub fn create_imported_module_from_alias(
        &self,
        scope: &DIScope,
        imported_entity: &Metadata,
        file: &Metadata,
        line: u32,
        elements: &[&Metadata],
    ) -> &'c Metadata {
        unsafe {
            Metadata::from_raw(LLVMDIBuilderCreateImportedModuleFromAlias(
                self.as_raw(),
                scope.as_raw(),
                imported_entity.as_raw(),
                file.as_raw(),
                line,
                elements.as_ptr() as _,
                elements.len() as _,
            ))
        }
    }

    pub fn create_imported_module_from_module(
        &self,
        scope: &DIScope,
        m: &Metadata,
        file: &Metadata,
        line: u32,
        elements: &[&Metadata],
    ) -> &'c Metadata {
        unsafe {
            Metadata::from_raw(LLVMDIBuilderCreateImportedModuleFromModule(
                self.as_raw(),
                scope.as_raw(),
                m.as_raw(),
                file.as_raw(),
                line,
                elements.as_ptr() as _,
                elements.len() as _,
            ))
        }
    }

    pub fn create_imported_declaration(
        &self,
        scope: &DIScope,
        decl: &Metadata,
        file: &Metadata,
        line: u32,
        name: &[u8],
        elements: &[&Metadata],
    ) -> &'c Metadata {
        unsafe {
            Metadata::from_raw(LLVMDIBuilderCreateImportedDeclaration(
                self.as_raw(),
                scope.as_raw(),
                decl.as_raw(),
                file.as_raw(),
                line,
                name.as_ptr() as _,
                name.len(),
                elements.as_ptr() as _,
                elements.len() as _,
            ))
        }
    }
}

impl Context {
    pub fn create_debug_location(
        &self,
        line: u32,
        column: u32,
        scope: &DIScope,
        inline_at: &Metadata,
    ) -> &DILocation {
        unsafe {
            DILocation::from_raw(LLVMDIBuilderCreateDebugLocation(
                self.as_raw(),
                line,
                column,
                scope.as_raw(),
                inline_at.as_raw(),
            ))
        }
    }
}

impl DILocation {
    pub fn get_line(&self) -> u32 {
        unsafe { LLVMDILocationGetLine(self.as_raw()) }
    }

    pub fn get_column(&self) -> u32 {
        unsafe { LLVMDILocationGetColumn(self.as_raw()) }
    }

    pub fn get_scope(&self) -> &DIScope {
        unsafe { DIScope::from_raw(LLVMDILocationGetScope(self.as_raw())) }
    }

    pub fn get_inlined_ad(&self) -> &Metadata {
        unsafe { Metadata::from_raw(LLVMDILocationGetInlinedAt(self.as_raw())) }
    }
}

impl DIScope {
    pub fn get_file(&self) -> &Metadata {
        unsafe { Metadata::from_raw(LLVMDIScopeGetFile(self.as_raw())) }
    }
}

impl DIFile {
    pub fn get_directory(&self) -> &[u8] {
        unsafe {
            let mut len = 0;
            let ptr = LLVMDIFileGetDirectory(self.as_raw(), &mut len);
            std::slice::from_raw_parts(ptr as _, len as _)
        }
    }

    pub fn get_filename(&self) -> &[u8] {
        unsafe {
            let mut len = 0;
            let ptr = LLVMDIFileGetFilename(self.as_raw(), &mut len);
            std::slice::from_raw_parts(ptr as _, len as _)
        }
    }

    pub fn get_source(&self) -> &[u8] {
        unsafe {
            let mut len = 0;
            let ptr = LLVMDIFileGetSource(self.as_raw(), &mut len);
            std::slice::from_raw_parts(ptr as _, len as _)
        }
    }
}

impl<'m, 'c> DIBuilder<'m, 'c> {
    pub fn get_or_create_type_array(&self, data: &[&Metadata]) -> &'c Metadata {
        unsafe {
            Metadata::from_raw(LLVMDIBuilderGetOrCreateTypeArray(
                self.as_raw(),
                data.as_ptr() as _,
                data.len(),
            ))
        }
    }

    pub fn create_subroutine_type(
        &self,
        file: &Metadata,
        parameter_types: &[&Metadata],
        flags: LLVMDIFlags,
    ) -> &'c DISubroutineType {
        unsafe {
            DISubroutineType::from_raw(LLVMDIBuilderCreateSubroutineType(
                self.as_raw(),
                file.as_raw(),
                parameter_types.as_ptr() as _,
                parameter_types.len() as _,
                flags,
            ))
        }
    }

    pub fn create_macro(
        &self,
        parent_macro_file: &Metadata,
        line: u32,
        record_type: LLVMDWARFMacinfoRecordType,
        name: &[u8],
        value: &[u8],
    ) -> &'c DIMacro {
        unsafe {
            DIMacro::from_raw(LLVMDIBuilderCreateMacro(
                self.as_raw(),
                parent_macro_file.as_raw(),
                line,
                record_type,
                name.as_ptr() as _,
                name.len(),
                value.as_ptr() as _,
                value.len(),
            ))
        }
    }

    pub fn create_temp_macro_file(
        &self,
        parent_macro_file: &DIMacroFile,
        line: u32,
        file: &Metadata,
    ) -> &'c DIMacroFile {
        unsafe {
            DIMacroFile::from_raw(LLVMDIBuilderCreateTempMacroFile(
                self.as_raw(),
                parent_macro_file.as_raw(),
                line,
                file.as_raw(),
            ))
        }
    }

    pub fn create_enumerator(
        &self,
        name: &[u8],
        value: i64,
        is_unsigned: bool,
    ) -> &'c DIEnumerator {
        unsafe {
            DIEnumerator::from_raw(LLVMDIBuilderCreateEnumerator(
                self.as_raw(),
                name.as_ptr() as _,
                name.len(),
                value,
                is_unsigned as _,
            ))
        }
    }

    pub fn create_enumeration_type(
        &self,
        scope: &DIScope,
        name: &[u8],
        file: &DIFile,
        line_number: u32,
        size_in_bits: u64,
        align_in_bits: u32,
        elements: &[&Metadata],
        class_ty: &Metadata,
    ) -> &'c DIType {
        unsafe {
            DIType::from_raw(LLVMDIBuilderCreateEnumerationType(
                self.as_raw(),
                scope.as_raw(),
                name.as_ptr() as _,
                name.len(),
                file.as_raw(),
                line_number,
                size_in_bits,
                align_in_bits,
                elements.as_ptr() as _,
                elements.len() as _,
                class_ty.as_raw(),
            ))
        }
    }

    pub fn create_union_type(
        &self,
        scope: &DIScope,
        name: &[u8],
        file: &DIFile,
        line_number: u32,
        size_in_bits: u64,
        align_in_bits: u32,
        flags: LLVMDIFlags,
        elements: &[&Metadata],
        run_time_lang: u32,
        unique_id: &[u32],
    ) -> &'c DIType {
        unsafe {
            DIType::from_raw(LLVMDIBuilderCreateUnionType(
                self.as_raw(),
                scope.as_raw(),
                name.as_ptr() as _,
                name.len(),
                file.as_raw(),
                line_number,
                size_in_bits,
                align_in_bits,
                flags,
                elements.as_ptr() as _,
                elements.len() as _,
                run_time_lang,
                unique_id.as_ptr() as _,
                unique_id.len(),
            ))
        }
    }

    pub fn create_array_type(
        &self,
        size: u64,
        align_in_bits: u32,
        ty: &DIType,
        subscripts: &[&Metadata],
    ) -> &'c DIType {
        unsafe {
            DIType::from_raw(LLVMDIBuilderCreateArrayType(
                self.as_raw(),
                size,
                align_in_bits,
                ty.as_raw(),
                subscripts.as_ptr() as _,
                subscripts.len() as _,
            ))
        }
    }

    pub fn create_vector_type(
        &self,
        size: u64,
        align_in_bits: u32,
        ty: &DIType,
        subscripts: &[&Metadata],
    ) -> &'c DIType {
        unsafe {
            DIType::from_raw(LLVMDIBuilderCreateVectorType(
                self.as_raw(),
                size,
                align_in_bits,
                ty.as_raw(),
                subscripts.as_ptr() as _,
                subscripts.len() as _,
            ))
        }
    }

    pub fn create_unspecified_type(&self, name: &[u8]) -> &'c DIType {
        unsafe {
            DIType::from_raw(LLVMDIBuilderCreateUnspecifiedType(
                self.as_raw(),
                name.as_ptr() as _,
                name.len(),
            ))
        }
    }

    pub fn create_basic_type(
        &self,
        name: &[u8],
        size_in_bits: u64,
        encoding: LLVMDWARFTypeEncoding,
        flags: LLVMDIFlags,
    ) -> &'c DIType {
        unsafe {
            DIType::from_raw(LLVMDIBuilderCreateBasicType(
                self.as_raw(),
                name.as_ptr() as _,
                name.len(),
                size_in_bits,
                encoding,
                flags,
            ))
        }
    }

    pub fn create_pointer_type(
        &self,
        pointee_ty: &DIType,
        size_in_bits: u64,
        align_in_bits: u32,
        address_space: u32,
        name: &[u8],
    ) -> &'c DIType {
        unsafe {
            DIType::from_raw(LLVMDIBuilderCreatePointerType(
                self.as_raw(),
                pointee_ty.as_raw(),
                size_in_bits,
                align_in_bits,
                address_space,
                name.as_ptr() as _,
                name.len(),
            ))
        }
    }

    pub fn create_struct_type(
        &self,
        scope: &DIScope,
        name: &[u8],
        file: &Metadata,
        line_number: u32,
        size_in_bits: u64,
        align_in_bits: u32,
        flags: LLVMDIFlags,
        derived_from: &DIType,
        elements: &[&Metadata],
        run_time_lang: u32,
        v_table_holder: &Metadata,
        unique_id: &[u8],
    ) -> &'c DIType {
        unsafe {
            DIType::from_raw(LLVMDIBuilderCreateStructType(
                self.as_raw(),
                scope.as_raw(),
                name.as_ptr() as _,
                name.len(),
                file.as_raw(),
                line_number,
                size_in_bits,
                align_in_bits,
                flags,
                derived_from.as_raw(),
                elements.as_ptr() as _,
                elements.len() as _,
                run_time_lang,
                v_table_holder.as_raw(),
                unique_id.as_ptr() as _,
                unique_id.len(),
            ))
        }
    }

    pub fn create_member_type(
        &self,
        scope: &DIScope,
        name: &[u8],
        file: &DIFile,
        line_no: u32,
        size_in_bits: u64,
        align_in_bits: u32,
        offset_in_bits: u64,
        flags: LLVMDIFlags,
        ty: &DIType,
    ) -> &'c DIType {
        unsafe {
            DIType::from_raw(LLVMDIBuilderCreateMemberType(
                self.as_raw(),
                scope.as_raw(),
                name.as_ptr() as _,
                name.len(),
                file.as_raw(),
                line_no,
                size_in_bits,
                align_in_bits,
                offset_in_bits,
                flags,
                ty.as_raw(),
            ))
        }
    }

    pub fn create_static_member_type<T: TypeTag>(
        &self,
        scope: &DIScope,
        name: &[u8],
        file: &DIFile,
        line_number: u32,
        ty: &DIType,
        flags: LLVMDIFlags,
        constant_val: &&Value<T>,
        align_in_bits: u32,
    ) -> &'c DIType {
        unsafe {
            DIType::from_raw(LLVMDIBuilderCreateStaticMemberType(
                self.as_raw(),
                scope.as_raw(),
                name.as_ptr() as _,
                name.len(),
                file.as_raw(),
                line_number,
                ty.as_raw(),
                flags,
                constant_val.as_raw(),
                align_in_bits,
            ))
        }
    }

    pub fn create_member_pointer_type(
        &self,
        pointee_type: &DIType,
        class_type: &DIType,
        size_in_bits: u64,
        align_in_bits: u32,
        flags: LLVMDIFlags,
    ) -> &'c DIType {
        unsafe {
            DIType::from_raw(LLVMDIBuilderCreateMemberPointerType(
                self.as_raw(),
                pointee_type.as_raw(),
                class_type.as_raw(),
                size_in_bits,
                align_in_bits,
                flags,
            ))
        }
    }

    pub fn create_obj_c_i_var(
        &self,
        name: &[u8],
        file: &DIFile,
        line_no: u32,
        size_in_bits: u64,
        align_in_bits: u32,
        offset_in_bits: u64,
        flags: LLVMDIFlags,
        ty: &DIType,
        property_node: &DIObjCProperty,
    ) -> &'c DIVariable {
        unsafe {
            DIVariable::from_raw(LLVMDIBuilderCreateObjCIVar(
                self.as_raw(),
                name.as_ptr() as _,
                name.len(),
                file.as_raw(),
                line_no,
                size_in_bits,
                align_in_bits,
                offset_in_bits,
                flags,
                ty.as_raw(),
                property_node.as_raw(),
            ))
        }
    }

    pub fn create_object_pointer_type(&self, ty: &DIType) -> &'c DIType {
        unsafe {
            DIType::from_raw(LLVMDIBuilderCreateObjectPointerType(
                self.as_raw(),
                ty.as_raw(),
            ))
        }
    }

    pub fn create_qualified_type(&self, tag: u32, ty: &DIType) -> &'c DIType {
        unsafe {
            DIType::from_raw(LLVMDIBuilderCreateQualifiedType(
                self.as_raw(),
                tag,
                ty.as_raw(),
            ))
        }
    }

    pub fn create_reference_type(&self, tag: u32, ty: &DIType) -> &'c DIType {
        unsafe {
            DIType::from_raw(LLVMDIBuilderCreateReferenceType(
                self.as_raw(),
                tag,
                ty.as_raw(),
            ))
        }
    }

    pub fn create_null_ptr_type(&self) -> &'c DIType {
        unsafe { DIType::from_raw(LLVMDIBuilderCreateNullPtrType(self.as_raw())) }
    }

    pub fn create_typedef(
        &self,
        ty: &DIType,
        name: &[u8],
        file: DIFile,
        line_no: u32,
        scope: &DIScope,
        align_in_bits: u32,
    ) -> &'c Metadata {
        unsafe {
            Metadata::from_raw(LLVMDIBuilderCreateTypedef(
                self.as_raw(),
                ty.as_raw(),
                name.as_ptr() as _,
                name.len(),
                file.as_raw(),
                line_no,
                scope.as_raw(),
                align_in_bits,
            ))
        }
    }

    pub fn create_inheritance(
        &self,
        ty: &DIType,
        base_ty: &DIType,
        base_offset: u64,
        v_b_ptr_offset: u32,
        flags: LLVMDIFlags,
    ) -> &'c Metadata {
        unsafe {
            Metadata::from_raw(LLVMDIBuilderCreateInheritance(
                self.as_raw(),
                ty.as_raw(),
                base_ty.as_raw(),
                base_offset,
                v_b_ptr_offset,
                flags,
            ))
        }
    }

    pub fn create_forward_decl(
        &self,
        tag: u32,
        name: &[u8],
        scope: &DIScope,
        file: &DIFile,
        line: u32,
        runtime_lang: u32,
        size_in_bits: u64,
        align_in_bits: u32,
        unique_identifier: &[u8],
    ) -> &'c Metadata {
        unsafe {
            Metadata::from_raw(LLVMDIBuilderCreateForwardDecl(
                self.as_raw(),
                tag,
                name.as_ptr() as _,
                name.len(),
                scope.as_raw(),
                file.as_raw(),
                line,
                runtime_lang,
                size_in_bits,
                align_in_bits,
                unique_identifier.as_ptr() as _,
                unique_identifier.len(),
            ))
        }
    }

    pub fn create_replaceable_composite_type(
        &self,
        tag: u32,
        name: &[u8],
        scope: &DIScope,
        file: &DIFile,
        line: u32,
        runtime_lang: u32,
        size_in_bits: u64,
        align_in_bits: u32,
        flags: LLVMDIFlags,
        unique_identifier: &[u8],
    ) -> &'c DIType {
        unsafe {
            DIType::from_raw(LLVMDIBuilderCreateReplaceableCompositeType(
                self.as_raw(),
                tag,
                name.as_ptr() as _,
                name.len(),
                scope.as_raw(),
                file.as_raw(),
                line,
                runtime_lang,
                size_in_bits,
                align_in_bits,
                flags,
                unique_identifier.as_ptr() as _,
                unique_identifier.len(),
            ))
        }
    }

    pub fn crete_bit_field_member_type(
        &self,
        scope: &DIScope,
        name: &[u8],
        file: &DIFile,
        line_number: u32,
        size_in_bits: u64,
        offset_in_bits: u64,
        storage_offset_in_bits: u64,
        flags: LLVMDIFlags,
        ty: &DIType,
    ) -> &'c DIType {
        unsafe {
            DIType::from_raw(LLVMDIBuilderCreateBitFieldMemberType(
                self.as_raw(),
                scope.as_raw(),
                name.as_ptr() as _,
                name.len(),
                file.as_raw(),
                line_number,
                size_in_bits,
                offset_in_bits,
                storage_offset_in_bits,
                flags,
                ty.as_raw(),
            ))
        }
    }

    pub fn create_class_type(
        &self,
        scope: &DIScope,
        name: &[u8],
        file: &DIFile,
        line_number: u32,
        size_in_bits: u64,
        align_in_bits: u32,
        offset_in_bits: u64,
        flags: LLVMDIFlags,
        derived_from: &DIType,
        elements: &[&Metadata],
        v_table_holder: &Metadata,
        template_params_node: &Metadata,
        unique_identifier: &[u8],
    ) -> &'c DIType {
        unsafe {
            DIType::from_raw(LLVMDIBuilderCreateClassType(
                self.as_raw(),
                scope.as_raw(),
                name.as_ptr() as _,
                name.len(),
                file.as_raw(),
                line_number,
                size_in_bits,
                align_in_bits,
                offset_in_bits,
                flags,
                derived_from.as_raw(),
                elements.as_ptr() as _,
                elements.len() as _,
                v_table_holder.as_raw(),
                template_params_node.as_raw(),
                unique_identifier.as_ptr() as _,
                unique_identifier.len(),
            ))
        }
    }

    pub fn create_artificial_type(&self, ty: &DIType) -> &'c DIType {
        unsafe {
            DIType::from_raw(LLVMDIBuilderCreateArtificialType(
                self.as_raw(),
                ty.as_raw(),
            ))
        }
    }
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

    pub fn get_offset_in_bits(&self) -> u64 {
        unsafe { LLVMDITypeGetOffsetInBits(self.as_raw()) }
    }

    pub fn get_align_in_bits(&self) -> u32 {
        unsafe { LLVMDITypeGetAlignInBits(self.as_raw()) }
    }

    pub fn get_line(&self) -> u32 {
        unsafe { LLVMDITypeGetLine(self.as_raw()) }
    }

    pub fn get_flags(&self) -> LLVMDIFlags {
        unsafe { LLVMDITypeGetFlags(self.as_raw()) }
    }
}

impl<'m, 'c> DIBuilder<'m, 'c> {
    pub fn get_or_create_subrange(&self, lower_bound: i64, count: i64) -> &'c DISubrange {
        unsafe {
            DISubrange::from_raw(LLVMDIBuilderGetOrCreateSubrange(
                self.as_raw(),
                lower_bound,
                count,
            ))
        }
    }

    pub fn get_or_create_array(&self, data: &[&Metadata]) -> &'c Metadata {
        unsafe {
            Metadata::from_raw(LLVMDIBuilderGetOrCreateArray(
                self.as_raw(),
                data.as_ptr() as _,
                data.len(),
            ))
        }
    }

    pub fn create_expression(&self, addr: &[u64]) -> &'c DIExpression {
        unsafe {
            DIExpression::from_raw(LLVMDIBuilderCreateExpression(
                self.as_raw(),
                addr.as_ptr() as _,
                addr.len(),
            ))
        }
    }

    pub fn create_constant_value_expression(&self, value: u64) -> &'c Metadata {
        unsafe {
            Metadata::from_raw(LLVMDIBuilderCreateConstantValueExpression(
                self.as_raw(),
                value,
            ))
        }
    }

    pub fn create_global_variable_expression(
        &self,
        scope: &DIScope,
        name: &[u8],
        linkage: &[u8],
        file: &DIFile,
        line_no: u32,
        ty: &DIType,
        local_to_unit: bool,
        expr: &Metadata,
        decl: &Metadata,
        align_in_bits: u32,
    ) -> &'c DIGlobalVariableExpression {
        unsafe {
            DIGlobalVariableExpression::from_raw(LLVMDIBuilderCreateGlobalVariableExpression(
                self.as_raw(),
                scope.as_raw(),
                name.as_ptr() as _,
                name.len(),
                linkage.as_ptr() as _,
                linkage.len(),
                file.as_raw(),
                line_no,
                ty.as_raw(),
                local_to_unit as _,
                expr.as_raw(),
                decl.as_raw(),
                align_in_bits,
            ))
        }
    }
}

impl DINode {
    pub fn get_tag(&self) -> u16 {
        unsafe { LLVMGetDINodeTag(self.as_raw()) }
    }
}

impl DIGlobalVariableExpression {
    pub fn get_variable(&self) -> &DIVariable {
        unsafe { DIVariable::from_raw(LLVMDIGlobalVariableExpressionGetVariable(self.as_raw())) }
    }

    pub fn get_expression(&self) -> &DIExpression {
        unsafe {
            DIExpression::from_raw(LLVMDIGlobalVariableExpressionGetExpression(self.as_raw()))
        }
    }
}

impl DIVariable {
    pub fn get_file(&self) -> &DIFile {
        unsafe { DIFile::from_raw(LLVMDIVariableGetFile(self.as_raw())) }
    }

    pub fn get_scope(&self) -> &DIScope {
        unsafe { DIScope::from_raw(LLVMDIVariableGetScope(self.as_raw())) }
    }

    pub fn get_line(&self) -> u32 {
        unsafe { LLVMDIVariableGetLine(self.as_raw()) }
    }
}

impl Context {
    pub fn temporary_md_node(&self, data: &[&Metadata]) -> &Metadata {
        unsafe {
            Metadata::from_raw(LLVMTemporaryMDNode(
                self.as_raw(),
                data.as_ptr() as _,
                data.len(),
            ))
        }
    }
}

impl MDNode {
    pub unsafe fn dispose_temporary_md_node(&self) {
        unsafe { LLVMDisposeTemporaryMDNode(self.as_raw()) }
    }
}

impl Metadata {
    pub unsafe fn replace_all_uses_with(&self, replacement: &Metadata) {
        unsafe { LLVMMetadataReplaceAllUsesWith(self.as_raw(), replacement.as_raw()) }
    }
}

impl<'m, 'c> DIBuilder<'m, 'c> {
    pub fn create_temp_global_variable_fwd_decl(
        &self,
        scope: &DIScope,
        name: &[u8],
        linkage: &[u8],
        file: &DIFile,
        line_no: u32,
        ty: &DIType,
        local_to_unit: bool,
        decl: &Metadata,
        align_in_bits: u32,
    ) -> &'c Metadata {
        unsafe {
            Metadata::from_raw(LLVMDIBuilderCreateTempGlobalVariableFwdDecl(
                self.as_raw(),
                scope.as_raw(),
                name.as_ptr() as _,
                name.len(),
                linkage.as_ptr() as _,
                linkage.len(),
                file.as_raw(),
                line_no,
                ty.as_raw(),
                local_to_unit as _,
                decl.as_raw(),
                align_in_bits,
            ))
        }
    }

    pub fn insert_declare_before<T: TypeTag>(
        &self,
        storage: &Value<T>,
        var_info: &Metadata,
        expr: &Metadata,
        debug_loc: &DILocation,
        instr: &Instruction<T>,
    ) -> &'c Value<any> {
        unsafe {
            Value::from_raw(LLVMDIBuilderInsertDeclareBefore(
                self.as_raw(),
                storage.as_raw(),
                var_info.as_raw(),
                expr.as_raw(),
                debug_loc.as_raw(),
                instr.as_raw(),
            ))
        }
    }

    pub fn insert_declare_at_end<T: TypeTag>(
        &self,
        storage: &Value<T>,
        var_info: &Metadata,
        expr: &Metadata,
        debug_loc: &DILocation,
        block: &BasicBlock,
    ) -> &'c Value<any> {
        unsafe {
            Value::from_raw(LLVMDIBuilderInsertDeclareAtEnd(
                self.as_raw(),
                storage.as_raw(),
                var_info.as_raw(),
                expr.as_raw(),
                debug_loc.as_raw(),
                block.as_raw(),
            ))
        }
    }

    pub fn insert_dbg_value_before<T: TypeTag>(
        &self,
        val: &Value<T>,
        val_info: &Metadata,
        expr: &Metadata,
        debug_loc: &DILocation,
        instr: &Instruction<T>,
    ) -> &'c Value<any> {
        unsafe {
            Value::from_raw(LLVMDIBuilderInsertDbgValueBefore(
                self.as_raw(),
                val.as_raw(),
                val_info.as_raw(),
                expr.as_raw(),
                debug_loc.as_raw(),
                instr.as_raw(),
            ))
        }
    }

    pub fn insert_dbg_value_at_end<T: TypeTag>(
        &self,
        val: &Value<T>,
        val_info: &Metadata,
        expr: &Metadata,
        debug_loc: &DILocation,
        block: &BasicBlock,
    ) -> &'c Value<any> {
        unsafe {
            Value::from_raw(LLVMDIBuilderInsertDbgValueAtEnd(
                self.as_raw(),
                val.as_raw(),
                val_info.as_raw(),
                expr.as_raw(),
                debug_loc.as_raw(),
                block.as_raw(),
            ))
        }
    }

    pub fn create_auto_variable(
        &self,
        scope: &DIScope,
        name: &[u8],
        file: &DIFile,
        line_no: u32,
        ty: &DIType,
        always_preserve: bool,
        flags: LLVMDIFlags,
        align_in_bits: u32,
    ) -> &'c Metadata {
        unsafe {
            Metadata::from_raw(LLVMDIBuilderCreateAutoVariable(
                self.as_raw(),
                scope.as_raw(),
                name.as_ptr() as _,
                name.len(),
                file.as_raw(),
                line_no,
                ty.as_raw(),
                always_preserve as _,
                flags,
                align_in_bits,
            ))
        }
    }

    pub fn create_parameter_variable(
        &self,
        scope: &DIScope,
        name: &[u8],
        arg_no: u32,
        file: &DIFile,
        line_no: u32,
        ty: &DIType,
        always_preserve: bool,
        flags: LLVMDIFlags,
    ) -> &'c Metadata {
        unsafe {
            Metadata::from_raw(LLVMDIBuilderCreateParameterVariable(
                self.as_raw(),
                scope.as_raw(),
                name.as_ptr() as _,
                name.len(),
                arg_no,
                file.as_raw(),
                line_no,
                ty.as_raw(),
                always_preserve as _,
                flags,
            ))
        }
    }
}

impl<T: FunTypeTag> Value<T> {
    pub fn get_subprogram(&self) -> &DISubprogram {
        unsafe { DISubprogram::from_raw(LLVMGetSubprogram(self.as_raw())) }
    }

    pub fn set_subprogram(&self, sp: &DISubprogram) {
        unsafe { LLVMSetSubprogram(self.as_raw(), sp.as_raw()) }
    }
}

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
