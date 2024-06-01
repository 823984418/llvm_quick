use std::fmt::Formatter;

use llvm_sys::core::LLVMGetIntTypeWidth;
use llvm_sys::LLVMTypeKind;

use crate::opaque::Opaque;
use crate::type_tag::{any, type_check_kind, TypeTag};
use crate::types::Type;

pub trait IntTypeTag: TypeTag {
    fn type_int_width(ty: &Type<Self>) -> u32;
}

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct int_any {}

impl TypeTag for int_any {
    fn type_debug_fmt(ty: &Type<Self>, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "i{}", ty.int_width())
    }

    fn type_kind(_ty: &Type<Self>) -> LLVMTypeKind {
        LLVMTypeKind::LLVMIntegerTypeKind
    }

    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        unsafe { type_check_kind(ty, LLVMTypeKind::LLVMIntegerTypeKind) }
    }
}

impl IntTypeTag for int_any {
    fn type_int_width(ty: &Type<Self>) -> u32 {
        unsafe { LLVMGetIntTypeWidth(ty.as_ptr()) }
    }
}

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct int<const N: u32> {}

impl<const N: u32> TypeTag for int<N> {
    fn type_debug_fmt(_ty: &Type<Self>, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "i{}", N)
    }

    fn type_kind(_ty: &Type<Self>) -> LLVMTypeKind {
        LLVMTypeKind::LLVMIntegerTypeKind
    }

    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        let ty = int_any::type_cast(ty)?;
        if ty.int_width() == N {
            Some(unsafe { ty.cast_unchecked() })
        } else {
            None
        }
    }
}

impl<const N: u32> IntTypeTag for int<N> {
    fn type_int_width(_ty: &Type<Self>) -> u32 {
        N
    }
}

impl<T: IntTypeTag> Type<T> {
    pub fn int_width(&self) -> u32 {
        T::type_int_width(self)
    }

    pub fn as_int_any(&self) -> &Type<int_any> {
        unsafe { self.cast_unchecked() }
    }
}

#[allow(non_camel_case_types)]
pub type int1 = int<1>;

#[allow(non_camel_case_types)]
pub type int8 = int<8>;

#[allow(non_camel_case_types)]
pub type int16 = int<16>;

#[allow(non_camel_case_types)]
pub type int32 = int<32>;

#[allow(non_camel_case_types)]
pub type int64 = int<64>;

#[allow(non_camel_case_types)]
pub type int128 = int<128>;
