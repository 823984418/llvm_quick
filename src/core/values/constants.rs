use std::ffi::CStr;
use std::ptr::null_mut;

use llvm_sys::core::*;
use llvm_sys::*;

use crate::opaque::Opaque;
use crate::type_tag::*;
use crate::{
    BasicBlock, Constant, Context, GlobalAlias, GlobalValue, Metadata, Module, Type, Value,
    ValueMetadataEntries,
};

impl<T: TypeTag> Type<T> {
    pub fn const_null(&self) -> &Constant<T> {
        unsafe { Constant::from_raw(LLVMConstNull(self.as_raw())) }
    }

    pub fn const_all_ones(&self) -> &Constant<T> {
        unsafe { Constant::from_raw(LLVMConstAllOnes(self.as_raw())) }
    }

    pub fn get_undef(&self) -> &Constant<T> {
        unsafe { Constant::from_raw(LLVMGetUndef(self.as_raw())) }
    }

    pub fn get_poison(&self) -> &Constant<T> {
        unsafe { Constant::from_raw(LLVMGetPoison(self.as_raw())) }
    }
}

impl<T: TypeTag> Value<T> {
    pub fn is_null(&self) -> bool {
        unsafe { LLVMIsNull(self.as_raw()) != 0 }
    }
}

impl<T: PtrTypeTag> Type<T> {
    pub fn const_pointer_null(&self) -> &Constant<T> {
        unsafe { Constant::from_raw(LLVMConstPointerNull(self.as_raw())) }
    }
}

impl<T: IntTypeTag> Type<T> {
    pub fn const_int(&self, n: u64, sign_extend: bool) -> &Constant<T> {
        unsafe { Constant::from_raw(LLVMConstInt(self.as_raw(), n, sign_extend as _)) }
    }

    pub fn const_int_of_arbitrary_precision(&self, words: &[u64]) -> &Constant<T> {
        unsafe {
            Constant::from_raw(LLVMConstIntOfArbitraryPrecision(
                self.as_raw(),
                words.len() as _,
                words.as_ptr(),
            ))
        }
    }

    pub fn const_int_of_string(&self, text: &CStr, radix: u8) -> &Constant<T> {
        unsafe { Constant::from_raw(LLVMConstIntOfString(self.as_raw(), text.as_ptr(), radix)) }
    }

    pub fn const_int_of_string_and_size(&self, text: &[u8], radix: u8) -> &Constant<T> {
        unsafe {
            Constant::from_raw(LLVMConstIntOfStringAndSize(
                self.as_raw(),
                text.as_ptr() as _,
                text.len() as _,
                radix,
            ))
        }
    }
}

impl<T: FloatTypeTag> Type<T> {
    pub fn const_real(&self, n: f64) -> &Constant<T> {
        unsafe { Constant::from_raw(LLVMConstReal(self.as_raw(), n)) }
    }

    pub fn const_real_of_string(&self, text: &CStr) -> &Constant<T> {
        unsafe { Constant::from_raw(LLVMConstRealOfString(self.as_raw(), text.as_ptr())) }
    }

    pub fn const_real_of_string_and_size(&self, text: &[u8]) -> &Constant<T> {
        unsafe {
            Constant::from_raw(LLVMConstRealOfStringAndSize(
                self.as_raw(),
                text.as_ptr() as _,
                text.len() as _,
            ))
        }
    }
}

impl<T: IntTypeTag> Constant<T> {
    pub fn get_z_ext_value(&self) -> u64 {
        unsafe { LLVMConstIntGetZExtValue(self.as_raw()) }
    }

    pub fn get_s_ext_value(&self) -> i64 {
        unsafe { LLVMConstIntGetSExtValue(self.as_raw()) }
    }
}

impl<T: FloatTypeTag> Constant<T> {
    pub fn get_double(&self) -> (f64, bool) {
        unsafe {
            let mut loses_info = 0;
            let v = LLVMConstRealGetDouble(self.as_raw(), &mut loses_info);
            (v, loses_info != 0)
        }
    }
}

