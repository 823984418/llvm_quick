use std::ffi::CStr;
use std::mem::MaybeUninit;
use std::ptr::null_mut;

use llvm_sys::core::*;
use llvm_sys::LLVMAttributeIndex;

use crate::core::IntrinsicId;
use crate::opaque::Opaque;
use crate::type_tag::*;
use crate::{Attribute, Context, Module, Type, Value};

impl<T: FunTypeTag> Value<T> {
    pub fn to_fun_any(&self) -> &Value<fun_any> {
        unsafe { self.cast_unchecked() }
    }
}

impl<T: FunTypeTag> Value<T> {
    pub unsafe fn delete_function(&self) {
        unsafe { LLVMDeleteFunction(self.as_raw()) }
    }

    pub fn has_personality_fn(&self) -> bool {
        unsafe { LLVMHasPersonalityFn(self.as_raw()) != 0 }
    }

    pub fn get_personality_fn(&self) -> &Value<any> {
        unsafe { Value::from_ref(LLVMGetPersonalityFn(self.as_raw())) }
    }

    pub fn set_personality_fn<F: FunTypeTag>(&self, personality_fn: Option<&Value<F>>) {
        unsafe {
            LLVMSetPersonalityFn(
                self.as_raw(),
                personality_fn.map(Value::as_raw).unwrap_or(null_mut()),
            )
        }
    }
}

pub fn lookup_intrinsic_id(name: &[u8]) -> IntrinsicId {
    unsafe { IntrinsicId(LLVMLookupIntrinsicID(name.as_ptr() as _, name.len())) }
}

impl<T: FunTypeTag> Value<T> {
    pub fn get_intrinsic_id(&self) -> IntrinsicId {
        unsafe { IntrinsicId(LLVMGetIntrinsicID(self.as_raw())) }
    }
}

impl<'s> Module<'s> {
    pub fn get_intrinsic_declaration(
        &self,
        id: IntrinsicId,
        param_types: &[&Type<any>],
    ) -> &Value<any> {
        unsafe {
            Value::from_ref(LLVMGetIntrinsicDeclaration(
                self.as_raw(),
                id.0,
                param_types.as_ptr() as _,
                param_types.len(),
            ))
        }
    }
}

impl Context {
    pub fn intrinsic_get_type(&self, id: IntrinsicId, param_types: &[&Type<any>]) -> &Type<any> {
        unsafe {
            Type::from_ref(LLVMIntrinsicGetType(
                self.as_raw(),
                id.0,
                param_types.as_ptr() as _,
                param_types.len(),
            ))
        }
    }
}

impl IntrinsicId {
    pub fn get_name(self) -> &'static [u8] {
        unsafe {
            let mut len = 0;
            let ptr = LLVMIntrinsicGetName(self.0, &mut len);
            std::slice::from_raw_parts(ptr as _, len)
        }
    }
}

impl<'s> Module<'s> {
    pub fn intrinsic_copy_overloaded_name(
        &self,
        id: IntrinsicId,
        param_types: &[&Type<any>],
    ) -> &[u8] {
        unsafe {
            let mut len = 0;
            let ptr = LLVMIntrinsicCopyOverloadedName2(
                self.as_raw(),
                id.0,
                param_types.as_ptr() as _,
                param_types.len(),
                &mut len,
            );
            std::slice::from_raw_parts(ptr as _, len)
        }
    }
}

impl IntrinsicId {
    pub fn is_overloaded(self) -> bool {
        unsafe { LLVMIntrinsicIsOverloaded(self.0) != 0 }
    }
}

impl<T: FunTypeTag> Value<T> {
    /// Obtain the calling function of a function.
    pub fn get_function_call_conv(&self) -> u32 {
        unsafe { LLVMGetFunctionCallConv(self.as_raw()) }
    }

    /// Set the calling convention of a function.
    pub fn set_function_call_conv(&self, conv: u32) {
        unsafe { LLVMSetFunctionCallConv(self.as_raw(), conv) }
    }

