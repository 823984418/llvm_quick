use std::ffi::{CStr, CString};
use std::marker::PhantomData;

use llvm_sys::core::*;
use llvm_sys::*;

use crate::core::type_tag::{any, type_check_kind, TagTuple, TypeTag};
use crate::core::types::Type;
use crate::core::values::Value;
use crate::opaque::Opaque;

pub trait FunTypeTag: TypeTag {
    fn type_get_param_count(ty: &Type<Self>) -> u32;

    fn type_is_var(ty: &Type<Self>) -> bool;

    #[allow(clippy::needless_lifetimes)]
    fn type_get_param_with_slice<'s, F: FnOnce(&[&'s Type<any>]) -> R, R>(
        ty: &'s Type<Self>,
        f: F,
    ) -> R;

    fn value_get_param_count(val: &Value<Self>) -> u32;

    #[allow(clippy::needless_lifetimes)]
    fn value_get_param_with_slice<'s, F: FnOnce(&[&'s Value<any>]) -> R, R>(
        ty: &'s Value<Self>,
        f: F,
    ) -> R;
}

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct fun_any {}

impl TypeTag for fun_any {
    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        unsafe { type_check_kind(ty, LLVMTypeKind::LLVMFunctionTypeKind) }
    }
}

impl FunTypeTag for fun_any {
    fn type_get_param_count(ty: &Type<Self>) -> u32 {
        unsafe { LLVMCountParamTypes(ty.as_raw()) }
    }

    fn type_is_var(ty: &Type<Self>) -> bool {
        unsafe { LLVMIsFunctionVarArg(ty.as_raw()) != 0 }
    }

    #[allow(clippy::needless_lifetimes)]
    fn type_get_param_with_slice<'s, F: FnOnce(&[&'s Type<any>]) -> R, R>(
        ty: &'s Type<Self>,
        f: F,
    ) -> R {
        let count = ty.get_param_count() as usize;
        let mut buffer = vec![None; count];
        f(ty.get_param_into_slice(&mut buffer))
    }

    fn value_get_param_count(val: &Value<Self>) -> u32 {
        unsafe { LLVMCountParams(val.as_raw()) }
    }

    #[allow(clippy::needless_lifetimes)]
    fn value_get_param_with_slice<'s, F: FnOnce(&[&'s Value<any>]) -> R, R>(
        val: &'s Value<Self>,
        f: F,
    ) -> R {
        let count = val.get_param_count() as usize;
        let mut buffer = vec![None; count];
        f(val.get_param_into_slice(&mut buffer))
    }
}

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct fun<Args: TagTuple, Output: TypeTag, const VAR: bool = false> {
    marker: PhantomData<fn(Args) -> Output>,
}

impl<Args: TagTuple, Output: TypeTag, const VAR: bool> TypeTag for fun<Args, Output, VAR> {
    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        let ty = fun_any::type_cast(ty)?;
        if ty.is_var() != VAR {
            return None;
        }
        if ty.get_param_count() as usize != Args::COUNT {
            return None;
        }
        ty.get_return_any().try_cast::<Output>()?;
        ty.get_param_with_slice(|slice| Args::type_from_slice(slice))?;
        unsafe { Some(ty.cast_unchecked()) }
    }
}

impl<Args: TagTuple, Output: TypeTag, const VAR: bool> FunTypeTag for fun<Args, Output, VAR> {
    fn type_get_param_count(_ty: &Type<Self>) -> u32 {
        Args::COUNT as u32
    }

    fn type_is_var(_ty: &Type<Self>) -> bool {
        VAR
    }

    #[allow(clippy::needless_lifetimes)]
    fn type_get_param_with_slice<'s, F: FnOnce(&[&'s Type<any>]) -> R, R>(
        ty: &'s Type<Self>,
        f: F,
    ) -> R {
        Args::stack_array(|array| f(ty.get_param_into_slice(array)))
    }

    fn value_get_param_count(_val: &Value<Self>) -> u32 {
        Args::COUNT as u32
    }

    #[allow(clippy::needless_lifetimes)]
    fn value_get_param_with_slice<'s, F: FnOnce(&[&'s Value<any>]) -> R, R>(
        val: &'s Value<Self>,
        f: F,
    ) -> R {
        Args::stack_array(|array| f(val.get_param_into_slice(array)))
    }
}

impl<T: FunTypeTag> Type<T> {
    pub fn to_fun_any(&self) -> &Type<fun_any> {
        unsafe { self.cast_unchecked() }
    }

    /// Returns whether a function type is variadic.
    pub fn is_var(&self) -> bool {
        T::type_is_var(self)
    }