impl Context {
    pub fn const_string(
        &self,
        s: &[u8],
        dont_null_terminate: bool,
    ) -> &Constant<array_any_len<int8>> {
        unsafe {
            Constant::from_raw(LLVMConstStringInContext(
                self.as_raw(),
                s.as_ptr() as _,
                s.len() as _,
                dont_null_terminate as _,
            ))
        }
    }
}

impl<T: TypeTag> Constant<T> {
    pub fn is_constant_string(&self) -> bool {
        unsafe { LLVMIsConstantString(self.as_raw()) != 0 }
    }

    pub fn get_as_string(&self) -> &[u8] {
        unsafe {
            let mut len = 0;
            let ptr = LLVMGetAsString(self.as_raw(), &mut len);
            std::slice::from_raw_parts(ptr as _, len)
        }
    }
}

impl Context {
    pub fn const_struct(
        &self,
        constant_vals: &[&Value<any>],
        packed: bool,
    ) -> &Constant<struct_any> {
        unsafe {
            Constant::from_raw(LLVMConstStructInContext(
                self.as_raw(),
                constant_vals.as_ptr() as _,
                constant_vals.len() as _,
                packed as _,
            ))
        }
    }
}

impl<T: TypeTag> Type<T> {
    pub fn const_array(&self, constant_vals: &[&Value<T>]) -> &Constant<array_any_len<T>> {
        unsafe {
            Constant::from_raw(LLVMConstArray2(
                self.as_raw(),
                constant_vals.as_ptr() as _,
                constant_vals.len() as _,
            ))
        }
    }
}

impl Type<struct_any> {
    pub fn const_named_struct(&self, constant_vals: &[&Value<any>]) -> &Constant<struct_any> {
        unsafe {
            Constant::from_raw(LLVMConstNamedStruct(
                self.as_raw(),
                constant_vals.as_ptr() as _,
                constant_vals.len() as _,
            ))
        }
    }
}

impl<T: TypeTag> Constant<T> {
    pub fn get_aggregate_element(&self, idx: u32) -> &Value<any> {
        unsafe { Value::from_raw(LLVMGetAggregateElement(self.as_raw(), idx)) }
    }
}

pub fn const_vector<'c, T: TypeTag>(
    scalar_constant_vals: &[&'c Value<T>],
) -> &'c Constant<vector_any_len<T>> {
    unsafe {
        Constant::from_raw(LLVMConstVector(
            scalar_constant_vals.as_ptr() as _,
            scalar_constant_vals.len() as _,
        ))
    }
}

impl<T: TypeTag> Constant<T> {
    pub fn get_const_opcode(&self) -> LLVMOpcode {
        unsafe { LLVMGetConstOpcode(self.as_raw()) }
    }
}

impl<T: TypeTag> Type<T> {
    pub fn align_of(&self) -> &Constant<int64> {
        unsafe { Constant::from_raw(LLVMAlignOf(self.as_raw())) }
    }

    pub fn size_of(&self) -> &Constant<int64> {
        unsafe { Constant::from_raw(LLVMSizeOf(self.as_raw())) }
    }
}

impl<T: IntMathTypeTag> Constant<T> {
    pub fn const_neg(&self) -> &Constant<T> {
        unsafe { Constant::from_raw(LLVMConstNeg(self.as_raw())) }
    }

    pub fn const_nsw_neg(&self) -> &Constant<T> {
        unsafe { Constant::from_raw(LLVMConstNSWNeg(self.as_raw())) }
    }

    pub fn const_nuw_neg(&self) -> &Constant<T> {
        unsafe { Constant::from_raw(LLVMConstNUWNeg(self.as_raw())) }
    }

    pub fn const_not(&self) -> &Constant<T> {
        unsafe { Constant::from_raw(LLVMConstNot(self.as_raw())) }
    }

    pub fn const_add(&self, rhs: &Self) -> &Constant<T> {
        unsafe { Constant::from_raw(LLVMConstAdd(self.as_raw(), rhs.as_raw())) }
    }

    pub fn const_nsw_add(&self, rhs: &Self) -> &Constant<T> {
        unsafe { Constant::from_raw(LLVMConstNSWAdd(self.as_raw(), rhs.as_raw())) }
    }

