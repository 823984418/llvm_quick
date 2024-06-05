use llvm_sys::core::LLVMGetIntTypeWidth;
use llvm_sys::*;

use crate::core::type_tag::{any, type_check_kind, TypeTag};
use crate::Opaque;
use crate::Type;

pub trait IntTypeTag: TypeTag {
    fn type_get_int_width(ty: &Type<Self>) -> u32 {
        unsafe { LLVMGetIntTypeWidth(ty.as_raw()) }
    }
}

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct int_any {}

impl TypeTag for int_any {
    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        unsafe { type_check_kind(ty, LLVMTypeKind::LLVMIntegerTypeKind) }
    }
}

impl IntTypeTag for int_any {}

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct int<const N: u32> {}

impl<const N: u32> TypeTag for int<N> {
    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        let ty = int_any::type_cast(ty)?;
        if ty.get_int_width() == N {
            Some(unsafe { ty.cast_unchecked() })
        } else {
            None
        }
    }
}

impl<const N: u32> IntTypeTag for int<N> {}

impl<T: IntTypeTag> Type<T> {
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