    /// Obtain the number of parameters this function accepts.
    pub fn get_param_count(&self) -> u32 {
        T::type_get_param_count(self)
    }

    /// Obtain the types of a function's parameters.
    #[allow(clippy::mut_from_ref)]
    pub fn get_param_into_slice<'s, 'a>(
        &'s self,
        slice: &'a mut [Option<&'s Type<any>>],
    ) -> &'a mut [&'s Type<any>] {
        assert_eq!(slice.len(), self.get_param_count() as usize);
        unsafe {
            LLVMGetParamTypes(self.as_raw(), slice.as_ptr() as _);
            std::mem::transmute(slice)
        }
    }

    /// Obtain the types of a function's parameters.
    #[allow(clippy::needless_lifetimes)]
    pub fn get_param_with_slice<'s, F: FnOnce(&[&'s Type<any>]) -> R, R>(&'s self, f: F) -> R {
        T::type_get_param_with_slice(self, f)
    }

    /// Obtain the types of a function's parameters.
    pub fn get_param_vec_any(&self) -> Vec<&Type<any>> {
        unsafe {
            let count = LLVMCountParamTypes(self.as_raw()) as usize;
            let mut buffer = Vec::with_capacity(count);
            LLVMGetParamTypes(self.as_raw(), buffer.as_ptr() as _);
            buffer.set_len(count);
            buffer
        }
    }

    /// Obtain the Type this function Type returns.
    pub fn get_return_any(&self) -> &Type<any> {
        unsafe { Type::from_ref(LLVMGetReturnType(self.as_raw())) }
    }
}

impl<Args: TagTuple, Output: TypeTag, const VAR: bool> Type<fun<Args, Output, VAR>> {
    /// Obtain the types of a function's parameters.
    #[allow(clippy::needless_lifetimes)]
    pub fn get_params<'s>(&'s self) -> Args::Types<'s> {
        self.get_param_with_slice(|slice| Args::type_from_slice(slice))
            .unwrap()
    }
}

impl<T: FunTypeTag> Value<T> {
    pub fn to_fun_any(&self) -> &Value<fun_any> {
        unsafe { self.cast_unchecked() }
    }

    /// Obtain the calling function of a function.
    pub fn get_call_conv(&self) -> u32 {
        unsafe { LLVMGetFunctionCallConv(self.as_raw()) }
    }

    /// Set the calling convention of a function.
    pub fn set_call_conv(&self, conv: u32) {
        unsafe { LLVMSetFunctionCallConv(self.as_raw(), conv) };
    }

    /// Obtain the name of the garbage collector to use during code generation.
    pub fn get_gc_raw(&self) -> *const CStr {
        unsafe {
            let ptr = LLVMGetGC(self.as_raw());
            if ptr.is_null() {
                std::ptr::slice_from_raw_parts(ptr, 0) as *const CStr
            } else {
                CStr::from_ptr(ptr)
            }
        }
    }

    /// Obtain the name of the garbage collector to use during code generation.
    pub fn get_gc(&self) -> Option<CString> {
        unsafe {
            let ptr = self.get_gc_raw();
            if ptr.is_null() {
                None
            } else {
                Some(CString::from(&*ptr))
            }
        }
    }

    /// Define the garbage collector to use during code generation.
    pub fn set_gc(&self, name: &CStr) {
        unsafe { LLVMSetGC(self.as_raw(), name.as_ptr()) }
    }

    /// Obtain the number of parameters in a function.
    pub fn get_param_count(&self) -> u32 {
        T::value_get_param_count(self)
    }

    /// Obtain the types of a function's parameters.
    #[allow(clippy::mut_from_ref)]
    pub fn get_param_into_slice<'s>(
        &'s self,
        slice: &mut [Option<&'s Value<any>>],
    ) -> &mut [&'s Value<any>] {
        assert_eq!(slice.len(), self.get_param_count() as usize);
        unsafe {
            LLVMGetParams(self.as_raw(), slice.as_ptr() as _);
            std::mem::transmute(slice)
        }
    }

    /// Obtain the types of a function's parameters.
    #[allow(clippy::needless_lifetimes)]
    pub fn get_param_with_slice<'s, F: FnOnce(&[&'s Value<any>]) -> R, R>(&'s self, f: F) -> R {
        T::value_get_param_with_slice(self, f)
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
}

impl<Args: TagTuple, Output: TypeTag, const VAR: bool> Value<fun<Args, Output, VAR>> {
    /// Obtain the parameters in a function.
    #[allow(clippy::needless_lifetimes)]
    pub fn get_params<'s>(&'s self) -> Args::Values<'s> {
        self.get_param_with_slice(|slice| Args::value_from_slice(slice))
            .unwrap()
    }
}
