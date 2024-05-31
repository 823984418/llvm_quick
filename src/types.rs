use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;

use llvm_sys::core::{LLVMArrayType2, LLVMFunctionType, LLVMGetTypeContext, LLVMPrintTypeToString};
use llvm_sys::{LLVMType, LLVMTypeKind};

use crate::context::Context;
use crate::message::Message;
use crate::opaque::{Opaque, PhantomOpaque};
use crate::type_tag::array::{array, array_sized};
use crate::type_tag::function::{fun, fun_any};
use crate::type_tag::{any, TagTuple, TypeTag, TypeTuple};

#[repr(transparent)]
pub struct Type<T: TypeTag> {
    opaque: PhantomOpaque,
    marker: PhantomData<fn(T) -> T>,
}

unsafe impl<T: TypeTag> Opaque for Type<T> {
    type Inner = LLVMType;
}

impl<T: TypeTag> Debug for Type<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        T::type_debug_fmt(self, f)
    }
}

impl<T: TypeTag> Type<T> {
    pub unsafe fn cast_unchecked<N: TypeTag>(&self) -> &Type<N> {
        unsafe { Type::from_ref(self.as_ptr()) }
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
    pub fn kind(&self) -> LLVMTypeKind {
        T::type_kind(self)
    }

    /// Return a string representation of the type.
    pub fn print_to_string(&self) -> Message {
        unsafe { Message::from_raw(LLVMPrintTypeToString(self.as_ptr())) }
    }

    /// Obtain the context to which this type instance is associated.
    pub fn context(&self) -> &Context {
        unsafe { Context::from_ref(LLVMGetTypeContext(self.as_ptr())) }
    }

    /// Obtain a function type consisting of a specified signature.
    ///
    /// The function is defined as a tuple of a return Type, a list of parameter types,
    /// and whether the function is variadic.
    pub fn fun_any<'s>(&'s self, args: &[&'s Type<any>], var: bool) -> &'s Type<fun_any> {
        unsafe {
            let ty = LLVMFunctionType(self.as_ptr(), args.as_ptr() as _, args.len() as _, var as _);
            Type::from_ref(ty)
        }
    }

    /// Obtain a function type consisting of a specified signature.
    pub fn fun<'s, ArgTypeTuple: TypeTuple<'s>>(
        &'s self,
        args: ArgTypeTuple,
    ) -> &'s Type<fun<ArgTypeTuple::Tags, T>> {
        ArgTypeTuple::Tags::type_with_slice(args, |slice| unsafe {
            self.fun_any(slice, false).cast_unchecked()
        })
    }

    /// Obtain a function type consisting of a specified signature.
    pub fn fun_var<'s, ArgTypeTuple: TypeTuple<'s>>(
        &'s self,
        args: ArgTypeTuple,
    ) -> &'s Type<fun<ArgTypeTuple::Tags, T, true>> {
        ArgTypeTuple::Tags::type_with_slice(args, |slice| unsafe {
            self.fun_any(slice, true).cast_unchecked()
        })
    }

    /// Create a fixed size array type that refers to a specific type.
    ///
    /// The created type will exist in the context that its element type exists in.
    pub fn array(&self, count: u64) -> &Type<array<T>> {
        unsafe { Type::from_ref(LLVMArrayType2(self.as_ptr(), count)) }
    }

    /// Create a fixed size array type that refers to a specific type.
    ///
    /// The created type will exist in the context that its element type exists in.
    pub fn array_sized<const N: u64>(&self) -> &Type<array_sized<T, N>> {
        unsafe { self.array(N).cast_unchecked() }
    }
}
