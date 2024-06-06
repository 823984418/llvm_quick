use llvm_sys::core::*;
use llvm_sys::*;

use crate::core::type_tag::{any, type_check_kind, TypeTag};
use crate::{Opaque, Type};

pub trait PtrTypeTag: TypeTag {
    fn type_get_address_space(ty: &Type<Self>) -> u32 {
        unsafe { LLVMGetPointerAddressSpace(ty.as_raw()) }
    }
}

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct ptr_any {}

impl TypeTag for ptr_any {
    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        unsafe { type_check_kind(ty, LLVMTypeKind::LLVMPointerTypeKind) }
    }
}

impl PtrTypeTag for ptr_any {}

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct ptr<const ADDRESS_SPACE: u32 = 0> {}

impl<const ADDRESS_SPACE: u32> TypeTag for ptr<ADDRESS_SPACE> {
    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        let ty = ptr_any::type_cast(ty)?;
        if ty.get_address_space() == ADDRESS_SPACE {
            Some(unsafe { ty.cast_unchecked() })
        } else {
            None
        }
    }
}

impl<const ADDRESS_SPACE: u32> PtrTypeTag for ptr<ADDRESS_SPACE> {
    fn type_get_address_space(_ty: &Type<Self>) -> u32 {
        ADDRESS_SPACE
    }
}