    pub fn const_nuw_add(&self, rhs: &Self) -> &Constant<T> {
        unsafe { Constant::from_raw(LLVMConstNUWAdd(self.as_raw(), rhs.as_raw())) }
    }

    pub fn const_sub(&self, rhs: &Self) -> &Constant<T> {
        unsafe { Constant::from_raw(LLVMConstSub(self.as_raw(), rhs.as_raw())) }
    }

    pub fn const_nsw_sub(&self, rhs: &Self) -> &Constant<T> {
        unsafe { Constant::from_raw(LLVMConstNSWSub(self.as_raw(), rhs.as_raw())) }
    }

    pub fn const_nuw_sub(&self, rhs: &Self) -> &Constant<T> {
        unsafe { Constant::from_raw(LLVMConstNUWSub(self.as_raw(), rhs.as_raw())) }
    }

    pub fn const_mul(&self, rhs: &Self) -> &Constant<T> {
        unsafe { Constant::from_raw(LLVMConstMul(self.as_raw(), rhs.as_raw())) }
    }

    pub fn const_nsw_mul(&self, rhs: &Self) -> &Constant<T> {
        unsafe { Constant::from_raw(LLVMConstNSWMul(self.as_raw(), rhs.as_raw())) }
    }

    pub fn const_nuw_mul(&self, rhs: &Self) -> &Constant<T> {
        unsafe { Constant::from_raw(LLVMConstNUWMul(self.as_raw(), rhs.as_raw())) }
    }

    pub fn const_xor(&self, rhs: &Self) -> &Constant<T> {
        unsafe { Constant::from_raw(LLVMConstXor(self.as_raw(), rhs.as_raw())) }
    }
}

pub fn const_i_cmp<'c, T: IntMathTypeTag>(
    predicate: LLVMIntPredicate,
    lhs: &'c Value<T>,
    rhs: &Value<T>,
) -> &'c Constant<T> {
    unsafe { Constant::from_raw(LLVMConstICmp(predicate, lhs.as_raw(), rhs.as_raw())) }
}

pub fn const_f_cmp<'c, T: FloatMathTypeTag>(
    predicate: LLVMRealPredicate,
    lhs: &'c Value<T>,
    rhs: &Value<T>,
) -> &'c Constant<T> {
    unsafe { Constant::from_raw(LLVMConstFCmp(predicate, lhs.as_raw(), rhs.as_raw())) }
}

impl<T: IntMathTypeTag> Constant<T> {
    pub fn const_shl(&self, rhs: &Self) -> &Constant<T> {
        unsafe { Constant::from_raw(LLVMConstShl(self.as_raw(), rhs.as_raw())) }
    }
}

impl<T: ElementTypeTag> Type<T> {
    pub fn const_gep(
        &self,
        constant_val: &Value<ptr_any>,
        constant_indices: &[&Constant<int_any>],
    ) -> &Constant<any> {
        unsafe {
            Constant::from_raw(LLVMConstGEP2(
                self.as_raw(),
                constant_val.as_raw(),
                constant_indices.as_ptr() as _,
                constant_indices.len() as _,
            ))
        }
    }

    pub fn const_gep_in_bounds(
        &self,
        constant_val: &Value<ptr_any>,
        constant_indices: &[&Constant<int_any>],
    ) -> &Constant<any> {
        unsafe {
            Constant::from_raw(LLVMConstInBoundsGEP2(
                self.as_raw(),
                constant_val.as_raw(),
                constant_indices.as_ptr() as _,
                constant_indices.len() as _,
            ))
        }
    }
}

impl<T: TypeTag> Constant<T> {
    pub fn const_trunc<D: TypeTag>(&self, to_type: &Type<D>) -> &Constant<D> {
        unsafe { Constant::from_raw(LLVMConstTrunc(self.as_raw(), to_type.as_raw())) }
    }

    pub fn const_ptr_to_int<D: TypeTag>(&self, to_type: &Type<D>) -> &Constant<D> {
        unsafe { Constant::from_raw(LLVMConstPtrToInt(self.as_raw(), to_type.as_raw())) }
    }

