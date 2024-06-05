use std::marker::PhantomData;

use llvm_sys::core::*;
use llvm_sys::*;

use crate::core::type_tag::{any, TypeTag};
use crate::Opaque;
use crate::Type;

pub trait ArrayTypeTag: TypeTag {
    type ElementType: TypeTag;

    fn type_length(ty: &Type<Self>) -> u64 {
        unsafe { LLVMGetArrayLength2(ty.as_raw()) }
    }
}

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct array_unsized<T: TypeTag> {
    marker: PhantomData<fn(T) -> T>,
}

impl<T: TypeTag> TypeTag for array_unsized<T> {
    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        unsafe {
            if ty.get_kind() == LLVMTypeKind::LLVMArrayTypeKind
                && T::type_cast(Type::from_ref(LLVMGetElementType(ty.as_raw()))).is_some()
            {
                Some(ty.cast_unchecked())
            } else {
                None
            }
        }
    }
}

impl<T: TypeTag> ArrayTypeTag for array_unsized<T> {
    type ElementType = T;
}

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct array<T: TypeTag, const N: u64> {
    marker: PhantomData<fn(T) -> T>,
}

impl<T: TypeTag, const N: u64> TypeTag for array<T, N> {
    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        let ty = array_unsized::<T>::type_cast(ty)?;
        if ty.length() == N {
            Some(unsafe { ty.cast_unchecked() })
        } else {
            None
        }
    }
}

impl<T: TypeTag, const N: u64> ArrayTypeTag for array<T, N> {
    type ElementType = T;

    fn type_length(_ty: &Type<Self>) -> u64 {
        N
    }
}

impl<T: ArrayTypeTag> Type<T> {
    pub fn to_array_any(&self) -> &Type<array_unsized<any>> {
        unsafe { self.cast_unchecked() }
    }

    pub fn length(&self) -> u64 {
        T::type_length(self)
    }
}
