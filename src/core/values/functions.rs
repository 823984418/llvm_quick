use std::ffi::CStr;
use std::mem::MaybeUninit;

use llvm_sys::core::*;

use crate::opaque::Opaque;
use crate::type_tag::*;
use crate::Value;

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
    pub fn get_gc_raw(&self) -> Option<&CStr> {
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
    pub fn set_gc(&self, name: &CStr) {
        unsafe { LLVMSetGC(self.as_raw(), name.as_ptr()) }
    }

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