    pub fn const_int_to_ptr<D: TypeTag>(&self, to_type: &Type<D>) -> &Constant<D> {
        unsafe { Constant::from_raw(LLVMConstIntToPtr(self.as_raw(), to_type.as_raw())) }
    }

    pub fn const_bit_cast<D: TypeTag>(&self, to_type: &Type<D>) -> &Constant<D> {
        unsafe { Constant::from_raw(LLVMConstBitCast(self.as_raw(), to_type.as_raw())) }
    }

    pub fn const_addr_space_cast<D: TypeTag>(&self, to_type: &Type<D>) -> &Constant<D> {
        unsafe { Constant::from_raw(LLVMConstAddrSpaceCast(self.as_raw(), to_type.as_raw())) }
    }

    pub fn const_trunc_or_bit_cast<D: TypeTag>(&self, to_type: &Type<D>) -> &Constant<D> {
        unsafe { Constant::from_raw(LLVMConstTruncOrBitCast(self.as_raw(), to_type.as_raw())) }
    }

    pub fn const_pointer_cast<D: TypeTag>(&self, to_type: &Type<D>) -> &Constant<D> {
        unsafe { Constant::from_raw(LLVMConstPointerCast(self.as_raw(), to_type.as_raw())) }
    }

    pub fn const_extract_element<I: IntTypeTag>(
        &self,
        index_constant: &Constant<I>,
    ) -> &Constant<any> {
        unsafe {
            Constant::from_raw(LLVMConstExtractElement(
                self.as_raw(),
                index_constant.as_raw(),
            ))
        }
    }

    pub fn const_insert_element<E: TypeTag, I: IntTypeTag>(
        &self,
        element_value_constant: &Constant<E>,
        index_constant: &Constant<I>,
    ) -> &Constant<any> {
        unsafe {
            Constant::from_raw(LLVMConstInsertElement(
                self.as_raw(),
                element_value_constant.as_raw(),
                index_constant.as_raw(),
            ))
        }
    }

    pub fn const_shuffle_vector(
        &self,
        vector_b_constant: &Self,
        mask_constant: &Self,
    ) -> &Constant<any>
    where
        T: VectorTypeTag,
    {
        unsafe {
            Constant::from_raw(LLVMConstShuffleVector(
                self.as_raw(),
                vector_b_constant.as_raw(),
                mask_constant.as_raw(),
            ))
        }
    }

    pub fn block_address(&self, bb: &BasicBlock) -> &Constant<any>
    where
        T: FunTypeTag,
    {
        unsafe { Constant::from_raw(LLVMBlockAddress(self.as_raw(), bb.as_raw())) }
    }
}

impl<T: TypeTag> GlobalValue<T> {
    pub fn get_global_parent(&self) -> &Module {
        unsafe { Module::from_raw(LLVMGetGlobalParent(self.as_raw())) }
    }

    pub fn is_declaration(&self) -> bool {
        unsafe { LLVMIsDeclaration(self.as_raw()) != 0 }
    }

    pub fn get_linkage(&self) -> LLVMLinkage {
        unsafe { LLVMGetLinkage(self.as_raw()) }
    }

    pub fn set_linkage(&self, linkage: LLVMLinkage) {
        unsafe { LLVMSetLinkage(self.as_raw(), linkage) }
    }

    pub fn get_section(&self) -> &CStr {
        unsafe { CStr::from_ptr(LLVMGetSection(self.as_raw())) }
    }

    pub fn set_section(&self, section: &CStr) {
        unsafe { LLVMSetSection(self.as_raw(), section.as_ptr()) }
    }

    pub fn get_visibility(&self) -> LLVMVisibility {
        unsafe { LLVMGetVisibility(self.as_raw()) }
    }

    pub fn set_visibility(&self, viz: LLVMVisibility) {
        unsafe { LLVMSetVisibility(self.as_raw(), viz) }
    }

    pub fn get_dll_storage_class(&self) -> LLVMDLLStorageClass {
        unsafe { LLVMGetDLLStorageClass(self.as_raw()) }
    }

