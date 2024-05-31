use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::mem::MaybeUninit;

use llvm_sys::core::*;
use llvm_sys::prelude::{LLVMTypeRef, LLVMValueRef};
use llvm_sys::LLVMTypeKind;

use crate::opaque::Opaque;
use crate::type_tag::{any, type_check_kind, Array, TagTuple, TypeTag, TypeTuple, ValueTuple};
use crate::types::Type;
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

    /// Returns whether a function type is variadic.
    pub fn is_var(&self) -> bool {
        T::type_is_var(self)
    }

    /// Obtain the number of parameters this function accepts.
    pub fn get_param_count(&self) -> u32 {
        unsafe { LLVMCountParamTypes(self.as_ptr()) }
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

impl<Args: TagTuple, Output: TypeTag, const VAR: bool> Type<fun<Args, Output, VAR>> {
    /// Obtain the types of a function's parameters.
    #[allow(clippy::needless_lifetimes)]
    pub fn get_params<'s>(&'s self) -> Args::Types<'s> {
        let array = MaybeUninit::<<Args::Types<'s> as TypeTuple>::AnyArray>::uninit();
        let count = self.get_param_count() as usize;
        assert_eq!(<Args::Types<'s> as TypeTuple>::AnyArray::LENGTH, count);
        unsafe {
            LLVMGetParamTypes(self.as_ptr(), array.as_ptr() as *mut LLVMTypeRef);
            Args::Types::<'s>::from_array_any(array.assume_init())
        }
    }
}

impl<T: FunTypeTag> Value<T> {
    pub fn to_fun_any(&self) -> &Value<fun_any> {
        unsafe { self.cast_unchecked() }
    }

    /// Obtain the calling function of a function.
    pub fn get_call_conv(&self) -> u32 {
        unsafe { LLVMGetFunctionCallConv(self.as_ptr()) }
    }

    /// Set the calling convention of a function.
    pub fn set_call_conv(&self, conv: u32) {
        unsafe { LLVMSetFunctionCallConv(self.as_ptr(), conv) };
    }

    /// Obtain the name of the garbage collector to use during code generation.
    pub fn get_gc_raw(&self) -> *const CStr {
        unsafe {
            let ptr = LLVMGetGC(self.as_ptr());
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
        unsafe { LLVMSetGC(self.as_ptr(), name.as_ptr()) }
    }

    /// Obtain the number of parameters in a function.
    pub fn get_param_count(&self) -> u32 {
        unsafe { LLVMCountParams(self.as_ptr()) }
    }

    /// Obtain the types of a function's parameters.
    pub fn get_param_vec_any(&self) -> Vec<&Value<any>> {
        unsafe {
            let count = self.get_param_count() as usize;
            let mut buffer = Vec::with_capacity(count);
            LLVMGetParams(self.as_ptr(), buffer.as_ptr() as _);
            buffer.set_len(count);
            buffer
        }
    }
}

impl<Args: TagTuple, Output: TypeTag, const VAR: bool> Value<fun<Args, Output, VAR>> {
    /// Obtain the parameters in a function.
    #[allow(clippy::needless_lifetimes)]
    pub fn get_params<'s>(&'s self) -> Args::Values<'s> {
        let array = MaybeUninit::<<Args::Values<'s> as ValueTuple>::AnyArray>::uninit();
        let count = self.get_param_count() as usize;
        assert_eq!(<Args::Values<'s> as ValueTuple>::AnyArray::LENGTH, count);
        unsafe {
            LLVMGetParams(self.as_ptr(), array.as_ptr() as *mut LLVMValueRef);
            Args::Values::<'s>::from_array_any(array.assume_init())
        }
    }
}
