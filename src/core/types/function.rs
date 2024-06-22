use std::mem::MaybeUninit;

use llvm_sys::core::*;

use crate::opaque::Opaque;
use crate::type_tag::{any, fun, fun_any, FunTypeTag, TagTuple, Tuple, TypeTag, TypeTuple};
use crate::Type;

impl<T: FunTypeTag> Type<T> {
    pub fn to_fun_any(&self) -> &Type<fun_any> {
        unsafe { self.cast_unchecked() }
    }
}

impl<T: TypeTag> Type<T> {
    /// Obtain a function type consisting of a specified signature.
    ///
    /// The function is defined as a tuple of a return Type, a list of parameter types,
    /// and whether the function is variadic.
    pub fn fun_any<'c>(&'c self, args: &[&Type<any>], var: bool) -> &'c Type<fun_any> {
        unsafe {
            let ty = LLVMFunctionType(self.as_raw(), args.as_ptr() as _, args.len() as _, var as _);
            Type::from_raw(ty)
        }
    }

    /// Obtain a function type consisting of a specified signature.
    pub fn fun<'c, ArgTypeTuple: TypeTuple<'c>>(
        &'c self,
        args: ArgTypeTuple,
    ) -> &'c Type<fun<ArgTypeTuple::Tags, T>> {
        let fun = self.fun_any(args.to_array_any().as_ref(), false);
        unsafe { fun.cast_unchecked() }
    }

    /// Obtain a function type consisting of a specified signature.
    pub fn fun_var<'c, ArgTypeTuple: TypeTuple<'c>>(
        &'c self,
        args: ArgTypeTuple,
    ) -> &'c Type<fun<ArgTypeTuple::Tags, T, true>> {
        let fun = self.fun_any(args.to_array_any().as_ref(), true);
        unsafe { fun.cast_unchecked() }
    }
}

impl<T: FunTypeTag> Type<T> {
    /// Returns whether a function type is variadic.
    pub fn is_var(&self) -> bool {
        unsafe { LLVMIsFunctionVarArg(self.as_raw()) != 0 }
    }

    /// Obtain the Type this function Type returns.
    pub fn get_return_any(&self) -> &Type<any> {
        unsafe { Type::from_raw(LLVMGetReturnType(self.as_raw())) }
    }

    /// Obtain the number of parameters this function accepts.
    pub fn get_param_count(&self) -> u32 {
        unsafe { LLVMCountParamTypes(self.as_raw()) }
    }

    /// Obtain the types of a function's parameters.
    pub fn get_param_into_slice<'s, 'c>(
        &'c self,
        slice: &'s mut [Option<&'c Type<any>>],
    ) -> &'s mut [&'c Type<any>] {
        assert_eq!(slice.len(), self.get_param_count() as usize);
        unsafe {
            LLVMGetParamTypes(self.as_raw(), slice.as_mut_ptr() as _);
            std::mem::transmute(slice)
        }
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
}

impl<Args: TagTuple, Output: TypeTag, const VAR: bool> Type<fun<Args, Output, VAR>> {
    /// Obtain the Type this function Type returns.
    pub fn get_return(&self) -> &Type<Output> {
        unsafe { self.get_return_any().cast_unchecked() }
    }

    /// Obtain the types of a function's parameters.
    pub fn get_params(&self) -> Args::Types<'_> {
        unsafe {
            let mut array =
                MaybeUninit::<<Args::Types<'_> as Tuple>::Array<Option<&Type<any>>>>::zeroed()
                    .assume_init();
            self.get_param_into_slice(array.as_mut());
            Args::Types::from_array_any_unchecked(std::mem::transmute(array.as_ref()))
        }
    }
}
