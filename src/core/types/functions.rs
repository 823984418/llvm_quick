use llvm_sys::core::{LLVMGetParamTypes, LLVMGetReturnType};

use crate::opaque::Opaque;
use crate::type_tag::{any, fun_any, FunTypeTag};
use crate::Type;

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
            let count = self.get_param_count() as usize;
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
