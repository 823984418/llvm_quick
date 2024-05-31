use std::fmt::Formatter;

use llvm_sys::core::LLVMGetPointerAddressSpace;
use llvm_sys::LLVMTypeKind;

use crate::opaque::Opaque;
use crate::type_tag::{any, type_check_kind, InstanceTypeTag, TypeTag};
use crate::types::Type;

pub trait PtrTypeTag: TypeTag {
    fn type_address_space(ty: &Type<Self>) -> u32;
}

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct ptr {}

impl TypeTag for ptr {
    fn type_debug_fmt(ty: &Type<Self>, f: &mut Formatter<'_>) -> std::fmt::Result {
        let address_space = ty.address_space();
        if address_space == 0 {
            f.write_str("ptr")
        } else {
            write!(f, "ptr addrspace({})", address_space)
        }
    }

    fn type_kind(_ty: &Type<Self>) -> LLVMTypeKind {
        LLVMTypeKind::LLVMPointerTypeKind
    }

    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        unsafe { type_check_kind(ty, LLVMTypeKind::LLVMPointerTypeKind) }
    }
}

impl PtrTypeTag for ptr {
    fn type_address_space(ty: &Type<Self>) -> u32 {
        unsafe { LLVMGetPointerAddressSpace(ty.as_ptr()) }
    }
}

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct ptr_in<const ADDRESS_SPACE: u32 = 0> {}

impl<const ADDRESS_SPACE: u32> TypeTag for ptr_in<ADDRESS_SPACE> {
    fn type_debug_fmt(_ty: &Type<Self>, f: &mut Formatter<'_>) -> std::fmt::Result {
        if ADDRESS_SPACE == 0 {
            f.write_str("ptr")
        } else {
            write!(f, "ptr addrspace({})", ADDRESS_SPACE)
        }
    }

    fn type_kind(_ty: &Type<Self>) -> LLVMTypeKind {
        LLVMTypeKind::LLVMPointerTypeKind
    }

    fn type_cast(ty: &Type<any>) -> Option<&Type<Self>> {
        let ty = ptr::type_cast(ty)?;
        if ty.address_space() == ADDRESS_SPACE {
            Some(unsafe { ty.cast_unchecked() })
        } else {
            None
        }
    }
}

impl<const ADDRESS_SPACE: u32> InstanceTypeTag for ptr_in<ADDRESS_SPACE> {}

impl<const ADDRESS_SPACE: u32> PtrTypeTag for ptr_in<ADDRESS_SPACE> {
    fn type_address_space(_ty: &Type<Self>) -> u32 {
        ADDRESS_SPACE
    }
}

impl<T: PtrTypeTag> Type<T> {
    pub fn address_space(&self) -> u32 {
        T::type_address_space(self)
    }
}
