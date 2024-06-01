use std::marker::PhantomData;

use llvm_sys::core::*;
use llvm_sys::*;

use crate::opaque::Opaque;
use crate::type_tag::{any, TypeTag};
use crate::types::Type;

pub trait ArrayTypeTag: TypeTag {
    type ElementType: TypeTag;

    fn type_length(ty: &Type<Self>) -> u64;
}

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct array<T: TypeTag> {
    marker: PhantomData<fn(T) -> T>,
}

impl<T: TypeTag> TypeTag for array<T> {
    fn type_kind(_ty: &Type<Self>) -> LLVMTypeKind {
        LLVMTypeKind::LLVMArrayTypeKind
    }

    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        unsafe {
            if LLVMGetTypeKind(ty.as_ptr()) == LLVMTypeKind::LLVMArrayTypeKind {
                if T::type_cast(Type::from_ref(LLVMGetElementType(ty.as_ptr()))).is_some() {
                    return Some(ty.cast_unchecked());
                }
            }
            None
        }
    }
}

impl<T: TypeTag> ArrayTypeTag for array<T> {
    type ElementType = T;

    fn type_length(ty: &Type<Self>) -> u64 {
        unsafe { LLVMGetArrayLength2(ty.as_ptr()) }
    }
}

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct array_sized<T: TypeTag, const N: u64> {
    marker: PhantomData<fn(T) -> T>,
}

impl<T: TypeTag, const N: u64> TypeTag for array_sized<T, N> {
    fn type_kind(_ty: &Type<Self>) -> LLVMTypeKind {
        LLVMTypeKind::LLVMArrayTypeKind
    }

    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        let ty = array::<T>::type_cast(ty)?;
        if ty.length() == N {
            Some(unsafe { ty.cast_unchecked() })
        } else {
            None
        }
    }
}

impl<T: TypeTag, const N: u64> ArrayTypeTag for array_sized<T, N> {
    type ElementType = T;

    fn type_length(_ty: &Type<Self>) -> u64 {
        N
    }
}

impl<T: ArrayTypeTag> Type<T> {
    pub fn to_array_any(&self) -> &Type<array<any>> {
        unsafe { self.cast_unchecked() }
    }

    pub fn element_type(&self) -> &Type<T::ElementType> {
        unsafe { Type::from_ref(LLVMGetElementType(self.as_ptr())) }
    }

    pub fn length(&self) -> u64 {
        T::type_length(self)
    }
}