    pub fn set_dll_storage_class(&self, class: LLVMDLLStorageClass) {
        unsafe { LLVMSetDLLStorageClass(self.as_raw(), class) }
    }

    pub fn get_unnamed_address(&self) -> LLVMUnnamedAddr {
        unsafe { LLVMGetUnnamedAddress(self.as_raw()) }
    }

    pub fn set_unnamed_address(&self, unnamed_addr: LLVMUnnamedAddr) {
        unsafe { LLVMSetUnnamedAddress(self.as_raw(), unnamed_addr) }
    }

    pub fn get_value_type(&self) -> &Type<T> {
        unsafe { Type::from_raw(LLVMGlobalGetValueType(self.as_raw())) }
    }
}

impl<T: TypeTag> Value<T> {
    pub fn get_alignment(&self) -> u32 {
        unsafe { LLVMGetAlignment(self.as_raw()) }
    }

    pub fn set_alignment(&self, bytes: u32) {
        unsafe { LLVMSetAlignment(self.as_raw(), bytes) }
    }
}

impl<T: TypeTag> GlobalValue<T> {
    pub fn set_metadata(&self, kind: u32, md: &Metadata) {
        unsafe { LLVMGlobalSetMetadata(self.as_raw(), kind, md.as_raw()) }
    }

    pub fn erase_metadata(&self, kind: u32) {
        unsafe { LLVMGlobalEraseMetadata(self.as_raw(), kind) }
    }

    pub fn clear_metadata(&self) {
        unsafe { LLVMGlobalClearMetadata(self.as_raw()) }
    }

    pub fn copy_all_metadata(&self) -> ValueMetadataEntries {
        unsafe {
            let mut len = 0;
            let ptr = LLVMGlobalCopyAllMetadata(self.as_raw(), &mut len);
            ValueMetadataEntries::from_raw(ptr, len)
        }
    }
}

impl<'m> Drop for ValueMetadataEntries<'m> {
    fn drop(&mut self) {
        unsafe { LLVMDisposeValueMetadataEntries(self.as_raw()) }
    }
}

impl<'m> ValueMetadataEntries<'m> {
    pub fn get_kind(&self, index: u32) -> u32 {
        assert!((index as usize) < self.len);
        unsafe { LLVMValueMetadataEntriesGetKind(self.as_raw(), index) }
    }

    pub fn get_metadata(&self, index: u32) -> &'m Metadata {
        assert!((index as usize) < self.len);
        unsafe { Metadata::from_raw(LLVMValueMetadataEntriesGetMetadata(self.as_raw(), index)) }
    }
}

impl<'c> Module<'c> {
    pub fn add_global<T: TypeTag>(&self, ty: &Type<T>, name: &CStr) -> &GlobalValue<T> {
        unsafe { GlobalValue::from_raw(LLVMAddGlobal(self.as_raw(), ty.as_raw(), name.as_ptr())) }
    }

    pub fn add_global_in_address_space<T: TypeTag>(
        &self,
        ty: &Type<T>,
        name: &CStr,
        address_space: u32,
    ) -> &GlobalValue<T> {
        unsafe {
            GlobalValue::from_raw(LLVMAddGlobalInAddressSpace(
                self.as_raw(),
                ty.as_raw(),
                name.as_ptr(),
                address_space,
            ))
        }
    }

    pub fn get_named_global(&self, name: &CStr) -> &GlobalValue<any> {
        unsafe { GlobalValue::from_raw(LLVMGetNamedGlobal(self.as_raw(), name.as_ptr())) }
    }

    pub fn get_first_global(&self) -> &GlobalValue<any> {
        unsafe { GlobalValue::from_raw(LLVMGetFirstGlobal(self.as_raw())) }
    }

    pub fn get_last_global(&self) -> &GlobalValue<any> {
        unsafe { GlobalValue::from_raw(LLVMGetLastGlobal(self.as_raw())) }
    }
}

impl<T: TypeTag> GlobalValue<T> {
    pub fn get_next_global(&self) -> &GlobalValue<any> {
        unsafe { GlobalValue::from_raw(LLVMGetNextGlobal(self.as_raw())) }
    }

