use std::fmt::{Debug, Formatter};

use llvm_sys::core::*;
use llvm_sys::*;

use crate::core::Message;
use crate::type_tag::*;
use crate::{Context, Opaque, Type};

pub mod arrays;
pub mod floats;
pub mod functions;
pub mod integers;
pub mod others;
pub mod pointers;

impl<T: TypeTag> Debug for Type<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.print_to_string().to_str().unwrap())
    }
}

impl<T: TypeTag> Type<T> {
    pub unsafe fn cast_unchecked<N: TypeTag>(&self) -> &Type<N> {
        unsafe { Type::from_ref(self.as_raw()) }
    }

    pub fn try_cast<N: TypeTag>(&self) -> Option<&Type<N>> {
        N::type_cast(self.to_any())
    }

    pub fn cast<N: TypeTag>(&self) -> &Type<N> {
        self.try_cast().unwrap()
    }

    pub fn to_any(&self) -> &Type<any> {
        unsafe { self.cast_unchecked() }
    }

    /// Obtain the enumerated type of a Type instance.
    pub fn get_kind(&self) -> LLVMTypeKind {
        unsafe { LLVMGetTypeKind(self.as_raw()) }
    }

    pub fn is_size(&self) -> bool {
        unsafe { LLVMTypeIsSized(self.as_raw()) != 0 }
    }

    /// Obtain the context to which this type instance is associated.
    pub fn get_context(&self) -> &Context {
        unsafe { Context::from_ref(LLVMGetTypeContext(self.as_raw())) }
    }

    pub fn dump(&self) {
        unsafe { LLVMDumpType(self.as_raw()) }
    }

    /// Return a string representation of the type.
    pub fn print_to_string(&self) -> Message {
        unsafe { Message::from_raw(LLVMPrintTypeToString(self.as_raw())) }
    }

    /// Obtain a function type consisting of a specified signature.
    ///
    /// The function is defined as a tuple of a return Type, a list of parameter types,
    /// and whether the function is variadic.
    pub fn fun_any<'s>(&'s self, args: &[&'s Type<any>], var: bool) -> &'s Type<fun_any> {
        unsafe {
            let ty = LLVMFunctionType(self.as_raw(), args.as_ptr() as _, args.len() as _, var as _);
            Type::from_ref(ty)
        }
    }

    /// Obtain a function type consisting of a specified signature.
    pub fn fun<'s, ArgTypeTuple: TypeTuple<'s>>(
        &'s self,
        args: ArgTypeTuple,
    ) -> &'s Type<fun<ArgTypeTuple::Tags, T>> {
        let fun = self.fun_any(args.to_array_any().as_ref(), false);
        unsafe { fun.cast_unchecked() }
    }

    /// Obtain a function type consisting of a specified signature.
    pub fn fun_var<'s, ArgTypeTuple: TypeTuple<'s>>(
        &'s self,
        args: ArgTypeTuple,
    ) -> &'s Type<fun<ArgTypeTuple::Tags, T, true>> {
        let fun = self.fun_any(args.to_array_any().as_ref(), true);
        unsafe { fun.cast_unchecked() }
    }
}

// TODO