    /// Obtain the name of the garbage collector to use during code generation.
    pub fn get_gc(&self) -> Option<&CStr> {
        unsafe {
            let ptr = LLVMGetGC(self.as_raw());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr))
            }
        }
    }

    /// Define the garbage collector to use during code generation.
    pub unsafe fn set_gc(&self, name: &CStr) {
        unsafe { LLVMSetGC(self.as_raw(), name.as_ptr()) }
    }

    pub fn add_attribute_at_index(&self, idx: LLVMAttributeIndex, a: &Attribute) {
        unsafe { LLVMAddAttributeAtIndex(self.as_raw(), idx, a.as_raw()) }
    }

    pub fn get_attribute_count_at_index(&self, idx: LLVMAttributeIndex) -> u32 {
        unsafe { LLVMGetAttributeCountAtIndex(self.as_raw(), idx) }
    }

    pub fn get_attribute_at_index<'a, 's>(
        &'s self,
        idx: LLVMAttributeIndex,
        slice: &'a mut [Option<&'s Attribute>],
    ) -> &'a mut [&'s Attribute] {
        assert_eq!(slice.len(), self.get_attribute_count_at_index(idx) as usize);
        unsafe {
            LLVMGetAttributesAtIndex(self.as_raw(), idx, slice.as_mut_ptr() as _);
            std::mem::transmute(slice)
        }
    }

    pub fn get_enum_attribute_at_index(&self, idx: LLVMAttributeIndex, kind_id: u32) -> &Attribute {
        unsafe { Attribute::from_ref(LLVMGetEnumAttributeAtIndex(self.as_raw(), idx, kind_id)) }
    }

    pub fn get_string_attribute_at_index(&self, idx: LLVMAttributeIndex, k: &[u8]) -> &Attribute {
        unsafe {
            Attribute::from_ref(LLVMGetStringAttributeAtIndex(
                self.as_raw(),
                idx,
                k.as_ptr() as _,
                k.len() as _,
            ))
        }
    }

    pub fn remove_enum_attribute_at_index(&self, idx: LLVMAttributeIndex, kind_id: u32) {
        unsafe { LLVMRemoveEnumAttributeAtIndex(self.as_raw(), idx, kind_id) }
    }

    pub fn remove_string_attribute_at_index(&self, idx: LLVMAttributeIndex, k: &[u8]) {
        unsafe {
            LLVMRemoveStringAttributeAtIndex(self.as_raw(), idx, k.as_ptr() as _, k.len() as _)
        }
    }

    pub fn add_target_dependent_function_attr(&self, a: &CStr, v: &CStr) {
        unsafe { LLVMAddTargetDependentFunctionAttr(self.as_raw(), a.as_ptr(), v.as_ptr()) }
    }
}

impl<T: FunTypeTag> Value<T> {
    /// Obtain the number of parameters in a function.
    pub fn get_param_count(&self) -> u32 {
        unsafe { LLVMCountParams(self.as_raw()) }
    }

    /// Obtain the types of a function's parameters.
    pub fn get_param_into_slice<'a, 's>(
        &'s self,
        slice: &'a mut [Option<&'s Value<any>>],
    ) -> &'a mut [&'s Value<any>] {
        assert_eq!(slice.len(), self.get_param_count() as usize);
        unsafe {
            LLVMGetParams(self.as_raw(), slice.as_mut_ptr() as _);
            std::mem::transmute(slice)
        }
    }

    /// Obtain the types of a function's parameters.
    pub fn get_param_vec_any(&self) -> Vec<&Value<any>> {
        unsafe {
            let count = self.get_param_count() as usize;
            let mut buffer = Vec::with_capacity(count);
            LLVMGetParams(self.as_raw(), buffer.as_ptr() as _);
            buffer.set_len(count);
            buffer
        }
    }

    pub fn get_param(&self, index: u32) -> Option<&Value<any>> {
        unsafe { Value::try_from_ref(LLVMGetParam(self.as_raw(), index)) }
    }
}

impl<T: TypeTag> Value<T> {
    pub fn get_param_parent(&self) -> &Value<fun_any> {
        unsafe { Value::from_ref(LLVMGetParamParent(self.as_raw())) }
    }
}

impl<T: FunTypeTag> Value<T> {
    pub fn get_first_param(&self) -> Option<&Value<any>> {
        unsafe { Value::try_from_ref(LLVMGetFirstParam(self.as_raw())) }
    }

    pub fn get_last_param(&self) -> Option<&Value<any>> {
        unsafe { Value::try_from_ref(LLVMGetLastParam(self.as_raw())) }
    }
}

impl<T: TypeTag> Value<T> {
    pub fn get_next_param(&self) -> Option<&Value<any>> {
        unsafe { Value::try_from_ref(LLVMGetNextParam(self.as_raw())) }
    }

    pub fn get_previous_param(&self) -> Option<&Value<any>> {
        unsafe { Value::try_from_ref(LLVMGetPreviousParam(self.as_raw())) }
    }

    pub fn set_param_alignment(&self, align: u32) {
        unsafe { LLVMSetParamAlignment(self.as_raw(), align) }
    }
}

impl<Args: TagTuple, Output: TypeTag, const VAR: bool> Value<fun<Args, Output, VAR>> {
    /// Obtain the parameters in a function.
    pub fn get_params(&self) -> Args::Values<'_> {
        unsafe {
            let mut array =
                MaybeUninit::<<Args::Values<'_> as Tuple>::Array<Option<&Value<any>>>>::zeroed()
                    .assume_init();
            self.get_param_into_slice(array.as_mut());
            Args::Values::from_array_any_unchecked(std::mem::transmute(array.as_ref()))
        }
    }
}
