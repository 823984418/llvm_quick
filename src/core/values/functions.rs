use std::ffi::{CStr, CString};

use llvm_sys::core::*;

use crate::opaque::Opaque;
use crate::type_tag::*;
use crate::{Type, Value};

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
        unsafe { LLVMSetFunctionCallConv(self.as_raw(), conv) }
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