    pub fn get_previous_global(&self) -> &GlobalValue<any> {
        unsafe { GlobalValue::from_raw(LLVMGetPreviousGlobal(self.as_raw())) }
    }

    pub unsafe fn delete_global(&self) {
        unsafe { LLVMDeleteGlobal(self.as_raw()) }
    }

    pub fn get_initializer(&self) -> Option<&Constant<T>> {
        unsafe { Constant::from_ptr(LLVMGetInitializer(self.as_raw())) }
    }

    pub fn set_initializer(&self, constant_val: Option<&Constant<T>>) {
        unsafe {
            LLVMSetInitializer(
                self.as_raw(),
                constant_val.map(Constant::as_raw).unwrap_or(null_mut()),
            )
        }
    }

    pub fn is_thread_local(&self) -> bool {
        unsafe { LLVMIsThreadLocal(self.as_raw()) != 0 }
    }

    pub fn set_thread_local(&self, is_thread_local: bool) {
        unsafe { LLVMSetThreadLocal(self.as_raw(), is_thread_local as _) }
    }

    pub fn is_global_constant(&self) -> bool {
        unsafe { LLVMIsGlobalConstant(self.as_raw()) != 0 }
    }

    pub fn set_global_constant(&self, is_constant: bool) {
        unsafe { LLVMSetGlobalConstant(self.as_raw(), is_constant as _) }
    }

    pub fn get_thread_local_mode(&self) -> LLVMThreadLocalMode {
        unsafe { LLVMGetThreadLocalMode(self.as_raw()) }
    }

    pub fn set_thread_local_mode(&self, mode: LLVMThreadLocalMode) {
        unsafe { LLVMSetThreadLocalMode(self.as_raw(), mode) }
    }

    pub fn is_externally_initialized(&self) -> bool {
        unsafe { LLVMIsExternallyInitialized(self.as_raw()) != 0 }
    }

    pub fn set_externally_initialized(&self, is_ext_init: bool) {
        unsafe { LLVMSetExternallyInitialized(self.as_raw(), is_ext_init as _) }
    }
}

impl<'c> Module<'c> {
    pub fn get_named_global_alias(&self, name: &[u8]) -> &GlobalAlias<any> {
        unsafe {
            GlobalAlias::from_raw(LLVMGetNamedGlobalAlias(
                self.as_raw(),
                name.as_ptr() as _,
                name.len(),
            ))
        }
    }

    pub fn get_first_global_alias(&self) -> &GlobalAlias<any> {
        unsafe { GlobalAlias::from_raw(LLVMGetFirstGlobalAlias(self.as_raw())) }
    }

    pub fn get_last_global_alias(&self) -> &GlobalAlias<any> {
        unsafe { GlobalAlias::from_raw(LLVMGetLastGlobalAlias(self.as_raw())) }
    }
}

impl<T: TypeTag> GlobalAlias<T> {
    pub fn get_next_global_alias(&self) -> &GlobalAlias<any> {
        unsafe { GlobalAlias::from_raw(LLVMGetNextGlobalAlias(self.as_raw())) }
    }

    pub fn get_previous_global_alias(&self) -> &GlobalAlias<any> {
        unsafe { GlobalAlias::from_raw(LLVMGetPreviousGlobalAlias(self.as_raw())) }
    }

    pub fn get_aliasee(&self) -> &Constant<T> {
        unsafe { Constant::from_raw(LLVMAliasGetAliasee(self.as_raw())) }
    }

    pub fn set_aliasee(&self, aliasee: &Constant<T>) {
        unsafe { LLVMAliasSetAliasee(self.as_raw(), aliasee.as_raw()) }
    }
}

impl<'c> Module<'c> {
    pub fn add_alias<T: TypeTag>(
        &self,
        value_type: &Type<T>,
        addr_space: u32,
        aliasee: &Value<T>,
        name: &CStr,
    ) -> &Value<T> {
        unsafe {
            Value::from_raw(LLVMAddAlias2(
                self.as_raw(),
                value_type.as_raw(),
                addr_space,
                aliasee.as_raw(),
                name.as_ptr(),
            ))
        }
    }
}
