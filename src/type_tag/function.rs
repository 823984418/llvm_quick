use std::ffi::{CStr, CString};
use std::marker::PhantomData;

use crate::opaque::Opaque;
use llvm_sys::core::*;
use llvm_sys::{LLVMCallConv, LLVMTypeKind};

use crate::type_tag::{any, type_check_kind, TagTuple, TypeTag, TypeTuple, ValueTuple};
use crate::types::Type;
use crate::util::{c_string, llvm_call_conv_from_u32};
use crate::values::Value;

pub trait FunTypeTag: TypeTag {
    fn type_is_var(ty: &Type<Self>) -> bool;
}

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct fun_any {}

impl TypeTag for fun_any {
    fn type_kind(_ty: &Type<Self>) -> LLVMTypeKind {
        LLVMTypeKind::LLVMFunctionTypeKind
    }

    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        unsafe { type_check_kind(ty, LLVMTypeKind::LLVMFunctionTypeKind) }
    }
}

impl FunTypeTag for fun_any {
    fn type_is_var(ty: &Type<Self>) -> bool {
        unsafe { LLVMIsFunctionVarArg(ty.as_ptr()) != 0 }
    }
}

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct fun<Args: TagTuple, Output: TypeTag, const VAR: bool = false> {
    marker: PhantomData<fn(Args) -> Output>,
}

impl<Args: TagTuple, Output: TypeTag, const VAR: bool> TypeTag for fun<Args, Output, VAR> {
    fn type_kind(_ty: &Type<Self>) -> LLVMTypeKind {
        LLVMTypeKind::LLVMFunctionTypeKind
    }

    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        let ty = fun_any::type_cast(ty)?;
        if ty.is_var() != VAR {
            return None;
        }
        if ty.get_return_type_any().try_cast::<Output>().is_none() {
            return None;
        }
        let params = ty.get_param_type_vec_any();
        if Args::Types::from_slice_any(&params).is_none() {
            return None;
        }
        Some(unsafe { ty.cast_unchecked() })
    }
}

impl<Args: TagTuple, Output: TypeTag, const VAR: bool> FunTypeTag for fun<Args, Output, VAR> {
    fn type_is_var(_ty: &Type<Self>) -> bool {
        VAR
    }
}

impl<T: FunTypeTag> Type<T> {
    pub fn to_fun_any(&self) -> &Type<fun_any> {
        unsafe { self.cast_unchecked() }
    }

    pub fn is_var(&self) -> bool {
        T::type_is_var(self)
    }

    /// Obtain the types of a function's parameters.
    pub fn get_param_type_vec_any(&self) -> Vec<&Type<any>> {
        unsafe {
            let count = LLVMCountParamTypes(self.as_ptr()) as usize;
            let mut buffer = Vec::with_capacity(count);
            LLVMGetParamTypes(self.as_ptr(), buffer.as_ptr() as _);
            buffer.set_len(count);
            buffer
        }
    }

    /// Obtain the Type this function Type returns.
    pub fn get_return_type_any(&self) -> &Type<any> {
        unsafe { Type::from_ref(LLVMGetReturnType(self.as_ptr())) }
    }
}

impl<T: FunTypeTag> Value<T> {
    pub fn to_fun_any(&self) -> &Value<fun_any> {
        unsafe { self.cast_unchecked() }
    }

    /// Obtain the calling function of a function.
    pub fn get_call_conv(&self) -> LLVMCallConv {
        llvm_call_conv_from_u32(unsafe { LLVMGetFunctionCallConv(self.as_ptr()) })
    }

    /// Set the calling convention of a function.
    pub fn set_call_conv(&self, conv: LLVMCallConv) {
        unsafe { LLVMSetFunctionCallConv(self.as_ptr(), conv as _) };
    }
    /// Obtain the name of the garbage collector to use during code generation.
    pub fn get_gc(&self) -> Option<CString> {
        unsafe { c_string(LLVMGetGC(self.as_ptr())) }
    }

    /// Define the garbage collector to use during code generation.
    pub fn set_gc(&self, name: &CStr) {
        unsafe { LLVMSetGC(self.as_ptr(), name.as_ptr()) }
    }

    /// Obtain the types of a function's parameters.
    pub fn get_param_vec_any(&self) -> Vec<&Value<any>> {
        unsafe {
            let count = LLVMCountParams(self.as_ptr()) as usize;
            let mut buffer = Vec::with_capacity(count);
            LLVMGetParams(self.as_ptr(), buffer.as_ptr() as _);
            buffer.set_len(count);
            buffer
        }
    }
}

impl<Args: TagTuple, Output: TypeTag, const VAR: bool> Value<fun<Args, Output, VAR>> {
    #[allow(clippy::needless_lifetimes)]
    pub fn get_params<'s>(&'s self) -> Args::Values<'s> {
        let vec = self.get_param_vec_any();
        Args::Values::from_slice_any(&vec).unwrap()
    }
}
